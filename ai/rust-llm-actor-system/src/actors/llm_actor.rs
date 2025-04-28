use anyhow::Result;
use ractor::{Actor, ActorRef, RpcReplyPort};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::json;
use tracing::{info, error};
use std::env;
use reqwest::Client;

// Ensure dotenv is loaded somewhere in main, or add dotenvy::dotenv().ok(); here if needed.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMMessage {
    ProcessPrompt(String, RpcReplyPort<String>),
    HealthCheck(RpcReplyPort<bool>),
    UpdateModel(String),
}

impl ractor::Message for LLMMessage {}

impl ractor::BytesConvertable for LLMMessage {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))
    }
    
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
    }
}

#[derive(Debug, Clone)]
pub struct LLMActor {
    model: String,
    endpoint: String,
    api_key: String,
    client: Client,
}

impl LLMActor {
    pub fn new(model: String) -> Self {
        let endpoint = env::var("OPENAI_API_URL").expect("OPENAI_API_URL not set");
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        Self {
            model,
            endpoint,
            api_key,
            client: Client::new(),
        }
    }

    async fn process_prompt(&self, prompt: String) -> Result<String> {
        info!("LLMActor> Processing prompt with model: {}", self.model);

        // Read all relevant env vars (with defaults for optionals)
        let model = env::var("OPENAI_API_MODEL").unwrap_or_else(|_| self.model.clone());
        let stream = env::var("OPENAI_API_STREAM").unwrap_or_else(|_| "false".to_string());
        let max_tokens = env::var("OPENAI_API_MAX_TOKENS").ok().and_then(|v| v.parse::<u32>().ok()).unwrap_or(4096);
        let temperature = env::var("OPENAI_API_TEMPERATURE").ok().and_then(|v| v.parse::<f32>().ok()).unwrap_or(1.0);
        let top_p = env::var("OPENAI_API_TOP_P").ok().and_then(|v| v.parse::<f32>().ok()).unwrap_or(1.0);

        let messages = vec![json!({
            "role": "user",
            "content": prompt
        })];

        let body = json!({
            "messages": messages,
            "stream": stream == "true" || stream == "True" || stream == "1",
            "max_tokens": max_tokens,
            "temperature": temperature,
            "top_p": top_p,
            "model": model
        });

        let resp = self.client.post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("api-key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let resp_json: serde_json::Value = resp.json().await?;
        if !status.is_success() {
            error!("OpenAI API error: {:?}", resp_json);
            return Err(anyhow::anyhow!("OpenAI API error: {:?}", resp_json));
        }

        let content = resp_json["choices"]
            .get(0)
            .and_then(|choice| choice["message"]["content"].as_str())
            .unwrap_or("No response generated");
        Ok(content.to_string())
    }
}

#[async_trait]
impl Actor for LLMActor {
    type Msg = LLMMessage;
    type State = ();
    type Arguments = String; // Model name as argument

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, model: Self::Arguments) -> Result<Self::State> {
        info!("Starting LLM Actor with model: {}", model);
        Ok(())
    }

    async fn handle(&self, _myself: ActorRef<Self::Msg>, message: Self::Msg, _state: &mut Self::State) -> Result<()> {
        match message {
            LLMMessage::ProcessPrompt(prompt, reply) => {
                match self.process_prompt(prompt).await {
                    Ok(response) => {
                        let _ = reply.send(response);
                    },
                    Err(e) => {
                        error!("LLMActor> Error processing prompt: {:?}", e);
                        let _ = reply.send("Error processing your request".to_string());
                    }
                }
            },
            LLMMessage::HealthCheck(reply) => {
                let _ = reply.send(true); // Simple health check
            },
            LLMMessage::UpdateModel(new_model) => {
                info!("Updating model from {} to {}", self.model, new_model);
                // In a real implementation, we would update the model here
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ractor::concurrency::Duration;
    
    #[tokio::test]
    async fn test_llm_actor_health_check() {
        let (actor, handle) = Actor::spawn(None, LLMActor::new("gpt-3.5-turbo".to_string()), "gpt-3.5-turbo".to_string())
            .await
            .expect("Failed to spawn LLM actor");
        
        let (reply_tx, reply_rx) = ractor::call_rpc();
        actor.send_message(LLMMessage::HealthCheck(reply_tx))
            .expect("Failed to send health check message");
        
        let result = reply_rx.recv_timeout(Duration::from_secs(5))
            .expect("Failed to receive health check response");
        
        assert!(result);
        
        handle.stop(None);
        handle.await.expect("Failed to stop actor");
    }
}
