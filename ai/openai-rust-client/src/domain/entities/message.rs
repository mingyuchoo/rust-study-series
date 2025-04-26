//! Message entity module
//! Defines the message structure used in conversations

use super::role::Role;
use serde::{Deserialize, Serialize};

/// Represents a message in a conversation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    /// The role of the message sender
    pub role: Role,

    /// The content of the message
    pub content: String,
}

impl Message {
    /// Create a new message
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role,
            content,
        }
    }

    /// Create a new system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    /// Create a new user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}
