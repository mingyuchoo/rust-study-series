use rust_qdrant_vector_rag::app::AppContainer;
use rust_qdrant_vector_rag::config::AppConfig;
use std::env;
use std::time::Duration;
use tokio::time::timeout;

/// Common test configuration setup
pub fn setup_test_config(collection_name: &str, port: u16) -> AppConfig {
    cleanup_test_env();

    // Set test environment variables
    env::set_var("SERVER_HOST", "127.0.0.1");
    env::set_var("SERVER_PORT", port.to_string());
    env::set_var("SERVER_MAX_REQUEST_SIZE", "1048576");
    env::set_var("SERVER_TIMEOUT_SECONDS", "30");

    env::set_var("AZURE_OPENAI_ENDPOINT", "https://test.openai.azure.com");
    env::set_var("AZURE_OPENAI_API_KEY", "test-key-1234567890abcdef1234567890abcdef");
    env::set_var("AZURE_OPENAI_API_VERSION", "2024-02-01");
    env::set_var("AZURE_OPENAI_CHAT_DEPLOYMENT", "gpt-4");
    env::set_var("AZURE_OPENAI_EMBED_DEPLOYMENT", "text-embedding-3-large");
    env::set_var("AZURE_OPENAI_MAX_RETRIES", "3");
    env::set_var("AZURE_OPENAI_TIMEOUT_SECONDS", "60");

    env::set_var("QDRANT_URL", "http://localhost:6333");
    env::set_var("QDRANT_COLLECTION_NAME", collection_name);
    env::set_var("QDRANT_VECTOR_SIZE", "3072");
    env::set_var("QDRANT_TIMEOUT_SECONDS", "30");
    env::set_var("QDRANT_MAX_RETRIES", "3");

    env::set_var("SKIP_CONNECTIVITY_TEST", "true");

    AppConfig::from_env().expect("Failed to create test configuration")
}

/// Common test data
pub struct TestData {
    pub sample_markdown: &'static str,
    pub sample_filename: &'static str,
    pub test_questions: Vec<&'static str>,
    pub large_markdown: String,
}

impl TestData {
    pub fn new() -> Self {
        let sample_markdown = r#"# Integration Test Document

This is a comprehensive test document for integration testing of the RAG system.

## Overview

This document tests various aspects of the system:
- Markdown parsing and chunking
- Vector embedding generation
- Similarity search functionality
- Question answering capabilities

### System Components

The RAG system consists of several key components:

1. **Document Processing Pipeline**
   - Markdown parser using pulldown-cmark
   - Intelligent chunking with semantic boundaries
   - Metadata extraction and preservation

2. **Embedding Service**
   - Azure OpenAI text-embedding-3-large integration
   - Batch processing capabilities
   - Error handling and retry logic

3. **Vector Database**
   - Qdrant vector storage and retrieval
   - Collection management
   - Similarity search with configurable parameters

4. **RAG Pipeline**
   - Query embedding generation
   - Context retrieval and ranking
   - Answer generation with source attribution

## Technical Details

### Chunking Strategy

The system uses a sophisticated chunking strategy:
- Respects markdown structure (headers, paragraphs)
- Maintains semantic coherence
- Includes overlapping content for context preservation
- Configurable chunk sizes and overlap

### Search Algorithm

The similarity search uses:
- Cosine similarity for vector matching
- Configurable similarity thresholds
- Result ranking and filtering
- Source attribution and metadata preservation

## Code Examples

### Rust Implementation

```rust
use rust_qdrant_vector_rag::services::{DocumentService, RAGService};

async fn process_document(service: &DocumentService, content: String) -> Result<String, ServiceError> {
    service.process_document(content, "example.md".to_string()).await
}

async fn answer_question(service: &RAGService, question: String) -> Result<RAGResponse, ServiceError> {
    service.answer_question(question).await
}
```

### Configuration Example

```toml
[server]
host = "127.0.0.1"
port = 8080
max_request_size = 1048576

[azure_openai]
endpoint = "https://your-resource.openai.azure.com"
api_key = "your-api-key"
api_version = "2024-02-01"

[qdrant]
url = "http://localhost:6333"
collection_name = "documents"
vector_size = 3072
```

## Testing Scenarios

### Document Upload Tests
- Valid markdown files
- Invalid file formats
- Empty content
- Large documents
- Concurrent uploads

### Query Tests
- Simple questions
- Complex queries
- Empty questions
- Long questions
- Concurrent queries

### Error Handling Tests
- Network failures
- Service unavailability
- Invalid configurations
- Rate limiting
- Timeout scenarios

## Performance Considerations

The system is designed for:
- High throughput document processing
- Low latency query responses
- Efficient memory usage
- Scalable vector operations

### Benchmarks

Expected performance metrics:
- Document processing: < 5 seconds per MB
- Query response time: < 2 seconds
- Concurrent request handling: 100+ requests/second
- Memory usage: < 1GB for 10,000 documents

## Conclusion

This document provides comprehensive test coverage for the RAG system integration tests.
It includes various markdown elements, code examples, and technical details to ensure
thorough testing of all system components.

The integration tests verify:
- End-to-end document processing workflow
- Complete question-answering pipeline
- Error handling and edge cases
- Performance and scalability
- Data isolation and cleanup
"#;

        let large_markdown = (0 .. 5)
            .map(|i| format!("# Section {}\n\n{}\n\n", i + 1, sample_markdown))
            .collect::<Vec<_>>()
            .join("");

        Self {
            sample_markdown,
            sample_filename: "integration_test_document.md",
            test_questions: vec![
                "What is this document about?",
                "What are the main components of the RAG system?",
                "What chunking strategy does the system use?",
                "What programming language is used in the examples?",
                "How does the similarity search work?",
                "What are the expected performance metrics?",
                "What testing scenarios are covered?",
            ],
            large_markdown,
        }
    }

