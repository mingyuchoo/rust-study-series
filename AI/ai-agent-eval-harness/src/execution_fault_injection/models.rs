#![allow(dead_code)]
use chrono::{DateTime,
             Utc};
use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureMode {
    Timeout,
    PartialResult,
    IncorrectResult,
    Exception,
    NetworkError,
    PermissionDenied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultInjectionConfig {
    pub enabled: bool,
    pub global_failure_rate: f64,
    pub tool_specific_rates: HashMap<String, f64>,
    pub failure_mode_distribution: HashMap<String, f64>,
    pub seed: Option<u64>,
}

impl Default for FaultInjectionConfig {
    fn default() -> Self {
        let mut dist = HashMap::new();
        dist.insert("timeout".into(), 0.2);
        dist.insert("partial_result".into(), 0.25);
        dist.insert("incorrect_result".into(), 0.2);
        dist.insert("exception".into(), 0.2);
        dist.insert("network_error".into(), 0.1);
        dist.insert("permission_denied".into(), 0.05);

        Self {
            enabled: true,
            global_failure_rate: 0.1,
            tool_specific_rates: HashMap::new(),
            failure_mode_distribution: dist,
            seed: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectedFailure {
    pub tool_name: String,
    pub failure_mode: FailureMode,
    pub original_parameters: HashMap<String, serde_json::Value>,
    pub injected_result: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}
