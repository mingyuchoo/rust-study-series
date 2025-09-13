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
        )
    ),
    tags(
        (name = "health", description = "헬스체크 API"),
        (name = "query", description = "질의/응답 API"),
        (name = "upload", description = "문서 업로드 API")
    )
)]
pub struct ApiDoc;
