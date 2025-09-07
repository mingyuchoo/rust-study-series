pub mod error;
pub mod routes;
pub mod config;
pub mod azure;
pub mod auth;
pub mod health;
pub mod search;
pub mod chat;
pub mod models;

use actix_web::{App, HttpServer, *};
use lib_db::setup_database;
use log::{error, info};
use actix_web::web;

use crate::config::AppConfig;
use crate::azure::AzureOpenAI;
use crate::search::AppState;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database...");
    init_db().await?;
    info!("Database initializaed.");

    // 환경설정 및 Azure OpenAI 클라이언트 준비
    let cfg = AppConfig::from_env();
    let azure = AzureOpenAI::new(cfg.azure.clone());
    let state = web::Data::new(AppState { cfg: cfg.clone(), azure });

    info!("Starting HTTP server...");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // MVP 엔드포인트 등록
            .service(health::health)
            .service(auth::login)
            .service(auth::refresh)
            .service(auth::logout)
            .service(auth::me)
            .service(search::vector_search)
            .service(chat::chat_ask)
            // 기존 샘플 라우트 유지
            .service(routes::session)
            .service(routes::list_people)
            .service(routes::create_person)
            .service(routes::read_person)
            .service(routes::update_person)
            .service(routes::delete_person)
    })
    .bind(("localhost", 4000))?
    .run()
    .await?;

    Ok(())
}

pub async fn init_db() -> std::io::Result<()> {
    if let Err(err) = setup_database().await {
        error!("Failed to set up database: {:?}", err);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Database setup failed",
        ));
    }
    Ok(())
}
