// [REQ-F001] Track REST API 컨트롤러 (2026-02-07)
use crate::{
    models::{
        _entities::users,
        tracks::{CreateTrackParams, Model as TrackModel, UpdateTrackParams},
    },
    views::track::{TrackListResponse, TrackResponse},
};
use loco_rs::prelude::*;

/// GET /api/tracks — 공개 트랙 목록 조회
#[debug_handler]
async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let tracks = TrackModel::find_public(&ctx.db).await?;
    format::json(TrackListResponse::new(&tracks))
}

/// GET /api/tracks/:id — 트랙 상세 조회 (공개이거나 소유자)
#[debug_handler]
async fn get_one(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let track = TrackModel::find_by_id(&ctx.db, id).await?;
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !track.is_public && track.user_id != current_user.id {
        return unauthorized("You do not have access to this track");
    }

    format::json(TrackResponse::new(&track))
}

/// GET /api/tracks/:id/public — 공개 트랙 상세 조회 (인증 불필요)
#[debug_handler]
async fn get_public(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response> {
    let track = TrackModel::find_by_id(&ctx.db, id).await?;

    if !track.is_public {
        return not_found();
    }

    format::json(TrackResponse::new(&track))
}

/// POST /api/tracks — 트랙 등록
#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateTrackParams>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let track = TrackModel::create(&ctx.db, current_user.id, &params).await?;
    format::json(TrackResponse::new(&track))
}

/// PUT /api/tracks/:id — 트랙 수정 (소유자만)
#[debug_handler]
async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(params): Json<UpdateTrackParams>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let track = TrackModel::find_by_id(&ctx.db, id).await?;

    if track.user_id != current_user.id {
        return unauthorized("Only the owner can update this track");
    }

    let updated = track.into_active_model().update_track(&ctx.db, &params).await?;
    format::json(TrackResponse::new(&updated))
}

/// DELETE /api/tracks/:id — 트랙 삭제 (소유자만)
#[debug_handler]
async fn remove(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let track = TrackModel::find_by_id(&ctx.db, id).await?;

    if track.user_id != current_user.id {
        return unauthorized("Only the owner can delete this track");
    }

    track.into_active_model().delete(&ctx.db).await?;
    format::empty()
}

/// POST /api/tracks/:id/toggle-public — 공개/비공개 전환 (소유자만)
#[debug_handler]
async fn toggle_public(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let track = TrackModel::find_by_id(&ctx.db, id).await?;

    if track.user_id != current_user.id {
        return unauthorized("Only the owner can toggle public status");
    }

    let updated = track.into_active_model().toggle_public(&ctx.db).await?;
    format::json(TrackResponse::new(&updated))
}

/// GET /api/my/tracks — 내 트랙 목록 조회
#[debug_handler]
async fn my_tracks(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let tracks = TrackModel::find_by_user(&ctx.db, current_user.id).await?;
    format::json(TrackListResponse::new(&tracks))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/tracks")
        .add("/", get(list))
        .add("/", post(create))
        .add("/my", get(my_tracks))
        .add("/{id}", get(get_public))
        .add("/{id}/detail", get(get_one))
        .add("/{id}", put(update))
        .add("/{id}", delete(remove))
        .add("/{id}/toggle-public", post(toggle_public))
}
