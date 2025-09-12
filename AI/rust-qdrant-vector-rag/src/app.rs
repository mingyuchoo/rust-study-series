use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tracing::{info, warn};

use crate::clients::AzureOpenAIClient;
use crate::config::AppConfig;
use crate::models::ServiceError;
use crate::repository::{QdrantRepository, VectorRepository};
use crate::services::document::DocumentServiceImpl;
use crate::services::embedding::EmbeddingServiceImpl;
use crate::services::rag::RAGServiceImpl;
use crate::services::vector_search::VectorSearchServiceImpl;
use crate::services::{ChunkingConfig, DocumentService, EmbeddingService, RAGService, VectorSearchService};

/// Application container that holds all initialized services and dependencies
#[derive(Clone)]
pub struct AppContainer {
    pub config: AppConfig,
    pub azure_client: AzureOpenAIClient,
    pub vector_repository: Arc<dyn VectorRepository>,
    pub embedding_service: Arc<dyn EmbeddingService>,
    pub vector_search_service: Arc<dyn VectorSearchService>,
    pub document_service: Arc<dyn DocumentService>,
    pub rag_service: Arc<dyn RAGService>,
}

impl AppContainer {
    /// Initialize all application dependencies with proper error handling
    pub async fn new(config: AppConfig) -> Result<Self, ServiceError> {
        info!("Initializing application container...");

        // Initialize Azure OpenAI client
        let azure_client = Self::init_azure_client(&config).await?;

        // Initialize Qdrant repository
        let vector_repository = Self::init_vector_repository(&config).await?;

        // Initialize services with dependency injection
        let embedding_service = Self::init_embedding_service(azure_client.clone());
        let vector_search_service = Self::init_vector_search_service(vector_repository.clone());
        let document_service = Self::init_document_service(embedding_service.clone(), vector_repository.clone());
        let rag_service = Self::init_rag_service(embedding_service.clone(), vector_search_service.clone(), azure_client.clone());

        info!("Application container initialized successfully");

        Ok(AppContainer {
            config,
            azure_client,
            vector_repository,
            embedding_service,
            vector_search_service,
            document_service,
            rag_service,
        })
    }

    /// Initialize Azure OpenAI client with connectivity testing
    async fn init_azure_client(config: &AppConfig) -> Result<AzureOpenAIClient, ServiceError> {
        info!("Initializing Azure OpenAI client...");

        let client = AzureOpenAIClient::new(config.azure_openai.clone())
            .map_err(|e| ServiceError::Configuration(format!("Failed to create Azure OpenAI client: {}", e)))?;

        // Test connectivity unless explicitly disabled
        if std::env::var("SKIP_CONNECTIVITY_TEST").is_err() {
            info!("Testing Azure OpenAI connectivity...");
            match client.test_connectivity().await {
                | Ok(()) => info!("Azure OpenAI connectivity test passed"),
                | Err(e) => {
                    warn!("Azure OpenAI connectivity test failed: {}. Service will continue but may have issues.", e);
                    // Don't fail startup - allow service to start even if connectivity test fails
                },
            }
        } else {
            info!("Skipping Azure OpenAI connectivity test (SKIP_CONNECTIVITY_TEST is set)");
        }

        Ok(client)
    }

    /// Initialize Qdrant vector repository with collection setup
    async fn init_vector_repository(config: &AppConfig) -> Result<Arc<dyn VectorRepository>, ServiceError> {
        info!("Initializing Qdrant vector repository...");

        let repository = QdrantRepository::new(config.qdrant.clone()).await?;

        // Initialize collection if it doesn't exist
        info!("Ensuring vector collection exists...");
        if !repository.collection_exists().await? {
            info!("Creating vector collection: {}", config.qdrant.collection_name);
            repository.initialize_collection().await?;
        } else {
            info!("Vector collection already exists: {}", config.qdrant.collection_name);
        }

        Ok(Arc::new(repository))
    }

    /// Initialize embedding service
    fn init_embedding_service(azure_client: AzureOpenAIClient) -> Arc<dyn EmbeddingService> {
        info!("Initializing embedding service...");
        Arc::new(EmbeddingServiceImpl::new(azure_client))
    }

    /// Initialize vector search service
    fn init_vector_search_service(vector_repository: Arc<dyn VectorRepository>) -> Arc<dyn VectorSearchService> {
        info!("Initializing vector search service...");
        Arc::new(VectorSearchServiceImpl::new(vector_repository))
    }

