use chrono::NaiveDate;
use ratatui_diary::{model::Model, storage::Storage};
use std::collections::HashSet;
use tempfile::TempDir;

#[test]
fn test_calendar_preview_loads_from_storage() {
    // Given: 특정 날짜에 다이어리가 저장되어 있음
    let temp_dir = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp_dir.path()).unwrap();
    let test_date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();

    // 다이어리 작성 및 저장
    let _ = storage.save(test_date, "테스트 다이어리 내용");

    let entries = storage.scan_entries().unwrap();
    let mut model = Model::new(entries, storage);

    // 달력 화면으로 돌아옴
    model.calendar_state.selected_date = test_date;

    // When: 저장된 날짜에 대해 Storage.load() 호출
    let content = model.storage.load(test_date);

    // Then: 저장된 내용이 로드됨
    assert!(content.is_ok());
    assert_eq!(content.unwrap(), "테스트 다이어리 내용");
}

#[test]
fn test_calendar_preview_shows_empty_message() {
    // Given: 다이어리가 없는 날짜
    let temp_dir = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp_dir.path()).unwrap();
    let test_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

    let entries = HashSet::new();
    let model = Model::new(entries, storage);

    // When: 저장되지 않은 날짜에 대해 Storage.load() 호출
    let content = model.storage.load(test_date);

    // Then: 에러 반환
    assert!(content.is_err());
}
