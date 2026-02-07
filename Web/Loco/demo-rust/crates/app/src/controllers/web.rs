// [REQ-F004] 웹 프론트엔드 컨트롤러 — Tera SSR 페이지 라우트 (2026-02-07)
use loco_rs::prelude::*;

/// GET / — 홈 페이지 (공개 트랙 목록)
#[debug_handler]
async fn home(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "home/index.html", serde_json::json!({}))
}

/// GET /auth/login — 로그인 페이지
#[debug_handler]
async fn login(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "auth/login.html", serde_json::json!({}))
}

/// GET /auth/register — 회원가입 페이지
#[debug_handler]
async fn register(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "auth/register.html", serde_json::json!({}))
}

/// GET /tracks/:id — 트랙 상세 페이지
#[debug_handler]
async fn track_show(
    ViewEngine(v): ViewEngine<TeraView>,
    Path(_id): Path<i32>,
) -> Result<Response> {
    format::render().view(&v, "tracks/show.html", serde_json::json!({}))
}

/// GET /my/tracks — 내 트랙 관리 페이지
#[debug_handler]
async fn my_tracks(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "tracks/my.html", serde_json::json!({}))
}

/// GET /tracks/new — 트랙 등록 폼
#[debug_handler]
async fn track_new(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "tracks/new.html", serde_json::json!({}))
}

/// GET /tracks/:id/edit — 트랙 수정 폼
#[debug_handler]
async fn track_edit(
    ViewEngine(v): ViewEngine<TeraView>,
    Path(_id): Path<i32>,
) -> Result<Response> {
    format::render().view(&v, "tracks/edit.html", serde_json::json!({}))
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(home))
        .add("/auth/login", get(login))
        .add("/auth/register", get(register))
        .add("/tracks/new", get(track_new))
        .add("/tracks/{id}", get(track_show))
        .add("/tracks/{id}/edit", get(track_edit))
        .add("/my/tracks", get(my_tracks))
}
