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
