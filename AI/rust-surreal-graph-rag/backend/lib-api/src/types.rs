use crate::azure::AzureOpenAI;
use crate::config::AppConfig;
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

/// 애플리케이션 상태
pub struct AppState {
    pub cfg: AppConfig,
    pub azure: AzureOpenAI,
}

// 벡터 검색 결과
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct ChunkSearchResult {
    pub content: String,
    pub id: RecordId,
    #[serde(default)]
    pub doc_id: Option<String>,
    pub metadata: ChunkMetadata,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct ChunkMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_index: Option<i32>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
}
