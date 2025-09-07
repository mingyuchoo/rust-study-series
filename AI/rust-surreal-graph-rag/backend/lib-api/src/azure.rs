//! Azure OpenAI 호출 유틸리티
//! 모든 주석은 한국어로 작성됩니다.

use crate::config::AzureOpenAIConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequestBody {
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: Option<ChatMessage>,
}

#[derive(Debug, Deserialize)]
struct ChatResponseBody {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Serialize)]
struct EmbeddingInput<'a> {
    input: &'a [&'a str],
}

#[derive(Debug, Deserialize)]
struct EmbeddingDataItem {
    embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponseBody {
    data: Vec<EmbeddingDataItem>,
}

pub struct AzureOpenAI {
    client: Client,
    cfg: AzureOpenAIConfig,
}

impl AzureOpenAI {
    pub fn new(cfg: AzureOpenAIConfig) -> Self {
        Self {
            client: Client::new(),
            cfg,
        }
    }

    fn chat_url(&self) -> String {
        format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.cfg.endpoint.trim_end_matches('/'),
            self.cfg.chat_deployment,
            self.cfg.api_version,
        )
    }

    fn embed_url(&self) -> String {
        format!(
            "{}/openai/deployments/{}/embeddings?api-version={}",
            self.cfg.endpoint.trim_end_matches('/'),
            self.cfg.embed_deployment,
            self.cfg.api_version,
        )
    }

    pub async fn chat_complete(&self, system_prompt: &str, user_query: &str, temperature: Option<f32>) -> anyhow::Result<(String, u32)> {
        // 간단한 시스템/유저 메시지 구성
        let body = ChatRequestBody {
            messages: vec![
                ChatMessage { role: "system".into(), content: system_prompt.into() },
                ChatMessage { role: "user".into(), content: user_query.into() },
            ],
            temperature,
            max_tokens: None,
        };

        let resp = self.client
            .post(self.chat_url())
            .header("api-key", &self.cfg.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Azure OpenAI Chat 실패: {}", text);
        }

        let json: ChatResponseBody = resp.json().await?;
        let content = json
            .choices
            .get(0)
            .and_then(|c| c.message.as_ref())
            .map(|m| m.content.clone())
            .unwrap_or_default();
        // tokens_used를 usage에서 파싱하려면 스키마별 처리 필요. MVP로 0 반환.
        Ok((content, 0))
    }

    pub async fn embed(&self, texts: &[&str]) -> anyhow::Result<Vec<Vec<f32>>> {
        let body = EmbeddingInput { input: texts };
        let resp = self.client
            .post(self.embed_url())
            .header("api-key", &self.cfg.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Azure OpenAI Embedding 실패: {}", text);
        }

        let json: EmbeddingResponseBody = resp.json().await?;
        Ok(json.data.into_iter().map(|d| d.embedding).collect())
    }
}
