use crate::handlers::{health_handler, query_handler, upload_handler};
use crate::models::{RAGResponse, ServiceError};
use crate::services::{DocumentService, RAGService};
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

/// Mock document service for testing error scenarios
struct MockDocumentService {
    should_fail: bool,
    error_type: Option<ServiceError>,
}

impl MockDocumentService {
    fn new() -> Self {
        Self {
            should_fail: false,
            error_type: None,
        }
    }

    fn with_error(error: ServiceError) -> Self {
        Self {
            should_fail: true,
            error_type: Some(error),
        }
    }
}

#[async_trait]
impl DocumentService for MockDocumentService {
    async fn process_document(&self, _content: String, _filename: String) -> Result<String, ServiceError> {
        if self.should_fail {
            Err(self.error_type.clone().unwrap_or_else(|| ServiceError::internal("Mock error")))
        } else {
            Ok("test-document-id".to_string())
        }
    }

    async fn get_document_chunks(&self, _doc_id: String) -> Result<Vec<crate::models::DocumentChunk>, ServiceError> {
        if self.should_fail {
            Err(self.error_type.clone().unwrap_or_else(|| ServiceError::internal("Mock error")))
        } else {
            Ok(vec![])
        }
    }
}

/// Mock RAG service for testing error scenarios
struct MockRAGService {
    should_fail: bool,
    error_type: Option<ServiceError>,
}

impl MockRAGService {
    fn new() -> Self {
        Self {
            should_fail: false,
            error_type: None,
        }
    }

    fn with_error(error: ServiceError) -> Self {
        Self {
            should_fail: true,
            error_type: Some(error),
        }
    }
}

#[async_trait]
impl RAGService for MockRAGService {
    async fn answer_question(&self, question: String) -> Result<RAGResponse, ServiceError> {
        if self.should_fail {
            Err(self.error_type.clone().unwrap_or_else(|| ServiceError::internal("Mock error")))
        } else {
            Ok(RAGResponse::new("Mock answer".to_string(), vec![], 0.8, question, 100))
        }
    }

