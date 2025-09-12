mod app;
mod clients;
mod config;
mod handlers;
mod middleware;
mod models;
mod repository;
mod services;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer,
    dev::ServiceFactory,
    middleware::{Compress, DefaultHeaders, Logger},
    web,
};
use app::{AppContainer, ShutdownHandler};
use config::AppConfig;
use handlers::{health_handler, query_handler, simple_health_handler, simple_query_handler, upload_handler, upload_json_handler};
use middleware::{ErrorHandlerMiddleware, RequestLoggerMiddleware};
use std::time::Duration;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging first
    init_logging();

    info!("Starting Rust Qdrant Vector RAG Service...");

    // Load and validate configuration
    let config = match load_config().await {
        | Ok(config) => config,
        | Err(e) => {
            error!("Configuration error: {}", e);
            std::process::exit(1);
        },
    };

    // Initialize application container with all dependencies
    let container = match AppContainer::new(config.clone()).await {
        | Ok(container) => {
            info!("Application dependencies initialized successfully");
            container
        },
        | Err(e) => {
            error!("Failed to initialize application: {}", e);
            std::process::exit(1);
        },
    };

    // Perform initial health check
    match container.health_check().await {
        | Ok(status) => {
            if status.is_healthy() {
                info!("Initial health check passed - all services are healthy");
            } else {
                warn!("Initial health check shows some services are degraded: {:?}", status);
            }
        },
        | Err(e) => {
            error!("Initial health check failed: {}", e);
            std::process::exit(1);
        },
    }

    // Setup graceful shutdown handler
    let shutdown_handler = ShutdownHandler::new(Duration::from_secs(30));
    let shutdown_container = container.clone();

    // Start the HTTP server
    let server_host = config.server.host.clone();
    let server_port = config.server.port;

    info!("Starting HTTP server at {}:{}", server_host, server_port);

    let server = HttpServer::new({
        let container = container.clone();
        move || {
            App::new()
                // Application data - inject all services
                .app_data(web::Data::new(container.config.clone()))
                .app_data(web::Data::new(container.azure_client.clone()))
                .app_data(web::Data::new(container.document_service.clone()))
                .app_data(web::Data::new(container.rag_service.clone()))
                .app_data(web::Data::new(container.embedding_service.clone()))
                .app_data(web::Data::new(container.vector_search_service.clone()))
                
                // Configure JSON payload limits
                .app_data(web::JsonConfig::default().limit(container.config.server.max_request_size))
                
                // Configure multipart payload limits
                .app_data(
                    actix_multipart::form::MultipartFormConfig::default()
                        .total_limit(container.config.server.max_request_size)
                )
                
                // Middleware stack (order matters - applied in reverse order)
                .wrap(ErrorHandlerMiddleware)
                .wrap(RequestLoggerMiddleware)
                .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
                .wrap(Compress::default())
                .wrap(DefaultHeaders::new()
                    .add(("X-Version", "1.0"))
                    .add(("X-Service", "rust-qdrant-vector-rag"))
                )
                .wrap(
                    Cors::default()
                        .allowed_origin_fn(|origin, _req_head| {
                            // Allow localhost and development origins
                            origin.as_bytes().starts_with(b"http://localhost") ||
                            origin.as_bytes().starts_with(b"http://127.0.0.1") ||
                            origin.as_bytes().starts_with(b"https://")
                        })
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                        .allowed_headers(vec!["Content-Type", "Authorization", "Accept"])
                        .expose_headers(vec!["X-Request-ID"])
                        .max_age(3600)
                        .supports_credentials()
                )
                
                // API routes
                .service(
                    web::scope("/api/v1")
                        .route("/health", web::get().to(health_handler))
                        .route("/upload", web::post().to(upload_handler))
                        .route("/upload/json", web::post().to(upload_json_handler))
                        .route("/query", web::post().to(query_handler))
                        .route("/query/{question}", web::get().to(simple_query_handler))
                )
                
                // Legacy routes (for backward compatibility)
                .route("/health", web::get().to(health_handler))
                .route("/health/simple", web::get().to(simple_health_handler))
                .route("/upload", web::post().to(upload_handler))
                .route("/upload/json", web::post().to(upload_json_handler))
                .route("/query", web::post().to(query_handler))
                .route("/query/{question}", web::get().to(simple_query_handler))
                
                // Default route for unmatched paths
                .default_service(web::route().to(not_found_handler))
        }
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .workers(num_cpus::get())
    .shutdown_timeout(30);

    // Run server with graceful shutdown
    let server_handle = server.run();

    tokio::select! {
        result = server_handle => {
            match result {
                Ok(()) => info!("Server stopped normally"),
                Err(e) => error!("Server error: {}", e),
            }
        }
        _ = shutdown_handler.wait_for_shutdown() => {
            info!("Shutdown signal received, stopping server...");
            if let Err(e) = shutdown_handler.shutdown(&shutdown_container).await {
                error!("Error during graceful shutdown: {}", e);
            }
        }
    }

    info!("Application shutdown complete");
    Ok(())
}

/// Load and validate configuration
async fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    info!("Loading configuration from environment...");

    let config = AppConfig::from_env().map_err(|e| format!("Failed to load configuration: {}", e))?;

    config.validate().map_err(|e| format!("Configuration validation failed: {}", e))?;

    info!("Configuration loaded and validated successfully");
    Ok(config)
}

/// Default handler for unmatched routes
async fn not_found_handler() -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "Not Found",
        "message": "The requested endpoint does not exist",
        "status": 404,
        "available_endpoints": [
            "GET /health",
            "GET /api/v1/health",
            "POST /upload",
            "POST /api/v1/upload",
            "POST /upload/json",
            "POST /api/v1/upload/json",
            "POST /query",
            "POST /api/v1/query",
            "GET /query/{question}",
            "GET /api/v1/query/{question}"
        ]
    }))
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "rust_qdrant_vector_rag=info,actix_web=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
