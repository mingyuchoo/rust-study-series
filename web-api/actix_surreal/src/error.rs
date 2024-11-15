use actix_web::{HttpResponse,
                ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database error")]
    Db(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            | AppError::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}
