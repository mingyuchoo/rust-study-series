use anyhow::Result;
use ractor::{Actor, ActorRef, RpcReplyPort};
use async_trait::async_trait;
use ractor_supervisor::{DynamicSupervisor, SupervisorStrategy, SupervisorOptions};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tracing::{info, error, warn};

use crate::actors::llm_actor::{LLMActor, LLMMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupervisorMessage {
    SpawnAgent(String, String, RpcReplyPort<Result<ActorRef<LLMMessage>>>),
    KillAgent(String, RpcReplyPort<Result<()>>),
    ListAgents(RpcReplyPort<Vec<String>>),
    SendPrompt(String, String, RpcReplyPort<Result<String>>),
    GetAgentRef(String, RpcReplyPort<Result<ActorRef<LLMMessage>>>),
}

impl ractor::Message for SupervisorMessage {}

impl ractor::BytesConvertable for SupervisorMessage {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))
    }
    
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
    }
}

pub struct LLMSupervisor {
    agent_refs: std::collections::HashMap<String, ActorRef<LLMMessage>>,
    supervisor: ActorRef<ractor_supervisor::SupervisorMsg>,
}

impl LLMSupervisor {
    pub fn new(supervisor: ActorRef<ractor_supervisor::SupervisorMsg>) -> Self {
        Self {
            agent_refs: std::collections::HashMap::new(),
            supervisor,
        }
    }

    pub async fn spawn_supervisor() -> Result<ActorRef<SupervisorMessage>> {
        let options = SupervisorOptions {
            strategy: SupervisorStrategy::OneForOne,
            max_restarts: 5,
            max_window: Duration::from_secs(30),
            reset_after: Some(Duration::from_secs(60)),
        };

        let (sup_ref, _) = DynamicSupervisor::spawn(
            "llm_supervisor".to_string(),
            options,
        ).await?;

        let (actor, handle) = Actor::spawn(
            None,
            LLMSupervisor::new(sup_ref.clone()),
            (),
        ).await?;

        Ok(actor)
    }

    async fn spawn_agent(
        &mut self,
        supervisor: &ActorRef<ractor_supervisor::SupervisorMessage>,
        agent_id: String,
        model: String,
    ) -> Result<ActorRef<LLMMessage>> {
        info!("Spawning new LLM agent: {} with model: {}", agent_id, model);

        // Check if agent with this ID already exists
        if self.agent_refs.contains_key(&agent_id) {
            warn!("Agent with ID {} already exists", agent_id);
            return Ok(self.agent_refs.get(&agent_id).unwrap().clone());
        }

        // Create the LLM actor
        let llm_actor = LLMActor::new(model.clone());

        // Spawn the child actor through the supervisor
        let child_ref = supervisor.spawn_child(
            agent_id.clone(),
            llm_actor,
            model,
        ).await?;

        // Store the reference
        self.agent_refs.insert(agent_id, child_ref.clone());

        Ok(child_ref)
    }

    async fn kill_agent(
        &mut self,
        supervisor: &ActorRef<ractor_supervisor::SupervisorMessage>,
        agent_id: String,
    ) -> Result<()> {
        info!("Killing LLM agent: {}", agent_id);

        if let Some(agent_ref) = self.agent_refs.remove(&agent_id) {
            supervisor.kill_child(&agent_ref).await?;
            Ok(())
        } else {
            error!("Agent with ID {} not found", agent_id);
            Err(anyhow::anyhow!("Agent not found"))
        }
    }

    async fn send_prompt(
        &self,
        agent_id: String,
        prompt: String,
    ) -> Result<String> {
        if let Some(agent_ref) = self.agent_refs.get(&agent_id) {
            let (reply_tx, reply_rx) = ractor::call_rpc();
            agent_ref.send_message(LLMMessage::ProcessPrompt(prompt, reply_tx))?;
            
            // Wait for response with timeout
            match tokio::time::timeout(Duration::from_secs(30), reply_rx).await {
                Ok(result) => match result {
                    Ok(response) => Ok(response),
                    Err(e) => Err(anyhow::anyhow!("Failed to receive response: {:?}", e)),
                },
                Err(_) => Err(anyhow::anyhow!("Timeout waiting for LLM response")),
            }
        } else {
            Err(anyhow::anyhow!("Agent with ID {} not found", agent_id))
        }
    }
}

#[async_trait]
impl Actor for LLMSupervisor {
    type Msg = SupervisorMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, _args: Self::Arguments) -> Result<Self::State> {
        info!("Starting LLM Supervisor");
        Ok(())
    }

    async fn handle(&self, _myself: ActorRef<Self::Msg>, message: Self::Msg, _state: &mut Self::State) -> Result<()> {
        // Use the supervisor reference stored in the struct
        let supervisor = self.supervisor.clone();
        
        // Create a mutable copy of self to modify state
        let mut this = self.clone();
        
        match message {
            SupervisorMessage::SpawnAgent(agent_id, model, reply) => {
                let result = this.spawn_agent(&supervisor, agent_id, model).await;
                let _ = reply.send(result);
            },
            SupervisorMessage::KillAgent(agent_id, reply) => {
                let result = this.kill_agent(&supervisor, agent_id).await;
                let _ = reply.send(result);
            },
            SupervisorMessage::ListAgents(reply) => {
                let agent_ids: Vec<String> = this.agent_refs.keys().cloned().collect();
                let _ = reply.send(agent_ids);
            },
            SupervisorMessage::SendPrompt(agent_id, prompt, reply) => {
                let result = this.send_prompt(agent_id, prompt).await;
                let _ = reply.send(result);
            },
            SupervisorMessage::GetAgentRef(agent_id, reply) => {
                if let Some(agent_ref) = this.agent_refs.get(&agent_id) {
                    let _ = reply.send(Ok(agent_ref.clone()));
                } else {
                    let _ = reply.send(Err(anyhow::anyhow!("Agent with ID {} not found", agent_id)));
                }
            },
        }
        
        Ok(())
    }
}
