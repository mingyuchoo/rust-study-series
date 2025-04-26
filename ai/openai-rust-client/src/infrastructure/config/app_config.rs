//! Application configuration module
//! Contains configuration for the application

use std::env;

/// Application configuration error
#[derive(Debug)]
pub enum ConfigError {
    EnvVarMissing(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | Self::EnvVarMissing(var) => write!(f, "Environment variable missing: {}", var),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Server configuration constants
pub const SERVER_PORT: u16 = 3000;
pub const SERVER_HOST: [u8; 4] = [127, 0, 0, 1];

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// OpenAI API key
    pub openai_api_key: String,
    /// OpenAI API endpoint
    pub openai_endpoint: String,
    /// OpenAI model
    pub openai_model: String,
}

impl AppConfig {
    /// Create a new AppConfig with values from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            openai_api_key: env::var("AZURE_API_KEY").map_err(|_| ConfigError::EnvVarMissing("AZURE_API_KEY".to_string()))?,
            openai_endpoint: env::var("OPENAI_ENDPOINT").map_err(|_| ConfigError::EnvVarMissing("OPENAI_ENDPOINT".to_string()))?,
            openai_model: env::var("OPENAI_MODEL").map_err(|_| ConfigError::EnvVarMissing("OPENAI_MODEL".to_string()))?,
        })
    }
}
