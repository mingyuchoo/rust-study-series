use crate::adapters::http::models::*;
use crate::application::usecases::*;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

/// 애플리케이션 상태
#[derive(Clone)]
pub struct AppState {
    pub create_embedding_usecase: Arc<CreateEmbeddingUseCase>,
    pub search_similar_usecase: Arc<SearchSimilarEmbeddingsUseCase>,
    pub get_embedding_usecase: Arc<GetEmbeddingUseCase>,
    pub delete_embedding_usecase: Arc<DeleteEmbeddingUseCase>,
}

/// 헬스 체크 핸들러
pub async fn health_check() -> impl IntoResponse { (StatusCode::OK, "OK") }

/// 임베딩 생성 핸들러
pub async fn create_embedding(
    State(state): State<AppState>,
    Json(payload): Json<CreateEmbeddingRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.create_embedding_usecase.execute(payload.text).await {
        | Ok(embedding) => {
            let response = EmbeddingResponse {
                id: embedding.id,
                text: embedding.text,
                vector_length: embedding.vector.len(),
                created_at: embedding.created_at.to_rfc3339(),
            };
            Ok((StatusCode::CREATED, Json(response)))
        },
        | Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// 배치 임베딩 생성 핸들러
pub async fn create_batch_embeddings(
    State(state): State<AppState>,
    Json(payload): Json<CreateBatchEmbeddingRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.create_embedding_usecase.execute_batch(payload.texts).await {
        | Ok(embeddings) => {
            let responses: Vec<EmbeddingResponse> = embeddings
                .into_iter()
                .map(|embedding| EmbeddingResponse {
                    id: embedding.id,
                    text: embedding.text,
                    vector_length: embedding.vector.len(),
                    created_at: embedding.created_at.to_rfc3339(),
                })
                .collect();
            Ok((StatusCode::CREATED, Json(responses)))
        },
        | Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// 유사도 검색 핸들러
pub async fn search_similar(
    State(state): State<AppState>,
    Json(payload): Json<SearchSimilarRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.search_similar_usecase.execute(payload.query, payload.limit).await {
        | Ok(results) => {
            let responses: Vec<SimilarityResultResponse> = results
                .into_iter()
                .map(|result| SimilarityResultResponse {
                    id: result.id,
                    text: result.text,
                    similarity: result.similarity,
                })
                .collect();
            Ok((StatusCode::OK, Json(responses)))
        },
        | Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// 임베딩 조회 핸들러 (ID)
pub async fn get_embedding(State(state): State<AppState>, Path(id): Path<i64>) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.get_embedding_usecase.execute(id).await {
        | Ok(Some(embedding)) => {
            let response = EmbeddingResponse {
                id: embedding.id,
                text: embedding.text,
                vector_length: embedding.vector.len(),
                created_at: embedding.created_at.to_rfc3339(),
            };
            Ok((StatusCode::OK, Json(response)))
        },
        | Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "임베딩을 찾을 수 없습니다".to_string(),
            }),
        )),
        | Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// 모든 임베딩 조회 핸들러
pub async fn get_all_embeddings(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.get_embedding_usecase.execute_all().await {
        | Ok(embeddings) => {
            let responses: Vec<EmbeddingResponse> = embeddings
                .into_iter()
                .map(|embedding| EmbeddingResponse {
                    id: embedding.id,
                    text: embedding.text,
                    vector_length: embedding.vector.len(),
                    created_at: embedding.created_at.to_rfc3339(),
                })
                .collect();
            Ok((StatusCode::OK, Json(responses)))
        },
        | Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// 임베딩 삭제 핸들러
pub async fn delete_embedding(State(state): State<AppState>, Path(id): Path<i64>) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match state.delete_embedding_usecase.execute(id).await {
        | Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        | Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
