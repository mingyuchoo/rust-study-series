// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-2, FR-3, FR-4
// @trace file-type: test
// =============================================================================

use std::path::PathBuf;

use world_clock::config::{CityEntry, Config};
use world_clock::error::AppError;

/// @trace TC: SPEC-001/TC-5
/// @trace FR: PRD-001/FR-2
/// @trace scenario: 도시 추가 성공
#[test]
fn test_tc5_add_city_success() {
    let mut config = Config::default();
    let entry = CityEntry {
        name: "Seoul".to_string(),
        timezone: "Asia/Seoul".to_string(),
    };

    config.add(entry).unwrap();
    assert_eq!(config.cities.len(), 1);
    assert_eq!(config.cities[0].name, "Seoul");
    assert_eq!(config.cities[0].timezone, "Asia/Seoul");
}

/// @trace TC: SPEC-001/TC-6
/// @trace FR: PRD-001/FR-2
/// @trace scenario: 중복 도시 추가 시 에러
#[test]
fn test_tc6_add_duplicate_city_error() {
    let mut config = Config::default();
    let entry = CityEntry {
        name: "Seoul".to_string(),
        timezone: "Asia/Seoul".to_string(),
    };

    config.add(entry.clone()).unwrap();
    let result = config.add(entry);

    assert!(result.is_err());
    assert!(matches!(result, Err(AppError::DuplicateCity(_))));
}

/// @trace TC: SPEC-001/TC-7
/// @trace FR: PRD-001/FR-2
/// @trace scenario: 잘못된 타임존으로 추가 시 에러
#[test]
fn test_tc7_add_invalid_timezone_error() {
    let mut config = Config::default();
    let entry = CityEntry {
        name: "Test".to_string(),
        timezone: "Invalid/Zone".to_string(),
    };

    let result = config.add(entry);
    assert!(result.is_err());
    assert!(matches!(result, Err(AppError::UnknownTimezone(_))));
}

/// @trace TC: SPEC-001/TC-8
/// @trace FR: PRD-001/FR-3
/// @trace scenario: 도시 삭제 성공
#[test]
fn test_tc8_remove_city_success() {
    let mut config = Config::default();
    config
        .add(CityEntry {
            name: "Seoul".to_string(),
            timezone: "Asia/Seoul".to_string(),
        })
        .unwrap();

    config.remove("Seoul").unwrap();
    assert!(config.cities.is_empty());
}

/// @trace TC: SPEC-001/TC-9
/// @trace FR: PRD-001/FR-3
/// @trace scenario: 존재하지 않는 도시 삭제 시 에러
#[test]
fn test_tc9_remove_nonexistent_city_error() {
    let mut config = Config::default();
    let result = config.remove("Berlin");

    assert!(result.is_err());
    assert!(matches!(result, Err(AppError::CityNotFound(_))));
}

/// @trace TC: SPEC-001/TC-10
/// @trace FR: PRD-001/FR-2, PRD-001/FR-3
/// @trace scenario: 설정 파일 저장/로드 왕복
#[test]
fn test_tc10_save_and_load_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");

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

    config.save(&path).unwrap();

    let loaded = Config::load(&path).unwrap();
    assert_eq!(config, loaded);
}

/// @trace TC: SPEC-001/TC-11
/// @trace FR: PRD-001/FR-1, PRD-001/FR-4
/// @trace scenario: 설정 파일 없을 때 빈 설정 반환
#[test]
fn test_tc11_load_missing_file_returns_default() {
    let path = PathBuf::from("/nonexistent/path/config.json");
    let config = Config::load(&path).unwrap();
    assert_eq!(config, Config::default());
}

/// @trace TC: SPEC-001/TC-12
/// @trace FR: PRD-001/FR-4
/// @trace scenario: 도시 목록 조회
#[test]
fn test_tc12_list_cities() {
    let mut config = Config::default();
    config
        .add(CityEntry {
            name: "Seoul".to_string(),
            timezone: "Asia/Seoul".to_string(),
        })
        .unwrap();
    config
        .add(CityEntry {
            name: "London".to_string(),
            timezone: "Europe/London".to_string(),
        })
        .unwrap();

    assert_eq!(config.cities.len(), 2);
    assert_eq!(config.cities[0].name, "Seoul");
    assert_eq!(config.cities[1].name, "London");
}
