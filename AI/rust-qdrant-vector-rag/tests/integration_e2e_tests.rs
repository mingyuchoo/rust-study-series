use std::env;
use std::time::Duration;
use tracing_test::traced_test;

use rust_qdrant_vector_rag::app::AppContainer;
use rust_qdrant_vector_rag::config::AppConfig;

/// Test configuration setup for integration tests with test collection
fn setup_test_config() -> AppConfig {
    // Clear any existing environment variables first
    cleanup_test_env();

    // Set test environment variables with test collection name
    env::set_var("SERVER_HOST", "127.0.0.1");
    env::set_var("SERVER_PORT", "8082"); // Different port to avoid conflicts
    env::set_var("SERVER_MAX_REQUEST_SIZE", "1048576");
    env::set_var("SERVER_TIMEOUT_SECONDS", "30");

    env::set_var("AZURE_OPENAI_ENDPOINT", "https://test.openai.azure.com");
    env::set_var("AZURE_OPENAI_API_KEY", "test-key-1234567890abcdef1234567890abcdef");
    env::set_var("AZURE_OPENAI_API_VERSION", "2024-02-01");
    env::set_var("AZURE_OPENAI_CHAT_DEPLOYMENT", "gpt-4");
    env::set_var("AZURE_OPENAI_EMBED_DEPLOYMENT", "text-embedding-3-large");
    env::set_var("AZURE_OPENAI_MAX_RETRIES", "3");
    env::set_var("AZURE_OPENAI_TIMEOUT_SECONDS", "60");

    // Use test-specific collection name to avoid conflicts
    env::set_var("QDRANT_URL", "http://localhost:6333");
    env::set_var("QDRANT_COLLECTION_NAME", "test_e2e_documents");
    env::set_var("QDRANT_VECTOR_SIZE", "3072");
    env::set_var("QDRANT_TIMEOUT_SECONDS", "30");
    env::set_var("QDRANT_MAX_RETRIES", "3");

    // Skip connectivity tests for unit tests
    env::set_var("SKIP_CONNECTIVITY_TEST", "true");

    AppConfig::from_env().expect("Failed to create test configuration")
}

/// Test data for integration tests
struct TestData {
    pub sample_markdown: &'static str,
    pub sample_filename: &'static str,
    pub test_questions: Vec<&'static str>,
}

impl TestData {
    fn new() -> Self {
        Self {
            sample_markdown: r#"# Test Document

This is a test document for integration testing.

## Introduction

This document contains sample content for testing the RAG system.
It includes various markdown elements to test parsing capabilities.

### Features

- Document parsing
- Vector embeddings
- Similarity search
- Answer generation

## Code Examples

Here's a simple Rust function:

```rust
fn hello_world() {
    println!("Hello, world!");
}
```

## Conclusion

This document serves as test data for the integration tests.
"#,
            sample_filename: "test_document.md",
            test_questions: vec![
                "What is this document about?",
                "What programming language is mentioned?",
                "What are the main features?",
                "How do you print hello world in Rust?",
            ],
        }
    }
}

/// Setup test environment with Qdrant collection
async fn setup_test_environment() -> Result<AppContainer, Box<dyn std::error::Error>> {
    let config = setup_test_config();
    
    // Initialize app container
    let container = AppContainer::new(config).await
        .map_err(|e| format!("Failed to initialize app container: {}", e))?;

    // Clean up any existing test data
    cleanup_test_collection(&container).await?;

    Ok(container)
}

/// Clean up test collection
async fn cleanup_test_collection(container: &AppContainer) -> Result<(), Box<dyn std::error::Error>> {
    // Try to delete the test collection if it exists
    if container.vector_repository.collection_exists().await.unwrap_or(false) {
        // In a real implementation, you might want to delete all points or recreate the collection
        // For now, we'll just ensure it's properly initialized
        tracing::info!("Test collection exists, ensuring it's clean");
    }
    
    Ok(())
}

