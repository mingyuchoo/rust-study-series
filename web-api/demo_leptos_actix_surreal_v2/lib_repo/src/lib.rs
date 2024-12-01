use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, RecordId, Surreal};

pub type SurrealDBError = Error;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn setup_database() -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>("localhost:8000")
      .await?;
    DB.signin(Root { username: "root",
                     password: "root", })
      .await?;
    DB.use_ns("namespace")
      .use_db("database")
      .await?;

    Ok(())
}
