// [REQ-F002] Vote REST API 컨트롤러 (2026-02-07)
use crate::{
    models::{
        _entities::users,
        tracks::Model as TrackModel,
        votes::{Model as VoteModel, VoteParams},
    },
    views::vote::VoteResponse,
};
use loco_rs::prelude::*;

/// POST /api/tracks/:id/vote — 투표 (body: { vote_type: 1 or -1 })
#[debug_handler]
async fn vote(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(params): Json<VoteParams>,
) -> Result<Response> {
    // vote_type 유효성 검증
    if params.vote_type != 1 && params.vote_type != -1 {
        return bad_request("vote_type must be 1 (upvote) or -1 (downvote)");
    }

    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 트랙 존재 및 공개 여부 확인
    let track = TrackModel::find_by_id(&ctx.db, id).await?;
    if !track.is_public {
        return bad_request("Cannot vote on a private track");
    }

    let vote = VoteModel::vote(&ctx.db, id, current_user.id, params.vote_type).await?;
    format::json(VoteResponse::new(&vote))
}

/// DELETE /api/tracks/:id/vote — 투표 취소
#[debug_handler]
async fn remove_vote(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    VoteModel::remove_vote(&ctx.db, id, current_user.id).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/tracks")
        .add("/{id}/vote", post(vote))
        .add("/{id}/vote", delete(remove_vote))
}
