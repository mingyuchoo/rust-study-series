use ratatui_diary::{Model,
                    Msg,
                    model::{EditorMode,
                            Screen},
                    storage::Storage,
                    update::Command};
use std::collections::HashSet;
use tempfile::TempDir;

#[test]
fn test_calendar_next_month() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    let original_month = model.calendar_state.current_month;

    ratatui_diary::update::update(&mut model, Msg::CalendarNextMonth);

    let expected = if original_month == 12 { 1 } else { original_month + 1 };
    assert_eq!(model.calendar_state.current_month, expected);
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
fn test_editor_insert_mode() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Normal;

    ratatui_diary::update::update(&mut model, Msg::EditorEnterInsertMode);

    assert_eq!(model.editor_state.mode, EditorMode::Insert);
}

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

// TODO: Helix 스타일로 재작성 예정
// #[test]
// fn test_editor_command_w_saves() {
//     let temp = TempDir::new().unwrap();
//     let storage = Storage::with_dir(temp.path()).unwrap();
//     let mut model = Model::new(HashSet::new(), storage);
//     model.screen = Screen::Editor;
//     model.editor_state.mode = EditorMode::Command("w".to_string());
//     model.editor_state.content = vec!["test".to_string()];

//     let cmd = ratatui_diary::update::update(&mut model, Msg::EditorExecuteCommand);

//     assert!(matches!(cmd, Some(Command::SaveDiary(_, _))));
// }
