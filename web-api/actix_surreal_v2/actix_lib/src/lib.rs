use actix_web::{App, HttpServer, *};
use log::{error, info};
use surrealdb_lib::{routes, setup_database};

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database...");
    init_db().await?;
    info!("Database initializaed.");

    info!("Starting HTTP server...");
    HttpServer::new(|| {
        App::new()
            .service(routes::session)
    })
    .bind(("localhost", 4000))?
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
