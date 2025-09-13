use crate::models::{DocumentChunk, SearchResult, ServiceError};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Cache entry with expiration
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    created_at: Instant,
    ttl: Duration,
    access_count: u64,
    last_accessed: Instant,
}

impl<T> CacheEntry<T> {
    fn new(value: T, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            value,
            created_at: now,
            ttl,
            access_count: 0,
            last_accessed: now,
        }
    }

    fn is_expired(&self) -> bool { self.created_at.elapsed() > self.ttl }

    fn access(&mut self) -> &T {
        self.access_count += 1;
        self.last_accessed = Instant::now();
        &self.value
    }
}

/// In-memory cache with TTL and LRU eviction
#[derive(Clone)]
pub struct InMemoryCache<K, V> {
    data: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    max_size: usize,
    default_ttl: Duration,
    stats: Arc<RwLock<CacheStats>>,
}

/// Cache statistics
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub expired_entries: u64,
    pub total_entries: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { 0.0 } else { self.hits as f64 / total as f64 }
    }
}

impl<K, V> InMemoryCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new cache with specified capacity and default TTL
    pub fn new(max_size: usize, default_ttl: Duration) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            default_ttl,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Get a value from the cache
    pub async fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.write().await;
        let mut stats = self.stats.write().await;

        if let Some(entry) = data.get_mut(key) {
            if entry.is_expired() {
                data.remove(key);
                stats.expired_entries += 1;
                stats.misses += 1;
                debug!("Cache entry expired for key");
                None
            } else {
                stats.hits += 1;
                let value = entry.access().clone();
                debug!("Cache hit for key");
                Some(value)
            }
        } else {
            stats.misses += 1;
            debug!("Cache miss for key");
            None
        }
    }

    /// Put a value into the cache with default TTL
    pub async fn put(&self, key: K, value: V) { self.put_with_ttl(key, value, self.default_ttl).await; }

    /// Put a value into the cache with custom TTL
    pub async fn put_with_ttl(&self, key: K, value: V, ttl: Duration) {
        let mut data = self.data.write().await;
        let mut stats = self.stats.write().await;

        // Check if we need to evict entries
        if data.len() >= self.max_size {
            self.evict_lru(&mut data, &mut stats).await;
        }

        let entry = CacheEntry::new(value, ttl);
        data.insert(key, entry);
        stats.total_entries += 1;
        debug!("Added entry to cache");
    }

    /// Remove a value from the cache
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.write().await;
        data.remove(key).map(|entry| entry.value)
    }

    /// Clear all entries from the cache
    pub async fn clear(&self) {
        let mut data = self.data.write().await;
        data.clear();
        debug!("Cache cleared");
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats { self.stats.read().await.clone() }

    /// Get current cache size
    pub async fn size(&self) -> usize { self.data.read().await.len() }

    /// Clean up expired entries
    pub async fn cleanup_expired(&self) {
        let mut data = self.data.write().await;
        let mut stats = self.stats.write().await;

        let initial_size = data.len();
        data.retain(|_, entry| !entry.is_expired());
        let removed = initial_size - data.len();

        if removed > 0 {
            stats.expired_entries += removed as u64;
            info!("Cleaned up {} expired cache entries", removed);
        }
    }

    /// Evict least recently used entry
    async fn evict_lru(&self, data: &mut HashMap<K, CacheEntry<V>>, stats: &mut CacheStats) {
        if let Some((key_to_remove, _)) = data.iter().min_by_key(|(_, entry)| entry.last_accessed).map(|(k, v)| (k.clone(), v.clone())) {
            data.remove(&key_to_remove);
            stats.evictions += 1;
            debug!("Evicted LRU cache entry");
        }
    }
}

/// Cache key for embeddings
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EmbeddingCacheKey {
    pub text_hash: u64,
    pub model: String,
}

impl EmbeddingCacheKey {
    pub fn new(text: &str, model: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        text.hash(&mut hasher);
        let text_hash = hasher.finish();

        Self {
            text_hash,
            model: model.to_string(),
        }
    }
}

/// Cache key for vector search results
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SearchCacheKey {
    pub query_hash: u64,
    pub limit: usize,
    pub threshold: Option<u32>, // Store as u32 for hashing (f32 * 1000)
}

impl SearchCacheKey {
    pub fn new(query_embedding: &[f32], limit: usize, threshold: Option<f32>) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        // Hash the embedding vector
        for &value in query_embedding {
            (value as u32).hash(&mut hasher); // Convert to u32 for consistent hashing
        }

