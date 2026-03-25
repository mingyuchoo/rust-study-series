//! RAG 설정 관리

use std::env;

/// RAG 챗봇 설정
#[derive(Debug, Clone)]
pub struct RagConfig {
    /// Azure OpenAI 사용 여부
    pub use_azure: bool,

    // Azure OpenAI 설정
    /// Azure OpenAI 엔드포인트
    pub azure_endpoint: Option<String>,
    /// Azure OpenAI API 키
    pub azure_api_key: Option<String>,
    /// Azure API 버전
    pub azure_api_version: String,
    /// Azure 채팅 배포명
    pub azure_chat_deployment: String,
    /// Azure 임베딩 배포명
    pub azure_embedding_deployment: String,

    // OpenAI API 설정
    /// OpenAI API 키
    pub openai_api_key: Option<String>,
    /// OpenAI 모델명
    pub openai_model: String,
    /// OpenAI 임베딩 모델명
    pub embedding_model: String,

    /// Temperature (reasoning 모델은 자동 무시)
    pub temperature: Option<f64>,

    /// 벡터 검색 Top-K
    pub top_k: usize,

    // Langfuse 설정
    /// Langfuse 활성화 여부
    pub langfuse_enabled: bool,
    /// Langfuse 호스트
    pub langfuse_host: String,
    /// Langfuse 공개 키
    pub langfuse_public_key: Option<String>,
    /// Langfuse 비밀 키
    pub langfuse_secret_key: Option<String>,
}

impl RagConfig {
    /// 환경변수에서 설정을 로드한다.
    #[must_use]
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();

        let use_azure = env::var("USE_AZURE_OPENAI").unwrap_or_default().to_lowercase() == "true";

        let azure_chat_deployment = env::var("AZURE_CHAT_DEPLOYMENT").unwrap_or_else(|_| "gpt-5-mini".to_string());

        Self {
            use_azure,
            azure_endpoint: env::var("AZURE_OPENAI_ENDPOINT").ok(),
            azure_api_key: env::var("AZURE_OPENAI_API_KEY").ok(),
            azure_api_version: env::var("AZURE_OPENAI_API_VERSION").unwrap_or_else(|_| "2024-12-01-preview".to_string()),
            azure_chat_deployment,
            azure_embedding_deployment: env::var("AZURE_EMBEDDING_DEPLOYMENT").unwrap_or_else(|_| "text-embedding-3-small".to_string()),
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
            openai_model: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-5-mini".to_string()),
            embedding_model: env::var("EMBEDDING_MODEL").unwrap_or_else(|_| "text-embedding-3-small".to_string()),
            temperature: Some(0.0),
            top_k: 3,
            langfuse_enabled: env::var("LANGFUSE_ENABLED").unwrap_or_default().to_lowercase() == "true",
            langfuse_host: env::var("LANGFUSE_HOST").unwrap_or_else(|_| "http://localhost:3000".to_string()),
            langfuse_public_key: env::var("LANGFUSE_PUBLIC_KEY").ok(),
            langfuse_secret_key: env::var("LANGFUSE_SECRET_KEY").ok(),
        }
    }

    /// 채팅 모델명을 반환한다.
    #[must_use]
    pub fn chat_model(&self) -> &str { if self.use_azure { &self.azure_chat_deployment } else { &self.openai_model } }

    /// reasoning 모델 여부를 판별한다.
    #[must_use]
    pub fn is_reasoning_model(&self) -> bool {
        let model = self.chat_model();
        model.starts_with("o4") || model.starts_with("o1") || model.starts_with("gpt-5")
    }

    /// Langfuse 사용 가능 여부를 확인한다.
    #[must_use]
    pub const fn is_langfuse_available(&self) -> bool { self.langfuse_enabled && self.langfuse_public_key.is_some() && self.langfuse_secret_key.is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reasoning_모델을_올바르게_감지한다() {
        let mut config = RagConfig::from_env();
        config.openai_model = "gpt-5-mini".to_string();
        config.use_azure = false;
        assert!(config.is_reasoning_model());

        config.openai_model = "o4-mini".to_string();
        assert!(config.is_reasoning_model());

        config.openai_model = "gpt-4o".to_string();
        assert!(!config.is_reasoning_model());
    }
}
