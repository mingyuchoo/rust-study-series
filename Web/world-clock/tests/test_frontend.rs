// =============================================================================
// @trace SPEC-003
// @trace PRD: PRD-003
// @trace FR: FR-1, FR-2, FR-3, FR-4
// @trace file-type: test
// =============================================================================

use std::path::PathBuf;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use world_clock::config::{CityEntry, Config};
use world_clock::web::{AppState, create_router};

fn test_state_with_cities() -> Arc<AppState> {
    let mut config = Config::default();
    config
        .add(CityEntry {
            name: "Seoul".to_string(),
            timezone: "Asia/Seoul".to_string(),
        })
        .unwrap();
    Arc::new(AppState::new(
        config,
        PathBuf::from("/tmp/test-config.json"),
        PathBuf::from("/tmp/test-registry.json"),
    ))
}

/// @trace TC: SPEC-003/TC-1
/// @trace FR: PRD-003/FR-1
/// @trace scenario: GET / 요청 시 200 OK + text/html 응답
#[tokio::test]
async fn test_tc1_index_html_returns_ok_with_html_content_type() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(
        content_type.contains("text/html"),
        "Content-Type should contain text/html, got: {content_type}"
    );
}

/// @trace TC: SPEC-003/TC-2
/// @trace FR: PRD-003/FR-1, PRD-003/FR-2
/// @trace scenario: HTML에 시계 표시 영역 포함
#[tokio::test]
async fn test_tc2_html_contains_clocks_area() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("id=\"clocks\""),
        "HTML should contain clocks container"
    );
}

/// @trace TC: SPEC-003/TC-3
/// @trace FR: PRD-003/FR-1, PRD-003/FR-3
/// @trace scenario: HTML에 도시 추가 폼 포함
#[tokio::test]
async fn test_tc3_html_contains_add_form() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("id=\"add-form\""),
        "HTML should contain add city form"
    );
}

/// @trace TC: SPEC-003/TC-4
/// @trace FR: PRD-003/FR-2, PRD-003/FR-3, PRD-003/FR-4
/// @trace scenario: HTML에 API 호출 JavaScript 포함
#[tokio::test]
async fn test_tc4_html_contains_api_javascript() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("fetchClocks"),
        "HTML should contain fetchClocks function"
    );
    assert!(
        html.contains("/api/clocks"),
        "HTML should reference /api/clocks endpoint"
    );
    assert!(
        html.contains("/api/cities"),
        "HTML should reference /api/cities endpoint"
    );
}

/// @trace TC: SPEC-003/TC-5
/// @trace FR: PRD-003/FR-4
/// @trace scenario: HTML에 삭제 기능 JavaScript 포함
#[tokio::test]
async fn test_tc5_html_contains_remove_javascript() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("removeCity"),
        "HTML should contain removeCity function"
    );
}

/// @trace TC: SPEC-003/TC-6
/// @trace FR: PRD-003/FR-1
/// @trace scenario: 기존 API 엔드포인트 영향 없음 확인
#[tokio::test]
async fn test_tc6_existing_api_still_works() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/clocks")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(
        content_type.contains("application/json"),
        "API should still return JSON, got: {content_type}"
    );
}
