//! OpenAI / Azure OpenAI LLM 클라이언트

use crate::{config::RagConfig,
            error::RagError};
use serde::{Deserialize,
            Serialize};

/// LLM 클라이언트
pub struct LlmClient {
    http: reqwest::Client,
    api_url: String,
    api_key: String,
    model: String,
    temperature: Option<f64>,
    is_reasoning: bool,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    error: ApiErrorDetail,
}

#[derive(Deserialize)]
struct ApiErrorDetail {
    message: String,
}

impl LlmClient {
    /// LLM 클라이언트를 생성한다.
    ///
    /// # Errors
    ///
    /// 필수 환경변수(API 키, 엔드포인트)가 누락되면 `RagError::Config`를
    /// 반환한다.
    pub fn new(config: &RagConfig) -> Result<Self, RagError> {
        let (api_url, api_key, model) = if config.use_azure {
            let endpoint = config
                .azure_endpoint
                .as_deref()
                .ok_or_else(|| RagError::Config("AZURE_OPENAI_ENDPOINT가 설정되지 않았습니다".into()))?;
            let key = config
                .azure_api_key
                .as_deref()
                .ok_or_else(|| RagError::Config("AZURE_OPENAI_API_KEY가 설정되지 않았습니다".into()))?;

            let url = format!(
                "{}/openai/deployments/{}/chat/completions?api-version={}",
                endpoint.trim_end_matches('/'),
                config.azure_chat_deployment,
                config.azure_api_version,
            );
            (url, key.to_string(), config.azure_chat_deployment.clone())
        } else {
            let key = config
                .openai_api_key
                .as_deref()
                .ok_or_else(|| RagError::Config("OPENAI_API_KEY가 설정되지 않았습니다".into()))?;

            (
                "https://api.openai.com/v1/chat/completions".to_string(),
                key.to_string(),
                config.openai_model.clone(),
            )
        };

        let temperature = if config.is_reasoning_model() { None } else { config.temperature };

        Ok(Self {
            http: reqwest::Client::new(),
            api_url,
            api_key,
            model,
            temperature,
            is_reasoning: config.is_reasoning_model(),
        })
    }

    /// 채팅 완성을 요청한다.
    ///
    /// # Errors
    ///
    /// HTTP 요청 실패 또는 API 오류 시 에러를 반환한다.
    pub async fn chat(&self, system_prompt: &str, user_message: &str) -> Result<String, RagError> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_message.to_string(),
                },
            ],
            temperature: self.temperature,
        };

        let mut req_builder = self.http.post(&self.api_url).json(&request);

        // Azure vs OpenAI 인증 헤더
        if self.api_url.contains("openai.azure.com") || self.api_url.contains("cognitiveservices.azure.com") {
            req_builder = req_builder.header("api-key", &self.api_key);
        } else {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.map_err(RagError::Http)?;
            if let Ok(err) = serde_json::from_str::<ApiErrorResponse>(&body) {
                return Err(RagError::Api {
                    message: format!("[{status}] {}", err.error.message),
                });
            }
            return Err(RagError::Api {
                message: format!("[{status}] {body}"),
            });
        }

        let chat_response: ChatResponse = response.json().await?;
        chat_response.choices.into_iter().next().map(|c| c.message.content).ok_or(RagError::Api {
            message: "응답에 choices가 비어있습니다".into(),
        })
    }

    /// reasoning 모델 여부를 반환한다.
    #[must_use]
    pub const fn is_reasoning_model(&self) -> bool { self.is_reasoning }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_request_에서_reasoning_모델은_temperature를_제외한다() {
        let request = ChatRequest {
            model: "gpt-5-mini".to_string(),
            messages: vec![],
            temperature: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("temperature"));
    }
}
