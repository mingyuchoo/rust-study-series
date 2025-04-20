use std::fmt;
use tonic::Status;

#[derive(Debug)]
pub enum AppError {
    ConnectionError(String),
    RequestError(String),
    ResponseError(String),
    ServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            | AppError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            | AppError::RequestError(msg) => write!(f, "Request error: {}", msg),
            | AppError::ResponseError(msg) => write!(f, "Response error: {}", msg),
            | AppError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

unsafe impl Send for AppError {}
unsafe impl Sync for AppError {}

impl From<tonic::transport::Error> for AppError {
    fn from(err: tonic::transport::Error) -> Self { AppError::ConnectionError(err.to_string()) }
}

impl From<Status> for AppError {
    fn from(status: Status) -> Self { AppError::ResponseError(status.to_string()) }
}

pub type AppResult<T> = Result<T, AppError>;
