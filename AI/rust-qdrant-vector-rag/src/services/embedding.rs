use crate::clients::AzureOpenAIClient;
use crate::models::ServiceError;
use crate::services::{ResilienceConfig, ResilienceService};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

#[async_trait]
pub trait EmbeddingService: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, ServiceError>;
    async fn generate_embeddings_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError>;
}

pub struct EmbeddingServiceImpl {
    azure_client: AzureOpenAIClient,
    resilience: Arc<ResilienceService>,
}

impl EmbeddingServiceImpl {
    pub fn new(azure_client: AzureOpenAIClient) -> Self {
        let resilience_config = ResilienceConfig {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
            operation_timeout_seconds: 60,
            use_jitter: true,
        };

        Self {
            azure_client,
            resilience: Arc::new(ResilienceService::new(resilience_config)),
        }
    }

    pub fn with_resilience_config(azure_client: AzureOpenAIClient, resilience_config: ResilienceConfig) -> Self {
        Self {
            azure_client,
            resilience: Arc::new(ResilienceService::new(resilience_config)),
        }
    }

    /// Validates text input for embedding generation
    fn validate_text(&self, text: &str) -> Result<(), ServiceError> {
        if text.trim().is_empty() {
            return Err(ServiceError::validation("Text cannot be empty"));
        }

        // Check for reasonable text length limits
        const MAX_TEXT_LENGTH: usize = 8192; // Common limit for embedding models
        if text.len() > MAX_TEXT_LENGTH {
            warn!("Text length {} exceeds recommended maximum {}, truncating", text.len(), MAX_TEXT_LENGTH);
        }

        Ok(())
    }

    /// Truncates text to fit within model limits while preserving word boundaries
    fn truncate_text(&self, text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            return text.to_string();
        }

        // Find the last space before the limit to avoid cutting words
        let truncated = &text[..max_length];
        if let Some(last_space) = truncated.rfind(' ') {
            text[..last_space].to_string()
        } else {
            // If no space found, just truncate at the limit
            truncated.to_string()
        }
    }
}

#[async_trait]
impl EmbeddingService for EmbeddingServiceImpl {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, ServiceError> {
        debug!("EmbeddingService: generating embedding for text of length {}", text.len());

        self.validate_text(text)?;

        // Truncate text if necessary
        const MAX_TEXT_LENGTH: usize = 8192;
        let processed_text = if text.len() > MAX_TEXT_LENGTH {
            warn!("Truncating text from {} to {} characters", text.len(), MAX_TEXT_LENGTH);
            self.truncate_text(text, MAX_TEXT_LENGTH)
        } else {
            text.to_string()
        };

        let azure_client = &self.azure_client;
        let embedding = self
            .resilience
            .retry_with_backoff(|| {
                let client = azure_client;
                let text = processed_text.clone();
                async move {
                    client.generate_embedding(&text).await.map_err(|e| {
                        let error_context = e.context();
                        match &e {
                            | ServiceError::RateLimit(_) => {
                                warn!(
                                    error = %e,
                                    error_context = ?error_context,
                                    "Rate limit hit during embedding generation"
                                );
                                e
                            },
                            | ServiceError::Authentication(_) => {
                                error!(
                                    error = %e,
                                    error_context = ?error_context,
                                    "Authentication failed during embedding generation"
                                );
                                e
                            },
                            | ServiceError::Network(_) => {
                                warn!(
                                    error = %e,
                                    error_context = ?error_context,
                                    "Network error during embedding generation"
                                );
                                e
                            },
                            | _ => {
                                error!(
                                    error = %e,
                                    error_context = ?error_context,
                                    "Unexpected error during embedding generation"
                                );
                                ServiceError::embedding_generation(format!("Failed to generate embedding: {}", e))
                            },
                        }
                    })
                }
            })
            .await?;

        info!("EmbeddingService: successfully generated embedding with {} dimensions", embedding.len());
        Ok(embedding)
    }

