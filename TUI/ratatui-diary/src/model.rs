use crate::storage::Storage;
use chrono::{Datelike,
             NaiveDate};
use std::collections::HashSet;

pub struct Model {
    pub screen: Screen,
    pub calendar_state: CalendarState,
    pub editor_state: EditorState,
    pub diary_entries: DiaryIndex,
    pub error_message: Option<String>,
    pub show_error_popup: bool,
    pub storage: Storage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Calendar,
    Editor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub anchor_line: usize,
    pub anchor_col: usize,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

pub struct EditorSnapshot {
    pub content: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub selection: Option<Selection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorSubMode {
    Goto,
    SpaceCommand,
    Search,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarSubMode {
    Space,
}

pub struct CalendarState {
    pub current_year: i32,
    pub current_month: u32,
    pub selected_date: NaiveDate,
    pub cursor_pos: usize,
    pub submode: Option<CalendarSubMode>,
}

pub struct EditorState {
    pub mode: EditorMode,
    pub date: NaiveDate,
    pub content: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_modified: bool,
    pub selection: Option<Selection>,
    pub edit_history: Vec<EditorSnapshot>,
    pub history_index: usize,
    pub clipboard: String,
    pub submode: Option<EditorSubMode>,
    pub search_pattern: String,
    pub search_matches: Vec<(usize, usize)>,
    pub current_match_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
}

pub struct DiaryIndex {
    pub entries: HashSet<NaiveDate>,
}

impl Model {
    pub fn new(entries: HashSet<NaiveDate>, storage: Storage) -> Self {
        let today = chrono::Local::now().date_naive();

        Self {
            screen: Screen::Calendar,
            calendar_state: CalendarState::new(today.year(), today.month()),
            editor_state: EditorState::new(today),
            diary_entries: DiaryIndex {
                entries,
            },
            error_message: None,
            show_error_popup: false,
            storage,
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
            submode: None,
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

    pub fn move_cursor_left(&mut self) { self.selected_date = self.selected_date.pred_opt().unwrap_or(self.selected_date); }

    pub fn move_cursor_right(&mut self) { self.selected_date = self.selected_date.succ_opt().unwrap_or(self.selected_date); }

    pub fn move_cursor_up(&mut self) { self.selected_date = self.selected_date.checked_sub_days(chrono::Days::new(7)).unwrap_or(self.selected_date); }

    pub fn move_cursor_down(&mut self) { self.selected_date = self.selected_date.checked_add_days(chrono::Days::new(7)).unwrap_or(self.selected_date); }

    fn adjust_selected_date(&mut self) {
        // 선택된 날짜가 새 월에 유효한지 확인
        let day = self.selected_date.day();
        self.selected_date = NaiveDate::from_ymd_opt(
            self.current_year,
            self.current_month,
            day.min(days_in_month(self.current_year, self.current_month)),
        )
        .unwrap();
    }
}

impl EditorState {
    pub fn new(date: NaiveDate) -> Self {
        let mut state = Self {
            mode: EditorMode::Normal,
            date,
            content: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            is_modified: false,
            selection: None,
            edit_history: Vec::new(),
            history_index: 0,
            clipboard: String::new(),
            submode: None,
            search_pattern: String::new(),
            search_matches: Vec::new(),
            current_match_index: 0,
        };

        // 초기 스냅샷 저장
        state.save_snapshot();
        state
    }

    fn save_snapshot(&mut self) {
        let snapshot = EditorSnapshot {
            content: self.content.clone(),
            cursor_line: self.cursor_line,
            cursor_col: self.cursor_col,
            selection: self.selection.clone(),
        };
        self.edit_history.push(snapshot);
        self.history_index = self.edit_history.len() - 1;
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
        let remaining = current_line[self.cursor_col ..].to_string();
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

    pub fn get_content(&self) -> String { self.content.join("\n") }

    pub fn get_selection_range(&self) -> Option<((usize, usize), (usize, usize))> {
        self.selection.as_ref().map(|sel| {
            let start = if sel.anchor_line < sel.cursor_line
                || (sel.anchor_line == sel.cursor_line && sel.anchor_col < sel.cursor_col)
            {
                (sel.anchor_line, sel.anchor_col)
            } else {
                (sel.cursor_line, sel.cursor_col)
            };

            let end = if sel.anchor_line > sel.cursor_line
                || (sel.anchor_line == sel.cursor_line && sel.anchor_col > sel.cursor_col)
            {
                (sel.anchor_line, sel.anchor_col)
            } else {
                (sel.cursor_line, sel.cursor_col)
            };

            (start, end)
        })
    }

    pub fn get_selected_text(&self) -> Option<String> {
        let ((start_line, start_col), (end_line, end_col)) = self.get_selection_range()?;

        // 범위 검증 추가
        if end_line >= self.content.len() {
            return None;
        }

        if start_line == end_line {
            let line = &self.content[start_line];
            let safe_end = end_col.min(line.len());
            let safe_start = start_col.min(safe_end);
            Some(line[safe_start..safe_end].to_string())
        } else {
            let mut result = String::new();

            // 시작 줄
            let start_line_content = &self.content[start_line];
            let safe_start_col = start_col.min(start_line_content.len());
            result.push_str(&start_line_content[safe_start_col..]);
            result.push('\n');

            // 중간 줄들
            for line in (start_line + 1)..end_line {
                if line < self.content.len() {
                    result.push_str(&self.content[line]);
                    result.push('\n');
                }
            }

            // 끝 줄
            let end_line_content = &self.content[end_line];
            let safe_end_col = end_col.min(end_line_content.len());
            result.push_str(&end_line_content[..safe_end_col]);

            Some(result)
        }
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}
