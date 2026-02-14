use ratatui_diary::{Model, Msg};
use ratatui_diary::model::Screen;
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
