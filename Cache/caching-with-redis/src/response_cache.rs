// 질의-응답 캐시
use anyhow::Result;
use redis::{AsyncCommands, aio::ConnectionManager};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ResponseCache {
    connection: Arc<RwLock<ConnectionManager>>,
    ttl: u64,
}

impl ResponseCache {
    pub async fn new(redis_url: &str, ttl: u64) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_connection_manager().await?;
        Ok(Self {
            connection: Arc::new(RwLock::new(connection)),
            ttl,
        })
    }

    fn make_key(&self, question: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(question.as_bytes());
        let hash = hex::encode(hasher.finalize());
        format!("resp:{}", &hash[..16])
    }

    pub async fn get(&self, question: &str) -> Result<Option<String>> {
        let key = self.make_key(question);
        let mut conn = self.connection.write().await;
        let val: Option<String> = conn.get(&key).await?;
        Ok(val)
    }

    pub async fn set(&self, question: &str, answer: &str) -> Result<()> {
        let key = self.make_key(question);
        let mut conn = self.connection.write().await;
        conn.set_ex::<_, _, ()>(key, answer, self.ttl).await?;
        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        let mut conn = self.connection.write().await;
        let script = redis::Script::new(
            r#"
            local keys = redis.call('KEYS', 'resp:*')
            for i=1,#keys do redis.call('DEL', keys[i]) end
            return #keys
            "#,
        );
        let _: () = script.invoke_async(&mut *conn).await?;
        Ok(())
    }
}
