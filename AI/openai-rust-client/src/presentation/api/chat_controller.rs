//! Chat controller module
//! Contains handlers for chat endpoints

use crate::application::ports::input::ChatUseCase;
use crate::infrastructure::adapters::openai_adapter::OpenAIAdapterError;
use crate::presentation::api::models::ChatRequest;
use axum::body::StreamBody;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use std::sync::Arc;

/// Chat controller error
#[derive(Debug)]
pub enum ChatControllerError {
    UseCaseError(Box<dyn std::error::Error + Send + Sync>),
    InvalidHeader(String),
}

impl std::fmt::Display for ChatControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | Self::UseCaseError(err) => write!(f, "Use case error: {}", err),
            | Self::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
        }
    }
}

impl std::error::Error for ChatControllerError {}

// Custom conversion for specific error types we need
impl From<Box<dyn std::error::Error + Send + Sync>> for ChatControllerError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self { Self::UseCaseError(err) }
}

// Implementation for OpenAIAdapterError to maintain proper dependency flow
// In Onion Architecture, the presentation layer can depend on the
// infrastructure layer
impl From<OpenAIAdapterError> for ChatControllerError {
    fn from(err: OpenAIAdapterError) -> Self { Self::UseCaseError(Box::new(err)) }
}

impl From<ChatControllerError> for StatusCode {
    fn from(err: ChatControllerError) -> Self {
        match err {
            | ChatControllerError::UseCaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            | ChatControllerError::InvalidHeader(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Chat controller
/// Handles chat endpoints
pub struct ChatController<U> {
    use_case: Arc<U>,
}

impl<U> ChatController<U> {
    /// Create a new chat controller
    pub fn new(use_case: Arc<U>) -> Self {
        Self {
            use_case,
        }
    }

    /// Chat endpoint handler
    pub async fn chat(&self, request: ChatRequest) -> Result<Response, ChatControllerError>
    where
        U: ChatUseCase,
        ChatControllerError: From<U::Error>,
    {
        // Send chat request to use case
        let stream = self.use_case.send_chat_request(request.messages).await?;

        // ROP: propagate errors as structured errors, not as strings
        let stream = stream;

        // Create a response with the stream body
        let body = StreamBody::new(stream);
        let mut response = Response::new(body.into_response().into_body());

        // Set appropriate headers for streaming text
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            "text/plain; charset=utf-8"
                .parse()
                .map_err(|_| ChatControllerError::InvalidHeader("Invalid content-type header".to_string()))?,
        );

        Ok(response)
    }
}