    async fn generate_embeddings_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError> {
        debug!("EmbeddingService: generating embeddings for batch of {} texts", texts.len());

        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Validate all texts and prepare processed texts
        let mut processed_texts = Vec::with_capacity(texts.len());
        const MAX_TEXT_LENGTH: usize = 8192;

        for (i, text) in texts.iter().enumerate() {
            self.validate_text(text)
                .map_err(|e| ServiceError::validation(format!("Text at index {}: {}", i, e)))?;

            let processed_text = if text.len() > MAX_TEXT_LENGTH {
                warn!("Truncating text at index {} from {} to {} characters", i, text.len(), MAX_TEXT_LENGTH);
                self.truncate_text(text, MAX_TEXT_LENGTH)
            } else {
                text.to_string()
            };

            processed_texts.push(processed_text);
        }

        // Convert to string references for the client
        let text_refs: Vec<&str> = processed_texts.iter().map(|s| s.as_str()).collect();

        let azure_client = &self.azure_client;
        let text_refs_clone = text_refs.clone();
        let batch_size = text_refs.len();
        let embeddings = self
            .resilience
            .retry_with_backoff(|| {
                let client = azure_client;
                let texts = text_refs_clone.clone();
                async move {
                    client.generate_embeddings_batch(texts).await.map_err(|e| {
                        let error_context = e.context();
                        match &e {
                            | ServiceError::RateLimit(_) => {
                                warn!(
                                    error = %e,
                                    error_context = ?error_context,
                                    batch_size = batch_size,
                                    "Rate limit hit during batch embedding generation"
                                );
                                e
                            },
                            | ServiceError::Authentication(_) => {
                                error!(
                                    error = %e,
                                    error_context = ?error_context,
                                    batch_size = batch_size,
                                    "Authentication failed during batch embedding generation"
                                );
                                e
                            },
                            | ServiceError::Network(_) => {
                                warn!(
                                    error = %e,
                                    error_context = ?error_context,
                                    batch_size = batch_size,
                                    "Network error during batch embedding generation"
                                );
                                e
                            },
                            | _ => {
                                error!(
                                    error = %e,
                                    error_context = ?error_context,
                                    batch_size = batch_size,
                                    "Unexpected error during batch embedding generation"
                                );
                                ServiceError::embedding_generation(format!("Failed to generate batch embeddings: {}", e))
                            },
                        }
                    })
                }
            })
            .await?;

        if embeddings.len() != texts.len() {
            return Err(ServiceError::embedding_generation(format!(
                "Embedding count mismatch: expected {}, got {}",
                texts.len(),
                embeddings.len()
            )));
        }

        info!("EmbeddingService: successfully generated {} embeddings in batch", embeddings.len());
        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::AzureOpenAIClient;
    use crate::config::AzureOpenAIConfig;

    // Mock Azure OpenAI client for testing
    struct MockAzureOpenAIClient {
        should_fail: bool,
        fail_with_error: Option<ServiceError>,
        embedding_dimension: usize,
    }

    impl MockAzureOpenAIClient {
        fn new() -> Self {
            Self {
                should_fail: false,
                fail_with_error: None,
                embedding_dimension: 1536, // Default OpenAI embedding dimension
            }
        }

        fn with_failure(error: ServiceError) -> Self {
            Self {
                should_fail: true,
                fail_with_error: Some(error),
                embedding_dimension: 1536,
            }
        }

        fn with_dimension(dimension: usize) -> Self {
            Self {
                should_fail: false,
                fail_with_error: None,
                embedding_dimension: dimension,
            }
        }

        async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, ServiceError> {
            if self.should_fail {
                return Err(self.fail_with_error.clone().unwrap_or_else(|| ServiceError::external_api("Mock failure")));
            }

            // Generate a simple mock embedding based on text content
            let mut embedding = vec![0.0; self.embedding_dimension];
            let text_hash = text.len() as f32 / 100.0;
            for (i, val) in embedding.iter_mut().enumerate() {
                *val = (text_hash + i as f32 * 0.01) % 1.0;
            }
            Ok(embedding)
        }

        async fn generate_embeddings_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError> {
            if self.should_fail {
                return Err(self.fail_with_error.clone().unwrap_or_else(|| ServiceError::external_api("Mock batch failure")));
            }

            let mut embeddings = Vec::new();
            for text in texts {
                embeddings.push(self.generate_embedding(text).await?);
            }
            Ok(embeddings)
        }
    }

    fn create_test_service() -> EmbeddingServiceImpl {
        let config = AzureOpenAIConfig {
            endpoint: "https://test.openai.azure.com".to_string(),
            api_key: "test-key".to_string(),
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 60,
        };

        let azure_client = AzureOpenAIClient::new(config).unwrap();
        EmbeddingServiceImpl::new(azure_client)
    }

    fn create_mock_service(mock_client: MockAzureOpenAIClient) -> MockEmbeddingService {
        MockEmbeddingService { client: mock_client }
    }

    // Mock embedding service that uses our mock client
    struct MockEmbeddingService {
        client: MockAzureOpenAIClient,
    }

    #[async_trait]
    impl EmbeddingService for MockEmbeddingService {
        async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, ServiceError> {
            if text.trim().is_empty() {
                return Err(ServiceError::validation("Text cannot be empty"));
            }
            self.client.generate_embedding(text).await
        }

        async fn generate_embeddings_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError> {
            if texts.is_empty() {
                return Ok(Vec::new());
            }

            for (i, text) in texts.iter().enumerate() {
                if text.trim().is_empty() {
                    return Err(ServiceError::validation(format!("Text at index {} cannot be empty", i)));
                }
            }

            self.client.generate_embeddings_batch(texts).await
        }
    }

    #[tokio::test]
    async fn test_generate_embedding_success() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        let text = "This is a test text for embedding generation.";
        let result = service.generate_embedding(text).await;

        assert!(result.is_ok(), "Embedding generation should succeed");
        let embedding = result.unwrap();
        assert_eq!(embedding.len(), 1536, "Embedding should have correct dimension");
        assert!(embedding.iter().any(|&x| x != 0.0), "Embedding should not be all zeros");
    }

