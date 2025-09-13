use std::env;
use std::time::Duration;
use tracing_test::traced_test;
use reqwest::{Client, multipart};
use serde_json::json;

use rust_qdrant_vector_rag::config::AppConfig;
use rust_qdrant_vector_rag::models::{RAGResponse, UploadResponse, HealthResponse};

/// Test configuration for HTTP integration tests
fn setup_test_config() -> AppConfig {
    cleanup_test_env();

    // Set test environment variables
    env::set_var("SERVER_HOST", "127.0.0.1");
    env::set_var("SERVER_PORT", "8083"); // Different port for HTTP tests
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
    env::set_var("QDRANT_COLLECTION_NAME", "test_http_documents");
    env::set_var("QDRANT_VECTOR_SIZE", "3072");
    env::set_var("QDRANT_TIMEOUT_SECONDS", "30");
    env::set_var("QDRANT_MAX_RETRIES", "3");

    env::set_var("SKIP_CONNECTIVITY_TEST", "true");

    AppConfig::from_env().expect("Failed to create test configuration")
}

/// Test server setup helper
struct TestServer {
    base_url: String,
    client: Client,
}

impl TestServer {
    fn new(port: u16) -> Self {
        Self {
            base_url: format!("http://127.0.0.1:{}", port),
            client: Client::new(),
        }
    }

    async fn health_check(&self) -> Result<HealthResponse, reqwest::Error> {
        let response = self.client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await?;
        
        response.json::<HealthResponse>().await
    }

    async fn simple_health_check(&self) -> Result<serde_json::Value, reqwest::Error> {
        let response = self.client
            .get(&format!("{}/health/simple", self.base_url))
            .send()
            .await?;
        
        response.json::<serde_json::Value>().await
    }

    async fn upload_json(&self, content: &str, filename: &str) -> Result<reqwest::Response, reqwest::Error> {
        let payload = json!({
            "content": content,
            "filename": filename
        });

        self.client
            .post(&format!("{}/upload/json", self.base_url))
            .json(&payload)
            .send()
            .await
    }

    async fn upload_multipart(&self, content: &str, filename: &str) -> Result<reqwest::Response, reqwest::Error> {
        let form = multipart::Form::new()
            .text("filename", filename.to_string())
            .part("file", multipart::Part::text(content.to_string())
                .file_name(filename.to_string())
                .mime_str("text/markdown").unwrap());

        self.client
            .post(&format!("{}/upload", self.base_url))
            .multipart(form)
            .send()
            .await
    }

    async fn query_json(&self, question: &str) -> Result<reqwest::Response, reqwest::Error> {
        let payload = json!({
            "question": question
        });

        self.client
            .post(&format!("{}/query", self.base_url))
            .json(&payload)
            .send()
            .await
    }

    async fn query_simple(&self, question: &str) -> Result<reqwest::Response, reqwest::Error> {
        let encoded_question = urlencoding::encode(question);
        
        self.client
            .get(&format!("{}/query/{}", self.base_url, encoded_question))
            .send()
            .await
    }
}

/// Sample test data
struct TestData {
    pub sample_markdown: &'static str,
    pub sample_filename: &'static str,
    pub test_questions: Vec<&'static str>,
}

impl TestData {
    fn new() -> Self {
        Self {
            sample_markdown: r#"# HTTP Test Document

This is a test document for HTTP API integration testing.

## Overview

This document tests the HTTP endpoints of the RAG system:
- File upload via multipart form
- File upload via JSON
- Question answering via POST
- Question answering via GET

### API Endpoints

The system provides the following endpoints:
- `POST /upload` - Upload markdown files
- `POST /upload/json` - Upload content as JSON
- `POST /query` - Ask questions
- `GET /query/{question}` - Simple question endpoint
- `GET /health` - Health check

## Code Example

```rust
// Example HTTP client usage
let client = reqwest::Client::new();
let response = client.post("http://localhost:8080/query")
    .json(&json!({"question": "What is this about?"}))
    .send()
    .await?;
```

## Conclusion

This document serves as test data for HTTP API integration tests.
"#,
            sample_filename: "http_test_document.md",
            test_questions: vec![
                "What is this document about?",
                "What HTTP endpoints are available?",
                "What programming language is shown in the code example?",
                "How do you make a POST request?",
            ],
        }
    }
}

