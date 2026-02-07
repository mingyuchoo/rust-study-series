// [REQ-F002] Vote 컨트롤러 HTTP 테스트 (2026-02-07)
use demo_app::app::App;
use loco_rs::testing::prelude::*;
use loco_rs::TestServer;
use serial_test::serial;

use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("vote_request");
        let _guard = settings.bind_to_scope();
    };
}

/// 헬퍼: 트랙을 생성하고 공개 전환한 뒤 track_id를 반환
#[allow(dead_code)]
async fn create_public_track(
    request: &TestServer,
    header_name: axum::http::HeaderName,
    header_value: axum::http::HeaderValue,
) -> i64 {
    let payload = serde_json::json!({
        "title": "Vote Test Track",
        "url": "https://www.youtube.com/watch?v=votetest"
    });
    let create_resp = request
        .post("/api/tracks")
        .add_header(header_name.clone(), header_value.clone())
        .json(&payload)
        .await;
    let created: serde_json::Value = serde_json::from_str(&create_resp.text()).unwrap();
    let track_id = created["id"].as_i64().unwrap();

    // 공개 전환
    request
        .post(&format!("/api/tracks/{}/toggle-public", track_id))
        .add_header(header_name, header_value)
        .await;

    track_id
}

#[tokio::test]
#[serial]
async fn can_upvote_track() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        let vote_payload = serde_json::json!({ "vote_type": 1 });
        let vote_resp = request
            .post(&format!("/api/tracks/{}/vote", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&vote_payload)
            .await;
        assert_eq!(vote_resp.status_code(), 200, "Vote failed: {}", vote_resp.text());

        let body: serde_json::Value = serde_json::from_str(&vote_resp.text()).unwrap();
        assert_eq!(body["vote_type"], 1);
        assert_eq!(body["track_id"], track_id);

        // vote_score 확인
        let track_resp = request.get(&format!("/api/tracks/{}", track_id)).await;
        let track: serde_json::Value = serde_json::from_str(&track_resp.text()).unwrap();
        assert_eq!(track["vote_score"], 1);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_downvote_track() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        let vote_payload = serde_json::json!({ "vote_type": -1 });
        let vote_resp = request
            .post(&format!("/api/tracks/{}/vote", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&vote_payload)
            .await;
        assert_eq!(vote_resp.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&vote_resp.text()).unwrap();
        assert_eq!(body["vote_type"], -1);

        // vote_score 확인
        let track_resp = request.get(&format!("/api/tracks/{}", track_id)).await;
        let track: serde_json::Value = serde_json::from_str(&track_resp.text()).unwrap();
        assert_eq!(track["vote_score"], -1);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_change_vote() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        // 먼저 upvote
        let vote_payload = serde_json::json!({ "vote_type": 1 });
        request
            .post(&format!("/api/tracks/{}/vote", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&vote_payload)
            .await;

        // downvote로 변경
        let vote_payload2 = serde_json::json!({ "vote_type": -1 });
        let vote_resp = request
            .post(&format!("/api/tracks/{}/vote", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&vote_payload2)
            .await;
        assert_eq!(vote_resp.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&vote_resp.text()).unwrap();
        assert_eq!(body["vote_type"], -1);

        // vote_score: 1 → -1 이므로 diff = -2, 최종 -1
        let track_resp = request.get(&format!("/api/tracks/{}", track_id)).await;
        let track: serde_json::Value = serde_json::from_str(&track_resp.text()).unwrap();
        assert_eq!(track["vote_score"], -1);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_remove_vote() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        // upvote
        let vote_payload = serde_json::json!({ "vote_type": 1 });
        request
            .post(&format!("/api/tracks/{}/vote", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&vote_payload)
            .await;

        // 투표 취소
        let remove_resp = request
            .delete(&format!("/api/tracks/{}/vote", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .await;
        assert_eq!(remove_resp.status_code(), 200);

        // vote_score 0으로 복원 확인
        let track_resp = request.get(&format!("/api/tracks/{}", track_id)).await;
        let track: serde_json::Value = serde_json::from_str(&track_resp.text()).unwrap();
        assert_eq!(track["vote_score"], 0);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_vote_with_invalid_type() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        let vote_payload = serde_json::json!({ "vote_type": 2 });
        let vote_resp = request
            .post(&format!("/api/tracks/{}/vote", track_id))
            .add_header(header_name, header_value)
            .json(&vote_payload)
            .await;
        assert_eq!(vote_resp.status_code(), 400);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_vote_without_auth() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name, header_value).await;

        // 인증 없이 투표 시도
        let vote_payload = serde_json::json!({ "vote_type": 1 });
        let vote_resp = request
            .post(&format!("/api/tracks/{}/vote", track_id))
            .json(&vote_payload)
            .await;
        assert_eq!(vote_resp.status_code(), 401);
    })
    .await;
}
