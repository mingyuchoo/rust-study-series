use crate::models::ServiceError;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, SocketAddr};
use url::Url;

/// Main application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub azure_openai: AzureOpenAIConfig,
    pub qdrant: QdrantConfig,
}

/// Server configuration for the web service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_request_size: usize,
    pub timeout_seconds: u64,
}

/// Azure OpenAI service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureOpenAIConfig {
    pub endpoint: String,
    pub api_key: String,
    pub api_version: String,
    pub chat_deployment: String,
    pub embed_deployment: String,
    pub max_retries: u32,
    pub timeout_seconds: u64,
}

/// Qdrant vector database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantConfig {
    pub url: String,
    pub api_key: Option<String>,
    pub collection_name: String,
    pub vector_size: u64,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl AppConfig {
    /// Load configuration from environment variables with comprehensive
    /// validation
    pub fn from_env() -> Result<Self, ServiceError> {
        // Load .env file if it exists
        dotenvy::dotenv().ok();

        let config = AppConfig {
            server: ServerConfig::from_env()?,
            azure_openai: AzureOpenAIConfig::from_env()?,
            qdrant: QdrantConfig::from_env()?,
        };

        // Validate the complete configuration
        config.validate()?;

        Ok(config)
    }

    /// Validate the entire configuration
    pub fn validate(&self) -> Result<(), ServiceError> {
        self.server
            .validate()
            .map_err(|e| ServiceError::Configuration(format!("Server config validation failed: {}", e)))?;

        self.azure_openai
            .validate()
            .map_err(|e| ServiceError::Configuration(format!("Azure OpenAI config validation failed: {}", e)))?;

        self.qdrant
            .validate()
            .map_err(|e| ServiceError::Configuration(format!("Qdrant config validation failed: {}", e)))?;

        Ok(())
    }

    /// Get the server socket address
    #[allow(dead_code)]
    pub fn server_address(&self) -> Result<SocketAddr, ServiceError> {
        let ip: IpAddr = self
            .server
            .host
            .parse()
            .map_err(|e| ServiceError::Configuration(format!("Invalid server host '{}': {}", self.server.host, e)))?;

        Ok(SocketAddr::new(ip, self.server.port))
    }
}

impl ServerConfig {
    /// Load server configuration from environment variables
    pub fn from_env() -> Result<Self, ServiceError> {
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid SERVER_PORT: {}", e)))?;

        let max_request_size = env::var("SERVER_MAX_REQUEST_SIZE")
            .unwrap_or_else(|_| "10485760".to_string()) // 10MB default
            .parse::<usize>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid SERVER_MAX_REQUEST_SIZE: {}", e)))?;

        let timeout_seconds = env::var("SERVER_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid SERVER_TIMEOUT_SECONDS: {}", e)))?;

        Ok(ServerConfig {
            host,
            port,
            max_request_size,
            timeout_seconds,
        })
    }

    /// Validate server configuration
    pub fn validate(&self) -> Result<(), ServiceError> {
        // Validate host is a valid IP address
        self.host
            .parse::<IpAddr>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid server host '{}': {}", self.host, e)))?;

        // Validate port range
        if self.port == 0 {
            return Err(ServiceError::Configuration("Server port cannot be 0".to_string()));
        }

        // Validate max request size (minimum 1KB, maximum 100MB)
        if self.max_request_size < 1024 {
            return Err(ServiceError::Configuration("SERVER_MAX_REQUEST_SIZE must be at least 1024 bytes".to_string()));
        }
        if self.max_request_size > 104_857_600 {
            return Err(ServiceError::Configuration("SERVER_MAX_REQUEST_SIZE cannot exceed 100MB".to_string()));
        }

        // Validate timeout
        if self.timeout_seconds == 0 {
            return Err(ServiceError::Configuration("SERVER_TIMEOUT_SECONDS must be greater than 0".to_string()));
        }
        if self.timeout_seconds > 300 {
            return Err(ServiceError::Configuration("SERVER_TIMEOUT_SECONDS cannot exceed 300 seconds".to_string()));
        }

        Ok(())
    }
}

