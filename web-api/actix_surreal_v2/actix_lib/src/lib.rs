pub mod error;
pub mod routes;

use actix_web::{App, HttpServer, *};
use log::{error, info};
// use surrealdb_lib::{routes, setup_database};
use surrealdb_lib::setup_database;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database...");
    init_db().await?;
    info!("Database initializaed.");

    info!("Starting HTTP server...");
    HttpServer::new(|| {
        App::new().service(routes::session)
                  .service(routes::list_people)
                  .service(routes::create_person)
                  .service(routes::read_person)
                  .service(routes::update_person)
                  .service(routes::delete_person)
    }).bind(("localhost", 4000))?
      .run()
      .await?;

    Ok(())
}

pub async fn init_db() -> std::io::Result<()> {
    if let Err(err) = setup_database().await {
        error!("Failed to set up database: {:?}", err);
        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                       "Database setup failed"));
    }
    Ok(())
}