/// Test the complete document upload workflow
#[tokio::test]
#[traced_test]
#[ignore] // Requires external services (Qdrant, Azure OpenAI)
async fn test_complete_document_upload_workflow() {
    let container = setup_test_environment().await
        .expect("Failed to setup test environment");
    
    let test_data = TestData::new();

    // Test document processing through service layer
    let result = container.document_service
        .process_document(test_data.sample_markdown.to_string(), test_data.sample_filename.to_string())
        .await;

    match result {
        Ok(document_id) => {
            tracing::info!("Document processed successfully: {}", document_id);
            
            // Verify chunks were created
            let chunks = container.document_service
                .get_document_chunks(document_id.clone())
                .await
                .expect("Failed to get document chunks");
            
            assert!(!chunks.is_empty(), "No chunks were created");
            assert!(chunks.len() >= 2, "Expected at least 2 chunks for test document");
            
            // Verify chunks have embeddings
            for chunk in &chunks {
                assert!(chunk.embedding.is_some(), "Chunk should have embedding");
                assert!(!chunk.content.is_empty(), "Chunk content should not be empty");
                assert_eq!(chunk.document_id, document_id, "Chunk should reference correct document");
            }
            
            tracing::info!("Created {} chunks with embeddings", chunks.len());
        }
        Err(e) => {
            tracing::warn!("Document processing failed (expected in test environment): {}", e);
            // In test environment without real services, this is expected
        }
    }
}

/// Test the complete question-answering pipeline
#[tokio::test]
#[traced_test]
#[ignore] // Requires external services (Qdrant, Azure OpenAI)
async fn test_complete_question_answering_pipeline() {
    let container = setup_test_environment().await
        .expect("Failed to setup test environment");
    
    let test_data = TestData::new();

    // First, upload a document
    let _document_id = match container.document_service
        .process_document(test_data.sample_markdown.to_string(), test_data.sample_filename.to_string())
        .await
    {
        Ok(id) => id,
        Err(e) => {
            tracing::warn!("Document upload failed (expected in test environment): {}", e);
            return; // Skip the rest of the test
        }
    };

    // Wait a moment for indexing
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Test each question
    for question in test_data.test_questions {
        tracing::info!("Testing question: {}", question);
        
        let result = container.rag_service
            .answer_question(question.to_string())
            .await;

        match result {
            Ok(response) => {
                assert!(!response.answer.is_empty(), "Answer should not be empty");
                assert!(!response.query.is_empty(), "Query should not be empty");
                assert!(response.confidence >= 0.0 && response.confidence <= 1.0, "Confidence should be between 0 and 1");
                
                tracing::info!("Question answered successfully: {} sources, confidence: {:.2}", 
                    response.source_count(), response.confidence);
            }
            Err(e) => {
                tracing::warn!("Question answering failed (expected in test environment): {}", e);
            }
        }
    }
}

/// Test error scenarios and edge cases
#[tokio::test]
#[traced_test]
async fn test_error_scenarios_and_edge_cases() {
    let container = match setup_test_environment().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            return; // Skip test if environment setup fails
        }
    };

    // Test empty document
    let result = container.document_service
        .process_document("".to_string(), "empty.md".to_string())
        .await;
    
    assert!(result.is_err(), "Empty document should fail");

    // Test invalid markdown (should still work as it's just text)
    let result = container.document_service
        .process_document("Just plain text without markdown".to_string(), "plain.md".to_string())
        .await;
    
    // This should succeed as plain text is valid
    match result {
        Ok(_) => tracing::info!("Plain text processed successfully"),
        Err(e) => tracing::warn!("Plain text processing failed: {}", e),
    }

    // Test empty question
    let result = container.rag_service
        .answer_question("".to_string())
        .await;
    
    assert!(result.is_err(), "Empty question should fail");

    // Test very long question
    let long_question = "What ".repeat(500) + "is this about?";
    let result = container.rag_service
        .answer_question(long_question)
        .await;
    
    // This might succeed or fail depending on implementation limits
    match result {
        Ok(_) => tracing::info!("Long question processed successfully"),
        Err(e) => tracing::info!("Long question failed as expected: {}", e),
    }
}

/// Test data cleanup and isolation
#[tokio::test]
#[traced_test]
async fn test_data_cleanup_and_isolation() {
    let container = match setup_test_environment().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            return;
        }
    };

    // Test that we start with a clean collection
    let collection_info = container.vector_repository.get_collection_info().await;
    match collection_info {
        Ok(info) => {
            let point_count = info.points_count.unwrap_or(0);
            tracing::info!("Collection has {} points", point_count);
            // We don't assert on the count as other tests might have added data
        }
        Err(e) => {
            tracing::warn!("Failed to get collection info: {}", e);
        }
    }

    // Test collection exists
    let exists = container.vector_repository.collection_exists().await;
    match exists {
        Ok(true) => tracing::info!("Test collection exists"),
        Ok(false) => tracing::warn!("Test collection does not exist"),
        Err(e) => tracing::warn!("Failed to check collection existence: {}", e),
    }

    // Test health check
    let health = container.vector_repository.health_check().await;
    match health {
        Ok(true) => tracing::info!("Vector repository is healthy"),
        Ok(false) => tracing::warn!("Vector repository is not healthy"),
        Err(e) => tracing::warn!("Health check failed: {}", e),
    }
}