    async fn answer_question_with_config(&self, question: String, _config: crate::services::rag::RAGConfig) -> Result<RAGResponse, ServiceError> {
        self.answer_question(question).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::ErrorHandlerMiddleware;
    use actix_web::{App, test, web};

    #[actix_web::test]
    async fn test_upload_handler_document_processing_error() {
        let mock_service = Arc::new(MockDocumentService::with_error(ServiceError::document_processing("Failed to parse markdown")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/upload", web::post().to(upload_handler)),
        )
        .await;

        // Create a multipart form with a markdown file
        let boundary = "----formdata-test-boundary";
        let body = format!(
            "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.md\"\r\nContent-Type: text/markdown\r\n\r\n# Test Document\n\nThis is test content.\r\n--{}--\r\n",
            boundary, boundary
        );

        let req = test::TestRequest::post()
            .uri("/upload")
            .insert_header(("content-type", format!("multipart/form-data; boundary={}", boundary)))
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Document processing failed"));
        assert_eq!(body["status"], 500);
        assert_eq!(body["retryable"], false);
    }

    #[actix_web::test]
    async fn test_upload_handler_validation_error() {
        let mock_service = Arc::new(MockDocumentService::with_error(ServiceError::validation("Document content cannot be empty")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/upload", web::post().to(upload_handler)),
        )
        .await;

        let boundary = "----formdata-test-boundary";
        let body = format!(
            "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"empty.md\"\r\nContent-Type: text/markdown\r\n\r\n\r\n--{}--\r\n",
            boundary, boundary
        );

        let req = test::TestRequest::post()
            .uri("/upload")
            .insert_header(("content-type", format!("multipart/form-data; boundary={}", boundary)))
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Document content cannot be empty"));
        assert_eq!(body["status"], 400);
        assert_eq!(body["retryable"], false);
    }

    #[actix_web::test]
    async fn test_upload_handler_rate_limit_error() {
        let mock_service = Arc::new(MockDocumentService::with_error(ServiceError::rate_limit("Embedding API quota exceeded")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/upload", web::post().to(upload_handler)),
        )
        .await;

        let boundary = "----formdata-test-boundary";
        let body = format!(
            "--{}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.md\"\r\nContent-Type: text/markdown\r\n\r\n# Test\n\nContent\r\n--{}--\r\n",
            boundary, boundary
        );

        let req = test::TestRequest::post()
            .uri("/upload")
            .insert_header(("content-type", format!("multipart/form-data; boundary={}", boundary)))
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 429);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Rate limit exceeded"));
        assert_eq!(body["status"], 429);
        assert_eq!(body["retryable"], true);
    }

    #[actix_web::test]
    async fn test_query_handler_embedding_error() {
        let mock_service = Arc::new(MockRAGService::with_error(ServiceError::embedding_generation(
            "Failed to generate question embedding",
        )));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/query")
            .set_json(&json!({
                "question": "What is the answer to my question?"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 502);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Embedding generation failed"));
        assert_eq!(body["status"], 502);
        assert_eq!(body["retryable"], true);
    }

    #[actix_web::test]
    async fn test_query_handler_vector_search_error() {
        let mock_service = Arc::new(MockRAGService::with_error(ServiceError::vector_search("Qdrant connection timeout")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/query")
            .set_json(&json!({
                "question": "What is the answer?"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 502);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Vector search failed"));
        assert_eq!(body["status"], 502);
        assert_eq!(body["retryable"], true);
    }

    #[actix_web::test]
    async fn test_query_handler_validation_error() {
        let mock_service = Arc::new(MockRAGService::with_error(ServiceError::validation("Question cannot be empty")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/query")
            .set_json(&json!({
                "question": ""
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Question cannot be empty"));
        assert_eq!(body["status"], 400);
        assert_eq!(body["retryable"], false);
    }

    #[actix_web::test]
    async fn test_query_handler_external_api_error() {
        let mock_service = Arc::new(MockRAGService::with_error(ServiceError::external_api("Azure OpenAI service unavailable")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/query")
            .set_json(&json!({
                "question": "What is the answer?"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 502);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("External API error"));
        assert_eq!(body["message"], "External service unavailable");
        assert_eq!(body["status"], 502);
        assert_eq!(body["retryable"], true);
    }

    #[actix_web::test]
    async fn test_health_handler_service_errors() {
        // Test health handler with various service states
        let healthy_service = Arc::new(MockDocumentService::new());
        let unhealthy_service = Arc::new(MockDocumentService::with_error(ServiceError::database("Database connection failed")));

        // Test healthy service
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(healthy_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/health", web::get().to(health_handler)),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // Test unhealthy service
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(unhealthy_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/health", web::get().to(health_handler)),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        // Health endpoint should still return 200 but indicate unhealthy status
        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "unhealthy");
    }

    #[actix_web::test]
    async fn test_malformed_request_handling() {
        let mock_service = Arc::new(MockRAGService::new());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        // Test malformed JSON
        let req = test::TestRequest::post()
            .uri("/query")
            .insert_header(("content-type", "application/json"))
            .set_payload("{ invalid json }")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Invalid JSON"));
        assert_eq!(body["status"], 400);
        assert_eq!(body["retryable"], false);
    }

    #[actix_web::test]
    async fn test_request_timeout_handling() {
        // This test would require a mock service that simulates timeouts
        // For now, we'll test the error structure
        let timeout_service = Arc::new(MockRAGService::with_error(ServiceError::network("Operation timed out after 60 seconds")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(timeout_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/query")
            .set_json(&json!({
                "question": "What is the answer?"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 503);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("timed out"));
        assert_eq!(body["status"], 503);
        assert_eq!(body["retryable"], true);
    }

    #[actix_web::test]
    async fn test_error_response_headers() {
        let mock_service = Arc::new(MockRAGService::with_error(ServiceError::rate_limit("Rate limit exceeded")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/query")
            .set_json(&json!({
                "question": "What is the answer?"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 429);

        // Check that request ID header is present
        assert!(resp.headers().contains_key("x-request-id"));

        // For rate limit errors, we might want to add Retry-After header
        // This is a placeholder for future enhancement
    }

    #[actix_web::test]
    async fn test_sequential_error_handling() {
        let mock_service = Arc::new(MockRAGService::with_error(ServiceError::external_api("Service temporarily unavailable")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        // Test multiple sequential requests
        for i in 0 .. 5 {
            let req = test::TestRequest::post()
                .uri("/query")
                .set_json(&json!({
                    "question": format!("Question {}", i)
                }))
                .to_request();

            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), 502); // All should return the same error status
        }
    }

    #[actix_web::test]
    async fn test_error_context_in_logs() {
        // This test verifies that error context is properly structured for logging
        // In a real implementation, you would capture log output and verify it

        let mock_service = Arc::new(MockRAGService::with_error(ServiceError::embedding_generation("Azure OpenAI API error")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .wrap(ErrorHandlerMiddleware)
                .route("/query", web::post().to(query_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/query")
            .set_json(&json!({
                "question": "Test question"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 502);

        // In a real test, you would verify that the logs contain:
        // - request_id
        // - error_context with category, severity, retryable status
        // - structured error information
    }
}
