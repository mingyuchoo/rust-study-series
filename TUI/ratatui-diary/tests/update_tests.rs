use ratatui_diary::{Model,
                    Msg,
                    message::InsertPosition,
                    model::{CalendarSubMode,
                            EditorMode,
                            EditorSubMode,
                            Screen},
                    storage::Storage};
use std::collections::HashSet;
use tempfile::TempDir;

// ===== 달력 네비게이션 테스트 =====

#[test]
fn test_calendar_move_navigation() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);

    let original_date = model.calendar_state.selected_date;

    ratatui_diary::update::update(&mut model, Msg::CalendarMoveRight);
    assert_ne!(model.calendar_state.selected_date, original_date);

    ratatui_diary::update::update(&mut model, Msg::CalendarMoveLeft);
    assert_eq!(model.calendar_state.selected_date, original_date);
}

#[test]
fn test_calendar_select_date_switches_to_editor() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);

    let cmd = ratatui_diary::update::update(&mut model, Msg::CalendarSelectDate);

    assert_eq!(model.screen, Screen::Editor);
    assert!(cmd.is_some());
}

#[test]
fn test_calendar_space_mode() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);

    ratatui_diary::update::update(&mut model, Msg::CalendarEnterSpaceMode);
    assert_eq!(model.calendar_state.submode, Some(CalendarSubMode::Space));

    ratatui_diary::update::update(&mut model, Msg::CalendarExitSubMode);
    assert_eq!(model.calendar_state.submode, None);
}

#[test]
fn test_calendar_space_month_navigation() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.calendar_state.submode = Some(CalendarSubMode::Space);
    let original_month = model.calendar_state.current_month;

    ratatui_diary::update::update(&mut model, Msg::CalendarSpaceNextMonth);
    let expected = if original_month == 12 { 1 } else { original_month + 1 };
    assert_eq!(model.calendar_state.current_month, expected);
}

// ===== 에디터 네비게이션 테스트 =====

#[test]
fn test_editor_basic_movement() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["hello world".to_string(), "test".to_string()];
    model.editor_state.cursor_line = 0;
    model.editor_state.cursor_col = 0;

    ratatui_diary::update::update(&mut model, Msg::EditorMoveRight);
    assert_eq!(model.editor_state.cursor_col, 1);

    ratatui_diary::update::update(&mut model, Msg::EditorMoveDown);
    assert_eq!(model.editor_state.cursor_line, 1);
}

#[test]
fn test_editor_word_movement() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["hello world test".to_string()];
    model.editor_state.cursor_col = 0;

    ratatui_diary::update::update(&mut model, Msg::EditorWordNext);
    assert!(model.editor_state.cursor_col > 0);
}

// ===== 에디터 Goto 모드 테스트 =====

#[test]
fn test_editor_goto_mode() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;

    ratatui_diary::update::update(&mut model, Msg::EditorEnterGotoMode);
    assert_eq!(model.editor_state.submode, Some(EditorSubMode::Goto));

    ratatui_diary::update::update(&mut model, Msg::EditorExitSubMode);
    assert_eq!(model.editor_state.submode, None);
}

#[test]
fn test_editor_goto_doc_start() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["line1".to_string(), "line2".to_string(), "line3".to_string()];
    model.editor_state.cursor_line = 2;
    model.editor_state.cursor_col = 3;
    model.editor_state.submode = Some(EditorSubMode::Goto);

    ratatui_diary::update::update(&mut model, Msg::EditorGotoDocStart);
    assert_eq!(model.editor_state.cursor_line, 0);
    assert_eq!(model.editor_state.cursor_col, 0);
}

#[test]
fn test_editor_goto_line_end() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["hello world".to_string()];
    model.editor_state.cursor_col = 0;
    model.editor_state.submode = Some(EditorSubMode::Goto);

    ratatui_diary::update::update(&mut model, Msg::EditorGotoLineEnd);
    assert_eq!(model.editor_state.cursor_col, "hello world".len());
}

// ===== 에디터 Insert 모드 테스트 =====

#[test]
fn test_editor_insert_char() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Insert;

    ratatui_diary::update::update(&mut model, Msg::EditorInsertChar('a'));

    assert_eq!(model.editor_state.content[0], "a");
}

#[test]
fn test_editor_enter_insert_mode() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Normal;
    model.editor_state.content = vec!["test".to_string()];
    model.editor_state.cursor_col = 2;

    ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::BeforeCursor));
    assert_eq!(model.editor_state.mode, EditorMode::Insert);
    assert_eq!(model.editor_state.cursor_col, 2);

    // Reset
    model.editor_state.mode = EditorMode::Normal;
    ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::AfterCursor));
    assert_eq!(model.editor_state.cursor_col, 3);
}

// ===== 에디터 Selection 테스트 =====

#[test]
fn test_editor_toggle_selection() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;

    ratatui_diary::update::update(&mut model, Msg::EditorToggleSelection);
    assert!(model.editor_state.selection.is_some());

    ratatui_diary::update::update(&mut model, Msg::EditorToggleSelection);
    assert!(model.editor_state.selection.is_none());
}

