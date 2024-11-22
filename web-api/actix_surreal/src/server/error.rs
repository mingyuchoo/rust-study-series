use actix_web::{HttpResponse,
                ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("database error")]
    Db(String),
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            | ServerError::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}

impl From<surrealdb::Error> for ServerError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}
