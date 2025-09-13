use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Main service error type that encompasses all possible errors in the system
#[derive(Debug, Error, Clone)]
pub enum ServiceError {
    #[error("Document processing failed: {0}")]
    DocumentProcessing(String),

    #[error("Embedding generation failed: {0}")]
    EmbeddingGeneration(String),

    #[error("Vector search failed: {0}")]
    VectorSearch(String),

    #[error("External API error: {0}")]
    ExternalAPI(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Database error: {0}")]
    Database(String),
}

impl ServiceError {
    /// Creates a new document processing error
    pub fn document_processing<T: Into<String>>(msg: T) -> Self { ServiceError::DocumentProcessing(msg.into()) }

    /// Creates a new embedding generation error
    pub fn embedding_generation<T: Into<String>>(msg: T) -> Self { ServiceError::EmbeddingGeneration(msg.into()) }

    /// Creates a new vector search error
    pub fn vector_search<T: Into<String>>(msg: T) -> Self { ServiceError::VectorSearch(msg.into()) }

    /// Creates a new external API error
    pub fn external_api<T: Into<String>>(msg: T) -> Self { ServiceError::ExternalAPI(msg.into()) }

    /// Creates a new configuration error
    pub fn configuration<T: Into<String>>(msg: T) -> Self { ServiceError::Configuration(msg.into()) }

    /// Creates a new internal error
    pub fn internal<T: Into<String>>(msg: T) -> Self { ServiceError::Internal(msg.into()) }

    /// Creates a new validation error
    pub fn validation<T: Into<String>>(msg: T) -> Self { ServiceError::Validation(msg.into()) }

    /// Creates a new not found error
    #[allow(dead_code)]
    pub fn not_found<T: Into<String>>(msg: T) -> Self { ServiceError::NotFound(msg.into()) }

    /// Creates a new rate limit error
    pub fn rate_limit<T: Into<String>>(msg: T) -> Self { ServiceError::RateLimit(msg.into()) }

    /// Creates a new authentication error
    pub fn authentication<T: Into<String>>(msg: T) -> Self { ServiceError::Authentication(msg.into()) }

    /// Creates a new serialization error
    pub fn serialization<T: Into<String>>(msg: T) -> Self { ServiceError::Serialization(msg.into()) }

    /// Creates a new network error
    pub fn network<T: Into<String>>(msg: T) -> Self { ServiceError::Network(msg.into()) }

    /// Creates a new database error
    pub fn database<T: Into<String>>(msg: T) -> Self { ServiceError::Database(msg.into()) }

    /// Returns the HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            | ServiceError::Validation(_) => 400,
            | ServiceError::Authentication(_) => 401,
            | ServiceError::NotFound(_) => 404,
            | ServiceError::RateLimit(_) => 429,
            | ServiceError::Configuration(_) => 500,
            | ServiceError::Internal(_) => 500,
            | ServiceError::DocumentProcessing(_) => 500,
            | ServiceError::EmbeddingGeneration(_) => 502,
            | ServiceError::VectorSearch(_) => 502,
            | ServiceError::ExternalAPI(_) => 502,
            | ServiceError::Serialization(_) => 500,
            | ServiceError::Network(_) => 503,
            | ServiceError::Database(_) => 503,
        }
    }

    /// Returns whether this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ServiceError::Network(_) | ServiceError::Database(_) | ServiceError::ExternalAPI(_) | ServiceError::RateLimit(_)
        )
    }

    /// Returns the error category for structured logging
    pub fn category(&self) -> &'static str {
        match self {
            | ServiceError::DocumentProcessing(_) => "document_processing",
            | ServiceError::EmbeddingGeneration(_) => "embedding_generation",
            | ServiceError::VectorSearch(_) => "vector_search",
            | ServiceError::ExternalAPI(_) => "external_api",
            | ServiceError::Configuration(_) => "configuration",
            | ServiceError::Internal(_) => "internal",
            | ServiceError::Validation(_) => "validation",
            | ServiceError::NotFound(_) => "not_found",
            | ServiceError::RateLimit(_) => "rate_limit",
            | ServiceError::Authentication(_) => "authentication",
            | ServiceError::Serialization(_) => "serialization",
            | ServiceError::Network(_) => "network",
            | ServiceError::Database(_) => "database",
        }
    }

    /// Returns the severity level for logging
    pub fn severity(&self) -> &'static str {
        match self {
            | ServiceError::Validation(_) | ServiceError::NotFound(_) | ServiceError::Authentication(_) => "warn",
            | ServiceError::RateLimit(_) | ServiceError::Network(_) | ServiceError::Database(_) => "error",
            | ServiceError::ExternalAPI(_) => "error",
            | ServiceError::Configuration(_) | ServiceError::Internal(_) => "critical",
            | ServiceError::DocumentProcessing(_) | ServiceError::EmbeddingGeneration(_) | ServiceError::VectorSearch(_) => "error",
            | ServiceError::Serialization(_) => "warn",
        }
    }

    /// Returns additional context for structured logging
    pub fn context(&self) -> std::collections::HashMap<&'static str, String> {
        let mut context = std::collections::HashMap::new();

        context.insert("error_category", self.category().to_string());
        context.insert("severity", self.severity().to_string());
        context.insert("retryable", self.is_retryable().to_string());
        context.insert("status_code", self.status_code().to_string());

        // Add specific context based on error type
        match self {
            | ServiceError::RateLimit(_) => {
                context.insert("retry_after", "60".to_string()); // Default retry after 60 seconds
            },
            | ServiceError::Network(_) | ServiceError::Database(_) => {
                context.insert("health_check_required", "true".to_string());
            },
            | ServiceError::ExternalAPI(_) => {
                context.insert("external_service", "azure_openai".to_string());
            },
            | _ => {},
        }

        context
    }
}