#[test]
fn test_editor_select_line() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["hello".to_string()];

    ratatui_diary::update::update(&mut model, Msg::EditorSelectLine);
    assert!(model.editor_state.selection.is_some());
    let sel = model.editor_state.selection.as_ref().unwrap();
    assert_eq!(sel.anchor_col, 0);
    assert_eq!(sel.cursor_col, 5);
}

// ===== 에디터 편집 기능 테스트 =====

#[test]
fn test_editor_delete_with_selection() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["hello world".to_string()];
    model.editor_state.cursor_col = 0;

    // Select "hello"
    ratatui_diary::update::update(&mut model, Msg::EditorSelectLine);
    ratatui_diary::update::update(&mut model, Msg::EditorDelete);

    // Selection should be cleared after delete
    assert!(model.editor_state.selection.is_none());
}

#[test]
fn test_editor_yank_and_paste() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["hello".to_string()];

    ratatui_diary::update::update(&mut model, Msg::EditorSelectLine);
    ratatui_diary::update::update(&mut model, Msg::EditorYank);

    assert!(!model.editor_state.clipboard.is_empty());
}

// ===== 에디터 Undo/Redo 테스트 =====

#[test]
fn test_editor_undo_redo() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Insert;

    // Make a change
    ratatui_diary::update::update(&mut model, Msg::EditorInsertChar('a'));
    let content_after = model.editor_state.content.clone();

    // Undo
    ratatui_diary::update::update(&mut model, Msg::EditorUndo);

    // Redo
    ratatui_diary::update::update(&mut model, Msg::EditorRedo);
    assert_eq!(model.editor_state.content, content_after);
}

// ===== 에디터 검색 테스트 =====

#[test]
fn test_editor_search_mode() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;

    ratatui_diary::update::update(&mut model, Msg::EditorEnterSearchMode);
    assert_eq!(model.editor_state.submode, Some(EditorSubMode::Search));
}

#[test]
fn test_editor_search_execution() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["hello world".to_string(), "hello test".to_string()];
    model.editor_state.submode = Some(EditorSubMode::Search);
    model.editor_state.search_pattern = "hello".to_string();

    ratatui_diary::update::update(&mut model, Msg::EditorExecuteSearch);
    assert!(!model.editor_state.search_matches.is_empty());
}

// ===== 에디터 Space 명령 테스트 =====

#[test]
fn test_editor_space_mode() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;

    ratatui_diary::update::update(&mut model, Msg::EditorEnterSpaceMode);
    assert_eq!(model.editor_state.submode, Some(EditorSubMode::SpaceCommand));
}

#[test]
fn test_editor_space_save() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.submode = Some(EditorSubMode::SpaceCommand);
    model.editor_state.content = vec!["test".to_string()];

    let cmd = ratatui_diary::update::update(&mut model, Msg::EditorSpaceSave);
    assert!(cmd.is_some());
    assert_eq!(model.editor_state.submode, None);
}

#[test]
fn test_editor_space_quit() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.submode = Some(EditorSubMode::SpaceCommand);

    ratatui_diary::update::update(&mut model, Msg::EditorSpaceQuit);
    assert_eq!(model.screen, Screen::Calendar);
}

// ===== 파일 I/O 결과 테스트 =====

#[test]
fn test_load_diary_success() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;

    let date = model.editor_state.date;
    let content = "Test content\nLine 2".to_string();

    ratatui_diary::update::update(&mut model, Msg::LoadDiarySuccess(date, content));
    assert_eq!(model.editor_state.content.len(), 2);
    assert_eq!(model.editor_state.content[0], "Test content");
}

#[test]
fn test_save_diary_success() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.is_modified = true;

    ratatui_diary::update::update(&mut model, Msg::SaveDiarySuccess);
    assert!(!model.editor_state.is_modified);
}

#[test]
fn test_dismiss_error() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.show_error_popup = true;
    model.error_message = Some("Test error".to_string());

    ratatui_diary::update::update(&mut model, Msg::DismissError);
    assert!(!model.show_error_popup);
    assert!(model.error_message.is_none());
}

#[cfg(test)]
mod complete_message_coverage {
    use ratatui_diary::{Msg, Model, message::InsertPosition, model::{EditorMode, EditorSubMode, Screen}, storage::Storage};
    use std::collections::HashSet;
    use tempfile::TempDir;
    use chrono::NaiveDate;

    fn setup_model() -> (Model, TempDir) {
        let temp = TempDir::new().unwrap();
        let storage = Storage::with_dir(temp.path()).unwrap();
        let model = Model::new(HashSet::new(), storage);
        (model, temp)
    }

    // ===== Editor Insert Mode Tests =====

