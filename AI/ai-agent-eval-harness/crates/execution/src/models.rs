use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub task_id: String,
    pub task_description: String,
    pub success: bool,
    pub total_iterations: u32,
    pub metrics: HashMap<String, Option<f64>>,
    #[serde(default = "default_domain")]
    pub domain: String,
    #[serde(default)]
    pub scenario_id: String,
}

fn default_domain() -> String { "general".into() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReport {
    #[serde(default = "default_version")]
    pub version: String,
    pub timestamp: String,
    #[serde(default = "default_agent")]
    pub agent_name: String,
    #[serde(default = "default_eval_scenario")]
    pub eval_scenario: String,
    pub total_scenarios: usize,
    pub success_count: usize,
    pub success_rate: f64,
    #[serde(default)]
    pub average_metrics: HashMap<String, f64>,
    #[serde(default)]
    pub scenarios: Vec<ScenarioResult>,
}

fn default_version() -> String { "1.0".into() }
fn default_agent() -> String { "ppa".into() }
fn default_eval_scenario() -> String { "all".into() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDelta {
    pub metric_name: String,
    pub baseline_value: Option<f64>,
    pub current_value: Option<f64>,
    pub delta: Option<f64>,
    pub delta_percent: Option<f64>,
    pub is_regression: bool,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    #[serde(default = "default_version")]
    pub version: String,
    pub timestamp: String,
    pub baseline_timestamp: String,
    pub current_timestamp: String,
    #[serde(default)]
    pub metric_deltas: Vec<MetricDelta>,
    pub regression_count: usize,
    pub improvement_count: usize,
    pub verdict: String,
    pub threshold_percent: f64,
    pub summary: String,
}
