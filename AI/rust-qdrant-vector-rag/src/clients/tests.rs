use super::azure_openai::*;
use crate::config::AzureOpenAIConfig;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Mock HTTP server for testing Azure OpenAI client
#[allow(dead_code)]
pub struct MockAzureOpenAIServer {
    responses: Arc<Mutex<Vec<MockResponse>>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MockResponse {
    pub status_code: u16,
    pub body: String,
    pub delay_ms: Option<u64>,
}

impl MockAzureOpenAIServer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[allow(dead_code)]
    pub async fn add_response(&self, response: MockResponse) { self.responses.lock().await.push(response); }

    #[allow(dead_code)]
    pub async fn add_embedding_response(&self, embeddings: Vec<Vec<f32>>) {
        let data: Vec<_> = embeddings
            .into_iter()
            .enumerate()
            .map(|(index, embedding)| {
                json!({
                    "embedding": embedding,
                    "index": index
                })
            })
            .collect();

        let response_body = json!({
            "data": data,
            "usage": {
                "prompt_tokens": 10,
                "total_tokens": 10
            }
        });

        self.add_response(MockResponse {
            status_code: 200,
            body: response_body.to_string(),
            delay_ms: None,
        })
        .await;
    }

    #[allow(dead_code)]
    pub async fn add_chat_completion_response(&self, content: &str) {
        let response_body = json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": content
                },
                "index": 0,
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 20,
                "completion_tokens": 10,
                "total_tokens": 30
            }
        });

        self.add_response(MockResponse {
            status_code: 200,
            body: response_body.to_string(),
            delay_ms: None,
        })
        .await;
    }

    #[allow(dead_code)]
    pub async fn add_error_response(&self, status_code: u16, error_message: &str) {
        let response_body = json!({
            "error": {
                "message": error_message,
                "type": "invalid_request_error",
                "code": "invalid_request"
            }
        });

        self.add_response(MockResponse {
            status_code,
            body: response_body.to_string(),
            delay_ms: None,
        })
        .await;
    }

    #[allow(dead_code)]
    pub async fn add_rate_limit_response(&self) { self.add_error_response(429, "Rate limit exceeded").await; }
}

