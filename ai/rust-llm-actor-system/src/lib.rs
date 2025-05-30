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
        match is_error {
            | true => {
                self.error_count.fetch_add(1, Ordering::SeqCst);
            },
            | false => {},
        }
    }

    pub fn get_stats(&self) -> (usize, usize, usize, f64) {
        let prompts = self.prompt_count.load(Ordering::SeqCst);
        let errors = self.error_count.load(Ordering::SeqCst);
        let total_time = self.total_processing_time_ms.load(Ordering::SeqCst);

        let avg_time = match prompts {
            | 0 => 0.0,
            | _ => total_time as f64 / prompts as f64,
        };

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

    pub fn get_id(&self) -> &str { &self.id }

    pub fn get_model(&self) -> &String { &self.model }

    pub fn get_system_prompt(&self) -> &str { &self.system_prompt }

    pub fn update_health_status(&mut self, status: HealthStatus) {
        self.health_status = status;
        self.last_health_check = Instant::now();
        info!("Updated health status for model {}: {:?}", self.model, status);
    }

    pub async fn check_health(&mut self) -> HealthStatus {
        let now = Instant::now();
        match now.duration_since(self.last_health_check) > Duration::from_secs(60) {
            | true => {
                // Simulate random health status for demo purposes
                let rand_num = (now.elapsed().as_millis() % 10) as u8;
                let new_status = match rand_num {
                    | 0 ..= 7 => HealthStatus::Healthy,
                    | 8 => HealthStatus::Degraded,
                    | _ => HealthStatus::Unhealthy,
                };
                self.update_health_status(new_status);
            },
            | false => {},
        }
        self.health_status
    }

    pub async fn process_prompt(&self, prompt: String) -> Result<String> {
        use futures::StreamExt;
        use reqwest::Client;
        use serde_json::json;
        use std::io::{self, Write};

        info!("Processing prompt with model: {}", self.model);
        info!("Agent {} is processing: {}", self.id, prompt);

        match self.health_status {
            | HealthStatus::Unhealthy => return Err(anyhow!("Agent is unhealthy and cannot process prompts")),
            | _ => {},
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
            "stream": true,  // Enable streaming responses
            "messages": [
                { "role": "system", "content": self.system_prompt },
                { "role": "user", "content": prompt }
            ]
        });

        // Print agent header to console
        println!(
            "
[{}] 응답 스트리밍 시작:
",
            self.id
        );
        io::stdout().flush().ok();

        // Send request with streaming enabled
        let resp = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .header("api-key", api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        // Process the streaming response
        let mut stream = resp.bytes_stream();
        let mut full_response = String::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            let chunk_str = String::from_utf8_lossy(&chunk);

            // Split the chunk by lines (each line is a separate SSE event)
            for line in chunk_str.lines() {
                // Skip empty lines and "data: [DONE]" messages
                if line.is_empty() || !line.starts_with("data:") || line.contains("[DONE]") {
                    continue;
                }

                // Parse the JSON data
                if let Some(json_str) = line.strip_prefix("data: ") {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                        // Extract the content delta if it exists
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            // Append to the full response
                            full_response.push_str(content);

                            // Print the content delta to console
                            print!("{}", content);
                            io::stdout().flush().ok();
                        }
                    }
                }
            }
        }

        // Print newline after streaming is complete
        println!(
            "
[{}] 응답 스트리밍 완료
",
            self.id
        );
        io::stdout().flush().ok();

        // If we didn't get any response, return a default message
        if full_response.is_empty() {
            full_response = "(No response)".to_string();
        }

        Ok(full_response)
    }
}

// Message types for Router
#[derive(Debug)]
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
    manager_agent_id: Option<String>,
    metrics: Arc<Metrics>,
}

impl Default for AgentRouter {
    fn default() -> Self { Self::new() }
}

impl AgentRouter {
    pub fn get_agents(&self) -> &HashMap<String, LLMActor> { &self.agents }

