use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// 요청 구조체
#[derive(Serialize, Debug)]
struct EmbeddingRequest {
    input: Vec<String>,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<String>,
}

// 응답 구조체 (필요한 필드만 유지)
#[derive(Deserialize, Debug)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize, Debug)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

// Azure Embedding 클라이언트
pub struct AzureEmbeddingClient {
    client: Client,
    endpoint: String,
    api_key: String,
    deployment_name: String,
}

impl AzureEmbeddingClient {
    pub fn new(endpoint: String, api_key: String, deployment_name: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            api_key,
            deployment_name,
        }
    }

    pub async fn get_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let url = format!(
            "{}/openai/deployments/{}/embeddings?api-version=2024-02-01",
            self.endpoint.trim_end_matches('/'),
            self.deployment_name
        );

        let request = EmbeddingRequest {
            input: texts,
            model: "text-embedding-3-large".to_string(),
            dimensions: Some(3072), // text-embedding-3-large의 기본 차원
            encoding_format: Some("float".to_string()),
        };

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
            return Err(anyhow!("API request failed: {}", error_text));
        }

        let embedding_response: EmbeddingResponse = response.json().await?;

        let embeddings = embedding_response.data.into_iter().map(|data| data.embedding).collect();

        Ok(embeddings)
    }

    // 단일 텍스트에 대한 임베딩
    pub async fn get_embedding(&self, text: String) -> Result<Vec<f32>> {
        let embeddings = self.get_embeddings(vec![text]).await?;
        embeddings.into_iter().next().ok_or_else(|| anyhow!("No embedding returned"))
    }

    // 코사인 유사도 계산 유틸리티
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }
}
