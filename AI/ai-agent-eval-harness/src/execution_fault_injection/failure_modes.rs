#![allow(dead_code)]
use super::models::FailureMode;
use crate::execution_tools::base::ToolMetadata;
use rand::Rng;
use std::collections::HashMap;

pub struct FailureModeGenerator;

impl FailureModeGenerator {
    pub fn generate(
        failure_mode: &FailureMode,
        tool_metadata: &ToolMetadata,
        parameters: &HashMap<String, serde_json::Value>,
        rng: &mut impl Rng,
    ) -> HashMap<String, serde_json::Value> {
        match failure_mode {
            | FailureMode::Timeout => Self::timeout(tool_metadata),
            | FailureMode::PartialResult => Self::partial_result(tool_metadata),
            | FailureMode::IncorrectResult => Self::incorrect_result(tool_metadata, parameters, rng),
            | FailureMode::Exception => Self::exception(tool_metadata, rng),
            | FailureMode::NetworkError => Self::network_error(tool_metadata),
            | FailureMode::PermissionDenied => Self::permission_denied(tool_metadata),
        }
    }

    fn timeout(meta: &ToolMetadata) -> HashMap<String, serde_json::Value> {
        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(false));
        m.insert(
            "error".into(),
            serde_json::json!(format!("Timeout: {} 실행이 시간 초과되었습니다 (30초)", meta.name)),
        );
        m.insert("error_type".into(), serde_json::json!("timeout"));
        m
    }

    fn partial_result(meta: &ToolMetadata) -> HashMap<String, serde_json::Value> {
        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("partial".into(), serde_json::Value::Bool(true));
        m.insert("warning".into(), serde_json::json!(format!("{}에서 일부 데이터만 반환되었습니다", meta.name)));
        m.insert("error_type".into(), serde_json::json!("partial_result"));
        m
    }

    fn incorrect_result(_meta: &ToolMetadata, params: &HashMap<String, serde_json::Value>, rng: &mut impl Rng) -> HashMap<String, serde_json::Value> {
        let mut corrupted = HashMap::new();
        for (key, value) in params {
            if let Some(n) = value.as_f64() {
                if n != 0.0 {
                    let factor = 1.0 + rng.gen_range(-0.2 .. 0.2);
                    corrupted.insert(key.clone(), serde_json::json!((n * factor * 100.0).round() / 100.0));
                }
            }
        }
        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("data_corrupted".into(), serde_json::Value::Bool(true));
        m.insert("warning".into(), serde_json::json!("결과가 변조되었을 수 있습니다"));
        m.insert("corrupted_params".into(), serde_json::json!(corrupted));
        m.insert("error_type".into(), serde_json::json!("incorrect_result"));
        m
    }

    fn exception(meta: &ToolMetadata, rng: &mut impl Rng) -> HashMap<String, serde_json::Value> {
        let exceptions = [
            "RuntimeError: 내부 처리 오류",
            "ValueError: 유효하지 않은 데이터 형식",
            "IOError: 데이터 소스 접근 실패",
        ];
        let err = exceptions[rng.gen_range(0 .. exceptions.len())];
        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(false));
        m.insert("error".into(), serde_json::json!(format!("{}: {}", meta.name, err)));
        m.insert("error_type".into(), serde_json::json!("exception"));
        m
    }

    fn network_error(meta: &ToolMetadata) -> HashMap<String, serde_json::Value> {
        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(false));
        m.insert(
            "error".into(),
            serde_json::json!(format!("NetworkError: {} 서비스 연결 실패 (ConnectionRefused)", meta.name)),
        );
        m.insert("error_type".into(), serde_json::json!("network_error"));
        m
    }

    fn permission_denied(meta: &ToolMetadata) -> HashMap<String, serde_json::Value> {
        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(false));
        m.insert(
            "error".into(),
            serde_json::json!(format!("PermissionError: {} 실행 권한이 부족합니다", meta.name)),
        );
        m.insert("error_type".into(), serde_json::json!("permission_denied"));
        m
    }
}
