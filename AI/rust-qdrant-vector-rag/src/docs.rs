//! OpenAPI 스키마 및 Swagger UI 구성을 제공하는 모듈
//! 모든 주석은 한국어로 작성됩니다.

use utoipa::OpenApi;

/// OpenAPI 문서 정의
/// - paths: 문서화할 엔드포인트 목록
/// - components: 스키마에 포함할 타입들
/// - tags: 그룹 태그
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_handler,
        crate::handlers::health::simple_health_handler,
        crate::handlers::query::query_handler,
        crate::handlers::query::simple_query_handler,
        crate::handlers::upload::upload_json_handler,
        crate::handlers::upload::upload_handler,
        crate::handlers::upload::upload_handler_root,
        crate::handlers::monitoring::metrics_handler,
        crate::handlers::monitoring::prometheus_metrics_handler,
        crate::handlers::monitoring::cache_stats_handler,
        crate::handlers::monitoring::clear_cache_handler,
        crate::handlers::monitoring::benchmark_handler,
        crate::handlers::monitoring::health_with_performance_handler,
        crate::handlers::query::query_handler_root,
        crate::handlers::query::simple_query_handler_root,
    ),
    components(
        schemas(
            crate::models::RAGResponse,
            crate::models::SourceReference,
            crate::models::UploadResponse,
            crate::models::UploadStatus,
            crate::models::HealthResponse,
            crate::models::HealthStatus,
            crate::models::ServiceHealthStatus,
            crate::handlers::query::QueryRequest,
            crate::handlers::query::QueryConfig,
            crate::handlers::upload::UploadRequest,
            crate::handlers::monitoring::PerformanceMetricsResponse,
            crate::handlers::monitoring::SystemMetrics,
            crate::handlers::monitoring::ApplicationMetrics,
            crate::handlers::monitoring::CacheMetrics,
            crate::handlers::monitoring::CacheStats,
            crate::handlers::monitoring::ConnectionMetrics,
            crate::handlers::monitoring::PoolMetrics,
            crate::handlers::monitoring::ClearCacheRequest,
            crate::handlers::monitoring::BenchmarkRequest,
        )
    ),
    tags(
        (name = "health", description = "헬스체크 API"),
        (name = "query", description = "질의/응답 API"),
        (name = "upload", description = "문서 업로드 API"),
        (name = "monitoring", description = "모니터링 및 메트릭 API")
    )
)]
pub struct ApiDoc;
