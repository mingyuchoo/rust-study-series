use chrono::NaiveDate;
use ratatui_diary::{model::{EditorMode,
                            EditorSubMode,
                            Model,
                            Screen,
                            Selection},
                    storage::Storage};
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

#[test]
fn test_editor_content_updates_preview() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let mut model = Model::new(entries, storage);

    // 에디터로 전환
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Insert;

    // 텍스트 입력
    model.editor_state.insert_char('#');
    model.editor_state.insert_char(' ');
    model.editor_state.insert_char('H');

    // 콘텐츠 확인
    let content = model.editor_state.get_content();
    assert_eq!(content, "# H");

    // 렌더링 시 markdown::render_to_text가 호출됨 (실제 UI 테스트는 불가)
}

#[test]
fn test_calendar_preview_empty_diary() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let model = Model::new(entries, storage);

    // 다이어리가 없는 날짜 선택
    let date = model.calendar_state.selected_date;
    let result = model.storage.load(date);

    // 로드 실패 시 에러 반환 (view에서 처리)
    assert!(result.is_err());
}

#[test]
fn test_editor_selection_highlight_state() {
    // Given: 에디터에 텍스트가 있고 선택 영역이 설정됨
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let mut model = Model::new(entries, storage);
    model.screen = Screen::Editor;

    // 테스트 텍스트 작성
    model.editor_state.content = vec!["Hello World".to_string(), "Rust is great".to_string()];

    // When: 선택 영역 설정 (0,0) ~ (0,5) "Hello"
    model.editor_state.selection = Some(Selection {
        anchor_line: 0,
        anchor_col: 0,
        cursor_line: 0,
        cursor_col: 5,
    });

    // Then: selection_range가 올바르게 계산됨
    let range = model.editor_state.get_selection_range();
    assert!(range.is_some());
    let ((start_line, start_col), (end_line, end_col)) = range.unwrap();
    assert_eq!(start_line, 0);
    assert_eq!(start_col, 0);
    assert_eq!(end_line, 0);
    assert_eq!(end_col, 5);

    // 선택된 텍스트 확인
    let selected_text = model.editor_state.get_selected_text();
    assert_eq!(selected_text, Some("Hello".to_string()));
}

#[test]
fn test_editor_search_matches_state() {
    // Given: 에디터에 검색 가능한 텍스트가 있음
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let mut model = Model::new(entries, storage);
    model.screen = Screen::Editor;

    model.editor_state.content = vec!["test text test".to_string(), "another test".to_string()];

    // When: "test"를 검색
    model.editor_state.search_pattern = "test".to_string();
    model.editor_state.execute_search();

    // Then: 3개의 매치가 발견됨
    assert_eq!(model.editor_state.search_matches.len(), 3);
    assert_eq!(model.editor_state.search_matches[0], (0, 0));
    assert_eq!(model.editor_state.search_matches[1], (0, 10));
    assert_eq!(model.editor_state.search_matches[2], (1, 8));

    // 현재 매치는 첫 번째
    assert_eq!(model.editor_state.current_match_index, 0);

    // 선택 영역이 검색어 길이만큼 설정됨
    let selection = model.editor_state.selection.as_ref().unwrap();
    assert_eq!(selection.cursor_col - selection.anchor_col, 4); // "test".len()
}

#[test]
fn test_editor_submode_display_state() {
    // Given: 에디터가 Normal 모드
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let mut model = Model::new(entries, storage);
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Normal;

    // When: Goto 모드 활성화
    model.editor_state.submode = Some(EditorSubMode::Goto);

    // Then: submode가 설정됨
    assert_eq!(model.editor_state.submode, Some(EditorSubMode::Goto));

    // When: Space 모드 활성화
    model.editor_state.submode = Some(EditorSubMode::SpaceCommand);

    // Then: submode가 변경됨
    assert_eq!(model.editor_state.submode, Some(EditorSubMode::SpaceCommand));

    // When: Search 모드 활성화
    model.editor_state.submode = Some(EditorSubMode::Search);
    model.editor_state.search_pattern = "test".to_string();

    // Then: 검색 패턴이 설정됨
    assert_eq!(model.editor_state.submode, Some(EditorSubMode::Search));
    assert_eq!(model.editor_state.search_pattern, "test");
}

#[test]
fn test_editor_multi_line_selection_highlight() {
    // Given: 여러 줄에 걸친 선택 영역
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let mut model = Model::new(entries, storage);
    model.screen = Screen::Editor;

    model.editor_state.content = vec!["First line".to_string(), "Second line".to_string(), "Third line".to_string()];

    // When: 여러 줄 선택 (0,6) ~ (2,5) "line\nSecond line\nThird"
    model.editor_state.selection = Some(Selection {
        anchor_line: 0,
        anchor_col: 6,
        cursor_line: 2,
        cursor_col: 5,
    });

    // Then: 선택 범위가 올바르게 계산됨
    let range = model.editor_state.get_selection_range();
    assert!(range.is_some());

    let selected_text = model.editor_state.get_selected_text();
    assert!(selected_text.is_some());
    let text = selected_text.unwrap();

    // "line"부터 시작해서 "Third"까지 선택됨
    assert!(text.starts_with("line"));
    assert!(text.contains("Second line"));
    assert!(text.ends_with("Third"));
}

#[test]
fn test_editor_search_navigation_updates_selection() {
    // Given: 여러 검색 매치가 있음
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let mut model = Model::new(entries, storage);
    model.screen = Screen::Editor;

    model.editor_state.content = vec!["test abc test".to_string(), "test def".to_string()];

    model.editor_state.search_pattern = "test".to_string();
    model.editor_state.execute_search();

    // When: 다음 매치로 이동
    let first_selection = model.editor_state.selection.clone();
    model.editor_state.search_next();

    // Then: 선택 영역이 다음 매치로 업데이트됨
    let second_selection = model.editor_state.selection.clone();
    assert!(first_selection != second_selection);

    // 현재 매치 인덱스가 증가함
    assert_eq!(model.editor_state.current_match_index, 1);

    // When: 이전 매치로 이동
    model.editor_state.search_prev();

    // Then: 선택 영역이 이전 매치로 돌아감
    assert_eq!(model.editor_state.current_match_index, 0);
}