    pub fn new() -> Self {
        dotenv().ok(); // Initialize dotenv
        Self {
            agents: HashMap::new(),
            routing_rules: Vec::new(),
            manager_agent_id: None,
            metrics: Arc::new(Metrics::new()),
        }
    }

    pub fn add_agent(&mut self, agent_id: String, model: String, system_prompt: String) {
        info!("Adding new LLM agent: {} with model: {}", agent_id, model);
        self.agents.insert(agent_id.clone(), LLMActor::new(agent_id, model, system_prompt));
    }

    pub fn get_metrics(&self) -> Arc<Metrics> { self.metrics.clone() }

    pub fn set_manager_agent(&mut self, agent_id: String) -> Result<()> {
        match self.agents.contains_key(&agent_id) {
            | false => Err(anyhow!("Agent with ID {} not found", agent_id)),
            | true => {
                info!("Setting manager agent to: {}", agent_id);
                self.manager_agent_id = Some(agent_id);
                Ok(())
            },
        }
    }

    pub fn register_rule(&mut self, keyword: String, agent_id: String) -> Result<()> {
        // Default priority and confidence threshold
        self.register_rule_with_priority(keyword, agent_id, 5, 0.5)
    }

    pub fn register_rule_with_priority(&mut self, keyword: String, agent_id: String, priority: u8, confidence_threshold: f64) -> Result<()> {
        match self.agents.contains_key(&agent_id) {
            | false => return Err(anyhow!("Agent with ID {} not found", agent_id)),
            | true => {},
        }

        // Special case for manager rule
        match keyword.as_str() {
            | "manager" => return self.set_manager_agent(agent_id),
            | _ => {},
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
        // First try to use OpenAI to determine the best agent
        match self.select_agent_with_openai(prompt).await {
            | Ok(agent) => {
                info!("Selected agent {} using OpenAI recommendation", agent.get_id());
                return Ok(agent);
            },
            | Err(e) => {
                warn!("Failed to select agent using OpenAI: {}, falling back to rule-based selection", e);
                // Continue with the rule-based approach as fallback
            },
        }

        // Fallback: Track best match using rule-based approach
        let mut best_match: Option<(&RoutingRule, f64)> = None;

        // Check each rule in priority order
        for rule in &self.routing_rules {
            match prompt.to_lowercase().contains(&rule.keyword.to_lowercase()) {
                | true => {
                    // Calculate confidence for this match
                    let confidence = self.calculate_confidence(prompt, &rule.keyword);

                    // If confidence meets the threshold and is better than current best match
                    match (confidence >= rule.confidence_threshold, best_match) {
                        | (true, None) => best_match = Some((rule, confidence)),
                        | (true, Some((_, current_best))) if confidence > current_best => best_match = Some((rule, confidence)),
                        | _ => {},
                    }
                },
                | false => {},
            }
        }

        // If we found a match, use that agent
        match best_match {
            | Some((rule, confidence)) => {
                info!(
                    "Routing prompt to agent {} based on keyword: {} (confidence: {:.2})",
                    rule.agent_id, rule.keyword, confidence
                );

                match self.agents.get(&rule.agent_id) {
                    | Some(agent) => {
                        // Check if agent is healthy
                        match agent.get_health_status() {
                            | HealthStatus::Unhealthy => {
                                warn!("Selected agent {} is unhealthy, falling back to default", rule.agent_id);
                            },
                            | _ => return Ok(agent),
                        }
                    },
                    | None => {},
                }
            },
            | None => {},
        }

        // If no specific rule matches or selected agent is unhealthy, use manager agent
        // if available
        match &self.manager_agent_id {
            | Some(manager_id) => {
                info!("Routing prompt to manager agent: {}", manager_id);
                match self.agents.get(manager_id) {
                    | Some(agent) => {
                        // Only use manager agent if it's not unhealthy
                        match agent.get_health_status() {
                            | HealthStatus::Unhealthy => {
                                warn!("Manager agent {} is unhealthy", manager_id);
                            },
                            | _ => return Ok(agent),
                        }
                    },
                    | None => {},
                }
            },
            | None => {},
        }

        // If no manager agent is defined or it's unhealthy, try to find any healthy
        // agent
        for (agent_id, agent) in &self.agents {
            match agent.get_health_status() {
                | HealthStatus::Unhealthy => {},
                | _ => {
                    warn!("No suitable agent found, using available healthy agent: {}", agent_id);
                    return Ok(agent);
                },
            }
        }

        // If all agents are unhealthy, return an error
        Err(anyhow!("No suitable healthy agent found for the prompt"))
    }

    async fn select_agent_with_openai(&self, prompt: &str) -> Result<&LLMActor> {
        use reqwest::Client;
        use serde::Deserialize;
        use serde_json::json;

        #[derive(Deserialize)]
        struct OpenAIResponse {
            choices: Vec<Choice>,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: Message,
        }

        #[derive(Deserialize)]
        struct Message {
            content: String,
        }

        // Get available agent IDs for the system prompt
        let agent_ids: Vec<String> = self.agents.keys().cloned().collect();
        let agent_descriptions: Vec<String> = self.agents.iter().map(|(id, agent)| format!("{}: {}", id, agent.get_system_prompt())).collect();

        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| anyhow!("OPENAI_API_KEY not set"))?;
        let api_url = env::var("OPENAI_API_URL").map_err(|_| anyhow!("OPENAI_API_URL not set"))?;
        let api_model = env::var("OPENAI_API_MODEL").map_err(|_| anyhow!("OPENAI_API_MODEL not set"))?;

        // Use a simpler model for routing decisions to save costs
        let model = api_model;

        let system_prompt = format!(
            "You are a routing assistant. Your job is to analyze the user's prompt and select the most appropriate agent to handle it.

Available agents:
{}

Respond ONLY with the ID of the single best agent to handle this prompt. Your response must be exactly one of these IDs: {}. Do not include any explanation or additional text.",
            agent_descriptions.join("\n"),
            agent_ids.join(", ")
        );

        let client = Client::new();
        let body = json!({
            "model": model,
            "max_tokens": 50,  // Short response is all we need
            "temperature": 0.3,  // Lower temperature for more deterministic responses
            "messages": [
                { "role": "system", "content": system_prompt },
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

        let resp_json: OpenAIResponse = resp.json().await?;
        let agent_id = resp_json
            .choices
            .first()
            .ok_or_else(|| anyhow!("No choices in OpenAI response"))?
            .message
            .content
            .trim()
            .to_string();

        // Validate that the returned agent_id is one of our agents
        match self.agents.contains_key(&agent_id) {
            | true => match self.agents.get(&agent_id) {
                | Some(agent) => match agent.get_health_status() {
                    | HealthStatus::Unhealthy => Err(anyhow!("Selected agent is unhealthy")),
                    | _ => Ok(agent),
                },
                | None => Err(anyhow!("Agent not found despite key check")),
            },
            | false => Err(anyhow!("OpenAI returned invalid agent ID: {}", agent_id)),
        }
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
        stats.insert("router_avg_processing_time_ms".to_string(), format!("{avg_time:.2}"));

        // Agent metrics
        for (agent_id, agent) in &self.agents {
            let (prompts, errors, _, avg_time) = agent.get_metrics().get_stats();
            stats.insert(format!("agent_{agent_id}_prompts"), prompts.to_string());
            stats.insert(format!("agent_{agent_id}_errors"), errors.to_string());
            stats.insert(format!("agent_{agent_id}_avg_time_ms"), format!("{avg_time:.2}"));
            stats.insert(format!("agent_{agent_id}_health"), format!("{:?}", agent.get_health_status()));
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
            health: format!("{health_status:?}"),
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

    // Sequential agent delegation logic
    let router = app_state.router.lock().await;
    let mut current_input = prompt.clone();
    let mut last_agent_id = String::new();
    let agent_sequence = ["manager", "analysis_agent", "design_agent", "coding_agent", "testing_agent"];

    for agent_id in agent_sequence.iter() {
        let agent = match router.get_agents().get(*agent_id) {
            | Some(agent) => agent,
            | None => {
                let error_msg = format!("Agent '{agent_id}' not found");
                error!("{error_msg}");
                let error_message = ChatMessage {
                    id: Uuid::new_v4().to_string(),
                    content: error_msg.clone(),
                    agent_id: Some("system".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                    message_type: "agent".to_string(),
                };
                app_state.chat_history.lock().await.push(error_message);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": error_msg
                })));
            },
        };
        match agent.process_prompt(current_input.clone()).await {
            | Ok(output) => {
                last_agent_id = agent_id.to_string();
                // Log and store each agent's output in chat history
                let agent_message = ChatMessage {
                    id: Uuid::new_v4().to_string(),
                    content: output.clone(),
                    agent_id: Some(agent_id.to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                    message_type: "agent".to_string(),
                };
                app_state.chat_history.lock().await.push(agent_message);
                current_input = output;
            },
            | Err(e) => {
                error!("Error from agent {}: {:?}", agent_id, e);
                let error_message = ChatMessage {
                    id: Uuid::new_v4().to_string(),
                    content: format!("Error from {agent_id}: {e}"),
                    agent_id: Some(agent_id.to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                    message_type: "agent".to_string(),
                };
                app_state.chat_history.lock().await.push(error_message);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Error from {agent_id}: {e}")
                })));
            },
        }
    }

