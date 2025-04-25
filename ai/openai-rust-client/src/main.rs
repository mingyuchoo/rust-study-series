use axum::{
    body::StreamBody,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use futures::stream::StreamExt;
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::{env, fmt, net::SocketAddr, sync::Arc};

use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

/// Server configuration constants
const SERVER_PORT: u16 = 3000;
const SERVER_HOST: [u8; 4] = [127, 0, 0, 1];

/// OpenAI API configuration constants
const MAX_TOKENS: u32 = 4096;
const TEMPERATURE: f32 = 1.0;
const TOP_P: f32 = 1.0;

/// Custom error type for the application
#[derive(Debug)]
enum AppError {
    EnvVarMissing(String),
    RequestFailed(ReqwestError),
    InvalidHeader(String),
    ApiError(String),
    StreamProcessingError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnvVarMissing(var) => write!(f, "Environment variable missing: {}", var),
            Self::RequestFailed(err) => write!(f, "Request failed: {}", err),
            Self::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            Self::ApiError(msg) => write!(f, "API error: {}", msg),
            Self::StreamProcessingError(msg) => write!(f, "Stream processing error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ReqwestError> for AppError {
    fn from(err: ReqwestError) -> Self {
        Self::RequestFailed(err)
    }
}

impl From<AppError> for StatusCode {
    fn from(err: AppError) -> Self {
        match err {
            AppError::EnvVarMissing(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RequestFailed(_) => StatusCode::BAD_GATEWAY,
            AppError::InvalidHeader(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ApiError(_) => StatusCode::BAD_GATEWAY,
            AppError::StreamProcessingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Create a shared state for the application
    let app_state = Arc::new(AppState::new()?);

    // Create the static file server for the frontend
    let static_files_service = ServeDir::new("static");
    
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/chat", post(chat))
        .nest_service("/static", static_files_service)
        .layer(cors)
        .with_state(app_state);

    // Run the server
    let addr = SocketAddr::from((SERVER_HOST, SERVER_PORT));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Serve the index.html file
async fn serve_index() -> impl IntoResponse {
    let html = include_str!("../static/index.html");
    (StatusCode::OK, [("Content-Type", "text/html")], html)
}

/// Application state containing client and configuration
#[derive(Debug, Clone)]
struct AppState {
    client: Client,
    openai_api_key: String,
    openai_endpoint: String,
    openai_model: String,
}

impl AppState {
    /// Create a new AppState with values from environment variables
    fn new() -> Result<Self, AppError> {
        Ok(Self {
            client: Client::new(),
            openai_api_key: env::var("AZURE_API_KEY")
                .map_err(|_| AppError::EnvVarMissing("AZURE_API_KEY".to_string()))?,
            openai_endpoint: env::var("OPENAI_ENDPOINT")
                .map_err(|_| AppError::EnvVarMissing("OPENAI_ENDPOINT".to_string()))?,
            openai_model: env::var("OPENAI_MODEL")
                .map_err(|_| AppError::EnvVarMissing("OPENAI_MODEL".to_string()))?,
        })
    }
}

/// Represents a role in the conversation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

/// Module to serialize/deserialize Role as string
mod role_as_str {
    use super::Role;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(role: &Role, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let role_str = match role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        };
        serializer.serialize_str(role_str)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Role, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "system" => Ok(Role::System),
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            _ => Ok(Role::User), // Default to user for unknown roles
        }
    }
}

/// Request and response types
#[derive(Debug, Deserialize)]
struct ChatRequest {
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    #[serde(with = "role_as_str")]
    role: Role,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    model: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamResponse {
    choices: Vec<OpenAIStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    delta: OpenAIDelta,
}

#[derive(Debug, Deserialize)]
struct OpenAIDelta {
    content: Option<String>,
}

/// Chat endpoint handler
async fn chat(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatRequest>,
) -> Result<Response, StatusCode> {
    let result = process_chat(state, request).await;
    match result {
        Ok(response) => Ok(response),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err.into())
        }
    }
}

/// Process chat request and return streaming response
async fn process_chat(
    state: Arc<AppState>,
    request: ChatRequest,
) -> Result<Response, AppError> {
    // Create OpenAI API request
    let openai_request = OpenAIRequest {
        messages: request.messages,
        max_tokens: MAX_TOKENS,
        temperature: TEMPERATURE,
        top_p: TOP_P,
        model: state.openai_model.clone(),
        stream: true,
    };

    // Create headers with API key
    let mut headers = HeaderMap::new();
    headers.insert(
        "api-key",
        state.openai_api_key.parse()
            .map_err(|_| AppError::InvalidHeader("Invalid API key format".to_string()))?,
    );
    headers.insert(
        "Content-Type",
        "application/json".parse()
            .map_err(|_| AppError::InvalidHeader("Invalid content-type".to_string()))?,
    );

    // Send request to OpenAI API
    let response = state
        .client
        .post(&state.openai_endpoint)
        .headers(headers)
        .json(&openai_request)
        .send()
        .await?;

    // Check if response is successful
    if !response.status().is_success() {
        let error_text = response.text().await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::ApiError(format!("OpenAI API error: {}", error_text)));
    }

    // Get the response as a byte stream
    let stream = response.bytes_stream();

    // Process the stream and transform it into a text stream
    let stream = stream
        .map(move |chunk_result| {
            chunk_result
                .map_err(|e| AppError::StreamProcessingError(e.to_string()))
                .and_then(|chunk| process_chunk(&chunk))
        })
        .filter(|result| {
            futures::future::ready(match result {
                Ok(content) => !content.is_empty(),
                Err(_) => true, // Keep errors in the stream
            })
        })
        .map(|result| -> Result<String, std::io::Error> {
            match result {
                Ok(content) => Ok(content),
                Err(e) => Ok(format!("Error: {}", e)),
            }
        });

    // Create a response with the stream body
    let body = StreamBody::new(stream);
    let mut response = Response::new(body.into_response().into_body());
    
    // Set appropriate headers for streaming text
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse()
            .map_err(|_| AppError::InvalidHeader("Invalid content-type header".to_string()))?,
    );
    
    Ok(response)
}

/// Process a chunk of data from the OpenAI API stream
fn process_chunk(chunk: &[u8]) -> Result<String, AppError> {
    let chunk_str = String::from_utf8_lossy(chunk);
    let mut content = String::new();
    
    // Process each line in the chunk
    for line in chunk_str.split('\n') {
        let line = line.trim();
        if line.is_empty() || line == "\r" {
            continue;
        }
        
        // Remove the "data: " prefix if it exists
        let data = if line.starts_with("data: ") {
            &line[6..]
        } else {
            line
        };
        
        // Check for the stream end marker
        if data == "[DONE]" {
            break;
        }
        
        // Try to parse the data as JSON
        match serde_json::from_str::<OpenAIStreamResponse>(data) {
            Ok(stream_response) => {
                // Extract content from the stream response
                if let Some(choice) = stream_response.choices.first() {
                    if let Some(content_str) = &choice.delta.content {
                        content.push_str(content_str);
                    }
                }
            },
            Err(e) => {
                // Log parsing errors but continue processing
                eprintln!("Error parsing JSON: {} for data: {}", e, data);
            }
        }
    }
    
    Ok(content)
}
