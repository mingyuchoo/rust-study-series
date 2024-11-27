use log::info;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn setup_database() -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to SurrealDB...");
    DB.connect::<Ws>("localhost:8000").await?;
    info!("SurrealDB step 1");
    DB.signin(Root { username: "root", password: "root", }).await?;
    info!("SurrealDB step 2");
    DB.use_ns("namespace").use_db("database").await?;
    info!("SurrealDB step 3");

    DB.query(
             "DEFINE TABLE person SCHEMALESS
        PERMISSIONS FOR
            CREATE, SELECT WHERE $auth,
            FOR UPDATE, DELETE WHERE created_by = $auth;
        DEFINE FIELD name ON TABLE person TYPE string;
        DEFINE FIELD created_by ON TABLE person VALUE $auth READONLY;
        DEFINE INDEX unique_name ON TABLE user FIELDS name UNIQUE;
        DEFINE ACCESS account ON DATABASE TYPE RECORD
        SIGNUP ( CREATE user SET name = $name, pass = \
              crypto::argon2::generate($pass) )
        SIGNIN ( SELECT * FROM user WHERE name = $name AND \
              crypto::argon2::compare(pass, $pass) )
        DURATION FOR TOKEN 15m, FOR SESSION 12h
        ;",
    )
      .await?;

    info!("Connected to SurrealDB");

    Ok(())
}

pub mod routes {
    use crate::error::Error;
    use crate::DB;
    use actix_web::web::Json;
    use actix_web::get;

    #[get("/session")]
    pub async fn session() -> Result<Json<String>, Error> {
        let res: Option<String> = DB.query("RETURN <string>$session").await?.take(0)?;

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
                Error::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
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

