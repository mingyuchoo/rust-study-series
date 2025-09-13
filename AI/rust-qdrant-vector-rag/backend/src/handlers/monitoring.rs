use crate::app::AppContainer;
use crate::monitoring::PerformanceMonitor;
use crate::services::cache::CacheManager;
use actix_web::{HttpResponse, Result, web};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use utoipa::ToSchema;

/// 성능 메트릭 응답 스키마
#[derive(Debug, Serialize, ToSchema)]
pub struct PerformanceMetricsResponse {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub system: SystemMetrics,
    pub application: ApplicationMetrics,
    pub cache: Option<CacheMetrics>,
    pub connections: ConnectionMetrics,
}

/// 시스템 레벨 메트릭 스키마
#[derive(Debug, Serialize, ToSchema)]
pub struct SystemMetrics {
    pub memory_usage_bytes: u64,
    pub memory_usage_percent: f64,
    pub cpu_usage_percent: f64,
    pub process_memory_mb: Option<u64>,
}

/// 애플리케이션 레벨 메트릭 스키마
#[derive(Debug, Serialize, ToSchema)]
pub struct ApplicationMetrics {
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub active_requests: u64,
    pub error_rate: f64,
    pub avg_response_time_ms: f64,
}

/// 캐시 메트릭 스키마
#[derive(Debug, Serialize, ToSchema)]
pub struct CacheMetrics {
    pub embedding_cache: CacheStats,
    pub search_cache: CacheStats,
    pub chunk_cache: CacheStats,
    pub overall_hit_rate: f64,
    pub total_entries: u64,
}

/// 개별 캐시 통계 스키마
#[derive(Debug, Serialize, ToSchema)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub evictions: u64,
    pub expired_entries: u64,
    pub total_entries: u64,
}

/// 커넥션 풀 메트릭 스키마
#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectionMetrics {
    pub azure_openai_pool: Option<PoolMetrics>,
    pub qdrant_pool: Option<PoolMetrics>,
}

/// 풀 메트릭 스키마
#[derive(Debug, Serialize, ToSchema)]
pub struct PoolMetrics {
    pub size: usize,
    pub available: usize,
    pub max_size: usize,
    pub utilization_percent: f64,
    pub total_connections_created: u64,
    pub active_connections: u64,
    pub pool_hits: u64,
    pub pool_misses: u64,
    pub connection_errors: u64,
}

/// 어플리케이션 메트릭 조회 엔드포인트
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    tag = "monitoring",
    responses(
        (status = 200, description = "성능 메트릭 응답", body = PerformanceMetricsResponse),
        (status = 500, description = "메트릭 수집 실패")
    )
)]
pub async fn metrics_handler(
    container: web::Data<AppContainer>,
    performance_monitor: web::Data<PerformanceMonitor>,
    cache_manager: web::Data<CacheManager>,
) -> Result<HttpResponse> {
    debug!("Handling metrics request");

    match collect_metrics(&container, &performance_monitor, &cache_manager).await {
        | Ok(metrics) => {
            debug!("Successfully collected performance metrics");
            Ok(HttpResponse::Ok().json(metrics))
        },
        | Err(e) => {
            error!("Failed to collect metrics: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to collect metrics",
                "message": e.to_string()
            })))
        },
    }
}

/// Prometheus 메트릭 엔드포인트 프록시
#[utoipa::path(
    get,
    path = "/api/v1/metrics/prometheus",
    tag = "monitoring",
    responses(
        (status = 200, description = "Prometheus 포맷 메트릭 문자열")
    )
)]
pub async fn prometheus_metrics_handler() -> Result<HttpResponse> {
    debug!("Handling Prometheus metrics request");

    // The metrics-exporter-prometheus crate automatically handles this endpoint
    // This is just a placeholder - the actual metrics are served by the exporter
    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body("# Metrics are served by the Prometheus exporter\n"))
}

