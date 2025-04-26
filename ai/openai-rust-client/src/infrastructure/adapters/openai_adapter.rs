//! OpenAI adapter module
//! Implements the ChatGateway interface for OpenAI API

use crate::application::ports::output::ChatGateway;
use crate::domain::entities::message::Message;
use crate::infrastructure::config::app_config::AppConfig;
use futures::future::Future;
use futures::{Stream, StreamExt, TryStreamExt};
use reqwest::header::HeaderMap;
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// OpenAI request model
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    model: String,
    stream: bool,
}

/// OpenAI stream response model
#[derive(Debug, Deserialize)]
struct OpenAIStreamResponse {
    choices: Vec<OpenAIStreamChoice>,
}

/// OpenAI stream choice model
#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    delta: OpenAIDelta,
}

/// OpenAI delta model
#[derive(Debug, Deserialize)]
struct OpenAIDelta {
    content: Option<String>,
}

/// OpenAI adapter error
#[derive(Debug)]
pub enum OpenAIAdapterError {
    RequestFailed(ReqwestError),
    InvalidHeader(String),
    ApiError(String),
    StreamProcessingError(String),
}

impl std::fmt::Display for OpenAIAdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | Self::RequestFailed(err) => write!(f, "Request failed: {}", err),
            | Self::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            | Self::ApiError(msg) => write!(f, "API error: {}", msg),
            | Self::StreamProcessingError(msg) => write!(f, "Stream processing error: {}", msg),
        }
    }
}

impl std::error::Error for OpenAIAdapterError {}

impl From<ReqwestError> for OpenAIAdapterError {
    fn from(err: ReqwestError) -> Self { Self::RequestFailed(err) }
}

/// OpenAI adapter
/// Implements the ChatGateway interface for OpenAI API
pub struct OpenAIAdapter {
    client: Client,
    config: AppConfig,
}

impl OpenAIAdapter {
    /// Create a new OpenAI adapter
    pub fn new(config: AppConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Process a chunk of data from the OpenAI API stream
    fn process_chunk(chunk: &[u8]) -> Result<String, OpenAIAdapterError> {
        let chunk_str = String::from_utf8_lossy(chunk);
        let mut content = String::new();

        // Process each line in the chunk
        for line in chunk_str.split('\n') {
            let line = line.trim();
            if line.is_empty() || line == "\r" {
                continue;
            }

            // Remove the "data: " prefix if it exists
            let data = line.strip_prefix("data: ").unwrap_or(line);

            // Check for the stream end marker
            if data == "[DONE]" {
                break;
            }

            // Try to parse the data as JSON
            match serde_json::from_str::<OpenAIStreamResponse>(data) {
                | Ok(stream_response) => {
                    // Extract content from the stream response
                    if let Some(choice) = stream_response.choices.first() {
                        if let Some(content_str) = &choice.delta.content {
                            content.push_str(content_str);
                        }
                    }
                },
                | Err(e) => {
                    // Log parsing errors but continue processing
                    eprintln!("Error parsing JSON: {} for data: {}", e, data);
                },
            }
        }

        Ok(content)
    }
}

impl ChatGateway for OpenAIAdapter {
    type Error = OpenAIAdapterError;
    type MessageStream = Pin<Box<dyn Stream<Item = Result<String, Self::Error>> + Send>>;

    fn send_messages(
        &self,
        messages: Vec<Message>,
        model: &str,
        max_tokens: u32,
        temperature: f32,
        top_p: f32,
    ) -> impl Future<Output = Result<Self::MessageStream, Self::Error>> + Send + 'static {
        let client = self.client.clone();
        let endpoint = self.config.openai_endpoint.clone();
        let api_key = self.config.openai_api_key.clone();
        let model = model.to_string();

        Box::pin(async move {
            // Create OpenAI API request
            let openai_request = OpenAIRequest {
                messages,
                max_tokens,
                temperature,
                top_p,
                model,
                stream: true,
            };

            // Create headers with API key
            let mut headers = HeaderMap::new();
            headers.insert(
                "api-key",
                api_key
                    .parse()
                    .map_err(|_| OpenAIAdapterError::InvalidHeader("Invalid API key format".to_string()))?,
            );
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .map_err(|_| OpenAIAdapterError::InvalidHeader("Invalid content-type".to_string()))?,
            );

            // Send request to OpenAI API
            let response = client.post(&endpoint).headers(headers).json(&openai_request).send().await?;

            // Check if response is successful
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Err(OpenAIAdapterError::ApiError(format!("OpenAI API error: {}", error_text)));
            }

            // Get the response as a byte stream
            let stream = response.bytes_stream();

            // Process the stream and transform it into a text stream
            let stream = stream
                .map(move |chunk_result| {
                    chunk_result
                        .map_err(|e| OpenAIAdapterError::StreamProcessingError(e.to_string()))
                        .and_then(|chunk| Self::process_chunk(&chunk))
                })
                // Do NOT map errors to strings; propagate them as typed errors
                .filter(|result| {
                    futures::future::ready(match result {
                        Ok(content) => !content.is_empty(),
                        Err(_) => true, // Propagate errors down the stream
                    })
                })
                .boxed();

            Ok(stream)
        })
    }
}