    /// Initialize document service with chunking configuration
    fn init_document_service(embedding_service: Arc<dyn EmbeddingService>, vector_repository: Arc<dyn VectorRepository>) -> Arc<dyn DocumentService> {
        info!("Initializing document service...");

        // Configure chunking parameters
        let chunking_config = ChunkingConfig {
            max_chunk_size: 1000,
            overlap_size: 100,
            min_chunk_size: 100,
            respect_boundaries: true,
        };

        Arc::new(DocumentServiceImpl::with_chunking_config(embedding_service, vector_repository, chunking_config))
    }

    /// Initialize RAG service
    fn init_rag_service(
        embedding_service: Arc<dyn EmbeddingService>,
        vector_search_service: Arc<dyn VectorSearchService>,
        azure_client: AzureOpenAIClient,
    ) -> Arc<dyn RAGService> {
        info!("Initializing RAG service...");
        Arc::new(RAGServiceImpl::new(embedding_service, vector_search_service, azure_client))
    }

    /// Perform comprehensive health checks on all services
    pub async fn health_check(&self) -> Result<HealthStatus, ServiceError> {
        info!("Performing application health check...");

        let mut status = HealthStatus::new();

        // Check Azure OpenAI connectivity
        match self.azure_client.test_connectivity().await {
            | Ok(()) => {
                status.azure_openai = ServiceHealth::Healthy;
                info!("Azure OpenAI health check: OK");
            },
            | Err(e) => {
                status.azure_openai = ServiceHealth::Unhealthy(e.to_string());
                warn!("Azure OpenAI health check failed: {}", e);
            },
        }

        // Check Qdrant connectivity
        match self.vector_repository.health_check().await {
            | Ok(true) => {
                status.qdrant = ServiceHealth::Healthy;
                info!("Qdrant health check: OK");
            },
            | Ok(false) => {
                status.qdrant = ServiceHealth::Unhealthy("Qdrant returned false for health check".to_string());
                warn!("Qdrant health check: Not healthy");
            },
            | Err(e) => {
                status.qdrant = ServiceHealth::Unhealthy(e.to_string());
                warn!("Qdrant health check failed: {}", e);
            },
        }

        // Check collection status
        match self.vector_repository.get_collection_info().await {
            | Ok(info) => {
                status.collection_status = Some(format!("Collection exists with {} points", info.points_count.unwrap_or(0)));
                info!("Collection status: OK");
            },
            | Err(e) => {
                status.collection_status = Some(format!("Collection check failed: {}", e));
                warn!("Collection status check failed: {}", e);
            },
        }

        status.overall = if status.azure_openai.is_healthy() && status.qdrant.is_healthy() {
            ServiceHealth::Healthy
        } else {
            ServiceHealth::Degraded("Some services are unhealthy".to_string())
        };

        Ok(status)
    }
}

/// Health status for individual services
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ServiceHealth {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

impl ServiceHealth {
    pub fn is_healthy(&self) -> bool {
        matches!(self, ServiceHealth::Healthy)
    }
}

/// Overall application health status
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub overall: ServiceHealth,
    pub azure_openai: ServiceHealth,
    pub qdrant: ServiceHealth,
    pub collection_status: Option<String>,
}

impl HealthStatus {
    pub fn new() -> Self {
        Self {
            overall: ServiceHealth::Healthy,
            azure_openai: ServiceHealth::Healthy,
            qdrant: ServiceHealth::Healthy,
            collection_status: None,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.overall.is_healthy()
    }
}

/// Graceful shutdown handler
pub struct ShutdownHandler {
    pub shutdown_timeout: Duration,
}

impl ShutdownHandler {
    pub fn new(shutdown_timeout: Duration) -> Self {
        Self { shutdown_timeout }
    }

    /// Wait for shutdown signal (SIGINT or SIGTERM)
    pub async fn wait_for_shutdown(&self) {
        let ctrl_c = async {
            signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                info!("Received Ctrl+C signal, initiating graceful shutdown...");
            },
            _ = terminate => {
                info!("Received SIGTERM signal, initiating graceful shutdown...");
            },
        }
    }

    /// Perform graceful shutdown operations
    pub async fn shutdown(&self, _container: &AppContainer) -> Result<(), ServiceError> {
        info!("Starting graceful shutdown with timeout: {:?}", self.shutdown_timeout);

        // Give services time to finish current operations
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Here you could add cleanup operations like:
        // - Closing database connections
        // - Finishing pending requests
        // - Saving state
        // - Releasing resources

        info!("Graceful shutdown completed");
        Ok(())
    }
}
