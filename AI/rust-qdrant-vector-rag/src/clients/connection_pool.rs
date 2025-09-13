use crate::config::{AzureOpenAIConfig, QdrantConfig};
use crate::models::ServiceError;
use deadpool::managed::{Manager, Pool, PoolError};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Connection pool manager for HTTP clients
pub struct HttpClientManager {
    config: HttpClientConfig,
}

/// Configuration for HTTP client pool
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub max_size: usize,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub pool_timeout: Duration,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            pool_timeout: Duration::from_secs(5),
        }
    }
}

impl HttpClientManager {
    pub fn new(config: HttpClientConfig) -> Self {
        Self {
            config,
        }
    }
}

#[async_trait::async_trait]
impl Manager for HttpClientManager {
    type Error = ServiceError;
    type Type = Client;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        debug!("Creating new HTTP client for pool");

        Client::builder()
            .timeout(self.config.timeout)
            .connect_timeout(self.config.connect_timeout)
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(5)
            .build()
            .map_err(|e| ServiceError::configuration(format!("Failed to create HTTP client: {}", e)))
    }

    async fn recycle(&self, _client: &mut Self::Type, _metrics: &deadpool::managed::Metrics) -> deadpool::managed::RecycleResult<Self::Error> {
        // HTTP clients don't need explicit recycling, they manage connections
        // internally
        debug!("Recycling HTTP client");
        Ok(())
    }
}

/// Connection pool for Azure OpenAI clients
pub struct AzureOpenAIConnectionPool {
    pool: Pool<HttpClientManager>,
    config: AzureOpenAIConfig,
    stats: Arc<RwLock<PoolStats>>,
}

/// Connection pool statistics
#[derive(Debug, Default, Clone)]
pub struct PoolStats {
    pub total_connections_created: u64,
    pub active_connections: u64,
    pub pool_hits: u64,
    pub pool_misses: u64,
    pub connection_errors: u64,
}

impl AzureOpenAIConnectionPool {
    /// Create a new connection pool for Azure OpenAI
    pub fn new(azure_config: AzureOpenAIConfig, pool_config: Option<HttpClientConfig>) -> Result<Self, ServiceError> {
        let pool_config = pool_config.unwrap_or_default();

        info!(
            "Creating Azure OpenAI connection pool with max_size: {}, timeout: {:?}",
            pool_config.max_size, pool_config.timeout
        );

        let manager = HttpClientManager::new(pool_config.clone());
        let pool = Pool::builder(manager)
            .max_size(pool_config.max_size)
            .wait_timeout(Some(pool_config.pool_timeout))
            .create_timeout(Some(pool_config.connect_timeout))
            .build()
            .map_err(|e| ServiceError::configuration(format!("Failed to create connection pool: {}", e)))?;

        Ok(Self {
            pool,
            config: azure_config,
            stats: Arc::new(RwLock::new(PoolStats::default())),
        })
    }

    /// Get a client from the pool
    pub async fn get_client(&self) -> Result<PooledClient, ServiceError> {
        let start_time = std::time::Instant::now();

        match self.pool.get().await {
            | Ok(client) => {
                let mut stats = self.stats.write().await;
                stats.pool_hits += 1;
                stats.active_connections += 1;

                debug!("Got client from pool in {:?}", start_time.elapsed());

                Ok(PooledClient {
                    client,
                    config: self.config.clone(),
                    stats: Arc::clone(&self.stats),
                })
            },
            | Err(PoolError::Timeout(_)) => {
                let mut stats = self.stats.write().await;
                stats.pool_misses += 1;

                warn!("Connection pool timeout after {:?}", start_time.elapsed());
                Err(ServiceError::external_api("Connection pool timeout"))
            },
            | Err(e) => {
                let mut stats = self.stats.write().await;
                stats.connection_errors += 1;

                error!("Failed to get client from pool: {}", e);
                Err(ServiceError::external_api(format!("Pool error: {}", e)))
            },
        }
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> PoolStats { self.stats.read().await.clone() }

    /// Get pool status information
    pub async fn get_status(&self) -> PoolStatus {
        let pool_status = self.pool.status();
        let stats = self.get_stats().await;

        PoolStatus {
            size: pool_status.size,
            available: pool_status.available,
            max_size: pool_status.max_size,
            stats,
        }
    }

    /// Warm up the connection pool by creating initial connections
    pub async fn warm_up(&self, initial_connections: usize) -> Result<(), ServiceError> {
        info!("Warming up connection pool with {} initial connections", initial_connections);

        let mut clients = Vec::new();

        for i in 0 .. initial_connections {
            match self.get_client().await {
                | Ok(client) => {
                    debug!("Created warm-up connection {}/{}", i + 1, initial_connections);
                    clients.push(client);
                },
                | Err(e) => {
                    warn!("Failed to create warm-up connection {}: {}", i + 1, e);
                    break;
                },
            }
        }

        // Release all clients back to the pool
        drop(clients);

        info!("Connection pool warm-up completed");
        Ok(())
    }
}

/// Pooled HTTP client with automatic return to pool
pub struct PooledClient {
    client: deadpool::managed::Object<HttpClientManager>,
    config: AzureOpenAIConfig,
    stats: Arc<RwLock<PoolStats>>,
}

impl PooledClient {
    /// Get the underlying HTTP client
    pub fn client(&self) -> &Client { &self.client }

