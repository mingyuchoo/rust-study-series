use axum::extract::State;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use dotenv::dotenv;
// Import modules from our Onion Architecture
// Following strict dependency direction: domain → application → infrastructure → presentation
use openai_rust_client::{// 1. Domain Layer (Core)
                         // Contains business entities and repository interfaces
                         // Has no dependencies on other layers
                         // domain::entities::message::Message,  // Commented out as it's unused in this file

                         // 2. Application Layer (Use Cases)
                         // Contains business logic and use cases
                         // Only depends on the domain layer
                         application::{// ports::input::ChatUseCase,  // Commented out as it's unused in this file
                                       services::chat_service::ChatService},

                         // 3. Infrastructure Layer (Adapters)
                         // Contains implementations of interfaces defined in domain and application layers
                         // Depends on domain and application layers
                         infrastructure::{adapters::openai_adapter::OpenAIAdapter, config::app_config::{AppConfig, SERVER_HOST, SERVER_PORT}},

                         // 4. Presentation Layer (Interface)
                         // Contains UI components and API controllers
                         // Depends on domain, application, and infrastructure layers
                         presentation::{api::{chat_controller::ChatController, models::ChatRequest}, web::handlers::serve_index}};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // ===== INFRASTRUCTURE LAYER =====
    // Initialize infrastructure components first
    // These implement interfaces defined in domain and application layers

    // Create application configuration
    let app_config = AppConfig::from_env()?;

    // Create the OpenAI adapter (implements ChatGateway interface)
    let openai_adapter = Arc::new(OpenAIAdapter::new(app_config.clone()));

    // ===== APPLICATION LAYER =====
    // Initialize application services
    // These implement use cases and depend on domain interfaces

    // Create the chat service (implements ChatUseCase interface)
    let chat_service = Arc::new(ChatService::new(
        openai_adapter, // Infrastructure adapter injected into application service
        app_config.openai_model.clone(),
    ));

    // ===== PRESENTATION LAYER =====
    // Initialize presentation components last
    // These depend on application use cases and infrastructure

    // Create the chat controller
    let chat_controller = Arc::new(ChatController::new(chat_service));

    // Create the static file server for the frontend
    let static_files_service = ServeDir::new("static");

    // Set up CORS
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/chat", post(chat_handler))
        .nest_service("/static", static_files_service)
        .layer(cors)
        .with_state(chat_controller);

    // Run the server
    let addr = SocketAddr::from((SERVER_HOST, SERVER_PORT));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

/// Chat endpoint handler that delegates to the controller
// Railway Oriented Programming: propagate errors using combinators, no
// panics/unwraps
async fn chat_handler(
    State(controller): State<Arc<ChatController<ChatService<OpenAIAdapter>>>>,
    Json(request): Json<ChatRequest>,
) -> Result<Response, axum::http::StatusCode> {
    controller.chat(request).await.map_err(|err| {
        eprintln!("Error: {}", err);
        axum::http::StatusCode::from(err)
    })
}
