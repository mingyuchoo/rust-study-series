//! RAG 모듈 에러 타입

use thiserror::Error;

/// RAG 파이프라인에서 발생할 수 있는 에러
#[derive(Debug, Error)]
pub enum RagError {
    /// 설정 오류 (환경변수 누락 등)
    #[error("설정 오류: {0}")]
    Config(String),

    /// HTTP 요청 오류
    #[error("HTTP 요청 실패: {0}")]
    Http(#[from] reqwest::Error),

    /// `OpenAI` API 오류
    #[error("OpenAI API 오류: {message}")]
    Api {
        /// 오류 메시지
        message: String,
    },

    /// JSON 파싱 오류
    #[error("JSON 파싱 실패: {0}")]
    Json(#[from] serde_json::Error),

    /// 문서 미로드 오류
    #[error("문서가 로드되지 않았습니다. load_from_texts()를 먼저 호출하세요.")]
    NoDocuments,

    /// 임베딩 오류
    #[error("임베딩 생성 실패: {0}")]
    Embedding(String),
}
