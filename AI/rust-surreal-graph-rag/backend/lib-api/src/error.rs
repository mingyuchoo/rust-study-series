use actix_web::{HttpResponse, ResponseError};
use lib_db::SurrealDbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("external service error: {0}")]
    External(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            | Error::Db(e) => {
                HttpResponse::InternalServerError().body(e.to_string())
            },
            | Error::Unauthorized => {
                HttpResponse::Unauthorized().finish()
            },
            | Error::BadRequest(e) => {
                HttpResponse::BadRequest().body(e.to_string())
            },
            | Error::External(e) => {
                HttpResponse::BadGateway().body(e.to_string())
            },
        }
    }
}

impl From<SurrealDbError> for Error {
    fn from(error: SurrealDbError) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}
