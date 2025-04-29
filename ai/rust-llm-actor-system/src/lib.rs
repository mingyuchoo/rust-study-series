// Web application imports
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder, Result as ActixResult, get, post, web};
use anyhow::{Result, anyhow};
use chrono::Utc;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::oneshot;
use tracing::{Level, error, info, warn};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

// Message types for LLM Actor
#[derive(Debug)]
#[allow(dead_code)]
pub enum LLMMessage {
    ProcessPrompt { prompt: String, reply: oneshot::Sender<Result<String>> },
    HealthCheck { reply: oneshot::Sender<bool> },
    UpdateModel(String),
}

// Metrics for tracking system performance
pub struct Metrics {
    prompt_count: AtomicUsize,
    error_count: AtomicUsize,
    total_processing_time_ms: AtomicUsize,
    last_reset: Mutex<Instant>,
}

impl Default for Metrics {
    fn default() -> Self { Self::new() }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            prompt_count: AtomicUsize::new(0),
            error_count: AtomicUsize::new(0),
            total_processing_time_ms: AtomicUsize::new(0),
            last_reset: Mutex::new(Instant::now()),
        }
    }

    pub fn record_prompt(&self, duration_ms: u64, is_error: bool) {
        self.prompt_count.fetch_add(1, Ordering::SeqCst);
        self.total_processing_time_ms.fetch_add(duration_ms as usize, Ordering::SeqCst);
        if is_error {
            self.error_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn get_stats(&self) -> (usize, usize, usize, f64) {
        let prompts = self.prompt_count.load(Ordering::SeqCst);
        let errors = self.error_count.load(Ordering::SeqCst);
        let total_time = self.total_processing_time_ms.load(Ordering::SeqCst);

        let avg_time = if prompts > 0 { total_time as f64 / prompts as f64 } else { 0.0 };

        (prompts, errors, total_time, avg_time)
    }

    pub fn reset(&self) {
        self.prompt_count.store(0, Ordering::SeqCst);
        self.error_count.store(0, Ordering::SeqCst);
        self.total_processing_time_ms.store(0, Ordering::SeqCst);
        *self.last_reset.lock().unwrap() = Instant::now();
    }
}

// Health status for LLM actors
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

// LLM Actor implementation
pub struct LLMActor {
    id: String,
    model: String,
    health_status: HealthStatus,
    last_health_check: Instant,
    metrics: Arc<Metrics>,
    system_prompt: String,
}

impl LLMActor {
    pub fn new(id: String, model: String, system_prompt: String) -> Self {
        Self {
            id,
            model,
            health_status: HealthStatus::Healthy,
            last_health_check: Instant::now(),
            metrics: Arc::new(Metrics::new()),
            system_prompt,
        }
    }

    pub fn get_metrics(&self) -> Arc<Metrics> { self.metrics.clone() }

    pub fn get_health_status(&self) -> HealthStatus { self.health_status }

    pub fn get_id(&self) -> &String { &self.id }

    pub fn get_model(&self) -> &String { &self.model }

    pub fn update_health_status(&mut self, status: HealthStatus) {
        self.health_status = status;
        self.last_health_check = Instant::now();
        info!("Updated health status for model {}: {:?}", self.model, status);
    }

    pub async fn check_health(&mut self) -> HealthStatus {
        // In a real implementation, this would perform a health check
        // For now, we'll just simulate a health check
        let now = Instant::now();
        if now.duration_since(self.last_health_check) > Duration::from_secs(60) {
            // Simulate random health status for demo purposes
            let rand_num = (now.elapsed().as_millis() % 10) as u8;
            let new_status = match rand_num {
                | 0 ..= 7 => HealthStatus::Healthy,
                | 8 => HealthStatus::Degraded,
                | _ => HealthStatus::Unhealthy,
            };
            self.update_health_status(new_status);
        }
        self.health_status
    }

    pub async fn process_prompt(&self, prompt: String) -> Result<String> {
        use reqwest::Client;
        use serde_json::json;
        info!("Lib> Processing prompt with model: {}", self.model);
        if self.health_status == HealthStatus::Unhealthy {
            return Err(anyhow!("Agent is unhealthy and cannot process prompts"));
        }
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| anyhow!("OPENAI_API_KEY not set"))?;
        let api_url = env::var("OPENAI_API_URL").map_err(|_| anyhow!("OPENAI_API_URL not set"))?;
        let model = env::var("OPENAI_API_MODEL").unwrap_or_else(|_| self.model.clone());
        let max_tokens = env::var("OPENAI_API_MAX_TOKENS").ok().and_then(|v| v.parse::<u16>().ok()).unwrap_or(1024u16);
        let temperature = env::var("OPENAI_API_TEMPERATURE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
        let top_p = env::var("OPENAI_API_TOP_P").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);

        let client = Client::new();
        let body = json!({
            "model": model,
            "max_tokens": max_tokens,
            "temperature": temperature,
            "top_p": top_p,
            "messages": [
                { "role": "system", "content": self.system_prompt },
                { "role": "user", "content": prompt }
            ]
        });

        let resp = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .header("api-key", api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let resp_json: serde_json::Value = resp.json().await?;
        let answer = resp_json["choices"][0]["message"]["content"].as_str().unwrap_or("(No response)").to_string();
        Ok(answer)
    }
}

// Message types for Router
#[derive(Debug)]
#[allow(dead_code)]
pub enum RouterMessage {
    RoutePrompt { prompt: String, reply: oneshot::Sender<Result<String>> },
    RegisterRoutingRule { keyword: String, agent_id: String },
    RemoveRoutingRule { keyword: String },
}

// Routing rule with priority and confidence threshold
pub struct RoutingRule {
    keyword: String,
    agent_id: String,
    priority: u8,              // Higher number means higher priority
    confidence_threshold: f64, // Minimum confidence score to use this rule (0.0-1.0)
}

// Router implementation
pub struct AgentRouter {
    agents: HashMap<String, LLMActor>,
    routing_rules: Vec<RoutingRule>, // Prioritized routing rules
    default_agent_id: Option<String>,
    metrics: Arc<Metrics>,
}

impl Default for AgentRouter {
    fn default() -> Self { Self::new() }
}

impl AgentRouter {
    pub fn get_agents(&self) -> &HashMap<String, LLMActor> { &self.agents }

    pub fn new() -> Self {
        dotenv().ok(); // 환경변수 초기화
        Self {
            agents: HashMap::new(),
            routing_rules: Vec::new(),
            default_agent_id: None,
            metrics: Arc::new(Metrics::new()),
        }
    }

    pub fn add_agent(&mut self, agent_id: String, model: String, system_prompt: String) {
        info!("Adding new LLM agent: {} with model: {}", agent_id, model);
        self.agents.insert(agent_id.clone(), LLMActor::new(agent_id, model, system_prompt));
    }

    #[allow(dead_code)]
    pub fn get_metrics(&self) -> Arc<Metrics> { self.metrics.clone() }

    pub fn set_default_agent(&mut self, agent_id: String) -> Result<()> {
        if !self.agents.contains_key(&agent_id) {
            return Err(anyhow!("Agent with ID {} not found", agent_id));
        }
        info!("Setting default agent to: {}", agent_id);
        self.default_agent_id = Some(agent_id);
        Ok(())
    }

    pub fn register_rule(&mut self, keyword: String, agent_id: String) -> Result<()> {
        // Default priority and confidence threshold
        self.register_rule_with_priority(keyword, agent_id, 5, 0.5)
    }

    pub fn register_rule_with_priority(&mut self, keyword: String, agent_id: String, priority: u8, confidence_threshold: f64) -> Result<()> {
        if !self.agents.contains_key(&agent_id) {
            return Err(anyhow!("Agent with ID {} not found", agent_id));
        }

        // Special case for default rule
        if keyword == "default" {
            return self.set_default_agent(agent_id);
        }

        info!(
            "Registering routing rule: {} -> {} (priority: {}, threshold: {})",
            keyword, agent_id, priority, confidence_threshold
        );

        // Add the rule and sort by priority (highest first)
        self.routing_rules.push(RoutingRule {
            keyword,
            agent_id,
            priority,
            confidence_threshold,
        });

        // Sort rules by priority (highest first)
        self.routing_rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(())
    }

    #[allow(dead_code)]
    pub fn remove_rule(&mut self, keyword: &str) {
        info!("Removing routing rule for keyword: {}", keyword);
        self.routing_rules.retain(|rule| rule.keyword != keyword);
    }

    // Calculate confidence score for a rule match (0.0-1.0)
    fn calculate_confidence(&self, prompt: &str, keyword: &str) -> f64 {
        // Simple implementation: if keyword appears multiple times, higher confidence
        let prompt_lower = prompt.to_lowercase();
        let keyword_lower = keyword.to_lowercase();

        // Count occurrences of the keyword in the prompt
        let occurrences = prompt_lower.matches(&keyword_lower).count();

        // Calculate confidence based on occurrences and keyword length
        // Longer keywords with multiple occurrences get higher confidence
        let length_factor = keyword_lower.len() as f64 / 10.0; // Normalize by assuming 10 chars is a good length
        let occurrence_factor = (occurrences as f64).min(3.0) / 3.0; // Cap at 3 occurrences

        // Combine factors (max confidence is 1.0)
        (length_factor + occurrence_factor).min(1.0)
    }

    pub async fn check_all_agents_health(&self) -> HashMap<String, HealthStatus> {
        let mut results = HashMap::new();

        for (agent_id, agent) in &self.agents {
            // For now, just return the current health status without checking
            // In a real implementation, we would need to use interior mutability
            results.insert(agent_id.clone(), agent.get_health_status());
        }

        results
    }

    pub async fn select_best_agent(&self, prompt: &str) -> Result<&LLMActor> {
        // Track best match so far
        let mut best_match: Option<(&RoutingRule, f64)> = None;

        // Check each rule in priority order
        for rule in &self.routing_rules {
            if prompt.to_lowercase().contains(&rule.keyword.to_lowercase()) {
                // Calculate confidence for this match
                let confidence = self.calculate_confidence(prompt, &rule.keyword);

                // If confidence meets the threshold and is better than current best match
                if confidence >= rule.confidence_threshold && (best_match.is_none() || confidence > best_match.unwrap().1) {
                    best_match = Some((rule, confidence));
                }
            }
        }

        // If we found a match, use that agent
        if let Some((rule, confidence)) = best_match {
            info!(
                "Routing prompt to agent {} based on keyword: {} (confidence: {:.2})",
                rule.agent_id, rule.keyword, confidence
            );

            if let Some(agent) = self.agents.get(&rule.agent_id) {
                // Check if agent is healthy
                if agent.get_health_status() == HealthStatus::Unhealthy {
                    warn!("Selected agent {} is unhealthy, falling back to default", rule.agent_id);
                } else {
                    return Ok(agent);
                }
            }
        }

        // If no specific rule matches or selected agent is unhealthy, use default agent
        // if available
        if let Some(default_id) = &self.default_agent_id {
            info!("Routing prompt to default agent: {}", default_id);
            if let Some(agent) = self.agents.get(default_id) {
                // Only use default if it's not unhealthy
                if agent.get_health_status() != HealthStatus::Unhealthy {
                    return Ok(agent);
                }
                warn!("Default agent {} is unhealthy", default_id);
            }
        }

        // If no default agent is defined or it's unhealthy, try to find any healthy
        // agent
        for (agent_id, agent) in &self.agents {
            if agent.get_health_status() != HealthStatus::Unhealthy {
                warn!("No suitable agent found, using available healthy agent: {}", agent_id);
                return Ok(agent);
            }
        }

        // If all agents are unhealthy, return an error
        Err(anyhow!("No suitable healthy agent found for the prompt"))
    }

    pub async fn route_prompt(&self, prompt: String) -> Result<String> {
        // Start timing the routing process
        let start_time = Instant::now();
        let mut is_error = false;

        let result = match self.select_best_agent(&prompt).await {
            | Ok(agent) => agent.process_prompt(prompt).await,
            | Err(e) => {
                is_error = true;
                Err(e)
            },
        };

        // Record metrics for the router
        let duration_ms = start_time.elapsed().as_millis() as u64;
        self.metrics.record_prompt(duration_ms, is_error);

        result
    }

    pub fn get_system_stats(&self) -> HashMap<String, String> {
        let mut stats = HashMap::new();

        // Router metrics
        let (prompts, errors, _total_time, avg_time) = self.metrics.get_stats();
        stats.insert("router_total_prompts".to_string(), prompts.to_string());
        stats.insert("router_errors".to_string(), errors.to_string());
        stats.insert("router_avg_processing_time_ms".to_string(), format!("{:.2}", avg_time));

        // Agent metrics
        for (agent_id, agent) in &self.agents {
            let (prompts, errors, _, avg_time) = agent.get_metrics().get_stats();
            stats.insert(format!("agent_{}_prompts", agent_id), prompts.to_string());
            stats.insert(format!("agent_{}_errors", agent_id), errors.to_string());
            stats.insert(format!("agent_{}_avg_time_ms", agent_id), format!("{:.2}", avg_time));
            stats.insert(format!("agent_{}_health", agent_id), format!("{:?}", agent.get_health_status()));
        }

        stats
    }

    pub fn reset_metrics(&mut self) {
        // Reset router metrics
        self.metrics.reset();

        // Reset agent metrics
        for agent in self.agents.values() {
            agent.get_metrics().reset();
        }

        info!("All metrics have been reset");
    }
}

// Web Application Code

// Struct to hold application state
pub struct AppState {
    pub router: tokio::sync::Mutex<AgentRouter>,
    pub chat_history: tokio::sync::Mutex<Vec<ChatMessage>>,
}

// Struct for incoming prompt requests
#[derive(Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
}