/// HTTP error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status: u16,
    pub retryable: bool,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.to_string(),
            message: match self {
                | ServiceError::DocumentProcessing(_) => "Failed to process the document".to_string(),
                | ServiceError::EmbeddingGeneration(_) => "Failed to generate embeddings".to_string(),
                | ServiceError::VectorSearch(_) => "Failed to search vectors".to_string(),
                | ServiceError::ExternalAPI(_) => "External service unavailable".to_string(),
                | ServiceError::Configuration(_) => "Service configuration error".to_string(),
                | ServiceError::Internal(_) => "Internal server error".to_string(),
                | ServiceError::Validation(_) => "Invalid request data".to_string(),
                | ServiceError::NotFound(_) => "Resource not found".to_string(),
                | ServiceError::RateLimit(_) => "Rate limit exceeded, please try again later".to_string(),
                | ServiceError::Authentication(_) => "Authentication required".to_string(),
                | ServiceError::Serialization(_) => "Data serialization error".to_string(),
                | ServiceError::Network(_) => "Network connectivity issue".to_string(),
                | ServiceError::Database(_) => "Database unavailable".to_string(),
            },
            status: self.status_code(),
            retryable: self.is_retryable(),
        };

        match self.status_code() {
            | 400 => HttpResponse::BadRequest().json(error_response),
            | 401 => HttpResponse::Unauthorized().json(error_response),
            | 404 => HttpResponse::NotFound().json(error_response),
            | 429 => HttpResponse::TooManyRequests().json(error_response),
            | 500 => HttpResponse::InternalServerError().json(error_response),
            | 502 => HttpResponse::BadGateway().json(error_response),
            | 503 => HttpResponse::ServiceUnavailable().json(error_response),
            | _ => HttpResponse::InternalServerError().json(error_response),
        }
    }
}

// Error conversions from common error types
impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self { ServiceError::Serialization(err.to_string()) }
}

impl From<reqwest::Error> for ServiceError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() || err.is_connect() {
            ServiceError::Network(err.to_string())
        } else {
            ServiceError::ExternalAPI(err.to_string())
        }
    }
}

impl From<config::ConfigError> for ServiceError {
    fn from(err: config::ConfigError) -> Self { ServiceError::Configuration(err.to_string()) }
}

impl From<uuid::Error> for ServiceError {
    fn from(err: uuid::Error) -> Self { ServiceError::Internal(format!("UUID generation error: {}", err)) }
}

impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            | std::io::ErrorKind::NotFound => ServiceError::NotFound(err.to_string()),
            | std::io::ErrorKind::PermissionDenied => ServiceError::Authentication(err.to_string()),
            | std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::ConnectionAborted => ServiceError::Network(err.to_string()),
            | std::io::ErrorKind::TimedOut => ServiceError::Network(format!("Operation timed out: {}", err)),
            | _ => ServiceError::Internal(format!("IO error: {}", err)),
        }
    }
}

impl From<tokio::time::error::Elapsed> for ServiceError {
    fn from(err: tokio::time::error::Elapsed) -> Self { ServiceError::Network(format!("Operation timed out: {}", err)) }
}

// Additional error conversion for common crates
impl From<url::ParseError> for ServiceError {
    fn from(err: url::ParseError) -> Self { ServiceError::Configuration(format!("Invalid URL: {}", err)) }
}
