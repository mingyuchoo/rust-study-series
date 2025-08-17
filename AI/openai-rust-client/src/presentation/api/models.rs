//! API models module
//! Contains request and response models for the API

use crate::domain::entities::message::Message;
use serde::Deserialize;

/// Chat request model
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    /// Messages to send to the chat service
    pub messages: Vec<Message>,
}
