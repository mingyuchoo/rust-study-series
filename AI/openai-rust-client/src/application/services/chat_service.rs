//! Chat service module
//! Implements the chat use case

use crate::application::ports::input::ChatUseCase;
use crate::application::ports::output::ChatGateway;
use crate::domain::entities::message::Message;
use futures::future::Future;
use std::sync::Arc;

/// Chat service constants
const MAX_TOKENS: u32 = 4096;
const TEMPERATURE: f32 = 1.0;
const TOP_P: f32 = 1.0;

/// Chat service
/// Implements the chat use case
pub struct ChatService<G> {
    gateway: Arc<G>,
    model: String,
}

impl<G> ChatService<G> {
    /// Create a new chat service
    pub fn new(gateway: Arc<G>, model: String) -> Self {
        Self {
            gateway,
            model,
        }
    }
}

impl<G> ChatUseCase for ChatService<G>
where
    G: ChatGateway,
{
    type Error = G::Error;
    type MessageStream = G::MessageStream;

    fn send_chat_request(&self, messages: Vec<Message>) -> impl Future<Output = Result<Self::MessageStream, Self::Error>> + Send + 'static {
        let gateway = Arc::clone(&self.gateway);
        let model = self.model.clone();

        // Railway Oriented Programming: propagate errors explicitly, no panics/unwraps
        Box::pin(async move { gateway.send_messages(messages, &model, MAX_TOKENS, TEMPERATURE, TOP_P).await })
    }
}
