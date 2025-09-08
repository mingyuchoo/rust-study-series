use actix_web::{HttpResponse, ResponseError};
use lib_db::SurrealDbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            | Error::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}

impl From<SurrealDbError> for Error {
    fn from(error: SurrealDbError) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}
