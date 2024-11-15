use crate::error::AppError;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client,
                                    Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn setup_database() -> Result<(), AppError> {
    DB.connect::<Ws>("localhost:8000")
      .await?;
    DB.signin(Root { username: "root",
                     password: "root", })
      .await?;
    DB.use_ns("namespace")
      .use_db("database")
      .await?;

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

    Ok(())
}
