use chrono::NaiveDate;
use ratatui_diary::{model::{CalendarState,
                            EditorMode,
                            EditorState,
                            Model,
                            Screen},
                    storage::Storage};
use std::collections::HashSet;
use tempfile::TempDir;

#[test]
fn test_next_month() {
    let mut state = CalendarState::new(2026, 2);
    state.next_month();
    assert_eq!(state.current_month, 3);
    assert_eq!(state.current_year, 2026);
}

#[test]
fn test_next_month_year_rollover() {
    let mut state = CalendarState::new(2026, 12);
    state.next_month();
    assert_eq!(state.current_month, 1);
    assert_eq!(state.current_year, 2027);
}

#[test]
fn test_prev_month() {
    let mut state = CalendarState::new(2026, 2);
    state.prev_month();
    assert_eq!(state.current_month, 1);
    assert_eq!(state.current_year, 2026);
}

#[test]
fn test_prev_month_year_rollover() {
    let mut state = CalendarState::new(2026, 1);
    state.prev_month();
    assert_eq!(state.current_month, 12);
    assert_eq!(state.current_year, 2025);
}

#[test]
fn test_insert_char() {
    let mut state = EditorState::new(NaiveDate::from_ymd_opt(2026, 2, 14).unwrap());
    state.mode = EditorMode::Insert;

    state.insert_char('a');
    assert_eq!(state.content[0], "a");
    assert_eq!(state.cursor_col, 1);
    assert!(state.is_modified);
}

#[test]
fn test_new_line() {
    let mut state = EditorState::new(NaiveDate::from_ymd_opt(2026, 2, 14).unwrap());
    state.insert_char('a');
    state.new_line();

    assert_eq!(state.content.len(), 2);
    assert_eq!(state.cursor_line, 1);
    assert_eq!(state.cursor_col, 0);
}

#[test]
fn test_load_content() {
    let mut state = EditorState::new(NaiveDate::from_ymd_opt(2026, 2, 14).unwrap());
    let content = "Line 1\nLine 2\nLine 3";

    state.load_content(content);

    assert_eq!(state.content.len(), 3);
    assert_eq!(state.content[0], "Line 1");
    assert_eq!(state.content[1], "Line 2");
    assert!(!state.is_modified);
}

#[test]
fn test_model_with_storage() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = HashSet::new();

    let model = Model::new(entries, storage);
    assert_eq!(model.screen, Screen::Calendar);
}

#[cfg(test)]
mod selection_tests {
    use super::*;
    use ratatui_diary::model::Selection;

    #[test]
    fn test_selection_creation() {
        let selection = Selection {
            anchor_line: 0,
            anchor_col: 0,
            cursor_line: 0,
            cursor_col: 5,
        };
        assert_eq!(selection.anchor_line, 0);
        assert_eq!(selection.cursor_col, 5);
    }

    #[test]
    fn test_editor_state_has_selection_field() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let state = EditorState::new(date);
        assert!(state.selection.is_none());
    }

    #[test]
    fn test_get_selection_range_forward() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut state = EditorState::new(date);
        state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 2,
            cursor_line: 0,
            cursor_col: 5,
        });

        let range = state.get_selection_range();
        assert_eq!(range, Some(((0, 2), (0, 5))));
    }

    #[test]
    fn test_get_selection_range_backward() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut state = EditorState::new(date);
        state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 5,
            cursor_line: 0,
            cursor_col: 2,
        });

        let range = state.get_selection_range();
        assert_eq!(range, Some(((0, 2), (0, 5))));
    }

    #[test]
    fn test_get_selected_text_single_line() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut state = EditorState::new(date);
        state.content = vec!["Hello World".to_string()];
        state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 0,
            cursor_line: 0,
            cursor_col: 5,
        });

        let text = state.get_selected_text();
        assert_eq!(text, Some("Hello".to_string()));
    }

    #[test]
    fn test_get_selected_text_multi_line() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut state = EditorState::new(date);
        state.content = vec![
            "First line".to_string(),
            "Second line".to_string(),
            "Third line".to_string(),
        ];
        state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 6,
            cursor_line: 2,
            cursor_col: 5,
        });

        let text = state.get_selected_text();
        assert_eq!(text, Some("line\nSecond line\nThird".to_string()));
    }

    #[test]
    fn test_delete_selection_single_line() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut state = EditorState::new(date);
        state.content = vec!["Hello World".to_string()];
        state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 0,
            cursor_line: 0,
            cursor_col: 5,
        });

        state.delete_selection();

        assert_eq!(state.content[0], " World");
        assert_eq!(state.cursor_line, 0);
        assert_eq!(state.cursor_col, 0);
    }

    #[test]
    fn test_delete_selection_multi_line() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut state = EditorState::new(date);
        state.content = vec![
            "First line".to_string(),
            "Second line".to_string(),
            "Third line".to_string(),
        ];
        state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 6,
            cursor_line: 2,
            cursor_col: 5,
        });

        state.delete_selection();

        assert_eq!(state.content.len(), 1);
        assert_eq!(state.content[0], "First  line");
        assert_eq!(state.cursor_line, 0);
        assert_eq!(state.cursor_col, 6);
    }
}

#[cfg(test)]
mod update_selection_tests {
    use super::*;
    use ratatui_diary::{model::Selection, update, Msg};

    #[test]
    fn test_selection_toggle_on() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut model = Model::new(HashSet::new(), Storage::new().unwrap());
        model.screen = Screen::Editor;
        model.editor_state = EditorState::new(date);
        model.editor_state.cursor_line = 0;
        model.editor_state.cursor_col = 5;

        update::update(&mut model, Msg::EditorToggleSelection);

        assert!(model.editor_state.selection.is_some());
        let sel = model.editor_state.selection.unwrap();
        assert_eq!(sel.anchor_line, 0);
        assert_eq!(sel.anchor_col, 5);
    }

    #[test]
    fn test_selection_toggle_off() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut model = Model::new(HashSet::new(), Storage::new().unwrap());
        model.screen = Screen::Editor;
        model.editor_state = EditorState::new(date);
        model.editor_state.selection = Some(Selection {
            anchor_line: 0,
            anchor_col: 0,
            cursor_line: 0,
            cursor_col: 5,
        });

        update::update(&mut model, Msg::EditorToggleSelection);

        assert!(model.editor_state.selection.is_none());
    }

    #[test]
    fn test_select_line() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
        let mut model = Model::new(HashSet::new(), Storage::new().unwrap());
        model.screen = Screen::Editor;
        model.editor_state = EditorState::new(date);
        model.editor_state.content = vec!["Hello World".to_string()];
        model.editor_state.cursor_line = 0;
        model.editor_state.cursor_col = 5;

        update::update(&mut model, Msg::EditorSelectLine);

        assert!(model.editor_state.selection.is_some());
        let sel = model.editor_state.selection.unwrap();
        assert_eq!(sel.anchor_line, 0);
        assert_eq!(sel.anchor_col, 0);
        assert_eq!(sel.cursor_line, 0);
        assert_eq!(sel.cursor_col, 11);
    }
}
