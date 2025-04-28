use actix_files::Files;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, post, web};
use chrono::Utc;
use rust_llm_actor_system::{AgentRouter, HealthStatus};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{Level, error, info, warn};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

// Struct to hold application state
struct AppState {
    router: Mutex<AgentRouter>,
    chat_history: Mutex<Vec<ChatMessage>>,
}

// Struct for incoming prompt requests
#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
}

// Struct for prompt responses
#[derive(Serialize)]
struct PromptResponse {
    response: String,
    agent_id: String,
    timestamp: String,
}

// Struct for agent information
#[derive(Serialize)]
struct AgentInfo {
    id: String,
    model: String,
    health: String,
    health_class: String,
    prompts: String,
    avg_time: String,
}

// Struct for chat messages
#[derive(Serialize, Clone)]
struct ChatMessage {
    id: String,
    content: String,
    agent_id: Option<String>,
    timestamp: String,
    #[serde(rename = "type")]
    message_type: String, // "user" or "agent"
}

// Handler for the root page - redirects to static index.html
#[get("/")]
async fn index() -> Result<HttpResponse> { Ok(HttpResponse::Found().append_header(("Location", "/static/index.html")).finish()) }

// API handler to get all agents
#[get("/api/agents")]
async fn get_agents(app_state: web::Data<AppState>) -> impl Responder {
    let router = app_state.router.lock().await;

    let mut agents = Vec::new();
    for (agent_id, agent) in router.get_agents() {
        let (prompts, _errors, _, avg_time) = agent.get_metrics().get_stats();
        let health_status = agent.get_health_status();

        let health_class = match health_status {
            | HealthStatus::Healthy => "healthy",
            | HealthStatus::Degraded => "degraded",
            | HealthStatus::Unhealthy => "unhealthy",
        };

        agents.push(AgentInfo {
            id: agent_id.clone(),
            model: agent.get_model().clone(),
            health: format!("{:?}", health_status),
            health_class: health_class.to_string(),
            prompts: prompts.to_string(),
            avg_time: format!("{:.2}", avg_time),
        });
    }

    HttpResponse::Ok().json(agents)
}

// API handler to get system stats
#[get("/api/stats")]
async fn get_stats(app_state: web::Data<AppState>) -> impl Responder {
    let router = app_state.router.lock().await;
    let stats = router.get_system_stats();

    HttpResponse::Ok().json(stats)
}

// API handler to process prompts
#[post("/api/prompt")]
async fn process_prompt(req: web::Json<PromptRequest>, app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let prompt = req.prompt.clone();
    info!("Received prompt: {}", prompt);

    // Add user message to chat history
    let user_message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        content: prompt.clone(),
        agent_id: None,
        timestamp: Utc::now().to_rfc3339(),
        message_type: "user".to_string(),
    };

    app_state.chat_history.lock().await.push(user_message);

    // Process the prompt
    let agent_id = {
        let router = app_state.router.lock().await;
        match router.select_best_agent(&prompt).await {
            | Ok(agent) => agent.get_id().clone(),
            | Err(e) => {
                warn!("Failed to select agent: {}", e);
                return Ok(HttpResponse::InternalServerError().body("Failed to select agent"));
            },
        }
    };

    let response = app_state.router.lock().await.route_prompt(prompt.clone()).await;
    match response {
        | Ok(response) => {
            info!("Response from agent {}: {}", agent_id, response);

            // Add agent response to chat history
            let agent_message = ChatMessage {
                id: Uuid::new_v4().to_string(),
                content: response.clone(),
                agent_id: Some(agent_id.to_string()),
                timestamp: Utc::now().to_rfc3339(),
                message_type: "agent".to_string(),
            };

            app_state.chat_history.lock().await.push(agent_message);

            // Keep only the last 100 messages
            let mut history = app_state.chat_history.lock().await;
            if history.len() > 100 {
                *history = history.iter().skip(history.len() - 100).cloned().collect();
            }

            // Return the response
            let prompt_response = PromptResponse {
                response,
                agent_id: agent_id.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            };

            Ok(HttpResponse::Ok().json(prompt_response))
        },
        | Err(e) => {
            error!("Main> Error processing prompt: {:?}", e);

            // Add error message to chat history
            let error_message = ChatMessage {
                id: Uuid::new_v4().to_string(),
                content: format!("Error: {}", e),
                agent_id: Some("system".to_string()),
                timestamp: Utc::now().to_rfc3339(),
                message_type: "agent".to_string(),
            };

            app_state.chat_history.lock().await.push(error_message);

            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("{}", e)
            })))
        },
    }
}

// API handler to check health of all agents
#[get("/api/health")]
async fn check_health(app_state: web::Data<AppState>) -> impl Responder {
    let health_statuses = app_state.router.lock().await.check_all_agents_health().await;
    HttpResponse::Ok().json(health_statuses)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    info!("Starting Enhanced Multi-LLM Agent System Web Server");

    // Create the agent router
    let mut router = AgentRouter::new();

    // Add some initial LLM agents
    let agent_configs = vec![
        ("default", "gpt-4o", "You are a helpful, advanced assistant."),
        ("math_specialist", "gpt-4o", "You are a math expert. Only answer math questions in detail."),
        (
            "korean_specialist",
            "gpt-4o",
            "You are a Korean language specialist. Answer in fluent Korean and focus on Korean language/culture topics.",
        ),
    ];

    for (agent_id, model, system_prompt) in agent_configs {
        router.add_agent(agent_id.to_string(), model.to_string(), system_prompt.to_string());
        info!("Successfully added agent: {} with model: {}", agent_id, model);
    }

    // Register routing rules with different priorities and confidence thresholds
    router
        .register_rule_with_priority("math".to_string(), "math_specialist".to_string(), 10, 0.6)
        .map_err(std::io::Error::other)?;
    router
        .register_rule_with_priority("default".to_string(), "default".to_string(), 5, 0.4)
        .map_err(std::io::Error::other)?;
    router
        .register_rule_with_priority("한국".to_string(), "korean_specialist".to_string(), 8, 0.5)
        .map_err(std::io::Error::other)?;

    // Register default routing rule
    router
        .register_rule("default".to_string(), "default".to_string())
        .map_err(std::io::Error::other)?;

    // Check health status of all agents
    info!("Checking health status of all agents...");
    let health_statuses = router.check_all_agents_health().await;
    for (agent_id, status) in &health_statuses {
        info!("Agent {} health status: {:?}", agent_id, status);
    }

    // No need for Handlebars templates anymore

    // Create application state
    let app_state = Data::new(AppState {
        router: Mutex::new(router),
        chat_history: Mutex::new(Vec::new()),
    });

    // Start HTTP server
    info!("Starting web server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(get_agents)
            .service(get_stats)
            .service(process_prompt)
            .service(check_health)
            .service(Files::new("/static", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