    pub fn get_large_document(&self) -> (&str, &str) { (&self.large_markdown, "large_integration_test_document.md") }
}

/// Test environment setup with error handling
pub async fn setup_test_environment(collection_name: &str, port: u16) -> Result<AppContainer, Box<dyn std::error::Error>> {
    let config = setup_test_config(collection_name, port);

    // Initialize app container with timeout
    let container = timeout(Duration::from_secs(30), AppContainer::new(config))
        .await
        .map_err(|_| "Timeout initializing app container")?
        .map_err(|e| format!("Failed to initialize app container: {}", e))?;

    Ok(container)
}

/// Cleanup test environment
pub async fn cleanup_test_environment(container: &AppContainer) -> Result<(), Box<dyn std::error::Error>> {
    // Perform any necessary cleanup
    // In a real implementation, you might want to:
    // - Clear the test collection
    // - Reset any test state
    // - Clean up temporary files

    tracing::info!("Cleaning up test environment for collection: {}", container.config.qdrant.collection_name);

    // For now, just verify the collection exists
    if container.vector_repository.collection_exists().await.unwrap_or(false) {
        tracing::info!("Test collection exists and is ready for cleanup");
    }

    Ok(())
}

/// Wait for service readiness
#[allow(dead_code)]
pub async fn wait_for_service_readiness(container: &AppContainer, max_attempts: u32) -> Result<(), Box<dyn std::error::Error>> {
    for attempt in 1 ..= max_attempts {
        match container.health_check().await {
            | Ok(status) =>
                if status.is_healthy() {
                    tracing::info!("Service is ready after {} attempts", attempt);
                    return Ok(());
                } else {
                    tracing::warn!("Service not healthy on attempt {}: {:?}", attempt, status);
                },
            | Err(e) => {
                tracing::warn!("Health check failed on attempt {}: {}", attempt, e);
            },
        }

        if attempt < max_attempts {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    Err("Service did not become ready within the specified attempts".into())
}

/// Cleanup function for tests
pub fn cleanup_test_env() {
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

/// Verify test isolation
pub async fn verify_test_isolation(container1: &AppContainer, container2: &AppContainer) -> Result<(), Box<dyn std::error::Error>> {
    // Verify that different test containers use different collections
    assert_ne!(
        container1.config.qdrant.collection_name, container2.config.qdrant.collection_name,
        "Test containers should use different collections for isolation"
    );

    // Verify that both collections can exist independently
    let exists1 = container1.vector_repository.collection_exists().await.unwrap_or(false);
    let exists2 = container2.vector_repository.collection_exists().await.unwrap_or(false);

    tracing::info!("Collection {} exists: {}", container1.config.qdrant.collection_name, exists1);
    tracing::info!("Collection {} exists: {}", container2.config.qdrant.collection_name, exists2);

    Ok(())
}

/// Performance measurement utilities
pub struct PerformanceMetrics {
    pub start_time: std::time::Instant,
    pub operation_name: String,
}

impl PerformanceMetrics {
    pub fn new(operation_name: String) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            operation_name,
        }
    }

    pub fn elapsed(&self) -> Duration { self.start_time.elapsed() }

    pub fn log_completion(&self) {
        let elapsed = self.elapsed();
        tracing::info!("Operation '{}' completed in {:?}", self.operation_name, elapsed);
    }

    pub fn assert_performance(&self, max_duration: Duration) {
        let elapsed = self.elapsed();
        assert!(
            elapsed <= max_duration,
            "Operation '{}' took {:?}, expected <= {:?}",
            self.operation_name,
            elapsed,
            max_duration
        );
    }
}

/// Test result aggregation
pub struct TestResults {
    pub total_tests: u32,
    pub passed_tests: u32,
    pub failed_tests: u32,
    pub skipped_tests: u32,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.total_tests += 1;
        match result {
            | TestResult::Passed => self.passed_tests += 1,
            | TestResult::Failed => self.failed_tests += 1,
            | TestResult::Skipped => self.skipped_tests += 1,
        }
    }

    pub fn success_rate(&self) -> f32 {
        if self.total_tests == 0 {
            0.0
        } else {
            self.passed_tests as f32 / self.total_tests as f32
        }
    }

    pub fn log_summary(&self) {
        tracing::info!(
            "Test Results: {}/{} passed ({:.1}%), {} failed, {} skipped",
            self.passed_tests,
            self.total_tests,
            self.success_rate() * 100.0,
            self.failed_tests,
            self.skipped_tests
        );
    }
}

pub enum TestResult {
    Passed,
    Failed,
    Skipped,
}
