use crate::clients::AzureOpenAIClient;
use crate::config::AzureOpenAIConfig;
use crate::services::embedding::{EmbeddingService, EmbeddingServiceImpl};

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

    #[tokio::test]
    async fn test_embedding_service_creation() {
        let config = create_test_config();
        let azure_client = AzureOpenAIClient::new(config).unwrap();
        let embedding_service = EmbeddingServiceImpl::new(azure_client);

        // Test that the service was created successfully
        // We can't test actual embedding generation without real API credentials
        // but we can test that the service validates input correctly

        let result = embedding_service.generate_embedding("").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Text cannot be empty"));
    }

    #[tokio::test]
    async fn test_embedding_service_batch_validation() {
        let config = create_test_config();
        let azure_client = AzureOpenAIClient::new(config).unwrap();
        let embedding_service = EmbeddingServiceImpl::new(azure_client);

        // Test empty batch
        let result = embedding_service.generate_embeddings_batch(vec![]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        // Test batch with empty string
        let result = embedding_service.generate_embeddings_batch(vec!["test", ""]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Text at index 1 cannot be empty"));
    }
}
