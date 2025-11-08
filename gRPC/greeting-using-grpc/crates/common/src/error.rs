use thiserror::Error;
use tonic::Status;

/// Application error types for the gRPC service
#[derive(Debug, Error)]
pub enum AppError {
    /// Error when connecting to the gRPC server
    #[error("Connection error: {0}")]
    ConnectionError(String),

    /// Error when making a request to the gRPC server
    #[error("Request error: {0}")]
    RequestError(String),

    /// Error in the response from the gRPC server
    #[error("Response error: {0}")]
    ResponseError(String),

    /// Error from the server itself
    #[error("Server error: {0}")]
    ServerError(String),
}

// From implementations for common error conversions

impl From<tonic::transport::Error> for AppError {
    fn from(err: tonic::transport::Error) -> Self { AppError::ConnectionError(err.to_string()) }
}

impl From<Status> for AppError {
    fn from(status: Status) -> Self { AppError::ResponseError(status.to_string()) }
}

/// Type alias for Results that use AppError as the error type
pub type AppResult<T> = Result<T, AppError>;