/// 성능 지표를 포함한 헬스체크 엔드포인트
#[utoipa::path(
    get,
    path = "/api/v1/health/performance",
    tag = "health",
    responses(
        (status = 200, description = "헬스 및 성능 지표 응답"),
        (status = 503, description = "비정상 상태")
    )
)]
pub async fn health_with_performance_handler(container: web::Data<AppContainer>, performance_monitor: web::Data<PerformanceMonitor>) -> Result<HttpResponse> {
    debug!("Handling health check with performance indicators");

    let health_status = match container.health_check().await {
        | Ok(status) => status,
        | Err(e) => {
            error!("Health check failed: {}", e);
            return Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "unhealthy",
                "error": e.to_string()
            })));
        },
    };

    let performance_snapshot = match performance_monitor.get_performance_snapshot().await {
        | Ok(snapshot) => snapshot,
        | Err(e) => {
            error!("Failed to get performance snapshot: {}", e);
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": if health_status.is_healthy() { "healthy" } else { "degraded" },
                "services": {
                    "azure_openai": format!("{:?}", health_status.azure_openai),
                    "qdrant": format!("{:?}", health_status.qdrant),
                },
                "performance": "unavailable"
            })));
        },
    };

    let status = if health_status.is_healthy() && !performance_snapshot.is_memory_pressure() && !performance_snapshot.is_cpu_pressure() {
        "healthy"
    } else if health_status.is_healthy() {
        "degraded"
    } else {
        "unhealthy"
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": status,
        "timestamp": chrono::Utc::now(),
        "services": {
            "azure_openai": format!("{:?}", health_status.azure_openai),
            "qdrant": format!("{:?}", health_status.qdrant),
        },
        "performance": {
            "memory_usage_percent": performance_snapshot.memory_usage_percent(),
            "cpu_usage_percent": performance_snapshot.system_cpu_usage,
            "process_memory_mb": performance_snapshot.process_memory_mb(),
            "memory_pressure": performance_snapshot.is_memory_pressure(),
            "cpu_pressure": performance_snapshot.is_cpu_pressure(),
        },
        "collection_status": health_status.collection_status
    })))
}

/// Collect comprehensive metrics
async fn collect_metrics(
    _container: &AppContainer,
    performance_monitor: &PerformanceMonitor,
    cache_manager: &CacheManager,
) -> Result<PerformanceMetricsResponse, Box<dyn std::error::Error>> {
    // Get performance snapshot
    let performance_snapshot = performance_monitor.get_performance_snapshot().await?;

    // Get cache statistics
    let cache_stats = cache_manager.get_stats().await;

    // Build system metrics
    let system_metrics = SystemMetrics {
        memory_usage_bytes: performance_snapshot.system_memory_used,
        memory_usage_percent: performance_snapshot.memory_usage_percent(),
        cpu_usage_percent: performance_snapshot.system_cpu_usage as f64,
        process_memory_mb: performance_snapshot.process_memory_mb(),
    };

    // Build application metrics (these would typically come from a metrics
    // collector)
    let application_metrics = ApplicationMetrics {
        uptime_seconds: 0,         // Would be calculated from startup time
        total_requests: 0,         // Would come from request counter
        active_requests: 0,        // Would come from active request gauge
        error_rate: 0.0,           // Would be calculated from error counters
        avg_response_time_ms: 0.0, // Would come from response time histogram
    };

    // Build cache metrics
    let cache_metrics = CacheMetrics {
        embedding_cache: CacheStats {
            hits: cache_stats.embedding_cache.hits,
            misses: cache_stats.embedding_cache.misses,
            hit_rate: cache_stats.embedding_cache.hit_rate(),
            evictions: cache_stats.embedding_cache.evictions,
            expired_entries: cache_stats.embedding_cache.expired_entries,
            total_entries: cache_stats.embedding_cache.total_entries,
        },
        search_cache: CacheStats {
            hits: cache_stats.search_cache.hits,
            misses: cache_stats.search_cache.misses,
            hit_rate: cache_stats.search_cache.hit_rate(),
            evictions: cache_stats.search_cache.evictions,
            expired_entries: cache_stats.search_cache.expired_entries,
            total_entries: cache_stats.search_cache.total_entries,
        },
        chunk_cache: CacheStats {
            hits: cache_stats.chunk_cache.hits,
            misses: cache_stats.chunk_cache.misses,
            hit_rate: cache_stats.chunk_cache.hit_rate(),
            evictions: cache_stats.chunk_cache.evictions,
            expired_entries: cache_stats.chunk_cache.expired_entries,
            total_entries: cache_stats.chunk_cache.total_entries,
        },
        overall_hit_rate: cache_stats.overall_hit_rate(),
        total_entries: cache_stats.total_entries(),
    };

    // Build connection metrics (would be populated if connection pools are
    // available)
    let connection_metrics = ConnectionMetrics {
        azure_openai_pool: None, // Would be populated from actual pool
        qdrant_pool: None,       // Would be populated from actual pool
    };

    Ok(PerformanceMetricsResponse {
        timestamp: chrono::Utc::now(),
        system: system_metrics,
        application: application_metrics,
        cache: Some(cache_metrics),
        connections: connection_metrics,
    })
}

