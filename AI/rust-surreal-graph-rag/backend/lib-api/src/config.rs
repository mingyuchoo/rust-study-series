//! 환경설정 로더

use std::env;

#[derive(Clone, Debug)]
pub struct AzureOpenAIConfig {
    pub endpoint: String,
    pub api_key: String,
    pub api_version: String,
    pub chat_deployment: String,
    pub embed_deployment: String,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub azure: AzureOpenAIConfig,
    pub jwt_secret: String,
    pub access_token_ttl_secs: i64,
    pub refresh_token_ttl_secs: i64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let endpoint = env::var("AZURE_OPENAI_ENDPOINT").unwrap_or_else(|_| "".to_string());
        let api_key = env::var("AZURE_OPENAI_API_KEY").unwrap_or_else(|_| "".to_string());
        let api_version = env::var("AZURE_OPENAI_API_VERSION").unwrap_or_else(|_| "2024-06-01".to_string());
        let chat_deployment = env::var("AZURE_OPENAI_CHAT_DEPLOYMENT").unwrap_or_else(|_| "gpt-4.1".to_string());
        let embed_deployment = env::var("AZURE_OPENAI_EMBED_DEPLOYMENT").unwrap_or_else(|_| "text-embedding-3-large".to_string());

        let azure = AzureOpenAIConfig {
            endpoint,
            api_key,
            api_version,
            chat_deployment,
            embed_deployment,
        };

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_change_me".to_string());
        let access_token_ttl_secs = env::var("ACCESS_TOKEN_TTL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(3600);
        let refresh_token_ttl_secs = env::var("REFRESH_TOKEN_TTL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(7 * 24 * 3600);

        Self {
            azure,
            jwt_secret,
            access_token_ttl_secs,
            refresh_token_ttl_secs,
        }
    }
}
