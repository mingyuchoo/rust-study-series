//! 헬스체크 엔드포인트

use crate::models::HealthResponse;
use crate::search::AppState;
use actix_web::{Result, get, web};
use chrono::Utc;
use lib_db::DB;

#[utoipa::path(
    tag = "health",
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "시스템 상태 OK", body = HealthResponse)
    )
)]
#[get("/api/health")]
pub async fn health(state: web::Data<AppState>) -> Result<web::Json<HealthResponse>> {
    // 현재 시각
    let now = Utc::now().to_rfc3339();

    // 1) 데이터베이스 연결 상태 확인(간단 쿼리)
    let db_status = match DB.query("RETURN 1;").await {
        | Ok(_) => "ok".to_string(),
        | Err(e) => format!("error: {}", e),
    };

    // 2) 인덱스/그래프 테이블 카운트 조회 (GROUP ALL 사용)
    let (mut chunk_cnt, mut entity_cnt, mut relation_cnt) = (0_i64, 0_i64, 0_i64);
    if let Ok(mut res) = DB
        .query(
            r#"
            SELECT count() AS cnt FROM chunk GROUP ALL;
            SELECT count() AS cnt FROM entity GROUP ALL;
            SELECT count() AS cnt FROM relation GROUP ALL;
            "#,
        )
        .await
    {
        let v0: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
        if let Some(v) = v0.get(0) {
            chunk_cnt = v.get("cnt").and_then(|x| x.as_i64()).unwrap_or(0);
        }
        let v1: Vec<serde_json::Value> = res.take(1).unwrap_or_default();
        if let Some(v) = v1.get(0) {
            entity_cnt = v.get("cnt").and_then(|x| x.as_i64()).unwrap_or(0);
        }
        let v2: Vec<serde_json::Value> = res.take(2).unwrap_or_default();
        if let Some(v) = v2.get(0) {
            relation_cnt = v.get("cnt").and_then(|x| x.as_i64()).unwrap_or(0);
        }
    }

    let vector_index_status = if chunk_cnt > 0 {
        format!("ready (chunks: {})", chunk_cnt)
    } else {
        "empty".to_string()
    };

    let graph_engine_status = if entity_cnt > 0 || relation_cnt > 0 {
        format!("ready (entities: {}, relations: {})", entity_cnt, relation_cnt)
    } else {
        "empty".to_string()
    };

    // 3) LLM 서비스 구성 상태(Azure OpenAI)
    let a = &state.cfg.azure;
    let llm_service_status = if a.endpoint.is_empty() || a.api_key.is_empty() {
        "unconfigured".to_string()
    } else {
        format!("configured (chat={}, embed={})", a.chat_deployment, a.embed_deployment)
    };

    let body = HealthResponse {
        status: "healthy".into(),
        timestamp: now,
        services: serde_json::json!({
            "database": db_status,
            "vector_index": vector_index_status,
            "llm_service": llm_service_status,
            "graph_engine": graph_engine_status,
        }),
        version: "0.1.0".into(),
    };
    Ok(web::Json(body))
}
