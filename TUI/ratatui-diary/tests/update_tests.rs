use ratatui_diary::{Model,
                    Msg,
                    message::InsertPosition,
                    model::{EditorMode,
                            EditorSubMode,
                            CalendarSubMode,
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
