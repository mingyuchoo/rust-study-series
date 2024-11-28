mod person;
mod session;
mod token;

use actix_web::{web, HttpResponse, ResponseError};
use thiserror::Error;

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(person::create_person)
       .service(person::read_person)
       .service(person::update_person)
       .service(person::delete_person)
       .service(person::list_people)
       .service(session::session)
       .service(token::make_new_user)
       .service(token::get_new_token);
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("database error")]
    Db(String),
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            | ServerError::Db(e) => {
                HttpResponse::InternalServerError().body(e.to_string())
            },
        }
    }
}

impl From<surrealdb::Error> for ServerError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}
