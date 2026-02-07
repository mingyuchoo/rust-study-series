// [REQ-F001] Track 컨트롤러 HTTP 테스트 (2026-02-07)
use demo_app::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("track_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_create_track() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let payload = serde_json::json!({
            "title": "Test Song",
            "artist": "Test Artist",
            "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            "description": "A test track"
        });

        let response = request
            .post("/api/tracks")
            .add_header(header_name, header_value)
            .json(&payload)
            .await;

        let status = response.status_code();
        let text = response.text();
        assert_eq!(status, 200, "Create track failed: {}", text);

        let body: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(body["title"], "Test Song");
        assert_eq!(body["artist"], "Test Artist");
        assert_eq!(body["is_public"], false);
        assert_eq!(body["vote_score"], 0);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_public_tracks() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        // 트랙 생성
        let payload = serde_json::json!({
            "title": "Public Song",
            "url": "https://www.youtube.com/watch?v=abc123"
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
            .add_header(header_name.clone(), header_value.clone())
            .await;

        // 공개 트랙 목록 조회 (인증 불필요)
        let list_resp = request.get("/api/tracks").await;
        assert_eq!(list_resp.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&list_resp.text()).unwrap();
        let tracks = body["tracks"].as_array().unwrap();
        assert!(!tracks.is_empty());
        assert_eq!(tracks[0]["title"], "Public Song");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_toggle_public() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let payload = serde_json::json!({
            "title": "Toggle Song",
            "url": "https://www.youtube.com/watch?v=toggle"
        });
        let create_resp = request
            .post("/api/tracks")
            .add_header(header_name.clone(), header_value.clone())
            .json(&payload)
            .await;
        let created: serde_json::Value = serde_json::from_str(&create_resp.text()).unwrap();
        let track_id = created["id"].as_i64().unwrap();

        // 비공개 → 공개
        let toggle_resp = request
            .post(&format!("/api/tracks/{}/toggle-public", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .await;
        assert_eq!(toggle_resp.status_code(), 200);
        let toggled: serde_json::Value = serde_json::from_str(&toggle_resp.text()).unwrap();
        assert_eq!(toggled["is_public"], true);

        // 공개 → 비공개
        let toggle_resp2 = request
            .post(&format!("/api/tracks/{}/toggle-public", track_id))
            .add_header(header_name.clone(), header_value.clone())
            .await;
        let toggled2: serde_json::Value = serde_json::from_str(&toggle_resp2.text()).unwrap();
        assert_eq!(toggled2["is_public"], false);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_track() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let payload = serde_json::json!({
            "title": "Original Title",
            "url": "https://www.youtube.com/watch?v=original"
        });
        let create_resp = request
            .post("/api/tracks")
            .add_header(header_name.clone(), header_value.clone())
            .json(&payload)
            .await;
        let created: serde_json::Value = serde_json::from_str(&create_resp.text()).unwrap();
        let track_id = created["id"].as_i64().unwrap();

        let update_payload = serde_json::json!({
            "title": "Updated Title",
            "artist": "New Artist"
        });
        let update_resp = request
            .put(&format!("/api/tracks/{}", track_id))
            .add_header(header_name, header_value)
            .json(&update_payload)
            .await;
        assert_eq!(update_resp.status_code(), 200);
        let updated: serde_json::Value = serde_json::from_str(&update_resp.text()).unwrap();
        assert_eq!(updated["title"], "Updated Title");
        assert_eq!(updated["artist"], "New Artist");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_delete_track() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let payload = serde_json::json!({
            "title": "To Delete",
            "url": "https://www.youtube.com/watch?v=delete"
        });
        let create_resp = request
            .post("/api/tracks")
            .add_header(header_name.clone(), header_value.clone())
            .json(&payload)
            .await;
        let created: serde_json::Value = serde_json::from_str(&create_resp.text()).unwrap();
        let track_id = created["id"].as_i64().unwrap();

        let delete_resp = request
            .delete(&format!("/api/tracks/{}", track_id))
            .add_header(header_name, header_value)
            .await;
        assert_eq!(delete_resp.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_my_tracks() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (header_name, header_value) = prepare_data::auth_header(&user.token);

        let payload = serde_json::json!({
            "title": "My Song",
            "url": "https://www.youtube.com/watch?v=mine"
        });
        request
            .post("/api/tracks")
            .add_header(header_name.clone(), header_value.clone())
            .json(&payload)
            .await;

        let my_resp = request
            .get("/api/tracks/my")
            .add_header(header_name, header_value)
            .await;
        assert_eq!(my_resp.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&my_resp.text()).unwrap();
        let tracks = body["tracks"].as_array().unwrap();
        assert!(!tracks.is_empty());
        assert_eq!(tracks[0]["title"], "My Song");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_create_track_without_auth() {
    configure_insta!();

    request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "title": "Unauthorized",
            "url": "https://www.youtube.com/watch?v=noauth"
        });
        let response = request.post("/api/tracks").json(&payload).await;
        assert_eq!(response.status_code(), 401);
    })
    .await;
}
