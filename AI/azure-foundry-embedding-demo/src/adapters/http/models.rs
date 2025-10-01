use serde::{Deserialize, Serialize};

/// 임베딩 생성 요청 DTO
#[derive(Debug, Deserialize)]
pub struct CreateEmbeddingRequest {
    pub text: String,
}

/// 배치 임베딩 생성 요청 DTO
#[derive(Debug, Deserialize)]
pub struct CreateBatchEmbeddingRequest {
    pub texts: Vec<String>,
}

/// 유사도 검색 요청 DTO
#[derive(Debug, Deserialize)]
pub struct SearchSimilarRequest {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize { 10 }

/// 임베딩 응답 DTO
#[derive(Debug, Serialize)]
pub struct EmbeddingResponse {
    pub id: i64,
    pub text: String,
    pub vector_length: usize,
    pub created_at: String,
}

/// 유사도 검색 결과 응답 DTO
#[derive(Debug, Serialize)]
pub struct SimilarityResultResponse {
    pub id: i64,
    pub text: String,
    pub similarity: f32,
}

/// 에러 응답 DTO
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
