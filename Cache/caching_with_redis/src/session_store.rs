// 세션 및 대화 컨텍스트 저장소
use anyhow::Result;
use redis::{aio::ConnectionManager, AsyncCommands};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,   // system|user|assistant
    pub content: String,
    pub ts: i64,        // unix timestamp(ms)
}

pub struct SessionStore {
    connection: Arc<RwLock<ConnectionManager>>,
    ttl: u64,
}

impl SessionStore {
    pub async fn new(redis_url: &str, ttl: u64) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_connection_manager().await?;
        Ok(Self { connection: Arc::new(RwLock::new(connection)), ttl })
    }

    fn session_key(&self, user_id: &str) -> String { format!("sess:{}", user_id) }

    pub async fn append_message(&self, user_id: &str, msg: &ChatMessage) -> Result<()> {
        let key = self.session_key(user_id);
        let mut conn = self.connection.write().await;
        let payload = serde_json::to_string(msg)?;
        conn.rpush::<_, _, ()>(&key, payload).await?;
        conn.expire::<_, ()>(&key, self.ttl as i64).await?;
        Ok(())
    }

    pub async fn get_history(&self, user_id: &str, max: isize) -> Result<Vec<ChatMessage>> {
        let key = self.session_key(user_id);
        let mut conn = self.connection.write().await;
        let items: Vec<String> = conn.lrange(&key, -max, -1).await.unwrap_or_default();
        let mut out = Vec::with_capacity(items.len());
        for it in items { if let Ok(m) = serde_json::from_str::<ChatMessage>(&it) { out.push(m); } }
        Ok(out)
    }

    pub async fn clear(&self, user_id: &str) -> Result<()> {
        let key = self.session_key(user_id);
        let mut conn = self.connection.write().await;
        let _: () = conn.del(key).await?;
        Ok(())
    }

    pub async fn clear_all(&self) -> Result<()> {
        let mut conn = self.connection.write().await;
        let script = redis::Script::new(
            r#"
            local keys = redis.call('KEYS', 'sess:*')
            for i=1,#keys do redis.call('DEL', keys[i]) end
            return #keys
            "#,
        );
        let _: () = script.invoke_async(&mut *conn).await?;
        Ok(())
    }
}