fn create_test_config() -> AzureOpenAIConfig {
    AzureOpenAIConfig {
        endpoint: "https://test.openai.azure.com".to_string(),
        api_key: "test-api-key-12345678901234567890123456789012".to_string(),
        api_version: "2024-02-01".to_string(),
        chat_deployment: "gpt-4".to_string(),
        embed_deployment: "text-embedding-3-large".to_string(),
        max_retries: 3,
        timeout_seconds: 60,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_client_creation_success() {
        let config = create_test_config();
        let client = AzureOpenAIClient::new(config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_client_creation_with_invalid_timeout() {
        let mut config = create_test_config();
        config.timeout_seconds = 0;

        // The client creation should still succeed, but validation would catch this
        let client = AzureOpenAIClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_backoff_delay_calculation() {
        let config = create_test_config();
        let client = AzureOpenAIClient::new(config).unwrap();

        // Test exponential backoff
        let delay0 = client.calculate_backoff_delay(0);
        let delay1 = client.calculate_backoff_delay(1);
        let delay2 = client.calculate_backoff_delay(2);
        let delay10 = client.calculate_backoff_delay(10); // Should be capped

        assert_eq!(delay0.as_millis(), 1000);
        assert_eq!(delay1.as_millis(), 2000);
        assert_eq!(delay2.as_millis(), 4000);
        assert_eq!(delay10.as_secs(), 30); // Max delay
    }

    #[test]
    fn test_url_construction() {
        let config = create_test_config();
        let client = AzureOpenAIClient::new(config).unwrap();

        assert_eq!(client.config().base_url(), "https://test.openai.azure.com/openai");

        // Test with trailing slash
        let mut config_with_slash = create_test_config();
        config_with_slash.endpoint = "https://test.openai.azure.com/".to_string();
        let client_with_slash = AzureOpenAIClient::new(config_with_slash).unwrap();

        assert_eq!(client_with_slash.config().base_url(), "https://test.openai.azure.com/openai");
    }

    #[tokio::test]
    async fn test_chat_message_serialization() {
        let message = ChatMessage {
            role: "user".to_string(),
            content: "Hello, world!".to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: ChatMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(message.role, deserialized.role);
        assert_eq!(message.content, deserialized.content);
    }

    // Mock-based integration tests would require a more sophisticated setup
    // with a test HTTP server. For now, we'll focus on unit tests and
    // structure the code to be easily testable.

    #[tokio::test]
    async fn test_embedding_input_serialization() {
        use crate::clients::azure_openai::EmbeddingInput;

        // Test single input
        let single = EmbeddingInput::Single("test".to_string());
        let json = serde_json::to_string(&single).unwrap();
        assert_eq!(json, "\"test\"");

        // Test batch input
        let batch = EmbeddingInput::Batch(vec!["test1".to_string(), "test2".to_string()]);
        let json = serde_json::to_string(&batch).unwrap();
        assert_eq!(json, "[\"test1\",\"test2\"]");
    }

    #[tokio::test]
    async fn test_error_response_parsing() {
        let error_json = json!({
            "error": {
                "message": "Invalid request",
                "type": "invalid_request_error",
                "code": "invalid_request"
            }
        });

        let error_response: crate::clients::azure_openai::ApiErrorResponse = serde_json::from_value(error_json).unwrap();
        assert_eq!(error_response.error.message, "Invalid request");
        assert_eq!(error_response.error.error_type, Some("invalid_request_error".to_string()));
        assert_eq!(error_response.error.code, Some("invalid_request".to_string()));
    }
}

// Integration tests that would require actual API credentials
// These should be run separately with environment variables set
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::env;

    fn get_test_config_from_env() -> Option<AzureOpenAIConfig> {
        let endpoint = env::var("TEST_AZURE_OPENAI_ENDPOINT").ok()?;
        let api_key = env::var("TEST_AZURE_OPENAI_API_KEY").ok()?;
        let chat_deployment = env::var("TEST_AZURE_OPENAI_CHAT_DEPLOYMENT").ok()?;
        let embed_deployment = env::var("TEST_AZURE_OPENAI_EMBED_DEPLOYMENT").ok()?;

        Some(AzureOpenAIConfig {
            endpoint,
            api_key,
            api_version: "2024-02-01".to_string(),
            chat_deployment,
            embed_deployment,
            max_retries: 2,
            timeout_seconds: 30,
        })
    }

    #[tokio::test]
    #[ignore] // Only run with --ignored flag and proper env vars
    async fn test_real_embedding_generation() {
        let config = match get_test_config_from_env() {
            | Some(config) => config,
            | None => {
                println!("Skipping integration test - missing environment variables");
                return;
            },
        };

        let client = AzureOpenAIClient::new(config).unwrap();
        let result = client.generate_embedding("Hello, world!").await;

        match result {
            | Ok(embedding) => {
                assert!(!embedding.is_empty());
                assert_eq!(embedding.len(), 3072); // text-embedding-3-large dimension
                println!("Successfully generated embedding with {} dimensions", embedding.len());
            },
            | Err(e) => {
                panic!("Failed to generate embedding: {}", e);
            },
        }
    }

    #[tokio::test]
    #[ignore] // Only run with --ignored flag and proper env vars
    async fn test_real_batch_embedding_generation() {
        let config = match get_test_config_from_env() {
            | Some(config) => config,
            | None => {
                println!("Skipping integration test - missing environment variables");
                return;
            },
        };

        let client = AzureOpenAIClient::new(config).unwrap();
        let texts = vec!["Hello, world!", "This is a test", "Another test string"];
        let result = client.generate_embeddings_batch(texts.clone()).await;

        match result {
            | Ok(embeddings) => {
                assert_eq!(embeddings.len(), texts.len());
                for embedding in embeddings {
                    assert_eq!(embedding.len(), 3072);
                }
                println!("Successfully generated {} embeddings", texts.len());
            },
            | Err(e) => {
                panic!("Failed to generate batch embeddings: {}", e);
            },
        }
    }

    #[tokio::test]
    #[ignore] // Only run with --ignored flag and proper env vars
    async fn test_real_chat_completion() {
        let config = match get_test_config_from_env() {
            | Some(config) => config,
            | None => {
                println!("Skipping integration test - missing environment variables");
                return;
            },
        };

        let client = AzureOpenAIClient::new(config).unwrap();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Say hello in a friendly way.".to_string(),
        }];

        let result = client.generate_chat_completion(messages, Some(50), Some(0.7)).await;

        match result {
            | Ok(response) => {
                assert!(!response.is_empty());
                println!("Successfully generated chat completion: {}", response);
            },
            | Err(e) => {
                panic!("Failed to generate chat completion: {}", e);
            },
        }
    }

    #[tokio::test]
    #[ignore] // Only run with --ignored flag and proper env vars
    async fn test_connectivity() {
        let config = match get_test_config_from_env() {
            | Some(config) => config,
            | None => {
                println!("Skipping integration test - missing environment variables");
                return;
            },
        };

        let client = AzureOpenAIClient::new(config).unwrap();
        let result = client.test_connectivity().await;

        match result {
            | Ok(()) => {
                println!("Connectivity test passed");
            },
            | Err(e) => {
                panic!("Connectivity test failed: {}", e);
            },
        }
    }
}
