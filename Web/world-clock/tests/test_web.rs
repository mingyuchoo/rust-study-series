// =============================================================================
// @trace SPEC-002
// @trace PRD: PRD-002
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5
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

fn test_state() -> Arc<AppState> {
    Arc::new(AppState::new(
        Config::default(),
        PathBuf::from("/tmp/test-config.json"),
        PathBuf::from("/tmp/test-registry.json"),
    ))
}

fn test_state_with_cities() -> Arc<AppState> {
    let mut config = Config::default();
    config
        .add(CityEntry {
            name: "Seoul".to_string(),
            timezone: "Asia/Seoul".to_string(),
        })
        .unwrap();
    config
        .add(CityEntry {
            name: "New York".to_string(),
            timezone: "America/New_York".to_string(),
        })
        .unwrap();
    Arc::new(AppState::new(
        config,
        PathBuf::from("/tmp/test-config.json"),
        PathBuf::from("/tmp/test-registry.json"),
    ))
}

/// @trace TC: SPEC-002/TC-1
/// @trace FR: PRD-002/FR-1
/// @trace scenario: GET /api/clocks 정상 응답
#[tokio::test]
async fn test_tc1_get_clocks_success() {
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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let clocks: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(clocks.len(), 2);
    assert_eq!(clocks[0]["city"], "Seoul");
    assert_eq!(clocks[0]["timezone"], "Asia/Seoul");
    assert!(clocks[0]["time"].as_str().is_some());
    assert!(clocks[0]["utc_offset"].as_str().is_some());
    assert_eq!(clocks[1]["city"], "New York");
}

/// @trace TC: SPEC-002/TC-2
/// @trace FR: PRD-002/FR-1
/// @trace scenario: GET /api/clocks 빈 목록 시 빈 배열 반환
#[tokio::test]
async fn test_tc2_get_clocks_empty() {
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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let clocks: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(clocks.is_empty());
}

/// @trace TC: SPEC-002/TC-3
/// @trace FR: PRD-002/FR-2
/// @trace scenario: POST /api/cities 도시 추가 성공 (201)
#[tokio::test]
async fn test_tc3_add_city_success() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/cities")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Seoul","timezone":"Asia/Seoul"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let city: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(city["name"], "Seoul");
    assert_eq!(city["timezone"], "Asia/Seoul");
}

/// @trace TC: SPEC-002/TC-4
/// @trace FR: PRD-002/FR-2
/// @trace scenario: POST /api/cities 중복 도시 (409)
#[tokio::test]
async fn test_tc4_add_city_duplicate() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/cities")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Seoul","timezone":"Asia/Seoul"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

/// @trace TC: SPEC-002/TC-5
/// @trace FR: PRD-002/FR-2
/// @trace scenario: POST /api/cities 잘못된 타임존 (400)
#[tokio::test]
async fn test_tc5_add_city_invalid_timezone() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/cities")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Test","timezone":"Invalid/Zone"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// @trace TC: SPEC-002/TC-6
/// @trace FR: PRD-002/FR-3
/// @trace scenario: DELETE /api/cities/{name} 삭제 성공 (204)
#[tokio::test]
async fn test_tc6_remove_city_success() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/cities/Seoul")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

/// @trace TC: SPEC-002/TC-7
/// @trace FR: PRD-002/FR-3
/// @trace scenario: DELETE /api/cities/{name} 존재하지 않는 도시 (404)
#[tokio::test]
async fn test_tc7_remove_city_not_found() {
    let state = test_state();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/cities/Berlin")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// @trace TC: SPEC-002/TC-8
/// @trace FR: PRD-002/FR-4
/// @trace scenario: GET /api/cities 도시 목록 조회
#[tokio::test]
async fn test_tc8_list_cities() {
    let state = test_state_with_cities();
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/cities")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let cities: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(cities.len(), 2);
    assert_eq!(cities[0]["name"], "Seoul");
    assert_eq!(cities[0]["timezone"], "Asia/Seoul");
    assert_eq!(cities[1]["name"], "New York");
}

/// @trace TC: SPEC-002/TC-9
/// @trace FR: PRD-002/FR-5
/// @trace scenario: serve CLI 명령어 파싱
#[test]
fn test_tc9_serve_command_parse() {
    use clap::Parser;
    use world_clock::cli::{Cli, Commands};

    let cli = Cli::parse_from(["world-clock", "serve", "--port", "8080"]);
    match cli.command {
        Some(Commands::Serve { port }) => assert_eq!(port, 8080),
        _ => panic!("serve 명령어 파싱 실패"),
    }
}
