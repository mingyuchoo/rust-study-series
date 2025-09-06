// 기본 임베딩 캐시
use anyhow::{Context, Result};
use redis::{aio::ConnectionManager, AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

// 에러 타입
#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Redis connection error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    #[error("Cache miss")]
    Miss,
}

// 임베딩 벡터 타입
pub type EmbeddingVector = Vec<f32>;

// 캐시 통계
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_count: u64,
    pub miss_count: u64,
    pub total_keys: usize,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 { 0.0 } else { self.hit_count as f64 / total as f64 }
    }
}

// 메인 임베딩 캐시 구조체
pub struct EmbeddingCache {
    connection: Arc<RwLock<ConnectionManager>>,
    ttl: u64, // 초 단위 TTL
    stats: Arc<RwLock<CacheStats>>,
}

impl EmbeddingCache {
    /// 새로운 임베딩 캐시 인스턴스 생성
    pub async fn new(redis_url: &str, ttl: u64) -> Result<Self> {
        let client = Client::open(redis_url)?;
        // 최신 API: get_connection_manager 사용
        let connection = client.get_connection_manager().await?;
        Ok(Self {
            connection: Arc::new(RwLock::new(connection)),
            ttl,
            stats: Arc::new(RwLock::new(CacheStats { hit_count: 0, miss_count: 0, total_keys: 0 })),
        })
    }

    /// 텍스트와 모델명으로 캐시 키 생성
    fn make_key(&self, text: &str, model: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}:{}", model, text).as_bytes());
        let hash = hex::encode(hasher.finalize());
        format!("emb:{}", &hash[..12]) // 12자리 단축 해시
    }

    /// 캐시에서 임베딩 조회
    pub async fn get(&self, text: &str, model: &str) -> Result<Option<EmbeddingVector>, CacheError> {
        let key = self.make_key(text, model);
        let mut conn = self.connection.write().await;
        let data_opt: Option<Vec<u8>> = conn.get(&key).await?;
        if let Some(data) = data_opt {
            let embedding: EmbeddingVector = bincode::deserialize(&data)?;
            let mut stats = self.stats.write().await;
            stats.hit_count += 1;
            Ok(Some(embedding))
        } else {
            let mut stats = self.stats.write().await;
            stats.miss_count += 1;
            Ok(None)
        }
    }

    /// 임베딩을 캐시에 저장
    pub async fn set(&self, text: &str, embedding: &EmbeddingVector, model: &str) -> Result<(), CacheError> {
        let key = self.make_key(text, model);
        let data = bincode::serialize(embedding)?;
        let mut conn = self.connection.write().await;
        conn.set_ex::<_, _, ()>(&key, &data, self.ttl).await?;
        Ok(())
    }

    /// 캐시에서 조회하거나 새로 계산
    pub async fn get_or_compute<F, Fut>(&self, text: &str, model: &str, compute_fn: F) -> Result<EmbeddingVector>
    where
        F: FnOnce(String) -> Fut,
        Fut: std::future::Future<Output = Result<EmbeddingVector>>,
    {
        if let Some(cached) = self.get(text, model).await? { return Ok(cached); }
        let embedding = compute_fn(text.to_string()).await.context("Failed to compute embedding")?;
        self.set(text, &embedding, model).await?;
        Ok(embedding)
    }

    /// 배치 처리
    pub async fn get_or_compute_batch<F, Fut>(&self, texts: &[String], model: &str, compute_fn: F) -> Result<Vec<EmbeddingVector>>
    where
        F: FnOnce(Vec<String>) -> Fut,
        Fut: std::future::Future<Output = Result<Vec<EmbeddingVector>>>,
    {
        let mut results = vec![None; texts.len()];
        let mut uncached_texts = Vec::new();
        let mut uncached_indices = Vec::new();
        for (i, text) in texts.iter().enumerate() {
            if let Some(cached) = self.get(text, model).await? { results[i] = Some(cached); }
            else { uncached_texts.push(text.clone()); uncached_indices.push(i); }
        }
        if !uncached_texts.is_empty() {
            let new_embeddings = compute_fn(uncached_texts.clone()).await?;
            for (i, (text, embedding)) in uncached_texts.iter().zip(new_embeddings.iter()).enumerate() {
                self.set(text, embedding, model).await?;
                let original_index = uncached_indices[i];
                results[original_index] = Some(embedding.clone());
            }
        }
        results.into_iter().map(|opt| opt.ok_or_else(|| anyhow::anyhow!("Missing embedding result"))).collect()
    }

    /// 캐시 통계 조회
    pub async fn get_stats(&self) -> CacheStats {
        let stats = self.stats.read().await;
        let mut conn = self.connection.write().await;
        // Lua 실행은 Script API 사용
        let script = redis::Script::new(
            r#"
            local keys = redis.call('KEYS', 'emb:*')
            return #keys
            "#,
        );
        let key_count: usize = script.invoke_async(&mut *conn).await.unwrap_or(0usize);
        CacheStats { hit_count: stats.hit_count, miss_count: stats.miss_count, total_keys: key_count }
    }

    /// 모든 임베딩 캐시 삭제
    pub async fn clear(&self) -> Result<()> {
        let mut conn = self.connection.write().await;
        let script = redis::Script::new(
            r#"
            local keys = redis.call('KEYS', 'emb:*')
            for i=1,#keys do redis.call('DEL', keys[i]) end
            return #keys
            "#,
        );
        let _: () = script.invoke_async(&mut *conn).await?;
        Ok(())
    }
}
