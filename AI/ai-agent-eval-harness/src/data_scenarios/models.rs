use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub task_description: String,
    pub initial_environment: HashMap<String, serde_json::Value>,
    pub expected_tools: Vec<String>,
    pub success_criteria: HashMap<String, serde_json::Value>,
    pub difficulty: String,
    #[serde(default = "default_domain")]
    pub domain: String,
}

fn default_domain() -> String { "general".into() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetInput {
    pub task: String,
    pub environment: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetExpectedOutput {
    pub tool_sequence: Vec<String>,
    pub tool_results: HashMap<String, serde_json::Value>,
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,
}

fn default_tolerance() -> f64 { 0.01 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetEntry {
    pub scenario_id: String,
    pub input: GoldenSetInput,
    pub expected_output: GoldenSetExpectedOutput,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetFile {
    pub domain: String,
    #[serde(default = "default_version")]
    pub version: String,
    pub golden_sets: Vec<GoldenSetEntry>,
}

#[allow(dead_code)]
fn default_version() -> String { "1.0".into() }

impl GoldenSetFile {
    #[allow(dead_code)]
    pub fn get_by_scenario_id(&self, scenario_id: &str) -> Option<&GoldenSetEntry> { self.golden_sets.iter().find(|e| e.scenario_id == scenario_id) }
}
