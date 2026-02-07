// [REQ-F003] Comment 컨트롤러 HTTP 테스트 (2026-02-07)
use demo_app::app::App;
use loco_rs::testing::prelude::*;
use loco_rs::TestServer;
use serial_test::serial;

use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("comment_request");
        let _guard = settings.bind_to_scope();
    };
}

/// 헬퍼: 공개 트랙을 생성하고 track_id를 반환
#[allow(dead_code)]
async fn create_public_track(
    request: &TestServer,
    header_name: axum::http::HeaderName,
    header_value: axum::http::HeaderValue,
) -> i64 {
    let payload = serde_json::json!({
        "title": "Comment Test Track",
        "url": "https://www.youtube.com/watch?v=commenttest"
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
async fn can_create_comment() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        let payload = serde_json::json!({ "content": "Great song!" });
        let resp = request
            .post(&format!("/api/tracks/{}/comments", track_id))
            .add_header(header_name, header_value)
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), 200, "Create comment failed: {}", resp.text());

        let body: serde_json::Value = serde_json::from_str(&resp.text()).unwrap();
        assert_eq!(body["content"], "Great song!");
        assert_eq!(body["track_id"], track_id);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_comments() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        // 댓글 2개 작성
        for content in &["First comment", "Second comment"] {
            let payload = serde_json::json!({ "content": content });
            request
                .post(&format!("/api/tracks/{}/comments", track_id))
                .add_header(header_name.clone(), header_value.clone())
                .json(&payload)
                .await;
        }

        // 댓글 목록 조회 (인증 불필요)
        let list_resp = request
            .get(&format!("/api/tracks/{}/comments", track_id))
            .await;
        assert_eq!(list_resp.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&list_resp.text()).unwrap();
        let comments = body["comments"].as_array().unwrap();
        assert_eq!(comments.len(), 2);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_delete_own_comment() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name.clone(), header_value.clone()).await;

        // 댓글 작성
        let payload = serde_json::json!({ "content": "To be deleted" });
        let create_resp = request
            .post(&format!("/api/tracks/{}/comments", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&payload)
            .await;
        let created: serde_json::Value = serde_json::from_str(&create_resp.text()).unwrap();
        let comment_id = created["id"].as_i64().unwrap();

        // 댓글 삭제
        let delete_resp = request
            .delete(&format!("/api/tracks/{}/comments/{}", track_id, comment_id))
            .add_header(header_name, header_value)
            .await;
        assert_eq!(delete_resp.status_code(), 200);

        // 삭제 후 목록에서 제거 확인
        let list_resp = request
            .get(&format!("/api/tracks/{}/comments", track_id))
            .await;
        let body: serde_json::Value = serde_json::from_str(&list_resp.text()).unwrap();
        let comments = body["comments"].as_array().unwrap();
        assert!(comments.is_empty());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_comment_without_auth() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let track_id = create_public_track(&request, header_name, header_value).await;

        // 인증 없이 댓글 작성 시도
        let payload = serde_json::json!({ "content": "Unauthorized comment" });
        let resp = request
            .post(&format!("/api/tracks/{}/comments", track_id))
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), 401);
    })
    .await;
}