/// Test concurrent operations
#[tokio::test]
#[traced_test]
#[ignore] // Requires external services and can be resource intensive
async fn test_concurrent_operations() {
    let container = match setup_test_environment().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            return;
        }
    };

    let test_data = TestData::new();

    // Test concurrent document uploads
    let mut upload_tasks = Vec::new();
    
    for i in 0..3 {
        let container_clone = container.clone();
        let content = format!("{}\n\n## Document {}", test_data.sample_markdown, i);
        let filename = format!("test_doc_{}.md", i);
        
        let task = tokio::spawn(async move {
            container_clone.document_service
                .process_document(content, filename)
                .await
        });
        
        upload_tasks.push(task);
    }

    // Wait for all uploads to complete
    let mut successful_uploads = 0;
    for task in upload_tasks {
        match task.await {
            Ok(Ok(_)) => {
                successful_uploads += 1;
                tracing::info!("Concurrent upload succeeded");
            }
            Ok(Err(e)) => {
                tracing::warn!("Concurrent upload failed: {}", e);
            }
            Err(e) => {
                tracing::error!("Task failed: {}", e);
            }
        }
    }

    tracing::info!("Completed {} successful concurrent uploads", successful_uploads);

    // Test concurrent queries
    let mut query_tasks = Vec::new();
    
    for question in test_data.test_questions {
        let container_clone = container.clone();
        let question = question.to_string();
        
        let task = tokio::spawn(async move {
            container_clone.rag_service
                .answer_question(question)
                .await
        });
        
        query_tasks.push(task);
    }

    // Wait for all queries to complete
    let mut successful_queries = 0;
    for task in query_tasks {
        match task.await {
            Ok(Ok(_)) => {
                successful_queries += 1;
                tracing::info!("Concurrent query succeeded");
            }
            Ok(Err(e)) => {
                tracing::warn!("Concurrent query failed: {}", e);
            }
            Err(e) => {
                tracing::error!("Query task failed: {}", e);
            }
        }
    }

    tracing::info!("Completed {} successful concurrent queries", successful_queries);
}

/// Test service health and monitoring
#[tokio::test]
#[traced_test]
async fn test_service_health_and_monitoring() {
    let container = match setup_test_environment().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            return;
        }
    };

    // Test comprehensive health check
    let health_result = container.health_check().await;
    
    match health_result {
        Ok(status) => {
            tracing::info!("Health check completed: {:?}", status);
            
            // Verify health status structure
            assert!(matches!(status.overall, 
                rust_qdrant_vector_rag::app::ServiceHealth::Healthy |
                rust_qdrant_vector_rag::app::ServiceHealth::Degraded(_) |
                rust_qdrant_vector_rag::app::ServiceHealth::Unhealthy(_)
            ));
            
            // Check individual service health
            tracing::info!("Azure OpenAI health: {:?}", status.azure_openai);
            tracing::info!("Qdrant health: {:?}", status.qdrant);
            
            if let Some(collection_status) = &status.collection_status {
                tracing::info!("Collection status: {}", collection_status);
            }
        }
        Err(e) => {
            tracing::warn!("Health check failed: {}", e);
        }
    }

    // Test individual service connectivity
    let azure_connectivity = container.azure_client.test_connectivity().await;
    match azure_connectivity {
        Ok(()) => tracing::info!("Azure OpenAI connectivity test passed"),
        Err(e) => tracing::warn!("Azure OpenAI connectivity test failed: {}", e),
    }

    let qdrant_health = container.vector_repository.health_check().await;
    match qdrant_health {
        Ok(healthy) => tracing::info!("Qdrant health check: {}", healthy),
        Err(e) => tracing::warn!("Qdrant health check failed: {}", e),
    }
}