    // Final response from testing_agent
    let prompt_response = PromptResponse {
        response: current_input,
        agent_id: last_agent_id,
        timestamp: Utc::now().to_rfc3339(),
    };
    Ok(HttpResponse::Ok().json(prompt_response))
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

// Function to create and configure an agent router with manager agent and
// rules
pub fn create_agent_router() -> std::io::Result<AgentRouter> {
    // Create the agent router
    let mut router = AgentRouter::new();

    // Add some initial LLM agents
    let model_name = env::var("OPENAI_API_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
    let agent_configs = vec![
        (
            "analysis_agent",
            model_name.as_str(),
            "당신은 요구사항 분석 전문가입니다. 요구사항 분석 질문에만 자세히 답변하세요.",
        ),
        (
            "design_agent",
            model_name.as_str(),
            "당신은 소프트웨어 설계 전문가입니다. 소프트웨어 설계 질문에만 자세히 답변하세요.",
        ),
        (
            "coding_agent",
            model_name.as_str(),
            "당신은 소프트웨어 구현 전문가입니다. 소프트웨어 구현 부분만 프로그래밍 언어로 답변하세요.",
        ),
        (
            "testing_agent",
            model_name.as_str(),
            "당신은 소프트웨어 테스트 전문가입니다. 소프트웨어 테스트 질문에만 요청받은 프로그래밍 언어에 해당하는 단위테스트 코드로 자세히 답변하세요.",
        ),
        (
            "manager",
            model_name.as_str(),
            "당신은 도움이 되는, 소프트웨어 개발 프로젝트 관리자입니다. 모든 질문에 대해 의도를 분석하고 각 전문가에게 질문을 전달하세요.",
        ),
    ];

    for (agent_id, model, system_prompt) in agent_configs {
        router.add_agent(agent_id.to_string(), model.to_string(), system_prompt.to_string());
        info!("Successfully added agent: {} with model: {}", agent_id, model);
    }

    // Register routing rules with different priorities and confidence thresholds
    router
        .register_rule_with_priority("analysis".to_string(), "analysis_agent".to_string(), 10, 0.6)
        .map_err(std::io::Error::other)?;
    router
        .register_rule_with_priority("design".to_string(), "design_agent".to_string(), 10, 0.6)
        .map_err(std::io::Error::other)?;
    router
        .register_rule_with_priority("coding".to_string(), "coding_agent".to_string(), 10, 0.6)
        .map_err(std::io::Error::other)?;
    router
        .register_rule_with_priority("testing".to_string(), "testing_agent".to_string(), 8, 0.5)
        .map_err(std::io::Error::other)?;
    router
        .register_rule("manager".to_string(), "manager".to_string())
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
