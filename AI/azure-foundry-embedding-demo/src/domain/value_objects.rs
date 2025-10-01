use serde::{Deserialize, Serialize};

/// 임베딩 요청 값 객체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub input: Vec<String>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
}

impl EmbeddingRequest {
    /// 새로운 임베딩 요청 생성
    pub fn new(input: Vec<String>, model: String) -> Self {
        Self {
            input,
            model,
            dimensions: None,
            encoding_format: None,
        }
    }

    /// 차원 설정
    pub fn with_dimensions(mut self, dimensions: u32) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// 인코딩 포맷 설정
    pub fn with_encoding_format(mut self, format: String) -> Self {
        self.encoding_format = Some(format);
        self
    }
}

/// 임베딩 응답 값 객체
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingData>,
}

/// 임베딩 데이터 값 객체
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingData {
    pub embedding: Vec<f32>,
}

/// 유사도 검색 결과 값 객체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub id: i64,
    pub text: String,
    pub similarity: f32,
}

impl SimilarityResult {
    /// 새로운 유사도 검색 결과 생성
    pub fn new(id: i64, text: String, similarity: f32) -> Self {
        Self {
            id,
            text,
            similarity,
        }
    }
}
