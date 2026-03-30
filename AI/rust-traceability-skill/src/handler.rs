// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5
// @trace file-type: impl
// =============================================================================

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::model::{CreatePostRequest, Post, UpdatePostRequest};
use crate::store::BlogStore;

/// Axum 애플리케이션 라우터를 생성한다.
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-1 ~ TC-10
/// @trace FR: PRD-001/FR-1, PRD-001/FR-2, PRD-001/FR-3, PRD-001/FR-4, PRD-001/FR-5
pub fn app() -> Router {
    let store = BlogStore::new();

    Router::new()
        .route("/posts", post(create_post).get(list_posts))
        .route(
            "/posts/{id}",
            get(get_post).put(update_post).delete(delete_post),
        )
        .with_state(store)
}

/// POST /posts - 게시글 생성
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-1, SPEC-001/TC-2
/// @trace FR: PRD-001/FR-1
async fn create_post(
    State(store): State<BlogStore>,
    Json(req): Json<CreatePostRequest>,
) -> (StatusCode, Json<Post>) {
    let post = store.create_post(req);
    (StatusCode::CREATED, Json(post))
}

/// GET /posts/:id - 게시글 단건 조회
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-3, SPEC-001/TC-4
/// @trace FR: PRD-001/FR-2
async fn get_post(
    State(store): State<BlogStore>,
    Path(id): Path<u64>,
) -> Result<Json<Post>, StatusCode> {
    store.get_post(id).map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// GET /posts - 게시글 목록 조회
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-5, SPEC-001/TC-6
/// @trace FR: PRD-001/FR-3
async fn list_posts(State(store): State<BlogStore>) -> Json<Vec<Post>> {
    Json(store.list_posts())
}

/// PUT /posts/:id - 게시글 수정
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-7, SPEC-001/TC-8
/// @trace FR: PRD-001/FR-4
async fn update_post(
    State(store): State<BlogStore>,
    Path(id): Path<u64>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<Post>, StatusCode> {
    store
        .update_post(id, req)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// DELETE /posts/:id - 게시글 삭제
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-9, SPEC-001/TC-10
/// @trace FR: PRD-001/FR-5
async fn delete_post(
    State(store): State<BlogStore>,
    Path(id): Path<u64>,
) -> StatusCode {
    if store.delete_post(id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
