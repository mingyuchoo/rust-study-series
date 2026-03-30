// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5
// @trace file-type: test
// =============================================================================

use axum_test::TestServer;
use demo_rust::handler::app;
use serde_json::json;

fn setup() -> TestServer {
    TestServer::new(app())
}

/// @trace TC: SPEC-001/TC-1
/// @trace FR: PRD-001/FR-1
/// @trace scenario: 게시글 생성 성공 - 제목과 본문으로 생성하면 201과 자동 부여된 id/created_at 반환
#[tokio::test]
async fn tc1_create_post_success() {
    let server = setup();

    let response = server
        .post("/posts")
        .json(&json!({"title": "첫 번째 글", "content": "안녕하세요"}))
        .await;

    response.assert_status(axum::http::StatusCode::CREATED);
    let body: serde_json::Value = response.json();
    assert!(body["id"].is_u64());
    assert_eq!(body["title"], "첫 번째 글");
    assert_eq!(body["content"], "안녕하세요");
    assert!(body["created_at"].is_string());
    assert!(body["updated_at"].is_string());
}

/// @trace TC: SPEC-001/TC-2
/// @trace FR: PRD-001/FR-1
/// @trace scenario: 제목 누락 시 생성 실패 - 422 반환
#[tokio::test]
async fn tc2_create_post_missing_title() {
    let server = setup();

    let response = server
        .post("/posts")
        .json(&json!({"content": "본문만"}))
        .await;

    response.assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
}

/// @trace TC: SPEC-001/TC-3
/// @trace FR: PRD-001/FR-2
/// @trace scenario: 게시글 단건 조회 성공 - 존재하는 ID로 조회하면 해당 Post 반환
#[tokio::test]
async fn tc3_get_post_success() {
    let server = setup();

    // 먼저 게시글 생성
    let create_response = server
        .post("/posts")
        .json(&json!({"title": "조회 테스트", "content": "본문"}))
        .await;
    let created: serde_json::Value = create_response.json();
    let id = created["id"].as_u64().unwrap();

    // 조회
    let response = server.get(&format!("/posts/{id}")).await;

    response.assert_status(axum::http::StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["id"], id);
    assert_eq!(body["title"], "조회 테스트");
}

/// @trace TC: SPEC-001/TC-4
/// @trace FR: PRD-001/FR-2
/// @trace scenario: 존재하지 않는 게시글 조회 시 404
#[tokio::test]
async fn tc4_get_post_not_found() {
    let server = setup();

    let response = server.get("/posts/9999").await;

    response.assert_status(axum::http::StatusCode::NOT_FOUND);
}

/// @trace TC: SPEC-001/TC-5
/// @trace FR: PRD-001/FR-3
/// @trace scenario: 빈 목록 조회 - 게시글이 없으면 빈 배열 반환
#[tokio::test]
async fn tc5_list_posts_empty() {
    let server = setup();

    let response = server.get("/posts").await;

    response.assert_status(axum::http::StatusCode::OK);
    let body: Vec<serde_json::Value> = response.json();
    assert!(body.is_empty());
}

/// @trace TC: SPEC-001/TC-6
/// @trace FR: PRD-001/FR-3
/// @trace scenario: 복수 건 목록 조회 - 2개 생성 후 2개 반환
#[tokio::test]
async fn tc6_list_posts_multiple() {
    let server = setup();

    server
        .post("/posts")
        .json(&json!({"title": "글 1", "content": "본문 1"}))
        .await;
    server
        .post("/posts")
        .json(&json!({"title": "글 2", "content": "본문 2"}))
        .await;

    let response = server.get("/posts").await;

    response.assert_status(axum::http::StatusCode::OK);
    let body: Vec<serde_json::Value> = response.json();
    assert_eq!(body.len(), 2);
}

/// @trace TC: SPEC-001/TC-7
/// @trace FR: PRD-001/FR-4
/// @trace scenario: 게시글 수정 성공 - 제목과 본문 수정 후 updated_at 변경
#[tokio::test]
async fn tc7_update_post_success() {
    let server = setup();

    // 생성
    let create_response = server
        .post("/posts")
        .json(&json!({"title": "원본", "content": "원본 본문"}))
        .await;
    let created: serde_json::Value = create_response.json();
    let id = created["id"].as_u64().unwrap();

    // 수정
    let response = server
        .put(&format!("/posts/{id}"))
        .json(&json!({"title": "수정됨", "content": "수정된 본문"}))
        .await;

    response.assert_status(axum::http::StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["title"], "수정됨");
    assert_eq!(body["content"], "수정된 본문");
}

/// @trace TC: SPEC-001/TC-8
/// @trace FR: PRD-001/FR-4
/// @trace scenario: 존재하지 않는 게시글 수정 시 404
#[tokio::test]
async fn tc8_update_post_not_found() {
    let server = setup();

    let response = server
        .put("/posts/9999")
        .json(&json!({"title": "x", "content": "x"}))
        .await;

    response.assert_status(axum::http::StatusCode::NOT_FOUND);
}

/// @trace TC: SPEC-001/TC-9
/// @trace FR: PRD-001/FR-5
/// @trace scenario: 게시글 삭제 성공 - 삭제 후 204 반환
#[tokio::test]
async fn tc9_delete_post_success() {
    let server = setup();

    // 생성
    let create_response = server
        .post("/posts")
        .json(&json!({"title": "삭제할 글", "content": "본문"}))
        .await;
    let created: serde_json::Value = create_response.json();
    let id = created["id"].as_u64().unwrap();

    // 삭제
    let response = server.delete(&format!("/posts/{id}")).await;
    response.assert_status(axum::http::StatusCode::NO_CONTENT);

    // 삭제 확인 - 조회 시 404
    let get_response = server.get(&format!("/posts/{id}")).await;
    get_response.assert_status(axum::http::StatusCode::NOT_FOUND);
}

/// @trace TC: SPEC-001/TC-10
/// @trace FR: PRD-001/FR-5
/// @trace scenario: 존재하지 않는 게시글 삭제 시 404
#[tokio::test]
async fn tc10_delete_post_not_found() {
    let server = setup();

    let response = server.delete("/posts/9999").await;

    response.assert_status(axum::http::StatusCode::NOT_FOUND);
}