    #[test]
    fn test_editor_enter_insert_before_cursor() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.cursor_col = 5;

        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::BeforeCursor));

        assert_eq!(model.editor_state.mode, EditorMode::Insert);
        assert_eq!(model.editor_state.cursor_col, 5);
    }

    #[test]
    fn test_editor_enter_insert_after_cursor() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_col = 5;

        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::AfterCursor));

        assert_eq!(model.editor_state.mode, EditorMode::Insert);
        assert_eq!(model.editor_state.cursor_col, 5);
    }

    #[test]
    fn test_editor_enter_insert_line_below() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Line 1".to_string()];
        model.editor_state.cursor_line = 0;

        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::LineBelow));

        assert_eq!(model.editor_state.mode, EditorMode::Insert);
        assert_eq!(model.editor_state.cursor_line, 1);
        assert_eq!(model.editor_state.content.len(), 2);
    }

    #[test]
    fn test_editor_enter_insert_line_above() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Line 1".to_string()];
        model.editor_state.cursor_line = 0;

        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::LineAbove));

        assert_eq!(model.editor_state.mode, EditorMode::Insert);
        assert_eq!(model.editor_state.cursor_line, 0);
        assert_eq!(model.editor_state.content.len(), 2);
    }

    // ===== Editor Goto Mode Tests =====

    #[test]
    fn test_editor_goto_doc_end() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        model.editor_state.cursor_line = 0;
        model.editor_state.submode = Some(EditorSubMode::Goto);

        ratatui_diary::update::update(&mut model, Msg::EditorGotoDocEnd);

        assert_eq!(model.editor_state.cursor_line, 2);
        assert_eq!(model.editor_state.submode, None);
    }

    #[test]
    fn test_editor_goto_line_start() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Hello World".to_string()];
        model.editor_state.cursor_col = 5;
        model.editor_state.submode = Some(EditorSubMode::Goto);

        ratatui_diary::update::update(&mut model, Msg::EditorGotoLineStart);

        assert_eq!(model.editor_state.cursor_col, 0);
        assert_eq!(model.editor_state.submode, None);
    }

    // ===== Editor Edit Operations =====

    #[test]
    fn test_editor_delete_without_selection() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Hello World".to_string()];
        model.editor_state.cursor_line = 0;
        model.editor_state.cursor_col = 0;

        ratatui_diary::update::update(&mut model, Msg::EditorDelete);

        // 선택 없이 Delete 시 현재 줄이 선택되어야 함
        assert!(model.editor_state.clipboard.contains("Hello"));
    }

    #[test]
    fn test_editor_change_operation() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Hello".to_string()];

        ratatui_diary::update::update(&mut model, Msg::EditorChange);

        // Change는 Delete + Insert 모드
        assert_eq!(model.editor_state.mode, EditorMode::Insert);
    }

    #[test]
    fn test_editor_paste_before() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.clipboard = "World".to_string();
        model.editor_state.cursor_col = 5;

        ratatui_diary::update::update(&mut model, Msg::EditorPasteBefore);

        assert!(model.editor_state.content[0].contains("World"));
    }

    // ===== Editor Search Mode =====

    #[test]
    fn test_editor_search_char_input() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.submode = Some(EditorSubMode::Search);

        ratatui_diary::update::update(&mut model, Msg::EditorSearchChar('t'));

        assert_eq!(model.editor_state.search_pattern, "t");
    }

    #[test]
    fn test_editor_search_backspace() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.submode = Some(EditorSubMode::Search);
        model.editor_state.search_pattern = "test".to_string();

        ratatui_diary::update::update(&mut model, Msg::EditorSearchBackspace);

        assert_eq!(model.editor_state.search_pattern, "tes");
    }

    #[test]
    fn test_editor_search_prev() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["test test test".to_string()];
        model.editor_state.search_pattern = "test".to_string();
        model.editor_state.execute_search();

        ratatui_diary::update::update(&mut model, Msg::EditorSearchPrev);

        // 이전 매치로 이동
        assert!(model.editor_state.current_match_index < 3);
    }

    // ===== Calendar Space Mode =====

    #[test]
    fn test_calendar_space_next_year() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.calendar_state.submode = Some(ratatui_diary::model::CalendarSubMode::Space);
        let original_year = model.calendar_state.current_year;

        ratatui_diary::update::update(&mut model, Msg::CalendarSpaceNextYear);

        assert_eq!(model.calendar_state.current_year, original_year + 1);
        assert_eq!(model.calendar_state.submode, None);
    }

    #[test]
    fn test_calendar_space_prev_year() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.calendar_state.submode = Some(ratatui_diary::model::CalendarSubMode::Space);
        let original_year = model.calendar_state.current_year;

        ratatui_diary::update::update(&mut model, Msg::CalendarSpacePrevYear);

        assert_eq!(model.calendar_state.current_year, original_year - 1);
        assert_eq!(model.calendar_state.submode, None);
    }

    // ===== File I/O Result Messages =====

    #[test]
    fn test_load_diary_failed() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;

        ratatui_diary::update::update(&mut model, Msg::LoadDiaryFailed("File not found".to_string()));

        assert!(model.show_error_popup);
        assert!(model.error_message.unwrap().contains("File not found"));
    }

    #[test]
    fn test_save_diary_failed() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;

        ratatui_diary::update::update(&mut model, Msg::SaveDiaryFailed("Permission denied".to_string()));

        assert!(model.show_error_popup);
        assert!(model.error_message.unwrap().contains("Permission denied"));
    }

    #[test]
    fn test_delete_diary_success() {
        let (mut model, _temp) = setup_model();
        let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
        model.diary_entries.entries.insert(date);

        ratatui_diary::update::update(&mut model, Msg::DeleteDiarySuccess(date));

        assert!(!model.diary_entries.entries.contains(&date));
    }

    #[test]
    fn test_refresh_index() {
        let (mut model, _temp) = setup_model();
        let date1 = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2026, 2, 16).unwrap();
        let mut new_entries = HashSet::new();
        new_entries.insert(date1);
        new_entries.insert(date2);

        ratatui_diary::update::update(&mut model, Msg::RefreshIndex(new_entries));

        assert_eq!(model.diary_entries.entries.len(), 2);
    }

    // ===== Helper Function Tests (indirect) =====

    #[test]
    fn test_paste_clipboard_line_mode() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Line 1".to_string()];
        model.editor_state.clipboard = "New Line\n".to_string();
        model.editor_state.cursor_line = 0;

        ratatui_diary::update::update(&mut model, Msg::EditorPasteAfter);

        // 줄 단위 붙여넣기
        assert!(model.editor_state.content.len() >= 2);
    }

    #[test]
    fn test_paste_clipboard_char_mode() {
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.clipboard = "World".to_string(); // \n 없음
        model.editor_state.cursor_col = 5;

        ratatui_diary::update::update(&mut model, Msg::EditorPasteAfter);

        // 문자 단위 붙여넣기
        assert!(model.editor_state.content[0].contains("World"));
    }

    // ===== 누락된 Msg Variant 테스트 =====

    #[test]
    fn test_quit_message() {
        // Given: 초기화된 모델
        let (mut model, _temp) = setup_model();

        // When: Quit 메시지 처리
        let cmd = ratatui_diary::update::update(&mut model, Msg::Quit);

        // Then: 주 루프에서 처리되므로 반환값이 없음
        assert!(cmd.is_none());
    }

    #[test]
    fn test_calendar_move_up() {
        // Given: 달력 화면의 초기화된 모델
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        let original_date = model.calendar_state.selected_date;

        // When: 위로 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::CalendarMoveUp);

        // Then: 선택된 날짜가 7일 전으로 이동
        assert!(model.calendar_state.selected_date < original_date);
    }

    #[test]
    fn test_calendar_move_down() {
        // Given: 달력 화면의 초기화된 모델
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        let original_date = model.calendar_state.selected_date;

        // When: 아래로 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::CalendarMoveDown);

        // Then: 선택된 날짜가 7일 후로 이동
        assert!(model.calendar_state.selected_date > original_date);
    }

    #[test]
    fn test_calendar_space_quit() {
        // Given: Space 모드의 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.calendar_state.submode = Some(ratatui_diary::model::CalendarSubMode::Space);

        // When: Space Quit 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::CalendarSpaceQuit);

        // Then: 서브모드가 종료됨
        assert_eq!(model.calendar_state.submode, None);
    }

    #[test]
    fn test_editor_move_left() {
        // Given: 에디터 화면의 커서가 우측에 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_col = 3;

        // When: 왼쪽 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveLeft);

        // Then: 커서 열이 감소
        assert_eq!(model.editor_state.cursor_col, 2);
    }

    #[test]
    fn test_editor_move_up() {
        // Given: 에디터 화면의 커서가 2행에 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Line 1".to_string(), "Line 2".to_string()];
        model.editor_state.cursor_line = 1;

        // When: 위로 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveUp);

        // Then: 커서 행이 감소
        assert_eq!(model.editor_state.cursor_line, 0);
    }

    #[test]
    fn test_editor_word_prev() {
        // Given: 에디터 화면의 여러 단어가 있는 줄
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["hello world test".to_string()];
        model.editor_state.cursor_col = 12;

        // When: 이전 단어 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorWordPrev);

        // Then: 커서가 이전 단어로 이동
        assert!(model.editor_state.cursor_col < 12);
    }

    #[test]
    fn test_editor_word_end() {
        // Given: 에디터 화면의 여러 단어가 있는 줄
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["hello world test".to_string()];
        model.editor_state.cursor_col = 0;

        // When: 단어 끝 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorWordEnd);

        // Then: 커서가 현재 단어의 끝으로 이동
        assert!(model.editor_state.cursor_col > 0);
    }

    #[test]
    fn test_editor_enter_normal_mode() {
        // Given: Insert 모드의 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: Normal 모드 진입 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorEnterNormalMode);

        // Then: Normal 모드로 변경
        assert_eq!(model.editor_state.mode, EditorMode::Normal);
    }

    #[test]
    fn test_editor_backspace() {
        // Given: Insert 모드의 에디터 화면에 텍스트
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_line = 0;
        model.editor_state.cursor_col = 5;

        // When: Backspace 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorBackspace);

        // Then: 마지막 문자 삭제
        assert_eq!(model.editor_state.content[0], "Hell");
    }

    #[test]
    fn test_editor_new_line() {
        // Given: Insert 모드의 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_line = 0;
        model.editor_state.cursor_col = 5;

        // When: 새 줄 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorNewLine);

        // Then: 새로운 줄이 생성되고 커서가 이동
        assert_eq!(model.editor_state.content.len(), 2);
        assert_eq!(model.editor_state.cursor_line, 1);
    }

    #[test]
    fn test_editor_search_next() {
        // Given: 검색 패턴이 설정된 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["test test test".to_string()];
        model.editor_state.search_pattern = "test".to_string();
        model.editor_state.execute_search();

        // When: 다음 검색 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorSearchNext);

        // Then: 다음 매치로 이동됨 (또는 유효한 인덱스 범위 내)
        assert!(model.editor_state.current_match_index <= model.editor_state.search_matches.len());
    }

    #[test]
    fn test_editor_space_save_quit() {
        // Given: Space 모드의 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = Some(EditorSubMode::SpaceCommand);
        model.editor_state.is_modified = true;
        model.editor_state.content = vec!["test".to_string()];

        // When: Space Save Quit 메시지 처리
        let cmd = ratatui_diary::update::update(&mut model, Msg::EditorSpaceSaveQuit);

        // Then: SaveDiary 명령 반환 및 달력으로 복귀
        assert!(cmd.is_some());
        assert_eq!(model.screen, Screen::Calendar);
        assert_eq!(model.editor_state.submode, None);
    }

    #[test]
    fn test_editor_back() {
        // Given: 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;

        // When: Back 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorBack);

        // Then: 달력 화면으로 복귀
        assert_eq!(model.screen, Screen::Calendar);
    }

    // ===== 조건 거짓 케이스 테스트 (false branch coverage) =====

    #[test]
    fn test_editor_move_left_at_boundary() {
        // Given: 에디터 화면의 커서가 좌측 끝에 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.cursor_col = 0;

        // When: 왼쪽 이동 메시지 처리 (경계에서 더 이상 이동 불가)
        ratatui_diary::update::update(&mut model, Msg::EditorMoveLeft);

        // Then: 커서 위치 변경 없음
        assert_eq!(model.editor_state.cursor_col, 0);
    }

    #[test]
    fn test_editor_move_right_at_line_end() {
        // Given: 에디터 화면의 커서가 줄 끝에 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_col = 5;  // 줄 길이와 같음

        // When: 오른쪽 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveRight);

        // Then: 커서 위치 변경 없음
        assert_eq!(model.editor_state.cursor_col, 5);
    }

    #[test]
    fn test_editor_move_up_at_top() {
        // Given: 에디터 화면의 커서가 첫 번째 줄
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Line 1".to_string(), "Line 2".to_string()];
        model.editor_state.cursor_line = 0;

        // When: 위로 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveUp);

        // Then: 커서 줄 변경 없음
        assert_eq!(model.editor_state.cursor_line, 0);
    }

    #[test]
    fn test_editor_move_down_at_bottom() {
        // Given: 에디터 화면의 커서가 마지막 줄
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Line 1".to_string(), "Line 2".to_string()];
        model.editor_state.cursor_line = 1;

        // When: 아래로 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveDown);

        // Then: 커서 줄 변경 없음
        assert_eq!(model.editor_state.cursor_line, 1);
    }

    #[test]
    fn test_editor_move_when_insert_mode() {
        // Given: Insert 모드의 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_col = 2;

        // When: 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveLeft);

        // Then: Insert 모드에서는 이동 불가
        assert_eq!(model.editor_state.cursor_col, 2);
    }

    #[test]
    fn test_editor_move_when_calendar_screen() {
        // Given: 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.editor_state.cursor_col = 2;

        // When: 에디터 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveRight);

        // Then: 달력 화면에서는 에디터 이동 불가
        assert_eq!(model.editor_state.cursor_col, 2);
    }

    #[test]
    fn test_calendar_move_up_when_editor_screen() {
        // Given: 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        let original_date = model.calendar_state.selected_date;

        // When: 달력 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::CalendarMoveUp);

        // Then: 에디터 화면에서는 달력 이동 불가
        assert_eq!(model.calendar_state.selected_date, original_date);
    }

    #[test]
    fn test_calendar_move_down_when_editor_screen() {
        // Given: 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        let original_date = model.calendar_state.selected_date;

        // When: 달력 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::CalendarMoveDown);

        // Then: 에디터 화면에서는 달력 이동 불가
        assert_eq!(model.calendar_state.selected_date, original_date);
    }

    #[test]
    fn test_calendar_space_quit_when_not_in_space_mode() {
        // Given: Space 모드가 아닌 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.calendar_state.submode = None;

        // When: Space Quit 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::CalendarSpaceQuit);

        // Then: 상태 변화 없음
        assert_eq!(model.calendar_state.submode, None);
    }

    #[test]
    fn test_editor_word_next_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["hello world".to_string()];
        model.editor_state.cursor_col = 0;

        // When: 단어 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorWordNext);

        // Then: Insert 모드에서는 이동 불가
        assert_eq!(model.editor_state.cursor_col, 0);
    }

    #[test]
    fn test_editor_goto_doc_start_wrong_mode() {
        // Given: Goto 모드가 아닌 Normal 모드 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = None;
        model.editor_state.cursor_line = 2;
        model.editor_state.cursor_col = 3;

        // When: GotoDocStart 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorGotoDocStart);

        // Then: 조건 불만족으로 이동 불가
        assert_eq!(model.editor_state.cursor_line, 2);
        assert_eq!(model.editor_state.cursor_col, 3);
    }

    #[test]
    fn test_editor_enter_insert_before_at_beginning() {
        // Given: 커서가 줄 시작에 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_col = 0;

        // When: Insert 모드 진입 (BeforeCursor)
        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::BeforeCursor));

        // Then: 커서 위치 변경 없음 (이미 시작 위치)
        assert_eq!(model.editor_state.cursor_col, 0);
    }

    #[test]
    fn test_editor_enter_insert_after_at_line_end() {
        // Given: 커서가 줄 끝에 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.cursor_col = 5;

        // When: Insert 모드 진입 (AfterCursor)
        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::AfterCursor));

        // Then: 이미 끝에 있으므로 위치 변경 없음
        assert_eq!(model.editor_state.cursor_col, 5);
    }

    #[test]
    fn test_editor_space_save_quit_without_space_mode() {
        // Given: Space 모드가 아닌 Normal 모드
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = None;

        // When: SpaceSaveQuit 메시지 처리
        let cmd = ratatui_diary::update::update(&mut model, Msg::EditorSpaceSaveQuit);

        // Then: 조건 불만족으로 명령 반환 안 됨
        assert!(cmd.is_none());
        assert_eq!(model.screen, Screen::Editor);
    }

    #[test]
    fn test_editor_space_save_without_space_mode() {
        // Given: Space 모드가 아닌 Normal 모드
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = None;

        // When: SpaceSave 메시지 처리
        let cmd = ratatui_diary::update::update(&mut model, Msg::EditorSpaceSave);

        // Then: 조건 불만족으로 명령 반환 안 됨
        assert!(cmd.is_none());
    }

    #[test]
    fn test_editor_move_right_beyond_empty_line() {
        // Given: 빈 줄의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["".to_string()];
        model.editor_state.cursor_col = 0;

        // When: 오른쪽 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveRight);

        // Then: 커서 위치 변경 없음 (빈 줄이므로 길이 0)
        assert_eq!(model.editor_state.cursor_col, 0);
    }

    #[test]
    fn test_editor_goto_line_start_wrong_submode() {
        // Given: Search 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = Some(EditorSubMode::Search);
        model.editor_state.cursor_col = 5;

        // When: GotoLineStart 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorGotoLineStart);

        // Then: Goto 모드가 아니므로 실행 안 됨
        assert_eq!(model.editor_state.cursor_col, 5);
    }

    #[test]
    fn test_editor_insert_char_when_calendar_screen() {
        // Given: 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.editor_state.content = vec!["".to_string()];

        // When: 문자 삽입 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorInsertChar('a'));

        // Then: 달력 화면에서는 삽입 불가
        assert!(model.editor_state.content[0].is_empty());
    }

    #[test]
    fn test_editor_insert_char_when_normal_mode() {
        // Given: Normal 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["".to_string()];

        // When: 문자 삽입 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorInsertChar('a'));

        // Then: Normal 모드에서는 삽입 불가
        assert!(model.editor_state.content[0].is_empty());
    }

    #[test]
    fn test_editor_backspace_when_calendar_screen() {
        // Given: 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.editor_state.content = vec!["Hello".to_string()];

        // When: Backspace 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorBackspace);

        // Then: 달력 화면에서는 실행 안 됨
        assert_eq!(model.editor_state.content[0], "Hello");
    }

    #[test]
    fn test_editor_backspace_when_normal_mode() {
        // Given: Normal 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hello".to_string()];

        // When: Backspace 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorBackspace);

        // Then: Normal 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.content[0], "Hello");
    }

    #[test]
    fn test_editor_new_line_when_normal_mode() {
        // Given: Normal 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hello".to_string()];
        let original_len = model.editor_state.content.len();

        // When: NewLine 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorNewLine);

        // Then: Normal 모드에서는 줄 추가 안 됨
        assert_eq!(model.editor_state.content.len(), original_len);
    }

    #[test]
    fn test_editor_delete_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];

        // When: Delete 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorDelete);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.content[0], "Hello");
    }

    #[test]
    fn test_editor_change_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];

        // When: Change 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorChange);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.content[0], "Hello");
        assert_eq!(model.editor_state.mode, EditorMode::Insert);
    }

    #[test]
    fn test_editor_yank_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.clipboard = "".to_string();

        // When: Yank 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorYank);

        // Then: Insert 모드에서는 실행 안 됨
        assert!(model.editor_state.clipboard.is_empty());
    }

    #[test]
    fn test_editor_paste_after_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.clipboard = "World".to_string();

        // When: PasteAfter 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorPasteAfter);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.content[0], "Hello");
    }

    #[test]
    fn test_editor_paste_before_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.clipboard = "World".to_string();

        // When: PasteBefore 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorPasteBefore);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.content[0], "Hello");
    }

    #[test]
    fn test_editor_undo_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: Undo 메시지 처리
        let original_history_index = model.editor_state.history_index;
        ratatui_diary::update::update(&mut model, Msg::EditorUndo);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.history_index, original_history_index);
    }

    #[test]
    fn test_editor_redo_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: Redo 메시지 처리
        let original_history_index = model.editor_state.history_index;
        ratatui_diary::update::update(&mut model, Msg::EditorRedo);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.history_index, original_history_index);
    }

    #[test]
    fn test_editor_enter_search_mode_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: SearchMode 진입 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorEnterSearchMode);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.submode, None);
    }

    #[test]
    fn test_editor_search_char_wrong_mode() {
        // Given: Search 모드가 아닌 에디터 (또는 Insert 모드)
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = None;

        // When: SearchChar 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorSearchChar('a'));

        // Then: Search 모드가 아니므로 패턴 추가 안 됨
        assert!(model.editor_state.search_pattern.is_empty());
    }

    #[test]
    fn test_editor_search_backspace_wrong_mode() {
        // Given: Search 모드가 아닌 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = None;
        model.editor_state.search_pattern = "test".to_string();

        // When: SearchBackspace 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorSearchBackspace);

        // Then: Search 모드가 아니므로 패턴 변경 안 됨
        assert_eq!(model.editor_state.search_pattern, "test");
    }

    #[test]
    fn test_editor_execute_search_wrong_mode() {
        // Given: Search 모드가 아닌 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = None;

        // When: ExecuteSearch 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorExecuteSearch);

        // Then: Search 모드가 아니므로 실행 안 됨
        assert_eq!(model.editor_state.submode, None);
    }

    #[test]
    fn test_editor_search_next_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        let original_index = model.editor_state.current_match_index;

        // When: SearchNext 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorSearchNext);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.current_match_index, original_index);
    }

    #[test]
    fn test_editor_search_prev_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        let original_index = model.editor_state.current_match_index;

        // When: SearchPrev 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorSearchPrev);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.current_match_index, original_index);
    }

    #[test]
    fn test_editor_enter_space_mode_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: EnterSpaceMode 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorEnterSpaceMode);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.submode, None);
    }

    #[test]
    fn test_editor_enter_space_mode_when_insert_mode_2() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: EnterSpaceMode 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorEnterSpaceMode);

        // Then: Insert 모드에서는 실행 안 됨
        assert_eq!(model.editor_state.submode, None);
    }

    #[test]
    fn test_editor_space_quit_when_not_in_space_mode() {
        // Given: Space 모드가 아닌 Normal 모드
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.submode = None;

        // When: SpaceQuit 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorSpaceQuit);

        // Then: Space 모드가 아니므로 실행 안 됨
        assert_eq!(model.screen, Screen::Editor);
    }

    #[test]
    fn test_editor_goto_empty_content() {
        // Given: 빈 콘텐츠의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec![];
        model.editor_state.submode = Some(EditorSubMode::Goto);
        model.editor_state.cursor_line = 0;

        // When: GotoDocEnd 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorGotoDocEnd);

        // Then: 빈 콘텐츠이므로 줄 변경 안 됨
        assert_eq!(model.editor_state.cursor_line, 0);
        assert_eq!(model.editor_state.submode, None);
    }

    #[test]
    fn test_calendar_select_date_when_insert_mode() {
        // Given: Insert 모드의 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: CalendarSelectDate 메시지 처리
        let cmd = ratatui_diary::update::update(&mut model, Msg::CalendarSelectDate);

        // Then: 에디터 화면이 아니므로 실행 안 됨
        assert!(cmd.is_none());
        assert_eq!(model.screen, Screen::Editor);
    }

    #[test]
    fn test_calendar_enter_space_mode_when_editor() {
        // Given: 에디터 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;

        // When: CalendarEnterSpaceMode 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::CalendarEnterSpaceMode);

        // Then: 에디터 화면이므로 실행 안 됨
        assert_eq!(model.calendar_state.submode, None);
    }

    #[test]
    fn test_editor_move_right_at_end_of_line() {
        // Given: 줄의 정확히 마지막 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Test".to_string()];
        model.editor_state.cursor_col = 4;

        // When: 오른쪽 이동 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorMoveRight);

        // Then: 커서 위치 변경 없음
        assert_eq!(model.editor_state.cursor_col, 4);
    }

    #[test]
    fn test_editor_enter_insert_line_below_at_last_line() {
        // Given: 에디터가 마지막 줄에 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Line 1".to_string()];
        model.editor_state.cursor_line = 1;  // 마지막 줄 다음

        // When: LineBelow 위치에서 Insert 모드 진입
        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::LineBelow));

        // Then: 줄 길이가 1로 유지 (조건 불만족으로 실행 안 됨)
        assert_eq!(model.editor_state.content.len(), 1);
    }

    #[test]
    fn test_editor_enter_normal_mode_from_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.content = vec!["test".to_string()];
        let original_history_len = model.editor_state.edit_history.len();

        // When: Normal 모드 진입 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorEnterNormalMode);

        // Then: 모드 변경 및 스냅샷 저장
        assert_eq!(model.editor_state.mode, EditorMode::Normal);
        assert!(model.editor_state.edit_history.len() > original_history_len);
    }

    #[test]
    fn test_editor_enter_normal_mode_from_normal_mode() {
        // Given: 이미 Normal 모드인 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        let original_history_len = model.editor_state.edit_history.len();

        // When: Normal 모드 진입 메시지 처리 (중복)
        ratatui_diary::update::update(&mut model, Msg::EditorEnterNormalMode);

        // Then: 이미 Normal 모드이므로 스냅샷 저장 안 됨
        assert_eq!(model.editor_state.mode, EditorMode::Normal);
        assert_eq!(model.editor_state.edit_history.len(), original_history_len);
    }

    #[test]
    fn test_editor_enter_insert_insert_mode_when_already_insert() {
        // Given: 이미 Insert 모드인 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;

        // When: Insert 모드 진입 (조건 확인: mode == Normal이어야 함)
        ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::BeforeCursor));

        // Then: 조건 불만족으로 실행 안 됨
        assert_eq!(model.editor_state.mode, EditorMode::Insert);
    }

    #[test]
    fn test_editor_goto_doc_start_wrong_screen() {
        // Given: 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;
        model.editor_state.cursor_line = 2;
        model.editor_state.cursor_col = 3;

        // When: GotoDocStart 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorGotoDocStart);

        // Then: 에디터 화면이 아니므로 실행 안 됨
        assert_eq!(model.editor_state.cursor_line, 2);
        assert_eq!(model.editor_state.cursor_col, 3);
    }

    #[test]
    fn test_editor_goto_doc_start_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.submode = Some(EditorSubMode::Goto);
        model.editor_state.cursor_line = 2;
        model.editor_state.cursor_col = 3;

        // When: GotoDocStart 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorGotoDocStart);

        // Then: Normal 모드가 아니므로 실행 안 됨
        assert_eq!(model.editor_state.cursor_line, 2);
        assert_eq!(model.editor_state.cursor_col, 3);
    }

    #[test]
    fn test_editor_goto_line_end_when_insert_mode() {
        // Given: Insert 모드의 에디터
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Insert;
        model.editor_state.submode = Some(EditorSubMode::Goto);
        model.editor_state.cursor_col = 0;

        // When: GotoLineEnd 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorGotoLineEnd);

        // Then: Normal 모드가 아니므로 실행 안 됨
        assert_eq!(model.editor_state.cursor_col, 0);
    }

    #[test]
    fn test_editor_exit_submode_when_calendar() {
        // Given: 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;

        // When: ExitSubMode 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorExitSubMode);

        // Then: 에디터 화면이 아니므로 실행 안 됨
        // (calendar_state의 submode는 영향 받지 않음)
    }

    #[test]
    fn test_calendar_select_date_when_calendar() {
        // Given: 달력 화면
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Calendar;

        // When: CalendarSelectDate 메시지 처리
        let cmd = ratatui_diary::update::update(&mut model, Msg::CalendarSelectDate);

        // Then: 에디터 화면으로 전환 및 명령 반환
        assert_eq!(model.screen, Screen::Editor);
        assert!(cmd.is_some());
    }

    #[test]
    fn test_editor_paste_after_empty_clipboard() {
        // Given: 빈 클립보드
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hello".to_string()];
        model.editor_state.clipboard = "".to_string();

        // When: PasteAfter 메시지 처리
        ratatui_diary::update::update(&mut model, Msg::EditorPasteAfter);

        // Then: 클립보드가 비어있으므로 내용 변경 안 됨
        assert_eq!(model.editor_state.content[0], "Hello");
    }

    #[test]
    fn test_editor_move_right_on_cursor_at_line_boundary() {
        // Given: 다양한 줄 길이의 콘텐츠
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["Hi".to_string(), "Hello".to_string()];
        model.editor_state.cursor_line = 0;
        model.editor_state.cursor_col = 1;

        // When: 오른쪽 이동 메시지 처리 (줄 끝 도달)
        ratatui_diary::update::update(&mut model, Msg::EditorMoveRight);

        // Then: 다음 글자로 이동
        assert_eq!(model.editor_state.cursor_col, 2);
    }

    #[test]
    fn test_editor_move_right_at_max_position() {
        // Given: 줄의 끝 위치에서 커서 위치
        let (mut model, _temp) = setup_model();
        model.screen = Screen::Editor;
        model.editor_state.mode = EditorMode::Normal;
        model.editor_state.content = vec!["A".to_string()];
        model.editor_state.cursor_col = 1;

        // When: 오른쪽 이동 메시지 처리 (이미 끝)
        ratatui_diary::update::update(&mut model, Msg::EditorMoveRight);

        // Then: 더 이상 이동 불가
        assert_eq!(model.editor_state.cursor_col, 1);
    }
}
