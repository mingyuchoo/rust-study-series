use crate::config::AzureOpenAIConfig;
use crate::models::ServiceError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

/// Azure OpenAI client for handling embeddings and chat completions
#[derive(Debug, Clone)]
pub struct AzureOpenAIClient {
    config: AzureOpenAIConfig,
    client: Client,
}

/// Request structure for embedding generation
#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    input: EmbeddingInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

/// Input for embedding requests - can be string or array of strings
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum EmbeddingInput {
    Single(String),
    Batch(Vec<String>),
}

/// Response structure for embedding generation
#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
    usage: Usage,
}

/// Individual embedding data
#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

/// Request structure for chat completions
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

/// Chat message structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Response structure for chat completions
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
    usage: Usage,
}

/// Individual chat choice
#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
    index: usize,
    finish_reason: Option<String>,
}

/// Usage statistics
#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    #[serde(default)]
    completion_tokens: u32,
    total_tokens: u32,
}

/// Error response from Azure OpenAI API
#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub error: ApiError,
}

/// API error details
#[derive(Debug, Deserialize)]
pub(crate) struct ApiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
}

impl AzureOpenAIClient {
    /// Create a new Azure OpenAI client
    pub fn new(config: AzureOpenAIConfig) -> Result<Self, ServiceError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| ServiceError::configuration(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { config, client })
    }

    /// Get the configuration (for testing)
    #[cfg(test)]
    pub(crate) fn config(&self) -> &AzureOpenAIConfig {
        &self.config
    }

    /// Generate embedding for a single text
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, ServiceError> {
        debug!("Generating embedding for text of length: {}", text.len());

        let request = EmbeddingRequest {
            input: EmbeddingInput::Single(text.to_string()),
            user: None,
        };

        let response = self.execute_with_retry(|| self.create_embedding_request(&request)).await?;

        if response.data.is_empty() {
            return Err(ServiceError::embedding_generation("No embedding data returned from API"));
        }

        let embedding = response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| ServiceError::embedding_generation("No embedding found in response"))?
            .embedding;

        info!("Successfully generated embedding with {} dimensions", embedding.len());
        Ok(embedding)
    }

    /// Generate embeddings for multiple texts in batch
    pub async fn generate_embeddings_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        debug!("Generating embeddings for batch of {} texts", texts.len());

        let request = EmbeddingRequest {
            input: EmbeddingInput::Batch(texts.iter().map(|s| s.to_string()).collect()),
            user: None,
        };

        let response = self.execute_with_retry(|| self.create_embedding_request(&request)).await?;

        if response.data.len() != texts.len() {
            return Err(ServiceError::embedding_generation(format!(
                "Expected {} embeddings, got {}",
                texts.len(),
                response.data.len()
            )));
        }

        // Sort by index to ensure correct order
        let mut embeddings: Vec<_> = response.data.into_iter().collect();
        embeddings.sort_by_key(|e| e.index);

        let result: Vec<Vec<f32>> = embeddings.into_iter().map(|e| e.embedding).collect();

        info!("Successfully generated {} embeddings in batch", result.len());
        Ok(result)
    }

    /// Generate chat completion
    pub async fn generate_chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Result<String, ServiceError> {
        debug!("Generating chat completion for {} messages", messages.len());

        let request = ChatCompletionRequest {
            messages,
            max_tokens,
            temperature,
            user: None,
        };

        let response = self.execute_with_retry(|| self.create_chat_completion_request(&request)).await?;

        if response.choices.is_empty() {
            return Err(ServiceError::external_api("No choices returned from chat completion API"));
        }

        let content = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| ServiceError::external_api("No choice found in response"))?
            .message
            .content;

        info!("Successfully generated chat completion with {} characters", content.len());
        Ok(content)
    }

    /// Execute a request with retry logic and exponential backoff
    async fn execute_with_retry<F, Fut, T>(&self, mut request_fn: F) -> Result<T, ServiceError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, ServiceError>>,
    {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match request_fn().await {
                | Ok(response) => return Ok(response),
                | Err(error) => {
                    last_error = Some(error);

                    if attempt < self.config.max_retries {
                        let delay = self.calculate_backoff_delay(attempt);
                        warn!(
                            "Request failed (attempt {}/{}), retrying in {}ms",
                            attempt + 1,
                            self.config.max_retries + 1,
                            delay.as_millis()
                        );
                        sleep(delay).await;
                    }
                },
            }
        }

        Err(last_error.unwrap_or_else(|| ServiceError::external_api("All retry attempts failed")))
    }

    /// Calculate exponential backoff delay
    pub(crate) fn calculate_backoff_delay(&self, attempt: u32) -> Duration {
        let base_delay = Duration::from_millis(1000); // 1 second base
        let max_delay = Duration::from_secs(30); // 30 seconds max

        let delay = base_delay * 2_u32.pow(attempt);
        std::cmp::min(delay, max_delay)
    }

    /// Create embedding request
    async fn create_embedding_request(&self, request: &EmbeddingRequest) -> Result<EmbeddingResponse, ServiceError> {
        let url = format!(
            "{}/deployments/{}/embeddings?api-version={}",
            self.config.base_url(),
            self.config.embed_deployment,
            self.config.api_version
        );

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.config.api_key)
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await
            .map_err(|e| ServiceError::network(format!("Failed to send embedding request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Create chat completion request
    async fn create_chat_completion_request(&self, request: &ChatCompletionRequest) -> Result<ChatCompletionResponse, ServiceError> {
        let url = format!(
            "{}/deployments/{}/chat/completions?api-version={}",
            self.config.base_url(),
            self.config.chat_deployment,
            self.config.api_version
        );

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.config.api_key)
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await
            .map_err(|e| ServiceError::network(format!("Failed to send chat completion request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Handle HTTP response and parse JSON
    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, ServiceError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| ServiceError::network(format!("Failed to read response body: {}", e)))?;

        if !status.is_success() {
            return self.handle_error_response(status.as_u16(), &response_text);
        }

        serde_json::from_str(&response_text).map_err(|e| ServiceError::serialization(format!("Failed to parse response JSON: {}", e)))
    }

    /// Handle error responses from the API
    fn handle_error_response<T>(&self, status_code: u16, response_body: &str) -> Result<T, ServiceError> {
        // Try to parse as API error response
        if let Ok(error_response) = serde_json::from_str::<ApiErrorResponse>(response_body) {
            let error_msg = format!("Azure OpenAI API error: {}", error_response.error.message);

            return match status_code {
                | 400 => Err(ServiceError::validation(error_msg)),
                | 401 => Err(ServiceError::authentication(error_msg)),
                | 429 => Err(ServiceError::rate_limit(error_msg)),
                | 500..=599 => Err(ServiceError::external_api(error_msg)),
                | _ => Err(ServiceError::external_api(error_msg)),
            };
        }

        // Fallback for non-JSON error responses
        let error_msg = format!("HTTP {} error: {}", status_code, response_body);
        match status_code {
            | 400 => Err(ServiceError::validation(error_msg)),
            | 401 => Err(ServiceError::authentication(error_msg)),
            | 429 => Err(ServiceError::rate_limit(error_msg)),
            | 500..=599 => Err(ServiceError::external_api(error_msg)),
            | _ => Err(ServiceError::external_api(error_msg)),
        }
    }

    /// Test connectivity to Azure OpenAI API
    pub async fn test_connectivity(&self) -> Result<(), ServiceError> {
        info!("Testing Azure OpenAI connectivity");

        // Test with a simple embedding request
        let test_result = self.generate_embedding("test connectivity").await;

        match test_result {
            | Ok(_) => {
                info!("Azure OpenAI connectivity test successful");
                Ok(())
            },
            | Err(e) => {
                error!("Azure OpenAI connectivity test failed: {}", e);
                Err(e)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AzureOpenAIConfig;

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

    #[test]
    fn test_client_creation() {
        let config = create_test_config();
        let client = AzureOpenAIClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_backoff_delay_calculation() {
        let config = create_test_config();
        let client = AzureOpenAIClient::new(config).unwrap();

        let delay0 = client.calculate_backoff_delay(0);
        let delay1 = client.calculate_backoff_delay(1);
        let delay2 = client.calculate_backoff_delay(2);

        assert_eq!(delay0, Duration::from_millis(1000));
        assert_eq!(delay1, Duration::from_millis(2000));
        assert_eq!(delay2, Duration::from_millis(4000));
    }

    #[test]
    fn test_url_construction() {
        let config = create_test_config();
        let client = AzureOpenAIClient::new(config).unwrap();

        // Test base URL construction
        assert_eq!(client.config.base_url(), "https://test.openai.azure.com/openai");
    }

    // Integration tests would go here but require actual API credentials
    // These should be run separately with real credentials
}