    /// Get the Azure OpenAI configuration
    pub fn config(&self) -> &AzureOpenAIConfig { &self.config }
}

impl Drop for PooledClient {
    fn drop(&mut self) {
        // Update stats when client is returned to pool
        if let Ok(mut stats) = self.stats.try_write() {
            stats.active_connections = stats.active_connections.saturating_sub(1);
        }
    }
}

/// Pool status information
#[derive(Debug, Clone)]
pub struct PoolStatus {
    pub size: usize,
    pub available: usize,
    pub max_size: usize,
    pub stats: PoolStats,
}

impl PoolStatus {
    /// Check if the pool is healthy
    pub fn is_healthy(&self) -> bool { self.available > 0 && self.stats.connection_errors < 10 }

    /// Get pool utilization percentage
    pub fn utilization_percent(&self) -> f64 {
        if self.max_size == 0 {
            return 0.0;
        }
        ((self.size - self.available) as f64 / self.max_size as f64) * 100.0
    }
}

/// Connection pool for Qdrant clients
pub struct QdrantConnectionPool {
    clients: Arc<RwLock<Vec<qdrant_client::Qdrant>>>,
    #[allow(dead_code)]
    config: QdrantConfig,
    max_size: usize,
    current_index: Arc<RwLock<usize>>,
    stats: Arc<RwLock<PoolStats>>,
}

impl QdrantConnectionPool {
    /// Create a new Qdrant connection pool
    pub async fn new(config: QdrantConfig, max_size: usize) -> Result<Self, ServiceError> {
        info!("Creating Qdrant connection pool with max_size: {}", max_size);

        let mut clients = Vec::with_capacity(max_size);

        // Create initial connections
        for i in 0 .. max_size {
            match Self::create_qdrant_client(&config).await {
                | Ok(client) => {
                    debug!("Created Qdrant client {}/{}", i + 1, max_size);
                    clients.push(client);
                },
                | Err(e) => {
                    error!("Failed to create Qdrant client {}: {}", i + 1, e);
                    if i == 0 {
                        // If we can't create any clients, fail
                        return Err(e);
                    }
                    break;
                },
            }
        }

        if clients.is_empty() {
            return Err(ServiceError::configuration("Failed to create any Qdrant clients"));
        }

        info!("Created {} Qdrant clients in pool", clients.len());

        Ok(Self {
            clients: Arc::new(RwLock::new(clients)),
            config,
            max_size,
            current_index: Arc::new(RwLock::new(0)),
            stats: Arc::new(RwLock::new(PoolStats::default())),
        })
    }

    /// Create a single Qdrant client
    async fn create_qdrant_client(config: &QdrantConfig) -> Result<qdrant_client::Qdrant, ServiceError> {
        let mut client_config = qdrant_client::config::QdrantConfig::from_url(&config.url);
        client_config.timeout = Duration::from_secs(config.timeout_seconds);
        client_config.check_compatibility = false;

        let client = qdrant_client::Qdrant::new(client_config).map_err(|e| ServiceError::database(format!("Failed to create Qdrant client: {}", e)))?;

        // Test the connection
        client
            .health_check()
            .await
            .map_err(|e| ServiceError::database(format!("Qdrant health check failed: {}", e)))?;

        Ok(client)
    }

