//! 공용 요청/응답 모델 정의


use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// 인증
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// 사용자 정보 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct MeResponse {
    pub email: String,
}

// 헬스체크
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub services: serde_json::Value,
    pub version: String,
}

// 검색
#[derive(Debug, Deserialize, ToSchema)]
pub struct VectorSearchRequest {
    pub query: String,
    #[serde(default = "default_top_k")] pub top_k: u32,
    #[serde(default = "default_threshold")] pub threshold: f32,
    #[serde(default)] pub filters: Option<serde_json::Value>,
}

fn default_top_k() -> u32 { 10 }
fn default_threshold() -> f32 { 0.7 }

#[derive(Debug, Serialize, ToSchema)]
pub struct VectorSearchItem {
    pub id: String,
    pub content: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VectorSearchResponse {
    pub results: Vec<VectorSearchItem>,
    pub total: u32,
    pub query_time: f32,
}

// 챗/질의응답
#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatAskRequest {
    pub query: String,
    #[serde(default)] pub conversation_id: Option<String>,
    #[serde(default)] pub context: Option<serde_json::Value>,
    #[serde(default)] pub options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SourceItem {
    pub r#type: String,
    pub content: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GraphPathItem {
    pub path: String,
    pub nodes: serde_json::Value,
    pub relationships: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatAskResponse {
    pub response: String,
    pub conversation_id: Option<String>,
    pub sources: Vec<SourceItem>,
    pub graph_paths: Vec<GraphPathItem>,
    pub query_time: f32,
    pub tokens_used: u32,
}

// 인덱싱 생성
#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct IndexChunkInput {
    /// 청크 텍스트 내용
    pub content: String,
    /// 선택적 메타데이터(JSON)
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct IndexCreateRequest {
    /// 문서 식별자(미지정 시 서버에서 생성)
    #[serde(default)]
    pub document_id: Option<String>,
    /// 문서 제목(옵션)
    #[serde(default)]
    pub title: Option<String>,
    /// (옵션) PDF 파일 경로. 제공 시 서버가 직접 PDF를 처리하여 청킹/그래프/임베딩을 생성
    #[serde(default)]
    pub pdf_path: Option<String>,
    /// (옵션) 로컬 TF-IDF 임베딩 사용 여부.
    /// 기본값: false (기본은 Azure 임베딩을 사용하여 검색과 동일한 임베딩 공간을 보장)
    #[serde(default = "default_use_tfidf")]
    pub use_tfidf: bool,
    /// 분할된 청크 목록
    pub chunks: Vec<IndexChunkInput>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IndexCreateResponse {
    /// 생성/사용된 문서 식별자
    pub document_id: String,
    /// 인덱싱된 청크 개수
    pub chunks_indexed: u32,
    /// 전체 처리 시간(초)
    pub elapsed: f32,
}

/// 인덱싱 기본 모드: Azure 임베딩 사용(= TF-IDF 비활성)
fn default_use_tfidf() -> bool { false }

// 관리자 재인덱싱
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReindexRequest {
    /// 재인덱싱할 PDF 파일 경로 목록(서버 파일 경로)
    pub pdf_paths: Vec<String>,
    /// TF-IDF 사용 여부(기본: false → Azure 임베딩 사용)
    #[serde(default)]
    pub use_tfidf: Option<bool>,
    /// 기존 데이터 정리(삭제) 여부: true면 동일 source의 기존 chunk/entity/relation 삭제 후 재인덱싱
    #[serde(default)]
    pub clear_existing: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReindexItemResult {
    /// 입력 PDF 경로(에코)
    pub pdf_path: String,
    /// 생성/사용된 문서 ID
    pub document_id: Option<String>,
    /// 인덱싱된 청크 개수
    pub chunks_indexed: u32,
    /// 오류 메시지(성공 시 None)
    pub error: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReindexResponse {
    pub results: Vec<ReindexItemResult>,
    pub elapsed: f32,
}

// 파일 업로드 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct UploadResponse {
    /// 서버에 저장된 파일의 전체 경로
    pub path: String,
    /// 저장된 파일 크기(바이트)
    pub size: u64,
}
