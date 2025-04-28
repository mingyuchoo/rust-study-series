use anyhow::Result;
use ractor::{Actor, ActorRef, RpcReplyPort};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, error};

use crate::actors::llm_actor::LLMMessage;
use crate::supervisor::dynamic_supervisor::SupervisorMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouterMessage {
    RoutePrompt(String, RpcReplyPort<Result<String>>),
    RegisterRoutingRule(String, String),
    RemoveRoutingRule(String),
}

impl ractor::Message for RouterMessage {}

impl ractor::BytesConvertable for RouterMessage {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))
    }
    
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
    }
}

#[derive(Clone)]
pub struct AgentRouter {
    supervisor: ActorRef<SupervisorMessage>,
    routing_rules: HashMap<String, String>, // keyword -> agent_id mapping
}

impl AgentRouter {
    pub fn new(supervisor: ActorRef<SupervisorMessage>) -> Self {
        Self {
            supervisor,
            routing_rules: HashMap::new(),
        }
    }

    async fn select_best_agent(&self, prompt: &str) -> Result<String> {
        // Find the first matching rule
        for (keyword, agent_id) in &self.routing_rules {
            if prompt.to_lowercase().contains(&keyword.to_lowercase()) {
                info!("Routing prompt to agent {} based on keyword: {}", agent_id, keyword);
                return Ok(agent_id.clone());
            }
        }

        // If no specific rule matches, use a default agent if available
        if let Some((_, agent_id)) = self.routing_rules.get_key_value("default") {
            info!("Routing prompt to default agent: {}", agent_id);
            return Ok(agent_id.clone());
        }

        // If no default agent is defined, return an error
        Err(anyhow::anyhow!("No suitable agent found for the prompt"))
    }

    async fn route_prompt(&self, prompt: String) -> Result<String> {
        let agent_id = self.select_best_agent(&prompt).await?;
        
        // Send the prompt to the selected agent through the supervisor
        let (reply_tx, reply_rx) = ractor::call_rpc();
        self.supervisor.send_message(SupervisorMessage::SendPrompt(
            agent_id,
            prompt,
            reply_tx,
        ))?;
        
        // Wait for the response
        match tokio::time::timeout(std::time::Duration::from_secs(60), reply_rx).await {
            Ok(result) => match result {
                Ok(response) => response,
                Err(e) => Err(anyhow::anyhow!("Failed to receive response: {:?}", e)),
            },
            Err(_) => Err(anyhow::anyhow!("Timeout waiting for LLM response")),
        }
    }
}

#[async_trait]
impl Actor for AgentRouter {
    type Msg = RouterMessage;
    type State = HashMap<String, String>;
    type Arguments = ActorRef<SupervisorMessage>;

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, supervisor: Self::Arguments) -> Result<Self::State> {
        info!("Starting Agent Router");
        Ok(self.routing_rules.clone())
    }

    async fn handle(&self, myself: ActorRef<Self::Msg>, message: Self::Msg, state: &mut Self::State) -> Result<()> {
        match message {
            RouterMessage::RoutePrompt(prompt, reply) => {
                let result = self.route_prompt(prompt).await;
                let _ = reply.send(result);
            },
            RouterMessage::RegisterRoutingRule(keyword, agent_id) => {
                info!("Registering routing rule: {} -> {}", keyword, agent_id);
                state.insert(keyword, agent_id);
            },
            RouterMessage::RemoveRoutingRule(keyword) => {
                info!("Removing routing rule for keyword: {}", keyword);
                state.remove(&keyword);
            },
        }
        
        Ok(())
    }
}