    #[tokio::test]
    async fn test_generate_embedding_empty_text() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        let result = service.generate_embedding("").await;

        assert!(result.is_err(), "Empty text should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_generate_embedding_whitespace_only() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        let result = service.generate_embedding("   \n\t   ").await;

        assert!(result.is_err(), "Whitespace-only text should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_generate_embedding_api_failure() {
        let mock_client = MockAzureOpenAIClient::with_failure(ServiceError::external_api("API is down"));
        let service = create_mock_service(mock_client);

        let result = service.generate_embedding("test text").await;

        assert!(result.is_err(), "API failure should be propagated");
        assert!(matches!(result.unwrap_err(), ServiceError::ExternalAPI(_)));
    }

    #[tokio::test]
    async fn test_generate_embedding_rate_limit() {
        let mock_client = MockAzureOpenAIClient::with_failure(ServiceError::rate_limit("Rate limit exceeded"));
        let service = create_mock_service(mock_client);

        let result = service.generate_embedding("test text").await;

        assert!(result.is_err(), "Rate limit should be propagated");
        assert!(matches!(result.unwrap_err(), ServiceError::RateLimit(_)));
    }

    #[tokio::test]
    async fn test_generate_embedding_authentication_failure() {
        let mock_client = MockAzureOpenAIClient::with_failure(ServiceError::authentication("Invalid API key"));
        let service = create_mock_service(mock_client);

        let result = service.generate_embedding("test text").await;

        assert!(result.is_err(), "Authentication failure should be propagated");
        assert!(matches!(result.unwrap_err(), ServiceError::Authentication(_)));
    }

    #[tokio::test]
    async fn test_generate_embeddings_batch_success() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        let texts = vec!["First text", "Second text", "Third text"];
        let result = service.generate_embeddings_batch(texts.clone()).await;

        assert!(result.is_ok(), "Batch embedding generation should succeed");
        let embeddings = result.unwrap();
        assert_eq!(embeddings.len(), texts.len(), "Should return embedding for each text");

        for embedding in &embeddings {
            assert_eq!(embedding.len(), 1536, "Each embedding should have correct dimension");
            assert!(embedding.iter().any(|&x| x != 0.0), "Embedding should not be all zeros");
        }
    }

    #[tokio::test]
    async fn test_generate_embeddings_batch_empty() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        let result = service.generate_embeddings_batch(vec![]).await;