impl AzureOpenAIConfig {
    /// Load Azure OpenAI configuration from environment variables
    pub fn from_env() -> Result<Self, ServiceError> {
        let endpoint = env::var("AZURE_OPENAI_ENDPOINT").map_err(|_| ServiceError::Configuration("AZURE_OPENAI_ENDPOINT is required".to_string()))?;

        let api_key = env::var("AZURE_OPENAI_API_KEY").map_err(|_| ServiceError::Configuration("AZURE_OPENAI_API_KEY is required".to_string()))?;

        let api_version = env::var("AZURE_OPENAI_API_VERSION").unwrap_or_else(|_| "2024-02-01".to_string());

        let chat_deployment =
            env::var("AZURE_OPENAI_CHAT_DEPLOYMENT").map_err(|_| ServiceError::Configuration("AZURE_OPENAI_CHAT_DEPLOYMENT is required".to_string()))?;

        let embed_deployment =
            env::var("AZURE_OPENAI_EMBED_DEPLOYMENT").map_err(|_| ServiceError::Configuration("AZURE_OPENAI_EMBED_DEPLOYMENT is required".to_string()))?;

        let max_retries = env::var("AZURE_OPENAI_MAX_RETRIES")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<u32>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid AZURE_OPENAI_MAX_RETRIES: {}", e)))?;

        let timeout_seconds = env::var("AZURE_OPENAI_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u64>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid AZURE_OPENAI_TIMEOUT_SECONDS: {}", e)))?;

        Ok(AzureOpenAIConfig {
            endpoint,
            api_key,
            api_version,
            chat_deployment,
            embed_deployment,
            max_retries,
            timeout_seconds,
        })
    }

    /// Validate Azure OpenAI configuration
    pub fn validate(&self) -> Result<(), ServiceError> {
        // Validate endpoint is a valid URL
        Url::parse(&self.endpoint).map_err(|e| ServiceError::Configuration(format!("Invalid Azure OpenAI endpoint '{}': {}", self.endpoint, e)))?;

        // Validate endpoint is HTTPS
        if !self.endpoint.starts_with("https://") {
            return Err(ServiceError::Configuration("Azure OpenAI endpoint must use HTTPS".to_string()));
        }

        // Validate API key is not empty
        if self.api_key.trim().is_empty() {
            return Err(ServiceError::Configuration("Azure OpenAI API key cannot be empty".to_string()));
        }

        // Validate API key format (basic check for Azure OpenAI key format)
        if self.api_key.len() < 32 {
            return Err(ServiceError::Configuration(
                "Azure OpenAI API key appears to be invalid (too short)".to_string(),
            ));
        }

        // Validate API version format (YYYY-MM-DD or YYYY-MM-DD-preview)
        let parts: Vec<&str> = self.api_version.split('-').collect();
        let is_valid = parts.len() >= 3 &&
                      parts.len() <= 4 &&  // YYYY-MM-DD or YYYY-MM-DD-preview
                      parts[0].len() == 4 && parts[0].parse::<u32>().is_ok() &&  // YYYY
                      parts[1].len() == 2 && parts[1].parse::<u32>().is_ok() &&  // MM
                      parts[2].len() == 2 && parts[2].parse::<u32>().is_ok();   // DD
        
        if !is_valid {
            return Err(ServiceError::Configuration(format!(
                "Invalid API version format '{}', expected format: YYYY-MM-DD or YYYY-MM-DD-preview",
                self.api_version
            )));
        }

        // Validate deployment names are not empty
        if self.chat_deployment.trim().is_empty() {
            return Err(ServiceError::Configuration("Azure OpenAI chat deployment name cannot be empty".to_string()));
        }

        if self.embed_deployment.trim().is_empty() {
            return Err(ServiceError::Configuration(
                "Azure OpenAI embedding deployment name cannot be empty".to_string(),
            ));
        }

        // Validate retry and timeout settings
        if self.max_retries > 10 {
            return Err(ServiceError::Configuration("AZURE_OPENAI_MAX_RETRIES cannot exceed 10".to_string()));
        }

        if self.timeout_seconds == 0 {
            return Err(ServiceError::Configuration("AZURE_OPENAI_TIMEOUT_SECONDS must be greater than 0".to_string()));
        }

        if self.timeout_seconds > 300 {
            return Err(ServiceError::Configuration(
                "AZURE_OPENAI_TIMEOUT_SECONDS cannot exceed 300 seconds".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the base URL for Azure OpenAI API calls
    pub fn base_url(&self) -> String { format!("{}/openai", self.endpoint.trim_end_matches('/')) }
}

impl QdrantConfig {
    /// Load Qdrant configuration from environment variables
    pub fn from_env() -> Result<Self, ServiceError> {
        let url = env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());

        let api_key = env::var("QDRANT_API_KEY").ok();

        let collection_name = env::var("QDRANT_COLLECTION_NAME").unwrap_or_else(|_| "document_chunks".to_string());

        let vector_size = env::var("QDRANT_VECTOR_SIZE")
            .unwrap_or_else(|_| "3072".to_string()) // text-embedding-3-large size
            .parse::<u64>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid QDRANT_VECTOR_SIZE: {}", e)))?;

        let timeout_seconds = env::var("QDRANT_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid QDRANT_TIMEOUT_SECONDS: {}", e)))?;

        let max_retries = env::var("QDRANT_MAX_RETRIES")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<u32>()
            .map_err(|e| ServiceError::Configuration(format!("Invalid QDRANT_MAX_RETRIES: {}", e)))?;

        Ok(QdrantConfig {
            url,
            api_key,
            collection_name,
            vector_size,
            timeout_seconds,
            max_retries,
        })
    }

    /// Validate Qdrant configuration
    pub fn validate(&self) -> Result<(), ServiceError> {
        // Validate URL format
        Url::parse(&self.url).map_err(|e| ServiceError::Configuration(format!("Invalid Qdrant URL '{}': {}", self.url, e)))?;

        // Validate collection name
        if self.collection_name.trim().is_empty() {
            return Err(ServiceError::Configuration("Qdrant collection name cannot be empty".to_string()));
        }

        // Validate collection name format (alphanumeric, underscores, hyphens only)
        if !self.collection_name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(ServiceError::Configuration(
                "Qdrant collection name can only contain alphanumeric characters, underscores, and hyphens".to_string(),
            ));
        }

        // Validate vector size
        if self.vector_size == 0 {
            return Err(ServiceError::Configuration("QDRANT_VECTOR_SIZE must be greater than 0".to_string()));
        }

        if self.vector_size > 65536 {
            return Err(ServiceError::Configuration("QDRANT_VECTOR_SIZE cannot exceed 65536".to_string()));
        }

        // Validate timeout
        if self.timeout_seconds == 0 {
            return Err(ServiceError::Configuration("QDRANT_TIMEOUT_SECONDS must be greater than 0".to_string()));
        }

        if self.timeout_seconds > 300 {
            return Err(ServiceError::Configuration("QDRANT_TIMEOUT_SECONDS cannot exceed 300 seconds".to_string()));
        }

        // Validate max retries
        if self.max_retries > 10 {
            return Err(ServiceError::Configuration("QDRANT_MAX_RETRIES cannot exceed 10".to_string()));
        }

        Ok(())
    }
}
