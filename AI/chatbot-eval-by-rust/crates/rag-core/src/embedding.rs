//! OpenAI / Azure OpenAI 임베딩 클라이언트

use crate::{config::RagConfig,
            error::RagError};
use serde::{Deserialize,
            Serialize};

/// 임베딩 클라이언트
pub struct EmbeddingClient {
    http: reqwest::Client,
    api_url: String,
    api_key: String,
    model: String,
    is_azure: bool,
}

#[derive(Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

impl EmbeddingClient {
    /// 임베딩 클라이언트를 생성한다.
    ///
    /// # Errors
    ///
    /// 필수 환경변수(API 키, 엔드포인트)가 누락되면 `RagError::Config`를
    /// 반환한다.
    pub fn new(config: &RagConfig) -> Result<Self, RagError> {
        let (api_url, api_key, model, is_azure) = if config.use_azure {
            let endpoint = config
                .azure_endpoint
                .as_deref()
                .ok_or_else(|| RagError::Config("AZURE_OPENAI_ENDPOINT가 설정되지 않았습니다".into()))?;
            let key = config
                .azure_api_key
                .as_deref()
                .ok_or_else(|| RagError::Config("AZURE_OPENAI_API_KEY가 설정되지 않았습니다".into()))?;

            let url = format!(
                "{}/openai/deployments/{}/embeddings?api-version={}",
                endpoint.trim_end_matches('/'),
                config.azure_embedding_deployment,
                config.azure_api_version,
            );
            (url, key.to_string(), config.azure_embedding_deployment.clone(), true)
        } else {
            let key = config
                .openai_api_key
                .as_deref()
                .ok_or_else(|| RagError::Config("OPENAI_API_KEY가 설정되지 않았습니다".into()))?;

            (
                "https://api.openai.com/v1/embeddings".to_string(),
                key.to_string(),
                config.embedding_model.clone(),
                false,
            )
        };

        Ok(Self {
            http: reqwest::Client::new(),
            api_url,
            api_key,
            model,
            is_azure,
        })
    }

    /// 텍스트 리스트의 임베딩 벡터를 생성한다.
    ///
    /// # Errors
    ///
    /// HTTP 요청 실패 또는 임베딩 API 오류 시 에러를 반환한다.
    pub async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, RagError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let request = EmbeddingRequest {
            model: self.model.clone(),
            input: texts.to_vec(),
        };

        let mut req_builder = self.http.post(&self.api_url).json(&request);

        if self.is_azure {
            req_builder = req_builder.header("api-key", &self.api_key);
        } else {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.map_err(RagError::Http)?;
            return Err(RagError::Embedding(format!("[{status}] {body}")));
        }

        let embed_response: EmbeddingResponse = response.json().await?;
        Ok(embed_response.data.into_iter().map(|d| d.embedding).collect())
    }

    /// 단일 텍스트의 임베딩 벡터를 생성한다.
    ///
    /// # Errors
    ///
    /// HTTP 요청 실패 또는 임베딩 API 오류 시 에러를 반환한다.
    pub async fn embed_one(&self, text: &str) -> Result<Vec<f32>, RagError> {
        let results = self.embed(&[text.to_string()]).await?;
        results
            .into_iter()
            .next()
            .ok_or_else(|| RagError::Embedding("임베딩 결과가 비어있습니다".into()))
    }
}
