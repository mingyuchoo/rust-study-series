use ratatui_diary::{Model, Msg};
use ratatui_diary::model::{Screen, EditorMode};
use ratatui_diary::update::Command;
use std::collections::HashSet;

#[test]
fn test_calendar_next_month() {
    let mut model = Model::new(HashSet::new());
    let original_month = model.calendar_state.current_month;

    ratatui_diary::update::update(&mut model, Msg::CalendarNextMonth);

    let expected = if original_month == 12 { 1 } else { original_month + 1 };
    assert_eq!(model.calendar_state.current_month, expected);
}

#[test]
fn test_calendar_select_date_switches_to_editor() {
    let mut model = Model::new(HashSet::new());

    let cmd = ratatui_diary::update::update(&mut model, Msg::CalendarSelectDate);

    assert_eq!(model.screen, Screen::Editor);
    assert!(cmd.is_some());
}

#[test]
fn test_editor_insert_mode() {
    let mut model = Model::new(HashSet::new());
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Normal;

    ratatui_diary::update::update(&mut model, Msg::EditorEnterInsertMode);

    assert_eq!(model.editor_state.mode, EditorMode::Insert);
}

#[test]
fn test_editor_insert_char() {
    let mut model = Model::new(HashSet::new());
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Insert;

    ratatui_diary::update::update(&mut model, Msg::EditorInsertChar('a'));

    assert_eq!(model.editor_state.content[0], "a");
}

#[test]
fn test_editor_command_w_saves() {
    let mut model = Model::new(HashSet::new());
    model.screen = Screen::Editor;
    model.editor_state.mode = EditorMode::Command("w".to_string());
    model.editor_state.content = vec!["test".to_string()];

    let cmd = ratatui_diary::update::update(&mut model, Msg::EditorExecuteCommand);

    assert!(matches!(cmd, Some(Command::SaveDiary(_, _))));
}
