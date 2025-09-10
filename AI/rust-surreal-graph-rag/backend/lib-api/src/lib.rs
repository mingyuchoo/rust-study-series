pub mod reindex;
pub mod auth;
pub mod azure;
pub mod chat;
pub mod config;
pub mod error;
pub mod health;
pub mod models;
pub mod vector_search;
pub mod types;
pub mod graph;

use actix_web::web;
use actix_web::{App, HttpServer, *};
use lib_db::setup_database;
use log::{error, info};

use crate::azure::AzureOpenAI;
use crate::config::AppConfig;
use crate::types::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health,
        auth::login,
        auth::refresh,
        auth::logout,
        auth::me,
        vector_search::vector_search,
        graph::graph_search,
        chat::chat_ask,
        reindex::reindex_pdfs,
    ),
    components(
        schemas(
            models::LoginRequest,
            models::LoginResponse,
            models::RefreshResponse,
            models::MessageResponse,
            models::HealthResponse,
            models::VectorSearchRequest,
            models::VectorSearchItem,
            models::VectorSearchResponse,
            models::GraphSearchRequest,
            models::GraphSearchResponse,
            models::ChatAskRequest,
            models::SourceItem,
            models::GraphPathItem,
            models::ChatAskResponse,
            models::MeResponse,
            models::ReindexRequest,
            models::ReindexItemResult,
            models::ReindexResponse,
            models::UploadResponse
        )
    ),
    tags(
        (name = "health", description = "헬스체크"),
        (name = "auth", description = "인증"),
        (name = "vector_search", description = "벡터 검색"),
        (name = "graph", description = "그래프 검색"),
        (name = "chat", description = "통합 질의응답"),
        (name = "reindex", description = "관리자/운영 도구")
    )
)]
struct ApiDoc;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database...");
    init_db().await?;
    info!("Database initializaed.");

    // 환경설정 및 Azure OpenAI 클라이언트 준비
    let cfg = AppConfig::from_env();
    let azure = AzureOpenAI::new(cfg.azure.clone());
    let state = web::Data::new(AppState { cfg: cfg.clone(), azure });
    // 인증 핸들러는 web::Data<AppConfig>를 요구하므로 AppConfig도 별도로 주입
    let cfg_data = web::Data::new(cfg.clone());

    info!("Starting HTTP server...");
    HttpServer::new(move || {
        let openapi = ApiDoc::openapi();
        App::new()
            .app_data(state.clone())
            .app_data(cfg_data.clone())
            // Increase payload limit to allow large file uploads (e.g., PDFs)
            .app_data(web::PayloadConfig::default().limit(100 * 1024 * 1024)) // 100 MB
            .service(health::health)
            .service(auth::login)
            .service(auth::refresh)
            .service(auth::logout)
            .service(auth::me)
            .service(vector_search::vector_search)
            .service(graph::graph_search)
            .service(chat::chat_ask)
            .service(reindex::reindex_pdfs)
            .service(reindex::upload_file)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
    })
    .bind(("localhost", 4000))?
    .run()
    .await?;

    Ok(())
}

pub async fn init_db() -> std::io::Result<()> {
    if let Err(err) = setup_database().await {
        error!("Failed to set up database: {:?}", err);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database setup failed"));
    }
    Ok(())
}
