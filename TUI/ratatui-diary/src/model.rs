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

    pub fn next_month(&mut self) {
        if self.current_month == 12 {
            self.current_month = 1;
            self.current_year += 1;
        } else {
            self.current_month += 1;
        }
        self.adjust_selected_date();
    }

    pub fn prev_month(&mut self) {
        if self.current_month == 1 {
            self.current_month = 12;
            self.current_year -= 1;
        } else {
            self.current_month -= 1;
        }
        self.adjust_selected_date();
    }

    pub fn next_year(&mut self) {
        self.current_year += 1;
        self.adjust_selected_date();
    }

    pub fn prev_year(&mut self) {
        self.current_year -= 1;
        self.adjust_selected_date();
    }

    fn adjust_selected_date(&mut self) {
        // 선택된 날짜가 새 월에 유효한지 확인
        let day = self.selected_date.day();
        self.selected_date = NaiveDate::from_ymd_opt(
            self.current_year,
            self.current_month,
            day.min(days_in_month(self.current_year, self.current_month))
        ).unwrap();
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

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_line >= self.content.len() {
            self.content.push(String::new());
        }

        self.content[self.cursor_line].insert(self.cursor_col, c);
        self.cursor_col += 1;
        self.is_modified = true;
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.content[self.cursor_line].remove(self.cursor_col - 1);
            self.cursor_col -= 1;
            self.is_modified = true;
        } else if self.cursor_line > 0 {
            let current_line = self.content.remove(self.cursor_line);
            self.cursor_line -= 1;
            self.cursor_col = self.content[self.cursor_line].len();
            self.content[self.cursor_line].push_str(&current_line);
            self.is_modified = true;
        }
    }

    pub fn new_line(&mut self) {
        let current_line = &self.content[self.cursor_line];
        let remaining = current_line[self.cursor_col..].to_string();
        self.content[self.cursor_line].truncate(self.cursor_col);

        self.cursor_line += 1;
        self.content.insert(self.cursor_line, remaining);
        self.cursor_col = 0;
        self.is_modified = true;
    }

    pub fn load_content(&mut self, content: &str) {
        self.content = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(String::from).collect()
        };
        self.cursor_line = 0;
        self.cursor_col = 0;
        self.is_modified = false;
    }

    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}