// Struct for prompt responses
#[derive(Serialize)]
pub struct PromptResponse {
    pub response: String,
    pub agent_id: String,
    pub timestamp: String,
}

// Struct for agent information
#[derive(Serialize)]
pub struct AgentInfo {
    pub id: String,
    pub model: String,
    pub health: String,
    pub health_class: String,
    pub prompts: usize,
    pub avg_time: f64,
}

// Struct for chat messages
#[derive(Serialize, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub content: String,
    pub agent_id: Option<String>,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String, // "user" or "agent"
}

// Handler for the root page - redirects to static index.html
#[get("/")]
pub async fn index() -> ActixResult<HttpResponse> { Ok(HttpResponse::Found().append_header(("Location", "/static/index.html")).finish()) }

// API handler to get all agents
#[get("/api/agents")]
pub async fn get_agents(app_state: web::Data<AppState>) -> impl Responder {
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
            prompts,
            avg_time,
        });
    }

    HttpResponse::Ok().json(agents)
}

// API handler to get system stats
#[get("/api/stats")]
pub async fn get_stats(app_state: web::Data<AppState>) -> impl Responder {
    let router = app_state.router.lock().await;
    let stats = router.get_system_stats();

    HttpResponse::Ok().json(stats)
}

// API handler to process prompts
#[post("/api/prompt")]
pub async fn process_prompt(req: web::Json<PromptRequest>, app_state: web::Data<AppState>) -> ActixResult<HttpResponse> {
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
pub async fn check_health(app_state: web::Data<AppState>) -> impl Responder {
    let health_statuses = app_state.router.lock().await.check_all_agents_health().await;
    HttpResponse::Ok().json(health_statuses)
}

// Function to initialize the application
pub fn init_logging() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

// Function to create and configure an agent router with default agents and
// rules
pub fn create_agent_router() -> std::io::Result<AgentRouter> {
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

    Ok(router)
}

// Function to configure the web application
pub fn configure_app(cfg: &mut web::ServiceConfig, app_state: Data<AppState>) {
    cfg.app_data(app_state.clone())
        .service(index)
        .service(get_agents)
        .service(get_stats)
        .service(process_prompt)
        .service(check_health)
        .service(Files::new("/static", "./static").index_file("index.html"));
}