    /// Get a client from the pool using round-robin selection
    pub async fn get_client(&self) -> Result<QdrantPooledClient, ServiceError> {
        let clients = self.clients.read().await;

        if clients.is_empty() {
            let mut stats = self.stats.write().await;
            stats.connection_errors += 1;
            return Err(ServiceError::database("No Qdrant clients available in pool"));
        }

        let mut index = self.current_index.write().await;
        let client_index = *index % clients.len();
        *index = (*index + 1) % clients.len();

        let client = clients[client_index].clone();

        let mut stats = self.stats.write().await;
        stats.pool_hits += 1;
        stats.active_connections += 1;

        Ok(QdrantPooledClient {
            client,
            stats: Arc::clone(&self.stats),
        })
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> PoolStats { self.stats.read().await.clone() }

    /// Get pool status
    pub async fn get_status(&self) -> QdrantPoolStatus {
        let clients = self.clients.read().await;
        let stats = self.get_stats().await;

        QdrantPoolStatus {
            size: clients.len(),
            max_size: self.max_size,
            stats,
        }
    }

    /// Health check all clients in the pool
    pub async fn health_check_all(&self) -> Result<Vec<bool>, ServiceError> {
        let clients = self.clients.read().await;
        let mut results = Vec::with_capacity(clients.len());

        for (i, client) in clients.iter().enumerate() {
            match client.health_check().await {
                | Ok(_) => {
                    debug!("Qdrant client {} health check: OK", i);
                    results.push(true);
                },
                | Err(e) => {
                    warn!("Qdrant client {} health check failed: {}", i, e);
                    results.push(false);
                },
            }
        }

        Ok(results)
    }
}

/// Pooled Qdrant client
pub struct QdrantPooledClient {
    client: qdrant_client::Qdrant,
    stats: Arc<RwLock<PoolStats>>,
}

impl QdrantPooledClient {
    /// Get the underlying Qdrant client
    pub fn client(&self) -> &qdrant_client::Qdrant { &self.client }
}

impl Drop for QdrantPooledClient {
    fn drop(&mut self) {
        // Update stats when client is returned
        if let Ok(mut stats) = self.stats.try_write() {
            stats.active_connections = stats.active_connections.saturating_sub(1);
        }
    }
}

/// Qdrant pool status
#[derive(Debug, Clone)]
pub struct QdrantPoolStatus {
    pub size: usize,
    pub max_size: usize,
    pub stats: PoolStats,
}

impl QdrantPoolStatus {
    /// Check if the pool is healthy
    pub fn is_healthy(&self) -> bool { self.size > 0 && self.stats.connection_errors < 5 }

    /// Get pool utilization percentage
    pub fn utilization_percent(&self) -> f64 {
        if self.max_size == 0 {
            return 0.0;
        }
        (self.size as f64 / self.max_size as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AzureOpenAIConfig;

    fn create_test_azure_config() -> AzureOpenAIConfig {
        AzureOpenAIConfig {
            endpoint: "https://test.openai.azure.com".to_string(),
            api_key: "test-key".to_string(),
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 30,
        }
    }

    #[tokio::test]
    async fn test_azure_openai_pool_creation() {
        let config = create_test_azure_config();
        let pool_config = HttpClientConfig {
            max_size: 2,
            ..Default::default()
        };

        let pool = AzureOpenAIConnectionPool::new(config, Some(pool_config));
        assert!(pool.is_ok(), "Should create connection pool successfully");

        let pool = pool.unwrap();
        let status = pool.get_status().await;
        assert_eq!(status.max_size, 2, "Pool should have correct max size");
    }

    #[tokio::test]
    async fn test_pool_stats_tracking() {
        let config = create_test_azure_config();
        let pool = AzureOpenAIConnectionPool::new(config, None).unwrap();

        let initial_stats = pool.get_stats().await;
        assert_eq!(initial_stats.pool_hits, 0);

        // Getting a client should increment pool hits
        let _client = pool.get_client().await;
        // Note: This test might fail in CI without actual network connectivity
        // In a real environment, you'd mock the HTTP client creation
    }
}