        let query_hash = hasher.finish();
        let threshold = threshold.map(|t| (t * 1000.0) as u32);

        Self {
            query_hash,
            limit,
            threshold,
        }
    }
}

/// Specialized cache for embeddings
pub type EmbeddingCache = InMemoryCache<EmbeddingCacheKey, Vec<f32>>;

/// Specialized cache for search results
pub type SearchCache = InMemoryCache<SearchCacheKey, Vec<SearchResult>>;

/// Specialized cache for document chunks
pub type ChunkCache = InMemoryCache<String, Vec<DocumentChunk>>;

/// Cache manager that coordinates multiple caches
pub struct CacheManager {
    pub embedding_cache: EmbeddingCache,
    pub search_cache: SearchCache,
    pub chunk_cache: ChunkCache,
    cleanup_interval: Duration,
}

impl CacheManager {
    /// Create a new cache manager with default settings
    pub fn new() -> Self {
        Self {
            embedding_cache: EmbeddingCache::new(1000, Duration::from_secs(3600)), // 1 hour
            search_cache: SearchCache::new(500, Duration::from_secs(1800)),        // 30 minutes
            chunk_cache: ChunkCache::new(200, Duration::from_secs(7200)),          // 2 hours
            cleanup_interval: Duration::from_secs(600),                            // 10 minutes
        }
    }

    /// Create with custom cache sizes and TTLs
    pub fn with_config(
        embedding_cache_size: usize,
        embedding_ttl: Duration,
        search_cache_size: usize,
        search_ttl: Duration,
        chunk_cache_size: usize,
        chunk_ttl: Duration,
    ) -> Self {
        Self {
            embedding_cache: EmbeddingCache::new(embedding_cache_size, embedding_ttl),
            search_cache: SearchCache::new(search_cache_size, search_ttl),
            chunk_cache: ChunkCache::new(chunk_cache_size, chunk_ttl),
            cleanup_interval: Duration::from_secs(600), // 10 minutes
        }
    }

    /// Start background cleanup task
    pub async fn start_cleanup_task(&self) -> Result<(), ServiceError> {
        let embedding_cache = Arc::new(self.embedding_cache.clone());
        let search_cache = Arc::new(self.search_cache.clone());
        let chunk_cache = Arc::new(self.chunk_cache.clone());
        let interval = self.cleanup_interval;

        tokio::spawn(async move {
            let mut cleanup_interval = tokio::time::interval(interval);

            loop {
                cleanup_interval.tick().await;

                // Clean up expired entries in all caches
                embedding_cache.cleanup_expired().await;
                search_cache.cleanup_expired().await;
                chunk_cache.cleanup_expired().await;

                debug!("Cache cleanup completed");
            }
        });

        info!("Cache cleanup task started with interval: {:?}", self.cleanup_interval);
        Ok(())
    }

    /// Get comprehensive cache statistics
    pub async fn get_stats(&self) -> CacheManagerStats {
        let embedding_stats = self.embedding_cache.stats().await;
        let search_stats = self.search_cache.stats().await;
        let chunk_stats = self.chunk_cache.stats().await;

        CacheManagerStats {
            embedding_cache: embedding_stats,
            search_cache: search_stats,
            chunk_cache: chunk_stats,
        }
    }

    /// Clear all caches
    pub async fn clear_all(&self) {
        self.embedding_cache.clear().await;
        self.search_cache.clear().await;
        self.chunk_cache.clear().await;
        info!("All caches cleared");
    }
}

impl Default for CacheManager {
    fn default() -> Self { Self::new() }
}

/// Combined statistics for all caches
#[derive(Debug, Clone, serde::Serialize)]
pub struct CacheManagerStats {
    pub embedding_cache: CacheStats,
    pub search_cache: CacheStats,
    pub chunk_cache: CacheStats,
}

impl CacheManagerStats {
    /// Get overall hit rate across all caches
    pub fn overall_hit_rate(&self) -> f64 {
        let total_hits = self.embedding_cache.hits + self.search_cache.hits + self.chunk_cache.hits;
        let total_requests = total_hits + self.embedding_cache.misses + self.search_cache.misses + self.chunk_cache.misses;

        if total_requests == 0 {
            0.0
        } else {
            total_hits as f64 / total_requests as f64
        }
    }

    /// Get total cache entries across all caches
    pub fn total_entries(&self) -> u64 { self.embedding_cache.total_entries + self.search_cache.total_entries + self.chunk_cache.total_entries }
}

