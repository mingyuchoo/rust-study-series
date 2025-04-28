use anyhow::Result;
use ractor::{Actor, ActorRef, RpcReplyPort};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, warn, error};

use crate::actors::llm_actor::LLMMessage;
use crate::supervisor::dynamic_supervisor::SupervisorMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthMonitorMessage {
    CheckHealth(RpcReplyPort<HashMap<String, AgentHealth>>),
    StartMonitoring,
    StopMonitoring,
    IsMonitoring(RpcReplyPort<bool>),
}

impl ractor::Message for HealthMonitorMessage {}

impl ractor::BytesConvertable for HealthMonitorMessage {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))
    }
    
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHealth {
    pub agent_id: String,
    pub status: AgentStatus,
    pub last_check: u64, // Unix timestamp
    pub response_time_ms: u64,
    pub consecutive_failures: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

pub struct HealthMonitor {
    supervisor: ActorRef<SupervisorMessage>,
    health_data: HashMap<String, AgentHealth>,
    monitoring_interval: Duration,
    is_monitoring: bool,
}

impl HealthMonitor {
    pub fn new(supervisor: ActorRef<SupervisorMessage>, monitoring_interval: Duration) -> Self {
        Self {
            supervisor,
            health_data: HashMap::new(),
            monitoring_interval,
            is_monitoring: false,
        }
    }

    async fn check_agent_health(&self, agent_id: &str, agent_ref: &ActorRef<LLMMessage>) -> AgentHealth {
        let start_time = Instant::now();
        let (reply_tx, reply_rx) = ractor::call_rpc();
        
        // Send health check message
        let send_result = agent_ref.send_message(LLMMessage::HealthCheck(reply_tx));
        
        let status = if send_result.is_err() {
            AgentStatus::Unhealthy
        } else {
            // Wait for response with timeout
            match tokio::time::timeout(Duration::from_secs(5), reply_rx).await {
                Ok(result) => match result {
                    Ok(true) => AgentStatus::Healthy,
                    Ok(false) => AgentStatus::Degraded,
                    Err(_) => AgentStatus::Unhealthy,
                },
                Err(_) => AgentStatus::Degraded, // Timeout indicates degraded performance
            }
        };
        
        let elapsed = start_time.elapsed();
        let response_time_ms = elapsed.as_millis() as u64;
        
        // Get previous health data if available
        let consecutive_failures = if let Some(previous) = self.health_data.get(agent_id) {
            if status != AgentStatus::Healthy {
                previous.consecutive_failures + 1
            } else {
                0
            }
        } else {
            if status != AgentStatus::Healthy { 1 } else { 0 }
        };
        
        AgentHealth {
            agent_id: agent_id.to_string(),
            status,
            last_check: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            response_time_ms,
            consecutive_failures,
        }
    }

    async fn check_all_agents(&mut self) -> HashMap<String, AgentHealth> {
        // Get list of all agents from supervisor
        let (reply_tx, reply_rx) = ractor::call_rpc();
        if let Err(e) = self.supervisor.send_message(SupervisorMessage::ListAgents(reply_tx)) {
            error!("Failed to send ListAgents message: {:?}", e);
            return self.health_data.clone();
        }
        
        let agent_ids = match reply_rx.await {
            Ok(ids) => ids,
            Err(e) => {
                error!("Failed to receive agent list: {:?}", e);
                return self.health_data.clone();
            }
        };
        
        // Check health of each agent
        for agent_id in agent_ids {
            // Get agent reference
            let (reply_tx, reply_rx) = ractor::call_rpc();
            if let Err(e) = self.supervisor.send_message(SupervisorMessage::GetAgentRef(agent_id.clone(), reply_tx)) {
                error!("Failed to send GetAgentRef message: {:?}", e);
                continue;
            }
            
            let agent_ref = match reply_rx.await {
                Ok(Ok(agent_ref)) => agent_ref,
                _ => {
                    warn!("Failed to get reference for agent: {}", agent_id);
                    continue;
                }
            };
            
            // Check health and update data
            let health = self.check_agent_health(&agent_id, &agent_ref).await;
            self.health_data.insert(agent_id, health);
        }
        
        self.health_data.clone()
    }

    async fn start_monitoring_loop(&mut self, myself: ActorRef<HealthMonitorMessage>) -> Result<()> {
        if self.is_monitoring {
            return Ok(());
        }
        
        self.is_monitoring = true;
        info!("Starting health monitoring loop with interval: {:?}", self.monitoring_interval);
        
        // Clone necessary data for the monitoring task
        let myself_clone = myself.clone();
        let interval = self.monitoring_interval;
        
        // Spawn a background task for monitoring
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                // Send message to self to check health
                if let Err(e) = myself_clone.send_message(HealthMonitorMessage::CheckHealth(ractor::rpc::discard_reply())) {
                    error!("Failed to send health check message: {:?}", e);
                    break;
                }
                
                // Check if monitoring should continue
                let (reply_tx, reply_rx) = ractor::call_rpc::<bool>();
                if let Err(e) = myself_clone.send_message(HealthMonitorMessage::IsMonitoring(reply_tx)) {
                    error!("Failed to check monitoring status: {:?}", e);
                    break;
                }
                
                match reply_rx.await {
                    Ok(false) => break, // Stop monitoring
                    Err(_) => break,    // Error, stop monitoring
                    _ => continue,      // Continue monitoring
                }
            }
            
            info!("Health monitoring loop stopped");
        });
        
        Ok(())
    }
}

#[async_trait]
impl Actor for HealthMonitor {
    type Msg = HealthMonitorMessage;
    type State = ();
    type Arguments = (ActorRef<SupervisorMessage>, Duration);

    async fn pre_start(&self, myself: ActorRef<Self::Msg>, args: Self::Arguments) -> Result<Self::State> {
        info!("Starting Health Monitor");
        
        // Start monitoring automatically
        myself.send_message(HealthMonitorMessage::StartMonitoring)?;
        
        Ok(())
    }

    async fn handle(&self, myself: ActorRef<Self::Msg>, message: Self::Msg, _state: &mut Self::State) -> Result<()> {
        // Create a mutable copy of self
        let mut this = self.clone();
        
        match message {
            HealthMonitorMessage::CheckHealth(reply) => {
                let health_data = this.check_all_agents().await;
                let _ = reply.send(health_data);
            },
            HealthMonitorMessage::StartMonitoring => {
                this.start_monitoring_loop(myself.clone()).await?;
            },
            HealthMonitorMessage::StopMonitoring => {
                this.is_monitoring = false;
                info!("Health monitoring stopped");
            },
            HealthMonitorMessage::IsMonitoring(reply) => {
                let _ = reply.send(this.is_monitoring);
            },
        }
        
        Ok(())
    }
}
