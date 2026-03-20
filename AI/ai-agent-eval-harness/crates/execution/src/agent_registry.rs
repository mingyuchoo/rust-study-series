use agent_models::base_agent::BaseAgent;
use std::{collections::HashMap,
          sync::Arc};

pub struct AgentRegistry {
    agents: HashMap<String, Arc<dyn BaseAgent>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, agent: Arc<dyn BaseAgent>) { self.agents.insert(name.to_string(), agent); }

    pub fn get_agent(&self, name: &str) -> Option<Arc<dyn BaseAgent>> { self.agents.get(name).cloned() }

    pub fn get_agent_names(&self) -> Vec<String> { self.agents.keys().cloned().collect() }

    #[allow(dead_code)]
    pub fn has_agent(&self, name: &str) -> bool { self.agents.contains_key(name) }
}

impl Default for AgentRegistry {
    fn default() -> Self { Self::new() }
}