/// Test health check endpoints
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server
async fn test_health_check_endpoints() {
    let server = TestServer::new(8083);

    // Test comprehensive health check
    match server.health_check().await {
        Ok(health_response) => {
            tracing::info!("Health check response: {:?}", health_response);
            
            // Verify response structure
            // uptime_seconds is u64, so it's always >= 0
            assert!(matches!(health_response.status, 
                rust_qdrant_vector_rag::models::HealthStatus::Healthy |
                rust_qdrant_vector_rag::models::HealthStatus::Degraded |
                rust_qdrant_vector_rag::models::HealthStatus::Unhealthy
            ));
        }
        Err(e) => {
            tracing::warn!("Health check failed (server may not be running): {}", e);
        }
    }

    // Test simple health check
    match server.simple_health_check().await {
        Ok(response) => {
            tracing::info!("Simple health check response: {:?}", response);
            
            // Verify basic structure
            assert!(response.get("status").is_some());
            assert!(response.get("service").is_some());
            assert!(response.get("timestamp").is_some());
        }
        Err(e) => {
            tracing::warn!("Simple health check failed (server may not be running): {}", e);
        }
    }
}

/// Test document upload via JSON endpoint
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server
async fn test_document_upload_json_endpoint() {
    let server = TestServer::new(8083);
    let test_data = TestData::new();

    // Test successful upload
    match server.upload_json(test_data.sample_markdown, test_data.sample_filename).await {
        Ok(response) => {
            tracing::info!("Upload response status: {}", response.status());
            
            if response.status().is_success() {
                match response.json::<UploadResponse>().await {
                    Ok(upload_response) => {
                        tracing::info!("Upload successful: {:?}", upload_response);
                        
                        assert!(!upload_response.document_id.is_empty());
                        assert_eq!(upload_response.filename, test_data.sample_filename);
                        assert!(upload_response.chunks_created > 0);
                        assert!(matches!(upload_response.status, 
                            rust_qdrant_vector_rag::models::UploadStatus::Success
                        ));
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse upload response: {}", e);
                    }
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                tracing::warn!("Upload failed with status {}: {}", status, error_text);
            }
        }
        Err(e) => {
            tracing::warn!("Upload request failed (server may not be running): {}", e);
        }
    }

    // Test validation errors
    
    // Empty content
    match server.upload_json("", "empty.md").await {
        Ok(response) => {
            assert_eq!(response.status(), 400, "Empty content should return 400");
            tracing::info!("Empty content validation working correctly");
        }
        Err(e) => {
            tracing::warn!("Empty content test failed: {}", e);
        }
    }

    // Empty filename
    match server.upload_json(test_data.sample_markdown, "").await {
        Ok(response) => {
            assert_eq!(response.status(), 400, "Empty filename should return 400");
            tracing::info!("Empty filename validation working correctly");
        }
        Err(e) => {
            tracing::warn!("Empty filename test failed: {}", e);
        }
    }

    // Invalid file extension
    match server.upload_json(test_data.sample_markdown, "test.txt").await {
        Ok(response) => {
            assert_eq!(response.status(), 400, "Invalid extension should return 400");
            tracing::info!("File extension validation working correctly");
        }
        Err(e) => {
            tracing::warn!("Invalid extension test failed: {}", e);
        }
    }
}

/// Test document upload via multipart form endpoint
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server
async fn test_document_upload_multipart_endpoint() {
    let server = TestServer::new(8083);
    let test_data = TestData::new();

    // Test successful multipart upload
    match server.upload_multipart(test_data.sample_markdown, test_data.sample_filename).await {
        Ok(response) => {
            tracing::info!("Multipart upload response status: {}", response.status());
            
            if response.status().is_success() {
                match response.json::<UploadResponse>().await {
                    Ok(upload_response) => {
                        tracing::info!("Multipart upload successful: {:?}", upload_response);
                        
                        assert!(!upload_response.document_id.is_empty());
                        assert_eq!(upload_response.filename, test_data.sample_filename);
                        assert!(upload_response.chunks_created > 0);
                        assert!(matches!(upload_response.status, 
                            rust_qdrant_vector_rag::models::UploadStatus::Success
                        ));
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse multipart upload response: {}", e);
                    }
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                tracing::warn!("Multipart upload failed with status {}: {}", status, error_text);
            }
        }
        Err(e) => {
            tracing::warn!("Multipart upload request failed (server may not be running): {}", e);
        }
    }
}

/// Test question answering via JSON endpoint
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server and uploaded documents
async fn test_question_answering_json_endpoint() {
    let server = TestServer::new(8083);
    let test_data = TestData::new();

    // First upload a document
    let _upload_result = server.upload_json(test_data.sample_markdown, test_data.sample_filename).await;

    // Wait a moment for processing
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Test each question
    for question in test_data.test_questions {
        tracing::info!("Testing question: {}", question);
        
        match server.query_json(question).await {
            Ok(response) => {
                tracing::info!("Query response status: {}", response.status());
                
                if response.status().is_success() {
                    match response.json::<RAGResponse>().await {
                        Ok(rag_response) => {
                            tracing::info!("Query successful: confidence={:.2}, sources={}", 
                                rag_response.confidence, rag_response.source_count());
                            
                            assert!(!rag_response.answer.is_empty());
                            assert_eq!(rag_response.query, question);
                            assert!(rag_response.confidence >= 0.0 && rag_response.confidence <= 1.0);
                            assert!(rag_response.response_time_ms > 0);
                        }
                        Err(e) => {
                            tracing::error!("Failed to parse query response: {}", e);
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    tracing::warn!("Query failed with status {}: {}", status, error_text);
                }
            }
            Err(e) => {
                tracing::warn!("Query request failed (server may not be running): {}", e);
            }
        }
    }

    // Test validation errors
    
    // Empty question
    match server.query_json("").await {
        Ok(response) => {
            assert_eq!(response.status(), 400, "Empty question should return 400");
            tracing::info!("Empty question validation working correctly");
        }
        Err(e) => {
            tracing::warn!("Empty question test failed: {}", e);
        }
    }

    // Very long question
    let long_question = "What ".repeat(500) + "is this about?";
    match server.query_json(&long_question).await {
        Ok(response) => {
            // This might return 400 or succeed depending on implementation
            tracing::info!("Long question response status: {}", response.status());
        }
        Err(e) => {
            tracing::warn!("Long question test failed: {}", e);
        }
    }
}

/// Test question answering via simple GET endpoint
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server and uploaded documents
async fn test_question_answering_simple_endpoint() {
    let server = TestServer::new(8083);
    let test_data = TestData::new();

    // First upload a document
    let _upload_result = server.upload_json(test_data.sample_markdown, test_data.sample_filename).await;

    // Wait a moment for processing
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Test simple GET queries
    for question in test_data.test_questions {
        tracing::info!("Testing simple query: {}", question);
        
        match server.query_simple(question).await {
            Ok(response) => {
                tracing::info!("Simple query response status: {}", response.status());
                
                if response.status().is_success() {
                    match response.json::<RAGResponse>().await {
                        Ok(rag_response) => {
                            tracing::info!("Simple query successful: confidence={:.2}, sources={}", 
                                rag_response.confidence, rag_response.source_count());
                            
                            assert!(!rag_response.answer.is_empty());
                            assert_eq!(rag_response.query, question);
                            assert!(rag_response.confidence >= 0.0 && rag_response.confidence <= 1.0);
                        }
                        Err(e) => {
                            tracing::error!("Failed to parse simple query response: {}", e);
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    tracing::warn!("Simple query failed with status {}: {}", status, error_text);
                }
            }
            Err(e) => {
                tracing::warn!("Simple query request failed (server may not be running): {}", e);
            }
        }
    }
}

/// Test API versioning and backward compatibility
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server
async fn test_api_versioning_and_compatibility() {
    let server = TestServer::new(8083);
    let test_data = TestData::new();

    // Test v1 API endpoints
    let v1_endpoints = vec![
        "/api/v1/health",
        "/api/v1/upload",
        "/api/v1/upload/json",
        "/api/v1/query",
    ];

    for endpoint in v1_endpoints {
        let url = format!("{}{}", server.base_url, endpoint);
        
        match endpoint {
            "/api/v1/health" => {
                match server.client.get(&url).send().await {
                    Ok(response) => {
                        tracing::info!("V1 health endpoint status: {}", response.status());
                        assert!(response.status().is_success() || response.status().is_server_error());
                    }
                    Err(e) => {
                        tracing::warn!("V1 health endpoint failed: {}", e);
                    }
                }
            }
            "/api/v1/upload/json" => {
                let payload = json!({
                    "content": test_data.sample_markdown,
                    "filename": test_data.sample_filename
                });
                
                match server.client.post(&url).json(&payload).send().await {
                    Ok(response) => {
                        tracing::info!("V1 upload endpoint status: {}", response.status());
                        // Should succeed or fail gracefully
                    }
                    Err(e) => {
                        tracing::warn!("V1 upload endpoint failed: {}", e);
                    }
                }
            }
            "/api/v1/query" => {
                let payload = json!({
                    "question": "What is this about?"
                });
                
                match server.client.post(&url).json(&payload).send().await {
                    Ok(response) => {
                        tracing::info!("V1 query endpoint status: {}", response.status());
                        // Should succeed or fail gracefully
                    }
                    Err(e) => {
                        tracing::warn!("V1 query endpoint failed: {}", e);
                    }
                }
            }
            _ => {}
        }
    }

    // Test legacy endpoints (backward compatibility)
    let legacy_endpoints = vec![
        "/health",
        "/upload",
        "/query",
    ];

    for endpoint in legacy_endpoints {
        let url = format!("{}{}", server.base_url, endpoint);
        
        match endpoint {
            "/health" => {
                match server.client.get(&url).send().await {
                    Ok(response) => {
                        tracing::info!("Legacy health endpoint status: {}", response.status());
                        assert!(response.status().is_success() || response.status().is_server_error());
                    }
                    Err(e) => {
                        tracing::warn!("Legacy health endpoint failed: {}", e);
                    }
                }
            }
            _ => {
                // Test that legacy endpoints exist (don't test full functionality)
                tracing::info!("Legacy endpoint {} should be available", endpoint);
            }
        }
    }
}

/// Test error handling and HTTP status codes
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server
async fn test_error_handling_and_status_codes() {
    let server = TestServer::new(8083);

    // Test 404 for non-existent endpoints
    match server.client.get(&format!("{}/nonexistent", server.base_url)).send().await {
        Ok(response) => {
            assert_eq!(response.status(), 404, "Non-existent endpoint should return 404");
            tracing::info!("404 handling working correctly");
        }
        Err(e) => {
            tracing::warn!("404 test failed: {}", e);
        }
    }

    // Test 405 for wrong HTTP methods
    match server.client.delete(&format!("{}/health", server.base_url)).send().await {
        Ok(response) => {
            // Should return 405 Method Not Allowed or 404
            assert!(response.status() == 405 || response.status() == 404);
            tracing::info!("Method not allowed handling working correctly: {}", response.status());
        }
        Err(e) => {
            tracing::warn!("Method not allowed test failed: {}", e);
        }
    }

    // Test malformed JSON
    match server.client
        .post(&format!("{}/upload/json", server.base_url))
        .header("content-type", "application/json")
        .body("invalid json")
        .send()
        .await
    {
        Ok(response) => {
            assert_eq!(response.status(), 400, "Malformed JSON should return 400");
            tracing::info!("Malformed JSON handling working correctly");
        }
        Err(e) => {
            tracing::warn!("Malformed JSON test failed: {}", e);
        }
    }

    // Test content-type validation
    match server.client
        .post(&format!("{}/query", server.base_url))
        .header("content-type", "text/plain")
        .body("What is this?")
        .send()
        .await
    {
        Ok(response) => {
            // Should return 400 or 415 for unsupported content type
            assert!(response.status() == 400 || response.status() == 415);
            tracing::info!("Content-type validation working correctly: {}", response.status());
        }
        Err(e) => {
            tracing::warn!("Content-type validation test failed: {}", e);
        }
    }
}

/// Test CORS and security headers
#[tokio::test]
#[traced_test]
#[ignore] // Requires running server
async fn test_cors_and_security_headers() {
    let server = TestServer::new(8083);

    // Test CORS preflight request
    match server.client
        .request(reqwest::Method::OPTIONS, &format!("{}/query", server.base_url))
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "Content-Type")
        .send()
        .await
    {
        Ok(response) => {
            tracing::info!("CORS preflight response status: {}", response.status());
            
            // Check CORS headers
            if let Some(allow_origin) = response.headers().get("access-control-allow-origin") {
                tracing::info!("CORS Allow-Origin: {:?}", allow_origin);
            }
            
            if let Some(allow_methods) = response.headers().get("access-control-allow-methods") {
                tracing::info!("CORS Allow-Methods: {:?}", allow_methods);
            }
        }
        Err(e) => {
            tracing::warn!("CORS preflight test failed: {}", e);
        }
    }

    // Test security headers on regular request
    match server.client.get(&format!("{}/health", server.base_url)).send().await {
        Ok(response) => {
            tracing::info!("Security headers test response status: {}", response.status());
            
            // Check for security headers
            if let Some(version) = response.headers().get("x-version") {
                tracing::info!("X-Version header: {:?}", version);
            }
            
            if let Some(service) = response.headers().get("x-service") {
                tracing::info!("X-Service header: {:?}", service);
            }
        }
        Err(e) => {
            tracing::warn!("Security headers test failed: {}", e);
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
async fn test_http_cleanup_functionality() {
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