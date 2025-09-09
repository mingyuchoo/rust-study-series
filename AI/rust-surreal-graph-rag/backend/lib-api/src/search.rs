//! 벡터 검색 엔드포인트 (MVP)

use actix_web::{Result, post, web};
use std::time::Instant;

use crate::azure::AzureOpenAI;
use crate::config::AppConfig;
use crate::error::Error;
use crate::models::{VectorSearchItem, VectorSearchRequest, VectorSearchResponse};
use lib_db::DB;
use log::debug;

use serde::{Deserialize, Serialize};

/// 애플리케이션 상태
pub struct AppState {
    pub cfg: AppConfig,
    pub azure: AzureOpenAI,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkSearchResult {
    pub content: String,
    pub id: surrealdb::sql::Thing,
    pub metadata: ChunkMetadata,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_index: Option<i32>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
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
    debug!("Vector search request: {:?}", payload);
    let t0 = Instant::now();
    // 1) 쿼리 임베딩 생성
    let embeddings = state.azure.embed(&[&payload.query]).await.map_err(|e| Error::External(e.to_string()))?;
    let query_vec = embeddings.get(0).cloned().unwrap_or_default();

    // 2) SurrealDB에서 코사인 유사도 기반 검색
    //    - chunk 테이블: { id, doc_id, index, content, embedding_semantic(array<float>), metadata }
    //    - SurrealQL의 vector::similarity::cosine 사용
    let top_k = payload.top_k.max(1).min(100) as i64;
    let threshold = payload.threshold;

    // SurrealDB 쿼리 실행
    let mut res = DB
        .query(
            r#"
            SELECT id, content, metadata,
                   vector::similarity::cosine(embedding_semantic, $q) AS score
            FROM chunk
            WHERE embedding_type = 'azure'
              AND embedding_deployment = $dep
              AND array::len(embedding_semantic) = array::len($q)
            ORDER BY score DESC
            LIMIT $k;
            "#,
        )
        .bind(("q", query_vec))
        .bind(("dep", state.azure.embed_deployment().to_string()))
        .bind(("k", top_k))
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    // 결과 파싱 - SurrealDB의 실제 응답 형식 사용
    //debug!("[choo] Vector search res: {:?}", res);
    let rows: Vec<ChunkSearchResult> = res.take(0)?;
    println!("rows 값: {}", rows.len());
    //debug!("[choo] Vector search rows: {:?}", rows);

    let mut items: Vec<VectorSearchItem> = Vec::new();
    for v in rows {
        let score = v.score as f32;
        if score < threshold {
            continue;
        }
        let id = v.id.to_string();
        let content = v.content;
        let metadata = serde_json::to_value(v.metadata).unwrap_or(serde_json::Value::Null);
        items.push(VectorSearchItem { id, content, score, metadata });
    }

    let elapsed = t0.elapsed().as_secs_f32();
    let total = items.len() as u32;
    Ok(web::Json(VectorSearchResponse {
        results: items,
        total,
        query_time: elapsed,
    }))
}
