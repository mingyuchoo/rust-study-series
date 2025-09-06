// 설정 관리
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub redis_url: String,
    pub ttl_seconds: u64,
    pub max_memory: String,
    pub azure_openai_endpoint: Option<String>,
    pub azure_openai_api_key: Option<String>,
    pub azure_openai_embeddings_deployment: Option<String>,
    pub azure_openai_chat_deployment: Option<String>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://127.0.0.1:6379".to_string(),
            ttl_seconds: 3600,
            max_memory: "512mb".to_string(),
            azure_openai_endpoint: None,
            azure_openai_api_key: None,
            azure_openai_embeddings_deployment: None,
            azure_openai_chat_deployment: None,
        }
    }
}

impl CacheConfig {
    // 환경변수에서 설정 로드
    pub fn from_env() -> Self {
        Self {
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            ttl_seconds: env::var("CACHE_TTL").unwrap_or_else(|_| "3600".to_string()).parse().unwrap_or(3600),
            max_memory: env::var("REDIS_MAX_MEMORY").unwrap_or_else(|_| "512mb".to_string()),
            azure_openai_endpoint: env::var("AZURE_OPENAI_ENDPOINT").ok(),
            azure_openai_api_key: env::var("AZURE_OPENAI_API_KEY").ok(),
            azure_openai_embeddings_deployment: env::var("AZURE_OPENAI_EMBEDDINGS_DEPLOYMENT").ok(),
            azure_openai_chat_deployment: env::var("AZURE_OPENAI_CHAT_DEPLOYMENT").ok(),
        }
    }
}