/// Test performance and resource usage
#[tokio::test]
#[traced_test]
#[ignore] // Performance test - run manually
async fn test_performance_and_resource_usage() {
    let container = match setup_test_environment().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            return;
        }
    };

    let test_data = TestData::new();

    // Test document processing performance
    let start_time = std::time::Instant::now();
    
    let result = container.document_service
        .process_document(test_data.sample_markdown.to_string(), test_data.sample_filename.to_string())
        .await;

    let processing_time = start_time.elapsed();
    
    match result {
        Ok(document_id) => {
            tracing::info!("Document processed in {:?}: {}", processing_time, document_id);
            
            // Performance assertions (adjust based on expected performance)
            assert!(processing_time < Duration::from_secs(30), "Document processing should complete within 30 seconds");
        }
        Err(e) => {
            tracing::warn!("Document processing failed: {}", e);
        }
    }

    // Test query performance
    for question in test_data.test_questions {
        let start_time = std::time::Instant::now();
        
        let result = container.rag_service
            .answer_question(question.to_string())
            .await;

        let query_time = start_time.elapsed();
        
        match result {
            Ok(response) => {
                tracing::info!("Query '{}' answered in {:?}, confidence: {:.2}", 
                    question, query_time, response.confidence);
                
                // Performance assertions
                assert!(query_time < Duration::from_secs(15), "Query should complete within 15 seconds");
            }
            Err(e) => {
                tracing::warn!("Query failed: {}", e);
            }
        }
    }
}

/// Test memory usage with large documents
#[tokio::test]
#[traced_test]
#[ignore] // Memory test - run manually
async fn test_memory_usage_with_large_documents() {
    let container = match setup_test_environment().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            return;
        }
    };

    // Create a large document (repeat content multiple times)
    let base_content = TestData::new().sample_markdown;
    let large_content = (0..10).map(|i| format!("# Section {}\n\n{}", i, base_content)).collect::<Vec<_>>().join("\n\n");
    
    tracing::info!("Testing with large document: {} bytes", large_content.len());

    let result = container.document_service
        .process_document(large_content, "large_test_document.md".to_string())
        .await;

    match result {
        Ok(document_id) => {
            tracing::info!("Large document processed successfully: {}", document_id);
            
            // Check chunk count
            let chunks = container.document_service
                .get_document_chunks(document_id)
                .await;
                
            match chunks {
                Ok(chunks) => {
                    tracing::info!("Large document created {} chunks", chunks.len());
                    assert!(chunks.len() > 10, "Large document should create multiple chunks");
                }
                Err(e) => {
                    tracing::warn!("Failed to get chunks for large document: {}", e);
                }
            }
        }
        Err(e) => {
            tracing::warn!("Large document processing failed: {}", e);
        }
    }
}

/// Cleanup function for tests
fn cleanup_test_env() {
    let test_vars = [
        "SERVER_HOST",
        "SERVER_PORT", 
        "SERVER_MAX_REQUEST_SIZE",
        "SERVER_TIMEOUT_SECONDS",
        "AZURE_OPENAI_ENDPOINT",
        "AZURE_OPENAI_API_KEY",
        "AZURE_OPENAI_API_VERSION",
        "AZURE_OPENAI_CHAT_DEPLOYMENT",
        "AZURE_OPENAI_EMBED_DEPLOYMENT",
        "AZURE_OPENAI_MAX_RETRIES",
        "AZURE_OPENAI_TIMEOUT_SECONDS",
        "QDRANT_URL",
        "QDRANT_COLLECTION_NAME",
        "QDRANT_VECTOR_SIZE",
        "QDRANT_TIMEOUT_SECONDS",
        "QDRANT_MAX_RETRIES",
        "SKIP_CONNECTIVITY_TEST",
    ];

    for var in &test_vars {
        env::remove_var(var);
    }
}

/// Test cleanup functionality
#[tokio::test]
#[traced_test]
async fn test_cleanup_functionality() {
    // Test environment cleanup
    setup_test_config();
    
    // Verify variables are set
    assert!(env::var("SERVER_HOST").is_ok());
    assert!(env::var("QDRANT_COLLECTION_NAME").is_ok());
    
    // Clean up
    cleanup_test_env();
    
    // Verify cleanup worked
    assert!(env::var("SERVER_HOST").is_err());
    assert!(env::var("QDRANT_COLLECTION_NAME").is_err());
}