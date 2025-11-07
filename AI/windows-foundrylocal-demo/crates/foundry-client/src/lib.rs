mod error;

pub use error::{FoundryClientError, Result};
use foundry_local::FoundryLocalManager;
use foundry_types::{ChatCompletionRequest, ChatCompletionResponse};
use std::time::Duration;

/// Foundry API 클라이언트
pub struct FoundryClient {
    #[allow(dead_code)] // 서비스를 활성 상태로 유지하기 위해 필요
    manager: FoundryLocalManager,
    http_client: reqwest::Client,
    endpoint: String,
}

impl FoundryClient {
    /// 새로운 클라이언트 생성
    pub async fn new() -> Result<Self> {
        Self::builder().build().await
    }

    /// 빌더 패턴으로 클라이언트 생성
    pub fn builder() -> FoundryClientBuilder {
        FoundryClientBuilder::new()
    }

    /// 채팅 완성 요청
    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let endpoint = format!("{}/chat/completions", self.endpoint);

        tracing::debug!(
            endpoint = %endpoint,
            model = %request.model,
            "Sending chat completion request"
        );

        let response = self
            .http_client
            .post(&endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| FoundryClientError::RequestFailed(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "<응답 본문을 읽을 수 없음>".to_string());
            tracing::error!(status = %status, error = %error_text, "API request failed");
            return Err(FoundryClientError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| FoundryClientError::ResponseReadFailed(e.to_string()))?;

        if response_text.trim().is_empty() {
            return Err(FoundryClientError::EmptyResponse);
        }

        let result: ChatCompletionResponse = serde_json::from_str(&response_text)
            .map_err(|e| FoundryClientError::JsonParseFailed {
                error: e.to_string(),
                response: response_text,
            })?;

        tracing::debug!("Chat completion successful");
        Ok(result)
    }

    /// 엔드포인트 가져오기
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// Foundry 클라이언트 빌더
pub struct FoundryClientBuilder {
    bootstrap: bool,
    timeout: Duration,
}

impl FoundryClientBuilder {
    pub fn new() -> Self {
        Self {
            bootstrap: true,
            timeout: Duration::from_secs(360),
        }
    }

    pub fn bootstrap(mut self, bootstrap: bool) -> Self {
        self.bootstrap = bootstrap;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn build(self) -> Result<FoundryClient> {
        tracing::info!("Initializing Foundry Local manager");

        let manager = FoundryLocalManager::builder()
            .bootstrap(self.bootstrap)
            .build()
            .await
            .map_err(|e| FoundryClientError::ManagerInitFailed(e.to_string()))?;

        let endpoint = manager
            .endpoint()
            .map_err(|e| FoundryClientError::EndpointFailed(e.to_string()))?;

        let http_client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(|e| FoundryClientError::HttpClientBuildFailed(e.to_string()))?;

        tracing::info!(endpoint = %endpoint, "Foundry client initialized");

        Ok(FoundryClient {
            manager,
            http_client,
            endpoint,
        })
    }
}

impl Default for FoundryClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