/// 캐시 통계 조회 엔드포인트
#[utoipa::path(
    get,
    path = "/api/v1/cache/stats",
    tag = "monitoring",
    responses(
        (status = 200, description = "캐시 통계 응답")
    )
)]
pub async fn cache_stats_handler(cache_manager: web::Data<CacheManager>) -> Result<HttpResponse> {
    debug!("Handling cache stats request");

    let stats = cache_manager.get_stats().await;
    Ok(HttpResponse::Ok().json(stats))
}

/// Clear cache endpoint
#[derive(Deserialize, ToSchema)]
pub struct ClearCacheRequest {
    pub cache_type: Option<String>, // "embedding", "search", "chunk", or "all"
}

/// 캐시 초기화 엔드포인트
#[utoipa::path(
    post,
    path = "/api/v1/cache/clear",
    tag = "monitoring",
    request_body = ClearCacheRequest,
    responses(
        (status = 200, description = "요청된 캐시 초기화 완료"),
        (status = 400, description = "유효하지 않은 캐시 타입")
    )
)]
pub async fn clear_cache_handler(cache_manager: web::Data<CacheManager>, request: web::Json<ClearCacheRequest>) -> Result<HttpResponse> {
    debug!("Handling clear cache request: {:?}", request.cache_type);

    match request.cache_type.as_deref() {
        | Some("embedding") => {
            cache_manager.embedding_cache.clear().await;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Embedding cache cleared"
            })))
        },
        | Some("search") => {
            cache_manager.search_cache.clear().await;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Search cache cleared"
            })))
        },
        | Some("chunk") => {
            cache_manager.chunk_cache.clear().await;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Chunk cache cleared"
            })))
        },
        | Some("all") | None => {
            cache_manager.clear_all().await;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "All caches cleared"
            })))
        },
        | Some(unknown) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid cache type",
            "message": format!("Unknown cache type: {}. Valid types: embedding, search, chunk, all", unknown)
        }))),
    }
}

/// Performance benchmark endpoint
#[derive(Deserialize, ToSchema)]
pub struct BenchmarkRequest {
    pub operation: String, // "embedding", "search", "rag"
    pub iterations: Option<usize>,
}

/// 성능 벤치마크 실행 엔드포인트
#[utoipa::path(
    post,
    path = "/api/v1/benchmark",
    tag = "monitoring",
    request_body = BenchmarkRequest,
    responses(
        (status = 200, description = "벤치마크 결과 반환"),
        (status = 400, description = "유효하지 않은 operation 값")
    )
)]
pub async fn benchmark_handler(container: web::Data<AppContainer>, request: web::Json<BenchmarkRequest>) -> Result<HttpResponse> {
    debug!("Handling benchmark request: {:?}", request.operation);

    let iterations = request.iterations.unwrap_or(10);

    match request.operation.as_str() {
        | "embedding" => {
            let results = benchmark_embedding_generation(&container, iterations).await;
            Ok(HttpResponse::Ok().json(results))
        },
        | "search" => {
            let results = benchmark_vector_search(&container, iterations).await;
            Ok(HttpResponse::Ok().json(results))
        },
        | "rag" => {
            let results = benchmark_rag_pipeline(&container, iterations).await;
            Ok(HttpResponse::Ok().json(results))
        },
        | _ => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid operation",
            "message": "Valid operations: embedding, search, rag"
        }))),
    }
}

