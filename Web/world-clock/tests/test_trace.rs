// =============================================================================
// @trace SPEC-004
// @trace PRD: PRD-004
// @trace FR: FR-1, FR-2, FR-3, FR-4
// @trace file-type: test
// =============================================================================

use std::path::PathBuf;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use world_clock::config::Config;
use world_clock::web::{AppState, create_router};

fn test_state() -> Arc<AppState> {
    let registry_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/registry.json");
    Arc::new(AppState::new(
        Config::default(),
        PathBuf::from("/tmp/test-config.json"),
        registry_path,
    ))
}

/// @trace TC: SPEC-004/TC-1
/// @trace FR: PRD-004/FR-1
/// @trace scenario: GET /api/trace 정상 JSON 응답
#[tokio::test]
async fn test_tc1_get_trace_returns_json() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/trace")
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
        "Content-Type should be JSON, got: {content_type}"
    );
}

/// @trace TC: SPEC-004/TC-2
/// @trace FR: PRD-004/FR-1
/// @trace scenario: GET /api/trace 응답에 trace_map 포함
#[tokio::test]
async fn test_tc2_trace_response_contains_trace_map() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/trace")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(
        json.get("trace_map").is_some(),
        "Response should contain trace_map"
    );
    assert!(
        json.get("entries").is_some(),
        "Response should contain entries"
    );
}

/// @trace TC: SPEC-004/TC-3
/// @trace FR: PRD-004/FR-2
/// @trace scenario: GET /trace 200 OK + text/html 응답
#[tokio::test]
async fn test_tc3_trace_html_returns_ok() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/trace")
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
        content_type.contains("text/html"),
        "Content-Type should be HTML, got: {content_type}"
    );
}

/// @trace TC: SPEC-004/TC-4
/// @trace FR: PRD-004/FR-2, PRD-004/FR-3
/// @trace scenario: HTML에 그래프 SVG 컨테이너 포함
#[tokio::test]
async fn test_tc4_html_contains_graph_container() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/trace")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("id=\"trace-graph\""),
        "HTML should contain trace-graph container"
    );
}

/// @trace TC: SPEC-004/TC-5
/// @trace FR: PRD-004/FR-3
/// @trace scenario: HTML에 정방향 추적 렌더링 JS 포함
#[tokio::test]
async fn test_tc5_html_contains_forward_render() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/trace")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("renderForward"),
        "HTML should contain renderForward function"
    );
}

/// @trace TC: SPEC-004/TC-6
/// @trace FR: PRD-004/FR-4
/// @trace scenario: HTML에 역방향 추적 렌더링 JS 포함
#[tokio::test]
async fn test_tc6_html_contains_reverse_render() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/trace")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("renderReverse"),
        "HTML should contain renderReverse function"
    );
}

/// @trace TC: SPEC-004/TC-7
/// @trace FR: PRD-004/FR-3, PRD-004/FR-4
/// @trace scenario: HTML에 방향 전환 탭 포함
#[tokio::test]
async fn test_tc7_html_contains_direction_tabs() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/trace")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(
        html.contains("tab-forward"),
        "HTML should contain forward tab"
    );
    assert!(
        html.contains("tab-reverse"),
        "HTML should contain reverse tab"
    );
}

/// @trace TC: SPEC-004/TC-8
/// @trace FR: PRD-004/FR-2
/// @trace scenario: 기존 API 영향 없음 확인
#[tokio::test]
async fn test_tc8_existing_api_still_works() {
    let state = test_state();
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
}
