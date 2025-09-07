// Azure OpenAI REST 클라이언트
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::embedding_cache::EmbeddingVector;

#[derive(Clone)]
pub struct AzureOpenAI {
    pub endpoint: String, // 예: https://<resource>.openai.azure.com
    pub api_key: String,
    pub api_version: String, // 예: 2024-02-15-preview
    pub embeddings_deployment: String,
    pub chat_deployment: String,
    client: Client,
}

impl AzureOpenAI {
    pub fn new(
        endpoint: String,
        api_key: String,
        embeddings_deployment: String,
        chat_deployment: String,
    ) -> Self {
        Self {
            endpoint,
            api_key,
            api_version: "2024-02-15-preview".to_string(),
            embeddings_deployment,
            chat_deployment,
            client: Client::new(),
        }
    }

    // 임베딩 생성
    pub async fn embed(&self, input: String) -> Result<EmbeddingVector> {
        #[derive(Serialize)]
        struct Req {
            input: String,
        }
        #[derive(Deserialize)]
        struct Resp {
            data: Vec<Datum>,
        }
        #[derive(Deserialize)]
        struct Datum {
            embedding: Vec<f32>,
        }

        let url = format!(
            "{}/openai/deployments/{}/embeddings?api-version={}",
            self.endpoint, self.embeddings_deployment, self.api_version
        );

        let resp = self
            .client
            .post(url)
            .header("api-key", &self.api_key)
            .json(&Req { input })
            .send()
            .await?
            .error_for_status()?;

        let body: Resp = resp.json().await?;
        Ok(body
            .data
            .first()
            .map(|d| d.embedding.clone())
            .unwrap_or_default())
    }

    // 챗 생성 (응답 캐싱 용)
    pub async fn chat(&self, system: &str, user: &str) -> Result<String> {
        #[derive(Serialize)]
        struct ReqMessage {
            role: String,
            content: String,
        }
        #[derive(Serialize)]
        struct Req {
            messages: Vec<ReqMessage>,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: Msg,
        }
        #[derive(Deserialize)]
        struct Msg {
            content: String,
        }
        #[derive(Deserialize)]
        struct Resp {
            choices: Vec<Choice>,
        }

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint, self.chat_deployment, self.api_version
        );

        let req = Req {
            messages: vec![
                ReqMessage {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                ReqMessage {
                    role: "user".to_string(),
                    content: user.to_string(),
                },
            ],
        };

        let resp = self
            .client
            .post(url)
            .header("api-key", &self.api_key)
            .json(&req)
            .send()
            .await?
            .error_for_status()?;

        let body: Resp = resp.json().await?;
        Ok(body
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }
}
