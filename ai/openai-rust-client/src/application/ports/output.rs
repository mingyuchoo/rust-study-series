//! Output ports module
//! Contains output ports (interfaces to external systems) for the application

use crate::domain::entities::message::Message;
use std::future::Future;

/// Chat gateway trait
/// Defines the interface for communicating with external chat services
pub trait ChatGateway: Send + Sync + 'static {
    /// Error type returned by the gateway
    type Error: std::error::Error + Send + Sync + 'static;

    /// Stream type returned by the gateway
    type MessageStream: futures::Stream<Item = Result<String, Self::Error>> + Send + 'static;

    /// Send messages to the external chat service and get a stream of responses
    fn send_messages(
        &self,
        messages: Vec<Message>,
        model: &str,
        max_tokens: u32,
        temperature: f32,
        top_p: f32,
    ) -> impl Future<Output = Result<Self::MessageStream, Self::Error>> + Send + 'static;
}