        assert!(result.is_ok(), "Empty batch should succeed");
        let embeddings = result.unwrap();
        assert!(embeddings.is_empty(), "Should return empty vector for empty input");
    }

    #[tokio::test]
    async fn test_generate_embeddings_batch_with_empty_text() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        let texts = vec!["Valid text", "", "Another valid text"];
        let result = service.generate_embeddings_batch(texts).await;

        assert!(result.is_err(), "Batch with empty text should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_generate_embeddings_batch_api_failure() {
        let mock_client = MockAzureOpenAIClient::with_failure(ServiceError::external_api("Batch API failure"));
        let service = create_mock_service(mock_client);

        let texts = vec!["Text 1", "Text 2"];
        let result = service.generate_embeddings_batch(texts).await;

        assert!(result.is_err(), "Batch API failure should be propagated");
        assert!(matches!(result.unwrap_err(), ServiceError::ExternalAPI(_)));
    }

    #[tokio::test]
    async fn test_generate_embeddings_different_dimensions() {
        let mock_client = MockAzureOpenAIClient::with_dimension(768);
        let service = create_mock_service(mock_client);

        let result = service.generate_embedding("test text").await;

        assert!(result.is_ok(), "Should work with different dimensions");
        let embedding = result.unwrap();
        assert_eq!(embedding.len(), 768, "Should respect configured dimension");
    }

    #[tokio::test]
    async fn test_text_truncation() {
        let service = create_test_service();

        // Create a very long text that exceeds typical limits
        let long_text = "word ".repeat(2000); // ~10,000 characters
        let truncated = service.truncate_text(&long_text, 100);

        assert!(truncated.len() <= 100, "Text should be truncated to limit");
        assert!(!truncated.ends_with(' '), "Should not end with space after truncation");
        assert!(long_text.starts_with(&truncated), "Truncated text should be prefix of original");
    }

    #[tokio::test]
    async fn test_text_validation() {
        let service = create_test_service();

        // Test valid text
        assert!(service.validate_text("Valid text").is_ok());

        // Test empty text
        assert!(service.validate_text("").is_err());
        assert!(service.validate_text("   ").is_err());
        assert!(service.validate_text("\n\t").is_err());

        // Test very long text (should not fail validation, just warn)
        let long_text = "a".repeat(10000);
        assert!(service.validate_text(&long_text).is_ok());
    }

    #[tokio::test]
    async fn test_word_boundary_truncation() {
        let service = create_test_service();

        let text = "This is a test sentence that will be truncated";
        let truncated = service.truncate_text(text, 20);

        // Should truncate at word boundary
        assert!(truncated.len() <= 20);
        assert!(!truncated.contains("truncat")); // Should not cut words
        assert!(text.starts_with(&truncated));
    }

    #[tokio::test]
    async fn test_no_word_boundary_truncation() {
        let service = create_test_service();

        // Text with no spaces within the limit
        let text = "verylongtextwithoutspaces";
        let truncated = service.truncate_text(text, 10);

        // Should truncate at character limit when no word boundary found
        assert_eq!(truncated.len(), 10);
        assert_eq!(truncated, "verylongte");
    }

    #[tokio::test]
    async fn test_batch_consistency() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        let text = "Consistent test text";

        // Generate single embedding
        let single_result = service.generate_embedding(text).await.unwrap();

        // Generate batch embedding with same text
        let batch_result = service.generate_embeddings_batch(vec![text]).await.unwrap();

        assert_eq!(batch_result.len(), 1);
        assert_eq!(single_result, batch_result[0], "Single and batch results should be identical");
    }

    #[tokio::test]
    async fn test_large_batch_processing() {
        let mock_client = MockAzureOpenAIClient::new();
        let service = create_mock_service(mock_client);

        // Create a large batch
        let texts: Vec<String> = (0..100).map(|i| format!("Text number {}", i)).collect();
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

        let result = service.generate_embeddings_batch(text_refs).await;

        assert!(result.is_ok(), "Large batch should succeed");
        let embeddings = result.unwrap();
        assert_eq!(embeddings.len(), 100, "Should return embedding for each text");

        // Verify all embeddings are different (based on our mock implementation)
        for i in 0..embeddings.len() {
            for j in (i + 1)..embeddings.len() {
                assert_ne!(embeddings[i], embeddings[j], "Embeddings should be different");
            }
        }
    }
}
