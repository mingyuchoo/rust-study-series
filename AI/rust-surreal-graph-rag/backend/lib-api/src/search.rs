//! 벡터 검색 엔드포인트 (MVP)


use actix_web::{post, web, Result};
use std::time::Instant;

use crate::azure::AzureOpenAI;
use crate::config::AppConfig;
use crate::error::Error;
use crate::models::{VectorSearchRequest, VectorSearchResponse, VectorSearchItem};
use lib_db::DB;

pub struct AppState {
    pub cfg: AppConfig,
    pub azure: AzureOpenAI,
}

#[utoipa::path(
    tag = "search",
    post,
    path = "/api/search/vector",
    request_body = VectorSearchRequest,
    responses(
        (status = 200, description = "벡터 검색 결과", body = VectorSearchResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 500, description = "서버 오류"),
    )
)]
#[post("/api/search/vector")]
pub async fn vector_search(state: web::Data<AppState>, payload: web::Json<VectorSearchRequest>) -> Result<web::Json<VectorSearchResponse>, Error> {
    let t0 = Instant::now();
    // 1) 쿼리 임베딩 생성
    let embeddings = state.azure.embed(&[&payload.query]).await
        .map_err(|e| Error::External(e.to_string()))?;
    let query_vec = embeddings.get(0).cloned().unwrap_or_default();

    // 2) SurrealDB에서 코사인 유사도 기반 검색
    //    - chunk 테이블: { id, doc_id, index, content, embedding(array<float>), metadata }
    //    - SurrealQL의 vector::similarity::cosine 사용
    let top_k = payload.top_k.max(1).min(100) as i64;
    let threshold = payload.threshold;

    // SurrealDB 쿼리 실행
    let mut res = DB
        .query(
            r#"
            SELECT id, content, metadata,
                   vector::similarity::cosine(embedding, $q) AS score
            FROM chunk
            ORDER BY score DESC
            LIMIT $k;
            "#,
        )
        .bind(("q", query_vec))
        .bind(("k", top_k))
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    // 결과 파싱
    let rows: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
    let mut items: Vec<VectorSearchItem> = Vec::new();
    for v in rows {
        let score = v.get("score").and_then(|s| s.as_f64()).unwrap_or(0.0) as f32;
        if score < threshold {
            continue;
        }
        let id = v.get("id").map(|x| x.to_string()).unwrap_or_else(|| "null".into());
        let content = v
            .get("content")
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string();
        let metadata = v.get("metadata").cloned().unwrap_or(serde_json::json!({}));
        items.push(VectorSearchItem {
            id,
            content,
            score,
            metadata,
        });
    }

    let elapsed = t0.elapsed().as_secs_f32();
    let total = items.len() as u32;
    Ok(web::Json(VectorSearchResponse {
        results: items,
        total,
        query_time: elapsed,
    }))
}
