use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub class_name: String,
    pub module_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub task_description: String,
    pub initial_environment: HashMap<String, serde_json::Value>,
    pub expected_tools: Vec<String>,
    pub success_criteria: HashMap<String, serde_json::Value>,
    pub difficulty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    pub name: String,
    pub description: String,
    pub tools: Vec<ToolConfig>,
    pub scenarios: Vec<ScenarioConfig>,
}
