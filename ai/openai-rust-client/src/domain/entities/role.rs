//! Role entity module
//! Defines the roles in a conversation

use serde::{Deserialize, Serialize};

/// Represents a role in the conversation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Module to serialize/deserialize Role as string
pub mod serde_utils {
    use super::Role;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serialize a Role to a string
    pub fn serialize<S>(role: &Role, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let role_str = match role {
            | Role::System => "system",
            | Role::User => "user",
            | Role::Assistant => "assistant",
        };
        serializer.serialize_str(role_str)
    }

    /// Deserialize a string to a Role
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Role, D::Error>
    where
        D: Deserializer<'de>,
    {
        let role_str = String::deserialize(deserializer)?;
        match role_str.as_str() {
            | "system" => Ok(Role::System),
            | "user" => Ok(Role::User),
            | "assistant" => Ok(Role::Assistant),
            | _ => Err(serde::de::Error::custom(format!("Invalid role: {}", role_str))),
        }
    }
}
