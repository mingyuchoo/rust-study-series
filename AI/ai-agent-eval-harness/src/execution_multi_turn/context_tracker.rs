#![allow(dead_code)]
use crate::agent_core::models::{PpaStage,
                                Trajectory};
use std::collections::HashMap;

pub struct ContextTracker;

impl ContextTracker {
    pub fn new() -> Self { Self }

    pub fn update_context(&self, current: &HashMap<String, serde_json::Value>, trajectory: &Trajectory) -> HashMap<String, serde_json::Value> {
        let mut updated = current.clone();

        if let Some(final_state) = &trajectory.final_state {
            for (k, v) in &final_state.perceived_info {
                updated.insert(k.clone(), v.clone());
            }
        }

        for step in &trajectory.steps {
            if step.stage == PpaStage::Action {
                for tc in &step.tool_calls {
                    if tc.success {
                        if let Some(result) = &tc.result {
                            if let Some(obj) = result.as_object() {
                                updated.insert(format!("{}_result", tc.tool_name), serde_json::json!(obj));
                            }
                        }
                    }
                }
            }
        }

        updated.insert("_last_task_success".into(), serde_json::Value::Bool(trajectory.success));
        updated.insert("_last_task_description".into(), serde_json::Value::String(trajectory.task_description.clone()));
        updated
    }

    pub fn calculate_retention(&self, expected_keys: &[String], actual: &HashMap<String, serde_json::Value>) -> f64 {
        if expected_keys.is_empty() {
            return 1.0;
        }
        let retained = expected_keys.iter().filter(|k| actual.contains_key(*k)).count();
        retained as f64 / expected_keys.len() as f64
    }
}

impl Default for ContextTracker {
    fn default() -> Self { Self::new() }
}
