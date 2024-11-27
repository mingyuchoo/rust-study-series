use log::info;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn setup_database() -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to SurrealDB...");

    DB.connect::<Ws>("localhost:8000")
      .await?;

    DB.signin(Root { username: "root",
                     password: "root", })
      .await?;

    DB.use_ns("namespace")
      .use_db("database")
      .await?;

    DB.query("CREATE person")
      .await?;

    info!("Connected to SurrealDB");

    Ok(())
}

pub mod routes {
    use crate::error::Error;
    use crate::DB;
    use actix_web::get;
    use actix_web::web::Json;

    #[get("/session")]
    pub async fn session() -> Result<Json<String>, Error> {
        let res: Option<String> = DB.query("RETURN <string>$session")
                                    .await?
                                    .take(0)?;

        Ok(Json(res.unwrap_or("No session data found!".into())))
    }
}

pub mod error {
    use actix_web::{HttpResponse, ResponseError};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error")]
        Db(String),
    }

    impl ResponseError for Error {
        fn error_response(&self) -> HttpResponse {
            match self {
                | Error::Db(e) => {
                    HttpResponse::InternalServerError().body(e.to_string())
                },
            }
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error}");
            Self::Db(error.to_string())
        }
    }
}
