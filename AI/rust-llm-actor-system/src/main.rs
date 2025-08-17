use actix_web::web::Data;
use actix_web::{App, HttpServer};
use rust_llm_actor_system::{AppState, configure_app, create_agent_router, init_logging};
use tokio::sync::Mutex;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    init_logging();
    info!("Starting Enhanced Multi-LLM Agent System Web Server");

    // Create and configure the agent router
    let router = create_agent_router()?;

    // Check health status of all agents
    info!("Checking health status of all agents...");
    let health_statuses = router.check_all_agents_health().await;
    for (agent_id, status) in &health_statuses {
        info!("Agent {} health status: {:?}", agent_id, status);
    }

    // Create application state
    let app_state = Data::new(AppState {
        router: Mutex::new(router),
        chat_history: Mutex::new(Vec::new()),
    });

    // Start HTTP server
    info!("Starting web server at http://127.0.0.1:8080");
    HttpServer::new(move || App::new().configure(|cfg| configure_app(cfg, app_state.clone())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
