use crate::agent_core::{domain_config::DomainConfig,
                        models::{AgentState,
                                 PpaStage,
                                 PpaStep,
                                 Trajectory}};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AgentMetadata {
    pub name: String,
    #[allow(dead_code)]
    pub description: String,
    #[allow(dead_code)]
    pub version: String,
}

/// 테스트 대상 시스템(SUT) 에이전트 트레이트
pub trait BaseAgent: Send + Sync {
    fn metadata(&self) -> AgentMetadata;
    fn execute_task(&self, task_description: &str, initial_environment: Option<HashMap<String, serde_json::Value>>) -> Trajectory;
    #[allow(dead_code)]
    fn load_domain_tools(&mut self, _domain_config: &DomainConfig) {}
}

/// 패스스루(에코) 에이전트 - 테스트용
pub struct PassthroughAgent;

impl BaseAgent for PassthroughAgent {
    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            name: "passthrough".into(),
            description: "테스트용 패스스루 에이전트 (LLM 호출 없음)".into(),
            version: "0.1.0".into(),
        }
    }

    fn execute_task(&self, task_description: &str, initial_environment: Option<HashMap<String, serde_json::Value>>) -> Trajectory {
        let now = chrono::Utc::now();
        let task_id = uuid::Uuid::new_v4().to_string();

        let perceive_step = PpaStep {
            stage: PpaStage::Perceive,
            iteration: 1,
            timestamp: now,
            input_data: {
                let mut m = HashMap::new();
                m.insert("environment".into(), serde_json::json!(initial_environment.unwrap_or_default()));
                m
            },
            output_data: {
                let mut m = HashMap::new();
                m.insert("echo".into(), serde_json::Value::String(task_description.to_string()));
                m.insert("anomalies".into(), serde_json::json!([]));
                m
            },
            tool_calls: Vec::new(),
            duration_ms: Some(0.0),
        };

        let policy_step = PpaStep {
            stage: PpaStage::Policy,
            iteration: 1,
            timestamp: now,
            input_data: {
                let mut m = HashMap::new();
                m.insert("perceived_info".into(), serde_json::json!({"echo": task_description}));
                m
            },
            output_data: {
                let mut m = HashMap::new();
                m.insert("selected_tool".into(), serde_json::Value::Null);
                m.insert("task_completed".into(), serde_json::Value::Bool(true));
                m.insert("reasoning".into(), serde_json::Value::String("패스스루 에이전트: 작업 완료로 처리".into()));
                m
            },
            tool_calls: Vec::new(),
            duration_ms: Some(0.0),
        };

        Trajectory {
            task_id,
            task_description: task_description.to_string(),
            start_time: now,
            end_time: Some(now),
            steps: vec![perceive_step, policy_step],
            final_state: Some(AgentState {
                current_stage: PpaStage::Policy,
                iteration: 1,
                task_description: task_description.to_string(),
                perceived_info: HashMap::new(),
                planned_actions: Vec::new(),
                executed_actions: Vec::new(),
                is_complete: true,
                error_message: None,
            }),
            success: true,
            total_iterations: 1,
        }
    }
}
