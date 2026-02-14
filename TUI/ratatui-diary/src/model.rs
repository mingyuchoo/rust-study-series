use chrono::{Datelike, NaiveDate};
use std::collections::HashSet;

pub struct Model {
    pub screen: Screen,
    pub calendar_state: CalendarState,
    pub editor_state: EditorState,
    pub diary_entries: DiaryIndex,
    pub error_message: Option<String>,
    pub show_error_popup: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Calendar,
    Editor,
}

pub struct CalendarState {
    pub current_year: i32,
    pub current_month: u32,
    pub selected_date: NaiveDate,
    pub cursor_pos: usize,
}

pub struct EditorState {
    pub mode: EditorMode,
    pub date: NaiveDate,
    pub content: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_modified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command(String),
}

pub struct DiaryIndex {
    pub entries: HashSet<NaiveDate>,
}

impl Model {
    pub fn new(entries: HashSet<NaiveDate>) -> Self {
        let today = chrono::Local::now().date_naive();

        Self {
            screen: Screen::Calendar,
            calendar_state: CalendarState::new(today.year(), today.month()),
            editor_state: EditorState::new(today),
            diary_entries: DiaryIndex { entries },
            error_message: None,
            show_error_popup: false,
        }
    }
}

impl CalendarState {
    pub fn new(year: i32, month: u32) -> Self {
        let selected_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        Self {
            current_year: year,
            current_month: month,
            selected_date,
            cursor_pos: 0,
        }
    }
}

impl EditorState {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            mode: EditorMode::Normal,
            date,
            content: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            is_modified: false,
        }
    }
}
