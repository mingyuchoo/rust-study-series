use actix_web::{HttpResponse, Result, web};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, warn};

use crate::clients::AzureOpenAIClient;
use crate::config::AppConfig;
use crate::models::{HealthResponse, HealthStatus, ServiceHealthStatus};
use crate::repository::{QdrantRepository, VectorRepository};
use crate::services::{DocumentService, EmbeddingService, RAGService, VectorSearchService};

/// Comprehensive health check handler with dependency injection
pub async fn health_handler(
    config: web::Data<AppConfig>,
    azure_client: web::Data<AzureOpenAIClient>,
    _document_service: web::Data<Arc<dyn DocumentService>>,
    _rag_service: web::Data<Arc<dyn RAGService>>,
    _embedding_service: web::Data<Arc<dyn EmbeddingService>>,
    _vector_search_service: web::Data<Arc<dyn VectorSearchService>>,
) -> Result<HttpResponse> {
    debug!("Processing comprehensive health check request with injected services");

    // Calculate uptime (simplified - in production you'd track actual start time)
    let uptime_seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() % 86400; // Reset daily for demo

    // Check Azure OpenAI connectivity
    let azure_healthy = check_azure_openai_health(&azure_client).await;

    // Check Qdrant connectivity
    let qdrant_healthy = check_qdrant_health(&config).await;

    // Additional service checks could be added here using the injected services
    // For example:
    // - Test embedding generation with a small sample
    // - Check vector search functionality
    // - Verify document processing pipeline

    let service_status = ServiceHealthStatus::new(qdrant_healthy, azure_healthy);
    let health_response = HealthResponse::new(service_status, uptime_seconds);

    // Return appropriate HTTP status based on health
    match health_response.status {
        | HealthStatus::Healthy => {
            debug!("Health check passed - all services healthy");
            Ok(HttpResponse::Ok().json(health_response))
        },
        | HealthStatus::Degraded => {
            warn!("Health check shows degraded state - some services unhealthy");
            Ok(HttpResponse::Ok().json(health_response)) // Still return 200 for degraded
        },
        | HealthStatus::Unhealthy => {
            error!("Health check failed - critical services unhealthy");
            Ok(HttpResponse::ServiceUnavailable().json(health_response))
        },
    }
}

/// Simple health check that just returns basic status
pub async fn simple_health_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "rust-qdrant-vector-rag",
        "timestamp": chrono::Utc::now()
    })))
}

/// Check Azure OpenAI service health
async fn check_azure_openai_health(azure_client: &AzureOpenAIClient) -> bool {
    match azure_client.test_connectivity().await {
        | Ok(()) => {
            debug!("Azure OpenAI health check passed");
            true
        },
        | Err(e) => {
            warn!("Azure OpenAI health check failed: {}", e);
            false
        },
    }
}

/// Check Qdrant service health
async fn check_qdrant_health(config: &AppConfig) -> bool {
    match QdrantRepository::new(config.qdrant.clone()).await {
        | Ok(repo) => {
            // Try to perform a simple operation to verify connectivity
            match repo.health_check().await {
                | Ok(healthy) => {
                    if healthy {
                        debug!("Qdrant health check passed");
                    } else {
                        warn!("Qdrant health check returned false");
                    }
                    healthy
                },
                | Err(e) => {
                    warn!("Qdrant health check failed: {}", e);
                    false
                },
            }
        },
        | Err(e) => {
            warn!("Failed to create Qdrant repository for health check: {}", e);
            false
        },
    }
}
