//! 헬스체크 엔드포인트


use actix_web::{get, web, Result};
use chrono::Utc;

use crate::models::HealthResponse;

#[utoipa::path(
    tag = "health",
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "시스템 상태 OK", body = HealthResponse)
    )
)]
#[get("/api/health")]
pub async fn health() -> Result<web::Json<HealthResponse>> {
    let now = Utc::now().to_rfc3339();
    let body = HealthResponse {
        status: "healthy".into(),
        timestamp: now,
        services: serde_json::json!({
            "database": "unknown",
            "vector_index": "unknown",
            "llm_service": "unknown",
            "graph_engine": "unknown",
        }),
        version: "0.1.0".into(),
    };
    Ok(web::Json(body))
}
