use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ContactError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },
}