/// Benchmark embedding generation
async fn benchmark_embedding_generation(container: &AppContainer, iterations: usize) -> serde_json::Value {
    let test_text = "This is a test text for benchmarking embedding generation performance.";
    let mut durations = Vec::new();

    for _ in 0 .. iterations {
        let start = std::time::Instant::now();

        match container.embedding_service.generate_embedding(test_text).await {
            | Ok(_) => {
                durations.push(start.elapsed().as_millis() as u64);
            },
            | Err(e) => {
                error!("Embedding generation failed during benchmark: {}", e);
                return serde_json::json!({
                    "error": "Benchmark failed",
                    "message": e.to_string()
                });
            },
        }
    }

    let avg_duration = durations.iter().sum::<u64>() as f64 / durations.len() as f64;
    let min_duration = *durations.iter().min().unwrap_or(&0);
    let max_duration = *durations.iter().max().unwrap_or(&0);

    serde_json::json!({
        "operation": "embedding_generation",
        "iterations": iterations,
        "avg_duration_ms": avg_duration,
        "min_duration_ms": min_duration,
        "max_duration_ms": max_duration,
        "total_duration_ms": durations.iter().sum::<u64>(),
        "throughput_ops_per_sec": 1000.0 / avg_duration
    })
}

/// Benchmark vector search
async fn benchmark_vector_search(container: &AppContainer, iterations: usize) -> serde_json::Value {
    // First generate a test embedding
    let test_embedding = match container.embedding_service.generate_embedding("test query").await {
        | Ok(embedding) => embedding,
        | Err(e) => {
            return serde_json::json!({
                "error": "Failed to generate test embedding",
                "message": e.to_string()
            });
        },
    };

    let mut durations = Vec::new();

    for _ in 0 .. iterations {
        let start = std::time::Instant::now();

        match container.vector_search_service.search_similar(test_embedding.clone(), 5).await {
            | Ok(_) => {
                durations.push(start.elapsed().as_millis() as u64);
            },
            | Err(e) => {
                error!("Vector search failed during benchmark: {}", e);
                return serde_json::json!({
                    "error": "Benchmark failed",
                    "message": e.to_string()
                });
            },
        }
    }

    let avg_duration = durations.iter().sum::<u64>() as f64 / durations.len() as f64;
    let min_duration = *durations.iter().min().unwrap_or(&0);
    let max_duration = *durations.iter().max().unwrap_or(&0);

    serde_json::json!({
        "operation": "vector_search",
        "iterations": iterations,
        "avg_duration_ms": avg_duration,
        "min_duration_ms": min_duration,
        "max_duration_ms": max_duration,
        "total_duration_ms": durations.iter().sum::<u64>(),
        "throughput_ops_per_sec": 1000.0 / avg_duration
    })
}

/// Benchmark RAG pipeline
async fn benchmark_rag_pipeline(container: &AppContainer, iterations: usize) -> serde_json::Value {
    let test_question = "What is the main topic of the documents?";
    let mut durations = Vec::new();

    for _ in 0 .. iterations {
        let start = std::time::Instant::now();

        match container.rag_service.answer_question(test_question.to_string()).await {
            | Ok(_) => {
                durations.push(start.elapsed().as_millis() as u64);
            },
            | Err(e) => {
                error!("RAG pipeline failed during benchmark: {}", e);
                return serde_json::json!({
                    "error": "Benchmark failed",
                    "message": e.to_string()
                });
            },
        }
    }

    let avg_duration = durations.iter().sum::<u64>() as f64 / durations.len() as f64;
    let min_duration = *durations.iter().min().unwrap_or(&0);
    let max_duration = *durations.iter().max().unwrap_or(&0);

    serde_json::json!({
        "operation": "rag_pipeline",
        "iterations": iterations,
        "avg_duration_ms": avg_duration,
        "min_duration_ms": min_duration,
        "max_duration_ms": max_duration,
        "total_duration_ms": durations.iter().sum::<u64>(),
        "throughput_ops_per_sec": 1000.0 / avg_duration
    })
}
