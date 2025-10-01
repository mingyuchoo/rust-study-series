use crate::adapters::http::handlers::*;
use axum::Router;
use axum::routing::{delete, get, post};

/// HTTP 라우터 생성
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 헬스 체크
        .route("/health", get(health_check))
        // 임베딩 생성
        .route("/embeddings", post(create_embedding))
        .route("/embeddings/batch", post(create_batch_embeddings))
        // 유사도 검색
        .route("/embeddings/search", post(search_similar))
        // 임베딩 조회
        .route("/embeddings", get(get_all_embeddings))
        .route("/embeddings/:id", get(get_embedding))
        // 임베딩 삭제
        .route("/embeddings/:id", delete(delete_embedding))
        .with_state(state)
}
