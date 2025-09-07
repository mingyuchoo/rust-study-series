//! 벡터 검색 엔드포인트 (MVP)
//! 모든 주석은 한국어로 작성됩니다.

use actix_web::{post, web, Result};
use std::time::Instant;

use crate::azure::AzureOpenAI;
use crate::config::AppConfig;
use crate::error::Error;
use crate::models::{VectorSearchRequest, VectorSearchResponse, VectorSearchItem};

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
    // 1) 쿼리 임베딩
    let embeddings = state.azure.embed(&[&payload.query]).await
        .map_err(|e| Error::External(e.to_string()))?;
    let _query_vec = embeddings.get(0).cloned().unwrap_or_default();

    // 2) SurrealDB에서 유사도 검색 (MVP: 간단한 질의, 스키마가 없으므로 안전하게 빈 결과 처리)
    //    실제 구현에서는 chunk 테이블에 vector 필드가 있다고 가정하고 다음과 같은 쿼리를 사용할 수 있음:
    //    SELECT *, vector::cosine_similarity(embedding, $q) AS score FROM chunk ORDER BY score DESC LIMIT $k;
    let results: Vec<VectorSearchItem> = Vec::new();

    // 예시: DB가 준비되지 않았거나 스키마가 없는 경우를 대비하여 빈 결과 반환
    // 추후 스키마 도입 시 위의 SurrealQL을 이용해 실제 검색을 구현
    let elapsed = t0.elapsed().as_secs_f32();
    Ok(web::Json(VectorSearchResponse {
        results,
        total: 0,
        query_time: elapsed,
    }))
}
