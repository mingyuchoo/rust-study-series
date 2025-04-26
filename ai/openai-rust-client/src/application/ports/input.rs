//! Input ports module
//! Contains input ports (use cases) for the application

use crate::domain::entities::message::Message;
use std::future::Future;

/// Chat use case trait
/// Defines the interface for the chat use case
pub trait ChatUseCase: Send + Sync + 'static {
    /// Error type returned by the use case
    type Error: std::error::Error + Send + Sync + 'static;

    /// Stream type returned by the use case
    type MessageStream: futures::Stream<Item = Result<String, Self::Error>> + Send + 'static;

    /// Send a chat request and get a stream of responses
    fn send_chat_request(&self, messages: Vec<Message>) -> impl Future<Output = Result<Self::MessageStream, Self::Error>> + Send + 'static;
}
