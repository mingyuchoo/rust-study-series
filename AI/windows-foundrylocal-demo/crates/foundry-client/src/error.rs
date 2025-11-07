use thiserror::Error;

pub type Result<T> = std::result::Result<T, FoundryClientError>;

#[derive(Error, Debug)]
pub enum FoundryClientError {
    #[error("Failed to initialize Foundry Local manager: {0}")]
    ManagerInitFailed(String),

    #[error("Failed to get endpoint: {0}")]
    EndpointFailed(String),

    #[error("Failed to build HTTP client: {0}")]
    HttpClientBuildFailed(String),

    #[error("Failed to send request: {0}")]
    RequestFailed(String),

    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("Failed to read response: {0}")]
    ResponseReadFailed(String),

    #[error("Received empty response from API")]
    EmptyResponse,

    #[error("Failed to parse JSON response: {error}\nResponse: {response}")]
    JsonParseFailed { error: String, response: String },
}
