use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Contact not found")]
    ContactNotFound,

    #[error("Invalid contact data: {0}")]
    InvalidContactData(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal error: {0}")]
    #[allow(dead_code)]
    InternalError(String),
}

impl From<sqlx::Error> for DomainError {
    fn from(error: sqlx::Error) -> Self { DomainError::DatabaseError(error.to_string()) }
}
