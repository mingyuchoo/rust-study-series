use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

/// 도구 메타데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
    pub safety_level: String,
    pub requires_approval: bool,
}

/// 도구 기본 트레이트
pub trait BaseTool: Send + Sync {
    fn metadata(&self) -> &ToolMetadata;
    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value>;

    fn validate_parameters(&self, params: &HashMap<String, serde_json::Value>) -> bool {
        let required = self
            .metadata()
            .parameters_schema
            .get("required")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        required.iter().all(|r| params.contains_key(*r))
    }
}