/// Cache-aware wrapper for expensive operations
pub struct CachedOperation<K, V> {
    cache: InMemoryCache<K, V>,
    operation_name: String,
}

impl<K, V> CachedOperation<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(cache: InMemoryCache<K, V>, operation_name: String) -> Self {
        Self {
            cache,
            operation_name,
        }
    }

    /// Execute operation with caching
    pub async fn execute<F, Fut, E>(&self, key: K, operation: F) -> Result<V, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<V, E>>,
    {
        // Try to get from cache first
        if let Some(cached_value) = self.cache.get(&key).await {
            debug!("Cache hit for operation: {}", self.operation_name);
            return Ok(cached_value);
        }

        // Execute the operation
        debug!("Cache miss for operation: {}, executing...", self.operation_name);
        let result = operation().await?;

        // Store in cache
        self.cache.put(key, result.clone()).await;
        debug!("Cached result for operation: {}", self.operation_name);

        Ok(result)
    }

    /// Execute operation with custom TTL
    pub async fn execute_with_ttl<F, Fut, E>(&self, key: K, ttl: Duration, operation: F) -> Result<V, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<V, E>>,
    {
        // Try to get from cache first
        if let Some(cached_value) = self.cache.get(&key).await {
            debug!("Cache hit for operation: {}", self.operation_name);
            return Ok(cached_value);
        }

        // Execute the operation
        debug!("Cache miss for operation: {}, executing...", self.operation_name);
        let result = operation().await?;

        // Store in cache with custom TTL
        self.cache.put_with_ttl(key, result.clone(), ttl).await;
        debug!("Cached result for operation: {} with TTL: {:?}", self.operation_name, ttl);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = InMemoryCache::new(10, Duration::from_secs(60));

        // Test put and get
        cache.put("key1".to_string(), "value1".to_string()).await;
        let value = cache.get(&"key1".to_string()).await;
        assert_eq!(value, Some("value1".to_string()));

        // Test miss
        let missing = cache.get(&"missing".to_string()).await;
        assert_eq!(missing, None);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = InMemoryCache::new(10, Duration::from_millis(50));

        cache.put("key1".to_string(), "value1".to_string()).await;

        // Should be available immediately
        let value = cache.get(&"key1".to_string()).await;
        assert_eq!(value, Some("value1".to_string()));

        // Wait for expiration
        sleep(Duration::from_millis(100)).await;

        // Should be expired now
        let expired = cache.get(&"key1".to_string()).await;
        assert_eq!(expired, None);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = InMemoryCache::new(10, Duration::from_secs(60));

        // Initial stats
        let stats = cache.stats().await;
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);

        // Add entry and test hit
        cache.put("key1".to_string(), "value1".to_string()).await;
        cache.get(&"key1".to_string()).await;

        let stats = cache.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);

        // Test miss
        cache.get(&"missing".to_string()).await;

        let stats = cache.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate(), 0.5);
    }

    #[tokio::test]
    async fn test_embedding_cache_key() {
        let key1 = EmbeddingCacheKey::new("hello world", "text-embedding-3-large");
        let key2 = EmbeddingCacheKey::new("hello world", "text-embedding-3-large");
        let key3 = EmbeddingCacheKey::new("different text", "text-embedding-3-large");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[tokio::test]
    async fn test_search_cache_key() {
        let embedding = vec![0.1, 0.2, 0.3];
        let key1 = SearchCacheKey::new(&embedding, 10, Some(0.5));
        let key2 = SearchCacheKey::new(&embedding, 10, Some(0.5));
        let key3 = SearchCacheKey::new(&embedding, 5, Some(0.5));

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[tokio::test]
    async fn test_cached_operation() {
        let cache = InMemoryCache::new(10, Duration::from_secs(60));
        let cached_op = CachedOperation::new(cache, "test_operation".to_string());

        let mut call_count = 0;

        // First call should execute the operation
        let result1 = cached_op
            .execute("key1".to_string(), || async {
                call_count += 1;
                Ok::<String, &str>("result1".to_string())
            })
            .await;

        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), "result1");
        assert_eq!(call_count, 1);

        // Second call should use cache
        let result2 = cached_op
            .execute("key1".to_string(), || async {
                call_count += 1;
                Ok::<String, &str>("result2".to_string())
            })
            .await;

        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), "result1"); // Should be cached value
        assert_eq!(call_count, 1); // Operation should not be called again
    }
}
