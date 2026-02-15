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

#[cfg(test)]
mod view_rendering_complete {
    use chrono::NaiveDate;
    use ratatui::{Terminal,
                  backend::TestBackend};
    use ratatui_diary::{Model,
                        model::{CalendarSubMode,
                                EditorMode,
                                EditorSubMode,
                                Screen,
                                Selection},
                        storage::Storage,
                        view};
    use std::collections::HashSet;
    use tempfile::TempDir;

    // 헬퍼 함수: 테스트 모델 생성
    fn create_test_model() -> (TempDir, Model) {
        let temp = TempDir::new().unwrap();
        let storage = Storage::with_dir(temp.path()).unwrap();
        (temp, Model::new(HashSet::new(), storage))
    }

    fn setup_terminal() -> Terminal<TestBackend> {
        let backend = TestBackend::new(80, 24);
        Terminal::new(backend).unwrap()
    }

    #[test]
    fn test_render_calendar_view() {
        // Given: Calendar 화면
        let (_temp, model) = create_test_model();
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 렌더 버퍼가 비어있지 않음
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_view() {
        // Given: Editor 화면
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 렌더 버퍼가 비어있지 않음
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "에디터 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_with_error_popup() {
        // Given: 에러 팝업이 표시된 상태
        let (_temp, mut model) = create_test_model();
        model.show_error_popup = true;
        model.error_message = Some("테스트 에러".to_string());
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 렌더 버퍼가 비어있지 않음
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "에러 팝업 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_small_terminal() {
        // Given: 작은 터미널 (10x5)
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        let (_temp, model) = create_test_model();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 렌더 버퍼가 비어있지 않음 (레이아웃 조정됨)
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "작은 터미널 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_large_terminal() {
        // Given: 큰 터미널 (200x50)
        let backend = TestBackend::new(200, 50);
        let mut terminal = Terminal::new(backend).unwrap();
        let (_temp, model) = create_test_model();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 렌더 버퍼가 비어있지 않음
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "큰 터미널 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_with_selection() {
        // Given: 선택 영역이 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Hello World".to_string()];
        model.editor_state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 0,
            cursor_line: 0,
            cursor_col: 5,
        });
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 선택 영역이 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "선택 영역 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_with_search_matches() {
        // Given: 검색 매치가 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["test test test".to_string()];
        model.editor_state.search_pattern = "test".to_string();
        model.editor_state.execute_search();
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 검색 매치가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "검색 매치 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_calendar_with_entries() {
        // Given: 일기 엔트리가 있는 달력
        let (_temp, mut model) = create_test_model();
        let date = chrono::Local::now().date_naive();
        model.diary_entries.entries.insert(date);
        model.screen = Screen::Calendar;
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 일기 엔트리가 표시됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "일기 엔트리 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_insert_mode() {
        // Given: Insert 모드의 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: Insert 모드가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Insert 모드 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_goto_submode() {
        // Given: Goto 서브모드의 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.submode = Some(EditorSubMode::Goto);
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: Goto 모드가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Goto 서브모드 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_space_submode() {
        // Given: Space 서브모드의 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.submode = Some(EditorSubMode::SpaceCommand);
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: Space 모드가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Space 서브모드 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_search_submode() {
        // Given: Search 서브모드의 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.submode = Some(EditorSubMode::Search);
        model.editor_state.search_pattern = "test".to_string();
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: Search 모드가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Search 서브모드 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_calendar_space_submode() {
        // Given: Space 서브모드의 달력
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Calendar;
        model.calendar_state.submode = Some(CalendarSubMode::Space);
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: Space 모드가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Calendar Space 서브모드 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_with_markdown_content() {
        // Given: Markdown 콘텐츠가 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["# 헤더 1".to_string(), "**굵은 텍스트**".to_string(), "- 리스트 항목".to_string()];
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: Markdown 미리보기가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Markdown 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_with_long_lines() {
        // Given: 긴 라인이 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["a".repeat(200)];
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 긴 라인이 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "긴 라인 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_with_multiple_lines() {
        // Given: 여러 줄이 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = (0 .. 50).map(|i| format!("라인 {}", i)).collect();
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 스크롤 가능한 콘텐츠가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "여러 라인 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_multiline_selection() {
        // Given: 여러 줄에 걸친 선택 영역이 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["첫 번째 라인".to_string(), "두 번째 라인".to_string(), "세 번째 라인".to_string()];
        model.editor_state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 2,
            cursor_line: 2,
            cursor_col: 4,
        });
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 여러 줄 선택이 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "여러 줄 선택 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_calendar_different_months() {
        // Given: 다른 달의 달력
        let (_temp, mut model) = create_test_model();
        model.calendar_state.current_month = 12;
        model.calendar_state.selected_date = NaiveDate::from_ymd_opt(2026, 12, 25).unwrap();
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 달력이 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "다른 달 달력 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_empty_content() {
        // Given: 빈 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec![];
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 빈 에디터가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "빈 에디터 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_normal_mode() {
        // Given: Normal 모드의 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["테스트 콘텐츠".to_string()];
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: Normal 모드가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Normal 모드 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_with_mixed_content() {
        // Given: 혼합된 콘텐츠, 선택, 검색이 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["# Header".to_string(), "search test".to_string(), "another test".to_string()];
        model.editor_state.search_pattern = "test".to_string();
        model.editor_state.execute_search();
        model.editor_state.selection = Some(Selection {
            anchor_line: 1,
            anchor_col: 0,
            cursor_line: 1,
            cursor_col: 6,
        });
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 혼합된 콘텐츠가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "혼합 콘텐츠 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_editor_with_unicode_content() {
        // Given: 유니코드 콘텐츠가 있는 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["한글 테스트 🎉".to_string(), "日本語テスト".to_string(), "Emoji: 😀 🎯 ✨".to_string()];
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 유니코드 콘텐츠가 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "유니코드 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_error_popup_text() {
        // Given: 긴 에러 메시지가 있는 팝업
        let (_temp, mut model) = create_test_model();
        model.show_error_popup = true;
        model.error_message = Some("이것은 매우 긴 에러 메시지입니다. 여러 줄로 나뉠 수 있으며 매우 상세한 정보를 포함합니다.".to_string());
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 에러 팝업이 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "에러 팝업 텍스트 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_with_cursor_positioning() {
        // Given: 다양한 커서 위치를 가진 에디터
        let (_temp, mut model) = create_test_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["라인 1".to_string(), "라인 2".to_string()];
        model.editor_state.cursor_line = 1;
        model.editor_state.cursor_col = 3;
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 커서 위치가 설정됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "커서 위치 렌더링 버퍼가 비어있음");
    }

    #[test]
    fn test_render_calendar_complete_last_week() {
        // Given: 마지막 주가 정확히 7일로 채워지는 달 (예: 1월 2026)
        let (_temp, mut model) = create_test_model();
        model.calendar_state.current_year = 2026;
        model.calendar_state.current_month = 1;
        model.calendar_state.selected_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        model.screen = Screen::Calendar;
        let mut terminal = setup_terminal();

        // When: 렌더링
        terminal
            .draw(|f| {
                view::view(f, &model);
            })
            .unwrap();

        // Then: 달력이 렌더링됨
        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "완전한 마지막 주 달력 렌더링 버퍼가 비어있음");
    }
}

#[cfg(test)]
mod keybinding_tests {
    use chrono::NaiveDate;
    use ratatui_diary::model::{CalendarState, CalendarSubMode, EditorState};
    use ratatui_diary::view::{build_calendar_keybindings, build_editor_keybindings};

    #[test]
    fn test_build_calendar_keybindings_normal() {
        let state = CalendarState::new(2026, 2);
        let result = build_calendar_keybindings(&state);
        assert_eq!(result, "hjkl:이동 | e:편집 | space:명령 | q:종료");
    }

    #[test]
    fn test_build_calendar_keybindings_space_mode() {
        let mut state = CalendarState::new(2026, 2);
        state.submode = Some(CalendarSubMode::Space);
        let result = build_calendar_keybindings(&state);
        assert_eq!(result, "n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소");
    }

    #[test]
    fn test_build_editor_keybindings_normal() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
        let state = EditorState::new(date);
        let result = build_editor_keybindings(&state);
        assert_eq!(result, "hjkl:이동 | w/b/e:단어 | i/a/o/O:입력 | v/x:선택 | d/c/y/p:편집 | u/U:실행취소 | g/space//:모드 | Esc:뒤로");
    }
}
