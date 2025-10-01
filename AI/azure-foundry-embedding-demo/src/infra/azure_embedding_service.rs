use crate::application::ports::EmbeddingServicePort;
use crate::domain::value_objects::*;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::Client;

/// Azure OpenAI 임베딩 서비스 구현
pub struct AzureEmbeddingService {
    client: Client,
    endpoint: String,
    api_key: String,
    deployment_name: String,
}

impl AzureEmbeddingService {
    /// 새로운 Azure 임베딩 서비스 생성
    pub fn new(endpoint: String, api_key: String, deployment_name: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            api_key,
            deployment_name,
        }
    }
}

#[async_trait]
impl EmbeddingServicePort for AzureEmbeddingService {
    async fn generate_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let url = format!(
            "{}/openai/deployments/{}/embeddings?api-version=2024-02-01",
            self.endpoint.trim_end_matches('/'),
            self.deployment_name
        );

        let request = EmbeddingRequest::new(texts, "text-embedding-3-large".to_string())
            .with_dimensions(3072)
            .with_encoding_format("float".to_string());

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("API 요청 실패: {}", error_text));
        }

        let embedding_response: EmbeddingResponse = response.json().await?;

        let embeddings = embedding_response.data.into_iter().map(|data| data.embedding).collect();

        Ok(embeddings)
    }

    async fn generate_embedding(&self, text: String) -> Result<Vec<f32>> {
        let embeddings = self.generate_embeddings(vec![text]).await?;
        embeddings.into_iter().next().ok_or_else(|| anyhow!("임베딩이 반환되지 않았습니다"))
    }
}
