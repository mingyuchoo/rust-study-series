//! Domain error module
//! Defines domain-specific errors

use std::fmt;

/// Domain error type
#[derive(Debug)]
pub enum DomainError {
    /// Validation error
    Validation(String),
    /// Business rule violation
    BusinessRule(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            | Self::Validation(msg) => write!(f, "Validation error: {}", msg),
            | Self::BusinessRule(msg) => write!(f, "Business rule violation: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
