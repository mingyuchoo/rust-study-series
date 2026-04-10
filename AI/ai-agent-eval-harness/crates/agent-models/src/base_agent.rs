use crate::{domain_config::DomainConfig,
            models::Trajectory};
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
    fn load_domain_tools(&self, _domain_config: &DomainConfig) {}
}
