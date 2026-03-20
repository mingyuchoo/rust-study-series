#![allow(dead_code)]
use agent_models::models::{EvaluationResult,
                           Trajectory};
use chrono::{DateTime,
             Utc};
use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub turn_id: u32,
    pub user_input: String,
    pub scenario_id: Option<String>,
    pub context_from_previous: HashMap<String, serde_json::Value>,
    pub trajectory: Option<Trajectory>,
    pub evaluation_result: Option<EvaluationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnConfig {
    pub user_input: String,
    pub initial_environment: Option<HashMap<String, serde_json::Value>>,
    pub expected_context_keys: Vec<String>,
    pub scenario_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationScenario {
    pub conversation_id: String,
    pub domain: String,
    pub description: String,
    pub turns: Vec<TurnConfig>,
    pub overall_success_criteria: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub conversation_id: String,
    pub scenario: ConversationScenario,
    pub turns: Vec<ConversationTurn>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub overall_success: bool,
    pub context_retention_scores: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTurnMetrics {
    pub total_turns: usize,
    pub successful_turns: usize,
    pub average_context_retention: f64,
    pub conversation_coherence: f64,
    pub cross_turn_dependency_handling: f64,
    pub overall_success_rate: f64,
}
