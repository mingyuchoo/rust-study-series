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
    /// 문자 인덱스를 바이트 인덱스로 변환
    ///
    /// Rust의 String은 UTF-8로 인코딩되어 있으며, 한글과 같은 멀티바이트 문자는
    /// 여러 바이트를 차지합니다. String의 insert(), remove(), 슬라이싱 등의
    /// 연산은 바이트 인덱스를 요구하므로, 문자 개수 기반의 cursor_col을
    /// 바이트 인덱스로 변환해야 합니다.
    fn char_idx_to_byte_idx(&self, line: usize, char_idx: usize) -> usize {
        if line >= self.content.len() {
            return 0;
        }

        self.content[line]
            .char_indices()
            .nth(char_idx)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap_or_else(|| self.content[line].len())
    }

    /// 바이트 인덱스를 문자 인덱스로 변환
    ///
    /// String 연산 후 바이트 위치를 문자 개수로 변환합니다.
    fn byte_idx_to_char_idx(&self, line: usize, byte_idx: usize) -> usize {
        if line >= self.content.len() {
            return 0;
        }

        self.content[line][.. byte_idx.min(self.content[line].len())].chars().count()
    }

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

    pub fn save_snapshot(&mut self) {
        // 현재 index 이후의 히스토리 제거 (분기된 히스토리 삭제)
        self.edit_history.truncate(self.history_index + 1);

        // 현재 상태 저장
        let snapshot = EditorSnapshot {
            content: self.content.clone(),
            cursor_line: self.cursor_line,
            cursor_col: self.cursor_col,
            selection: self.selection.clone(),
        };

        self.edit_history.push(snapshot);
        self.history_index = self.edit_history.len() - 1;

        // 히스토리 크기 제한 (메모리 관리)
        const MAX_HISTORY: usize = 100;
        if self.edit_history.len() > MAX_HISTORY {
            self.edit_history.drain(0 .. 1);
            self.history_index -= 1;
        }
    }

    fn restore_snapshot(&mut self, index: usize) {
        if let Some(snapshot) = self.edit_history.get(index) {
            self.content = snapshot.content.clone();
            self.cursor_line = snapshot.cursor_line;
            self.cursor_col = snapshot.cursor_col;
            self.selection = snapshot.selection.clone();
            self.history_index = index;
            self.is_modified = true;
        }
    }

    pub fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.restore_snapshot(self.history_index);
        }
    }

    pub fn redo(&mut self) {
        if self.history_index + 1 < self.edit_history.len() {
            self.history_index += 1;
            self.restore_snapshot(self.history_index);
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_line >= self.content.len() {
            self.content.push(String::new());
        }

        let byte_idx = self.char_idx_to_byte_idx(self.cursor_line, self.cursor_col);
        self.content[self.cursor_line].insert(byte_idx, c);
        self.cursor_col += 1;
        self.is_modified = true;
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let byte_idx = self.char_idx_to_byte_idx(self.cursor_line, self.cursor_col - 1);
            self.content[self.cursor_line].remove(byte_idx);
            self.cursor_col -= 1;
            self.is_modified = true;
        } else if self.cursor_line > 0 {
            let current_line = self.content.remove(self.cursor_line);
            self.cursor_line -= 1;
            // len()은 바이트 길이이므로 문자 개수로 변환
            let line_len = self.content[self.cursor_line].chars().count();
            self.cursor_col = line_len;
            self.content[self.cursor_line].push_str(&current_line);
            self.is_modified = true;
        }
    }

    pub fn new_line(&mut self) {
        let byte_idx = self.char_idx_to_byte_idx(self.cursor_line, self.cursor_col);
        let current_line = &self.content[self.cursor_line];
        let remaining = current_line[byte_idx ..].to_string();
        self.content[self.cursor_line].truncate(byte_idx);

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

        // 로드 후 히스토리 초기화
        self.edit_history.clear();
        self.save_snapshot();
    }

    pub fn get_content(&self) -> String { self.content.join("\n") }

    pub fn get_selection_range(&self) -> Option<((usize, usize), (usize, usize))> {
        self.selection.as_ref().map(|sel| {
            let start = if sel.anchor_line < sel.cursor_line || (sel.anchor_line == sel.cursor_line && sel.anchor_col < sel.cursor_col) {
                (sel.anchor_line, sel.anchor_col)
            } else {
                (sel.cursor_line, sel.cursor_col)
            };

            let end = if sel.anchor_line > sel.cursor_line || (sel.anchor_line == sel.cursor_line && sel.anchor_col > sel.cursor_col) {
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
            let line_char_len = line.chars().count();
            let safe_end = end_col.min(line_char_len);
            let safe_start = start_col.min(safe_end);

            // 문자 인덱스를 바이트 인덱스로 변환
            let start_byte = self.char_idx_to_byte_idx(start_line, safe_start);
            let end_byte = self.char_idx_to_byte_idx(start_line, safe_end);
            Some(line[start_byte .. end_byte].to_string())
        } else {
            let mut result = String::new();

            // 시작 줄
            let start_line_content = &self.content[start_line];
            let start_line_char_len = start_line_content.chars().count();
            let safe_start_col = start_col.min(start_line_char_len);
            let start_byte = self.char_idx_to_byte_idx(start_line, safe_start_col);
            result.push_str(&start_line_content[start_byte ..]);
            result.push('\n');

            // 중간 줄들
            for line in (start_line + 1) .. end_line {
                if line < self.content.len() {
                    result.push_str(&self.content[line]);
                    result.push('\n');
                }
            }

            // 끝 줄
            let end_line_content = &self.content[end_line];
            let end_line_char_len = end_line_content.chars().count();
            let safe_end_col = end_col.min(end_line_char_len);
            let end_byte = self.char_idx_to_byte_idx(end_line, safe_end_col);
            result.push_str(&end_line_content[.. end_byte]);

            Some(result)
        }
    }

    pub fn delete_selection(&mut self) {
        let ((start_line, start_col), (end_line, end_col)) = match self.get_selection_range() {
            | Some(range) => range,
            | None => return,
        };

        if start_line == end_line {
            // 같은 줄에서 삭제
            let start_byte = self.char_idx_to_byte_idx(start_line, start_col);
            let end_byte = self.char_idx_to_byte_idx(start_line, end_col);
            self.content[start_line].replace_range(start_byte .. end_byte, "");
            self.cursor_line = start_line;
            self.cursor_col = start_col;
        } else {
            // 여러 줄 삭제
            let start_byte = self.char_idx_to_byte_idx(start_line, start_col);
            let end_byte = self.char_idx_to_byte_idx(end_line, end_col);
            let before = self.content[start_line][.. start_byte].to_string();
            let after = self.content[end_line][end_byte ..].to_string();

            // 중간 줄들 제거
            self.content.drain(start_line ..= end_line);

            // 합친 줄 삽입
            self.content.insert(start_line, before + &after);
            self.cursor_line = start_line;
            self.cursor_col = start_col;
        }

        self.selection = None;
        self.is_modified = true;
    }

    pub fn move_word_next(&mut self) {
        if self.cursor_line >= self.content.len() {
            return;
        }

        let line = &self.content[self.cursor_line];
        let byte_idx = self.char_idx_to_byte_idx(self.cursor_line, self.cursor_col);
        let mut chars = line[byte_idx ..].char_indices().peekable();

        // 현재 단어의 나머지 건너뛰기
        while let Some((_, ch)) = chars.peek() {
            if ch.is_whitespace() {
                break;
            }
            chars.next();
        }

        // 공백 건너뛰기
        while let Some((_, ch)) = chars.peek() {
            if !ch.is_whitespace() {
                break;
            }
            chars.next();
        }

        // 다음 단어 시작 위치
        if let Some((idx, _)) = chars.next() {
            self.cursor_col = self.byte_idx_to_char_idx(self.cursor_line, byte_idx + idx);
        } else {
            // 줄 끝 - 문자 개수로 변환
            self.cursor_col = line.chars().count();
        }
    }

    pub fn move_word_prev(&mut self) {
        if self.cursor_line >= self.content.len() || self.cursor_col == 0 {
            return;
        }

        let line = &self.content[self.cursor_line];
        let mut pos = self.cursor_col;

        // 현재 위치 뒤로 이동
        pos = pos.saturating_sub(1);

        // 공백 건너뛰기
        while pos > 0 && line.chars().nth(pos).is_some_and(|c| c.is_whitespace()) {
            pos -= 1;
        }

        // 단어 시작까지 이동
        while pos > 0 && line.chars().nth(pos - 1).is_some_and(|c| !c.is_whitespace()) {
            pos -= 1;
        }

        self.cursor_col = pos;
    }

    pub fn move_word_end(&mut self) {
        if self.cursor_line >= self.content.len() {
            return;
        }

        let line = &self.content[self.cursor_line];
        let line_char_len = line.chars().count();
        if self.cursor_col >= line_char_len {
            return;
        }

        let mut pos = self.cursor_col;

        // 현재 단어의 끝에 있다면 다음 단어로 이동
        let current_is_word = line.chars().nth(pos).is_some_and(|c| !c.is_whitespace());
        let next_is_space = line.chars().nth(pos + 1).is_none_or(|c| c.is_whitespace());

        if current_is_word && next_is_space {
            // 현재 단어 끝에 있으므로 공백 건너뛰고 다음 단어로
            pos += 1;
            while pos < line_char_len && line.chars().nth(pos).is_some_and(|c| c.is_whitespace()) {
                pos += 1;
            }
        }

        // 공백이면 건너뛰기
        while pos < line_char_len && line.chars().nth(pos).is_some_and(|c| c.is_whitespace()) {
            pos += 1;
        }

        // 단어 끝까지 이동
        while pos < line_char_len && line.chars().nth(pos).is_some_and(|c| !c.is_whitespace()) {
            pos += 1;
        }

        // 마지막 문자 위치로 (끝이 아니라 마지막 문자)
        pos = pos.saturating_sub(1);

        self.cursor_col = pos;
    }

    pub fn execute_search(&mut self) {
        if self.search_pattern.is_empty() {
            return;
        }

        // 전체 문서에서 검색
        self.search_matches.clear();
        for (line_idx, line) in self.content.iter().enumerate() {
            let mut start_byte = 0;
            while let Some(pos_byte) = line[start_byte ..].find(&self.search_pattern) {
                let match_byte = start_byte + pos_byte;
                // 바이트 인덱스를 문자 인덱스로 변환
                let match_char = self.byte_idx_to_char_idx(line_idx, match_byte);
                self.search_matches.push((line_idx, match_char));
                start_byte = match_byte + 1;
            }
        }

        // 현재 커서 이후 첫 매치로 이동
        if !self.search_matches.is_empty() {
            self.current_match_index = self
                .search_matches
                .iter()
                .position(|(line, col)| *line > self.cursor_line || (*line == self.cursor_line && *col >= self.cursor_col))
                .unwrap_or(0);

            let (line, col) = self.search_matches[self.current_match_index];
            self.cursor_line = line;
            self.cursor_col = col;

            // 검색어 길이만큼 선택 (문자 개수)
            let pattern_char_len = self.search_pattern.chars().count();
            self.selection = Some(Selection {
                anchor_line: line,
                anchor_col: col,
                cursor_line: line,
                cursor_col: col + pattern_char_len,
            });
        }
    }

    pub fn search_next(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }

        // 다음 매치로 순환
        self.current_match_index = (self.current_match_index + 1) % self.search_matches.len();

        let (line, col) = self.search_matches[self.current_match_index];
        self.cursor_line = line;
        self.cursor_col = col;

        // 검색어 선택 (문자 개수)
        let pattern_char_len = self.search_pattern.chars().count();
        self.selection = Some(Selection {
            anchor_line: line,
            anchor_col: col,
            cursor_line: line,
            cursor_col: col + pattern_char_len,
        });
    }

    pub fn search_prev(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }

        // 이전 매치로 순환
        if self.current_match_index == 0 {
            self.current_match_index = self.search_matches.len() - 1;
        } else {
            self.current_match_index -= 1;
        }

        let (line, col) = self.search_matches[self.current_match_index];
        self.cursor_line = line;
        self.cursor_col = col;

        // 검색어 선택 (문자 개수)
        let pattern_char_len = self.search_pattern.chars().count();
        self.selection = Some(Selection {
            anchor_line: line,
            anchor_col: col,
            cursor_line: line,
            cursor_col: col + pattern_char_len,
        });
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}
