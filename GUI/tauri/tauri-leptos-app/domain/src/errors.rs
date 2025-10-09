use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Clone)]
pub enum ContactError {
    #[error("Contact not found with id: {id}")]
    NotFound { id: Uuid },

    #[error("Contact name cannot be empty")]
    EmptyName,

    #[error("Invalid email format: {email}")]
    InvalidEmail { email: String },

    #[error("Database error: {message}")]
    DatabaseError { message: String },
}

pub type ContactResult<T> = Result<T, ContactError>;
