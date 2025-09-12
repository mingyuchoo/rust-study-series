use actix_web::{App, test, web};
use serde_json::json;

use crate::clients::AzureOpenAIClient;
use crate::config::AppConfig;
use crate::handlers::{health_handler, query_handler, simple_health_handler, upload_json_handler};

/// Helper function to create test app configuration
fn create_test_config() -> AppConfig {
    // Set test environment variables
    std::env::set_var("AZURE_OPENAI_ENDPOINT", "https://test.openai.azure.com");
    std::env::set_var("AZURE_OPENAI_API_KEY", "test-key-12345678901234567890123456789012");
    std::env::set_var("AZURE_OPENAI_CHAT_DEPLOYMENT", "gpt-4");
    std::env::set_var("AZURE_OPENAI_EMBED_DEPLOYMENT", "text-embedding-3-large");
    std::env::set_var("QDRANT_URL", "http://localhost:6333");

    AppConfig::from_env().expect("Failed to create test config")
}

/// Helper function to create test Azure OpenAI client
fn create_test_azure_client() -> AzureOpenAIClient {
    let config = create_test_config();
    AzureOpenAIClient::new(config.azure_openai).expect("Failed to create test Azure client")
}

#[actix_web::test]
async fn test_simple_health_handler() {
    let app = test::init_service(App::new().route("/health/simple", web::get().to(simple_health_handler))).await;

    let req = test::TestRequest::get().uri("/health/simple").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "rust-qdrant-vector-rag");
}

#[actix_web::test]
async fn test_health_handler() {
    let config = create_test_config();
    let azure_client = create_test_azure_client();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config))
            .app_data(web::Data::new(azure_client))
            .route("/health", web::get().to(health_handler)),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_service(&app, req).await;
    // Health check might fail due to external dependencies, but should return a response
    assert!(resp.status().as_u16() == 200 || resp.status().as_u16() == 503);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("status").is_some());
    assert!(body.get("services").is_some());
}

#[actix_web::test]
async fn test_upload_json_handler_validation() {
    let config = create_test_config();
    let azure_client = create_test_azure_client();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config))
            .app_data(web::Data::new(azure_client))
            .route("/upload/json", web::post().to(upload_json_handler)),
    )
    .await;

    // Test empty filename
    let req = test::TestRequest::post()
        .uri("/upload/json")
        .set_json(&json!({
            "filename": "",
            "content": "# Test content"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    // Test empty content
    let req = test::TestRequest::post()
        .uri("/upload/json")
        .set_json(&json!({
            "filename": "test.md",
            "content": ""
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    // Test invalid file extension
    let req = test::TestRequest::post()
        .uri("/upload/json")
        .set_json(&json!({
            "filename": "test.txt",
            "content": "# Test content"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_query_handler_validation() {
    let config = create_test_config();
    let azure_client = create_test_azure_client();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config))
            .app_data(web::Data::new(azure_client))
            .route("/query", web::post().to(query_handler)),
    )
    .await;

    // Test empty question
    let req = test::TestRequest::post()
        .uri("/query")
        .set_json(&json!({
            "question": ""
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    // Test question too long
    let long_question = "a".repeat(1001);
    let req = test::TestRequest::post()
        .uri("/query")
        .set_json(&json!({
            "question": long_question
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_query_handler_with_config() {
    let config = create_test_config();
    let azure_client = create_test_azure_client();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config))
            .app_data(web::Data::new(azure_client))
            .route("/query", web::post().to(query_handler)),
    )
    .await;

    // Test with custom config
    let req = test::TestRequest::post()
        .uri("/query")
        .set_json(&json!({
            "question": "What is the meaning of life?",
            "config": {
                "max_chunks": 3,
                "similarity_threshold": 0.8,
                "temperature": 0.5
            }
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    // This will likely fail due to missing vector data, but should validate the request structure
    // The important thing is that it doesn't fail with a 400 validation error
    assert!(resp.status().as_u16() != 400);
}
