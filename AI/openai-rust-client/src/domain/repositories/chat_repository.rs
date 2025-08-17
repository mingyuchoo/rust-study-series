//! Chat repository interface
//! Defines the interface for chat data access

use crate::domain::entities::message::Message;
use std::future::Future;

/// Chat repository trait
/// Defines methods for interacting with chat data
pub trait ChatRepository: Send + Sync + 'static {
    /// Error type returned by the repository
    type Error: std::error::Error + Send + Sync + 'static;

    /// Stream type returned by the repository
    type MessageStream: futures::Stream<Item = Result<String, Self::Error>> + Send + 'static;

    /// Send messages to the chat service and get a stream of responses
    fn send_messages(&self, messages: Vec<Message>) -> impl Future<Output = Result<Self::MessageStream, Self::Error>> + Send + 'static;
}
