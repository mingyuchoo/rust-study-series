// [REQ-F003] Comment REST API 컨트롤러 (2026-02-07)
use crate::{
    models::{
        _entities::users,
        comments::{CreateCommentParams, Model as CommentModel},
        tracks::Model as TrackModel,
    },
    views::comment::{CommentListResponse, CommentResponse},
};
use loco_rs::prelude::*;

/// GET /api/tracks/:id/comments — 트랙 댓글 목록 조회 (인증 불필요)
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response> {
    // 트랙 존재 및 공개 여부 확인
    let track = TrackModel::find_by_id(&ctx.db, id).await?;
    if !track.is_public {
        return not_found();
    }

    let comments = CommentModel::find_by_track(&ctx.db, id).await?;
    format::json(CommentListResponse::new(&comments))
}

/// POST /api/tracks/:id/comments — 댓글 작성
#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(params): Json<CreateCommentParams>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 트랙 존재 및 공개 여부 확인
    let track = TrackModel::find_by_id(&ctx.db, id).await?;
    if !track.is_public {
        return bad_request("Cannot comment on a private track");
    }

    let comment = CommentModel::create(&ctx.db, id, current_user.id, &params).await?;
    format::json(CommentResponse::new(&comment))
}

/// DELETE /api/tracks/:id/comments/:comment_id — 댓글 삭제 (작성자만)
#[debug_handler]
async fn remove(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path((_, comment_id)): Path<(i32, i32)>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let comment = CommentModel::find_by_id(&ctx.db, comment_id).await?;

    if comment.user_id != current_user.id {
        return unauthorized("Only the author can delete this comment");
    }

    CommentModel::remove(&ctx.db, comment_id).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/tracks")
        .add("/{id}/comments", get(list))
        .add("/{id}/comments", post(create))
        .add("/{id}/comments/{comment_id}", delete(remove))
}
