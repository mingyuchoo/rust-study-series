# Ratatui 다이어리 구현 계획

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** ELM 아키텍처 기반 터미널 다이어리 애플리케이션 구현

**Architecture:** Model-Update-View 패턴, 순수 함수 중심, 명시적 side effect 분리

**Tech Stack:** Rust, Ratatui 0.27, Crossterm 0.27, Chrono 0.4, Dirs 5.0

---

## Task 1: 프로젝트 초기화

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `src/main.rs`

**Step 1: Cargo 프로젝트 초기화**

Run: `cargo init --name ratatui-diary`
Expected: "Created binary (application) package"

**Step 2: Cargo.toml 의존성 추가**

```toml
[package]
name = "ratatui-diary"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.27"
crossterm = "0.27"
chrono = "0.4"
dirs = "5.0"

[dev-dependencies]
tempfile = "3.8"
```

**Step 3: 라이브러리 모듈 구조 생성**

`src/lib.rs`:
```rust
pub mod model;
pub mod message;
pub mod update;
pub mod view;
pub mod storage;

pub use model::Model;
pub use message::Msg;
```

**Step 4: 빌드 확인**

Run: `cargo build`
Expected: 성공 (빈 모듈들은 다음 태스크에서 생성)

**Step 5: 커밋**

```bash
git add Cargo.toml src/lib.rs src/main.rs
git commit -m "chore: Initialize Cargo project with dependencies

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Storage 모듈 - 디렉토리 초기화 (TDD)

**Files:**
- Create: `src/storage.rs`
- Create: `tests/storage_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/storage_tests.rs`:
```rust
use ratatui_diary::storage::Storage;
use tempfile::TempDir;

#[test]
fn test_new_creates_entries_directory() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let entries_dir = temp.path().join("entries");
    assert!(entries_dir.exists());
    assert!(entries_dir.is_dir());
}
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_new_creates_entries_directory`
Expected: FAIL - "no associated function `with_dir`"

**Step 3: 최소 구현**

`src/storage.rs`:
```rust
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

pub struct Storage {
    entries_dir: PathBuf,
}

impl Storage {
    pub fn with_dir(base_dir: &Path) -> io::Result<Self> {
        let entries_dir = base_dir.join("entries");
        fs::create_dir_all(&entries_dir)?;
        Ok(Self { entries_dir })
    }

    pub fn new() -> io::Result<Self> {
        let base_dir = dirs::data_local_dir()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::NotFound,
                "Cannot find local data directory"
            ))?
            .join("ratatui-diary");
        Self::with_dir(&base_dir)
    }
}
```

**Step 4: 테스트 통과 확인**

Run: `cargo test test_new_creates_entries_directory`
Expected: PASS

**Step 5: 커밋**

```bash
git add src/storage.rs tests/storage_tests.rs
git commit -m "feat(storage): Add Storage initialization with directory creation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Storage 모듈 - 저장 기능 (TDD)

**Files:**
- Modify: `src/storage.rs`
- Modify: `tests/storage_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/storage_tests.rs`에 추가:
```rust
use chrono::NaiveDate;

#[test]
fn test_save_creates_markdown_file() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let content = "Test diary content";

    storage.save(date, content).unwrap();

    let file_path = temp.path().join("entries/2026-02-14.md");
    assert!(file_path.exists());

    let saved = std::fs::read_to_string(file_path).unwrap();
    assert_eq!(saved, content);
}
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_save_creates_markdown_file`
Expected: FAIL - "no method named `save`"

**Step 3: 최소 구현**

`src/storage.rs`에 추가:
```rust
use chrono::NaiveDate;

impl Storage {
    pub fn save(&self, date: NaiveDate, content: &str) -> io::Result<()> {
        let path = self.get_path(date);
        fs::write(path, content)
    }

    fn get_path(&self, date: NaiveDate) -> PathBuf {
        self.entries_dir.join(format!("{}.md", date))
    }
}
```

**Step 4: 테스트 통과 확인**

Run: `cargo test test_save_creates_markdown_file`
Expected: PASS

**Step 5: 커밋**

```bash
git add src/storage.rs tests/storage_tests.rs
git commit -m "feat(storage): Add save method for diary entries

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Storage 모듈 - 로드 기능 (TDD)

**Files:**
- Modify: `src/storage.rs`
- Modify: `tests/storage_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/storage_tests.rs`에 추가:
```rust
#[test]
fn test_load_existing_diary() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let content = "Existing diary";
    storage.save(date, content).unwrap();

    let loaded = storage.load(date).unwrap();
    assert_eq!(loaded, content);
}

#[test]
fn test_load_nonexistent_diary() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let result = storage.load(date);

    assert!(result.is_err());
}
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_load`
Expected: FAIL - "no method named `load`"

**Step 3: 최소 구현**

`src/storage.rs`에 추가:
```rust
impl Storage {
    pub fn load(&self, date: NaiveDate) -> io::Result<String> {
        let path = self.get_path(date);
        fs::read_to_string(path)
    }
}
```

**Step 4: 테스트 통과 확인**

Run: `cargo test test_load`
Expected: PASS (2 tests)

**Step 5: 커밋**

```bash
git add src/storage.rs tests/storage_tests.rs
git commit -m "feat(storage): Add load method for diary entries

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Storage 모듈 - 삭제 및 스캔 기능 (TDD)

**Files:**
- Modify: `src/storage.rs`
- Modify: `tests/storage_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/storage_tests.rs`에 추가:
```rust
use std::collections::HashSet;

#[test]
fn test_delete_diary() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    storage.save(date, "test").unwrap();

    storage.delete(date).unwrap();

    let result = storage.load(date);
    assert!(result.is_err());
}

#[test]
fn test_scan_entries() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date1 = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let date2 = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    storage.save(date1, "test1").unwrap();
    storage.save(date2, "test2").unwrap();

    let entries = storage.scan_entries().unwrap();

    assert_eq!(entries.len(), 2);
    assert!(entries.contains(&date1));
    assert!(entries.contains(&date2));
}
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_delete && cargo test test_scan`
Expected: FAIL - methods not found

**Step 3: 최소 구현**

`src/storage.rs`에 추가:
```rust
use std::collections::HashSet;
use std::ffi::OsString;

impl Storage {
    pub fn delete(&self, date: NaiveDate) -> io::Result<()> {
        let path = self.get_path(date);
        fs::remove_file(path)
    }

    pub fn scan_entries(&self) -> io::Result<HashSet<NaiveDate>> {
        let mut entries = HashSet::new();

        for entry in fs::read_dir(&self.entries_dir)? {
            let entry = entry?;
            if let Some(date) = parse_filename(entry.file_name()) {
                entries.insert(date);
            }
        }

        Ok(entries)
    }
}

fn parse_filename(filename: OsString) -> Option<NaiveDate> {
    let name = filename.to_str()?.strip_suffix(".md")?;
    NaiveDate::parse_from_str(name, "%Y-%m-%d").ok()
}
```

**Step 4: 테스트 통과 확인**

Run: `cargo test`
Expected: PASS (all 5 tests)

**Step 5: 커밋**

```bash
git add src/storage.rs tests/storage_tests.rs
git commit -m "feat(storage): Add delete and scan_entries methods

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Model 구조체 정의

**Files:**
- Create: `src/model.rs`

**Step 1: Model 구조체 작성**

`src/model.rs`:
```rust
use chrono::NaiveDate;
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
```

**Step 2: 빌드 확인**

Run: `cargo build`
Expected: 성공

**Step 3: 커밋**

```bash
git add src/model.rs
git commit -m "feat(model): Define core data structures

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Message 열거형 정의

**Files:**
- Create: `src/message.rs`

**Step 1: Message enum 작성**

`src/message.rs`:
```rust
use chrono::NaiveDate;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Msg {
    // 앱 제어
    Quit,
    Tick,
    DismissError,

    // 달력 네비게이션
    CalendarMoveUp,
    CalendarMoveDown,
    CalendarMoveLeft,
    CalendarMoveRight,
    CalendarPrevMonth,
    CalendarNextMonth,
    CalendarPrevYear,
    CalendarNextYear,
    CalendarSelectDate,

    // 에디터
    EditorEnterInsertMode,
    EditorEnterNormalMode,
    EditorInsertChar(char),
    EditorBackspace,
    EditorNewLine,
    EditorDeleteLine,
    EditorMoveCursor(Direction),
    EditorStartCommand,
    EditorCommandChar(char),
    EditorExecuteCommand,
    EditorBack,

    // 파일 I/O
    LoadDiarySuccess(NaiveDate, String),
    LoadDiaryFailed(String),
    SaveDiarySuccess,
    SaveDiaryFailed(String),
    DeleteDiarySuccess(NaiveDate),
    RefreshIndex(HashSet<NaiveDate>),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

**Step 2: 빌드 확인**

Run: `cargo build`
Expected: 성공

**Step 3: 커밋**

```bash
git add src/message.rs
git commit -m "feat(message): Define message types for state updates

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: CalendarState 메서드 구현 (TDD)

**Files:**
- Modify: `src/model.rs`
- Create: `tests/model_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/model_tests.rs`:
```rust
use ratatui_diary::model::CalendarState;
use chrono::NaiveDate;

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
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_next_month && cargo test test_prev_month`
Expected: FAIL - methods not found

**Step 3: 최소 구현**

`src/model.rs`의 `CalendarState impl`에 추가:
```rust
impl CalendarState {
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

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}
```

**Step 4: 테스트 통과 확인**

Run: `cargo test`
Expected: PASS

**Step 5: 커밋**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat(model): Add calendar navigation methods

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: EditorState 메서드 구현 (TDD)

**Files:**
- Modify: `src/model.rs`
- Modify: `tests/model_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/model_tests.rs`에 추가:
```rust
use ratatui_diary::model::{EditorState, EditorMode};

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
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_insert_char && cargo test test_new_line && cargo test test_load_content`
Expected: FAIL - methods not found

**Step 3: 최소 구현**

`src/model.rs`의 `EditorState impl`에 추가:
```rust
impl EditorState {
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
```

**Step 4: 테스트 통과 확인**

Run: `cargo test`
Expected: PASS

**Step 5: 커밋**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat(model): Add editor text manipulation methods

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: Update 함수 - 달력 네비게이션 (TDD)

**Files:**
- Create: `src/update.rs`
- Create: `tests/update_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/update_tests.rs`:
```rust
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
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_calendar`
Expected: FAIL - update function not found

**Step 3: 최소 구현**

`src/update.rs`:
```rust
use crate::model::{Model, Screen};
use crate::message::Msg;
use chrono::NaiveDate;

pub enum Command {
    LoadDiary(NaiveDate),
    SaveDiary(NaiveDate, String),
    DeleteDiary(NaiveDate),
}

pub fn update(model: &mut Model, msg: Msg) -> Option<Command> {
    match msg {
        Msg::Quit => {
            // Handled by main loop
        }

        Msg::DismissError => {
            model.show_error_popup = false;
            model.error_message = None;
        }

        // 달력 네비게이션
        Msg::CalendarMoveLeft => {
            if model.screen == Screen::Calendar {
                // TODO: 커서 이동 구현
            }
        }
        Msg::CalendarNextMonth => {
            if model.screen == Screen::Calendar {
                model.calendar_state.next_month();
            }
        }
        Msg::CalendarPrevMonth => {
            if model.screen == Screen::Calendar {
                model.calendar_state.prev_month();
            }
        }
        Msg::CalendarNextYear => {
            if model.screen == Screen::Calendar {
                model.calendar_state.next_year();
            }
        }
        Msg::CalendarPrevYear => {
            if model.screen == Screen::Calendar {
                model.calendar_state.prev_year();
            }
        }
        Msg::CalendarSelectDate => {
            if model.screen == Screen::Calendar {
                let date = model.calendar_state.selected_date;
                model.screen = Screen::Editor;
                model.editor_state.date = date;
                return Some(Command::LoadDiary(date));
            }
        }

        _ => {}
    }

    None
}
```

**Step 4: lib.rs에 update 모듈 공개**

`src/lib.rs`에 추가:
```rust
pub mod update;
```

**Step 5: 테스트 통과 확인**

Run: `cargo test test_calendar`
Expected: PASS

**Step 6: 커밋**

```bash
git add src/update.rs src/lib.rs tests/update_tests.rs
git commit -m "feat(update): Add calendar navigation update logic

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 11: Update 함수 - 에디터 기능 (TDD)

**Files:**
- Modify: `src/update.rs`
- Modify: `tests/update_tests.rs`

**Step 1: 실패하는 테스트 작성**

`tests/update_tests.rs`에 추가:
```rust
use ratatui_diary::model::EditorMode;

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
```

**Step 2: 테스트 실행하여 실패 확인**

Run: `cargo test test_editor`
Expected: FAIL - logic not implemented

**Step 3: 구현**

`src/update.rs`의 `update` 함수에 추가:
```rust
// 에디터 - Normal 모드
Msg::EditorEnterInsertMode => {
    if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
        model.editor_state.mode = EditorMode::Insert;
    }
}
Msg::EditorEnterNormalMode => {
    if model.screen == Screen::Editor {
        model.editor_state.mode = EditorMode::Normal;
    }
}
Msg::EditorDeleteLine => {
    if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
        let date = model.editor_state.date;
        return Some(Command::DeleteDiary(date));
    }
}
Msg::EditorStartCommand => {
    if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
        model.editor_state.mode = EditorMode::Command(String::new());
    }
}

// 에디터 - Insert 모드
Msg::EditorInsertChar(c) => {
    if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
        model.editor_state.insert_char(c);
    }
}
Msg::EditorBackspace => {
    if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
        model.editor_state.backspace();
    }
}
Msg::EditorNewLine => {
    if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
        model.editor_state.new_line();
    }
}

// 에디터 - Command 모드
Msg::EditorCommandChar(c) => {
    if let EditorMode::Command(ref mut cmd) = model.editor_state.mode {
        cmd.push(c);
    }
}
Msg::EditorExecuteCommand => {
    if let EditorMode::Command(ref cmd) = model.editor_state.mode.clone() {
        let date = model.editor_state.date;
        let content = model.editor_state.get_content();

        match cmd.as_str() {
            "w" => {
                model.editor_state.mode = EditorMode::Normal;
                return Some(Command::SaveDiary(date, content));
            }
            "q" => {
                model.screen = Screen::Calendar;
                model.editor_state = EditorState::new(date);
            }
            "wq" => {
                model.screen = Screen::Calendar;
                let old_state = std::mem::replace(
                    &mut model.editor_state,
                    EditorState::new(date)
                );
                return Some(Command::SaveDiary(date, old_state.get_content()));
            }
            _ => {
                model.error_message = Some(format!("Unknown command: {}", cmd));
                model.show_error_popup = true;
            }
        }
        model.editor_state.mode = EditorMode::Normal;
    }
}
Msg::EditorBack => {
    if model.screen == Screen::Editor {
        model.screen = Screen::Calendar;
    }
}
```

필요한 import 추가:
```rust
use crate::model::{Model, Screen, EditorMode, EditorState};
```

**Step 4: 테스트 통과 확인**

Run: `cargo test test_editor`
Expected: PASS

**Step 5: 커밋**

```bash
git add src/update.rs tests/update_tests.rs
git commit -m "feat(update): Add editor mode update logic

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 12: Update 함수 - 파일 I/O 결과 처리

**Files:**
- Modify: `src/update.rs`

**Step 1: 파일 I/O 메시지 처리 추가**

`src/update.rs`의 `update` 함수에 추가:
```rust
// 파일 I/O 결과
Msg::LoadDiarySuccess(date, content) => {
    if model.screen == Screen::Editor {
        model.editor_state.date = date;
        model.editor_state.load_content(&content);
    }
}
Msg::LoadDiaryFailed(error) => {
    // 파일 없음 = 새 다이어리, 에러 표시 안함
    if !error.contains("No such file") {
        model.error_message = Some(format!("로드 실패: {}", error));
        model.show_error_popup = true;
    }
}
Msg::SaveDiarySuccess => {
    model.editor_state.is_modified = false;
}
Msg::SaveDiaryFailed(error) => {
    model.error_message = Some(format!("저장 실패: {}", error));
    model.show_error_popup = true;
}
Msg::DeleteDiarySuccess(date) => {
    model.diary_entries.entries.remove(&date);
    model.screen = Screen::Calendar;
}
Msg::RefreshIndex(entries) => {
    model.diary_entries.entries = entries;
}
```

**Step 2: 빌드 확인**

Run: `cargo build`
Expected: 성공

**Step 3: 커밋**

```bash
git add src/update.rs
git commit -m "feat(update): Add file I/O result handling

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 13: View 모듈 - 달력 화면 렌더링

**Files:**
- Create: `src/view.rs`

**Step 1: View 모듈 기본 구조**

`src/view.rs`:
```rust
use crate::model::{Model, Screen};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn view(f: &mut Frame, model: &Model) {
    match model.screen {
        Screen::Calendar => render_calendar(f, model),
        Screen::Editor => render_editor(f, model),
    }
}

fn render_calendar(f: &mut Frame, model: &Model) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // 헤더
            Constraint::Min(0),      // 달력
            Constraint::Length(2),   // 상태바
        ])
        .split(f.size());

    // 헤더
    let header = Paragraph::new(format!(
        "{}년 {}월",
        model.calendar_state.current_year,
        model.calendar_state.current_month
    ))
    .alignment(Alignment::Center)
    .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // 달력 그리드
    render_calendar_grid(f, chunks[1], model);

    // 상태바
    let statusbar = Paragraph::new("h/l: 달 | H/L: 연도 | Enter: 작성 | q: 종료")
        .alignment(Alignment::Center);
    f.render_widget(statusbar, chunks[2]);
}

fn render_calendar_grid(f: &mut Frame, area: Rect, model: &Model) {
    use chrono::{Datelike, NaiveDate};

    let year = model.calendar_state.current_year;
    let month = model.calendar_state.current_month;

    // 요일 헤더
    let weekdays = vec!["일", "월", "화", "수", "목", "금", "토"];
    let mut lines = vec![Line::from(
        weekdays.iter()
            .map(|&day| Span::styled(format!("{:^4}", day), Style::default()))
            .collect::<Vec<_>>()
    )];

    // 월의 첫날
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let weekday = first_day.weekday().num_days_from_sunday() as usize;

    // 달력 생성
    let days_in_month = first_day
        .with_month(month + 1)
        .unwrap_or_else(|| first_day.with_year(year + 1).unwrap().with_month(1).unwrap())
        .pred_opt()
        .unwrap()
        .day();

    let mut week = vec![Span::raw("    "); 7];
    let mut day = 1;

    // 첫 주 빈 칸 채우기
    for i in weekday..7 {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        week[i] = format_day(day, date, model);
        day += 1;
    }
    lines.push(Line::from(week.clone()));

    // 나머지 주
    while day <= days_in_month {
        week = vec![Span::raw("    "); 7];
        for i in 0..7 {
            if day <= days_in_month {
                let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                week[i] = format_day(day, date, model);
                day += 1;
            }
        }
        lines.push(Line::from(week.clone()));
    }

    let calendar = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(calendar, area);
}

fn format_day(day: u32, date: chrono::NaiveDate, model: &Model) -> Span<'static> {
    let has_entry = model.diary_entries.entries.contains(&date);
    let is_selected = date == model.calendar_state.selected_date;
    let is_today = date == chrono::Local::now().date_naive();

    let mut style = Style::default();

    if has_entry {
        style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
    }
    if is_selected {
        style = style.bg(Color::Blue);
    }
    if is_today {
        style = style.add_modifier(Modifier::UNDERLINED);
    }

    let marker = if has_entry { "●" } else { " " };
    Span::styled(format!("{:>2}{} ", day, marker), style)
}

fn render_editor(f: &mut Frame, _model: &Model) {
    // TODO: 다음 태스크에서 구현
    let placeholder = Paragraph::new("Editor (Coming soon)");
    f.render_widget(placeholder, f.size());
}
```

**Step 2: lib.rs에 view 모듈 공개**

`src/lib.rs`에 추가:
```rust
pub mod view;
```

**Step 3: 빌드 확인**

Run: `cargo build`
Expected: 성공

**Step 4: 커밋**

```bash
git add src/view.rs src/lib.rs
git commit -m "feat(view): Add calendar screen rendering

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 14: View 모듈 - 에디터 화면 렌더링

**Files:**
- Modify: `src/view.rs`

**Step 1: 에디터 렌더링 구현**

`src/view.rs`의 `render_editor` 함수 교체:
```rust
use crate::model::EditorMode;

fn render_editor(f: &mut Frame, model: &Model) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),   // 날짜 헤더
            Constraint::Min(0),      // 에디터 영역
            Constraint::Length(1),   // 모드 표시
        ])
        .split(f.size());

    // 헤더: 날짜
    let header = Paragraph::new(model.editor_state.date.to_string())
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // 에디터 내용
    let content = model.editor_state.get_content();
    let text = Paragraph::new(content)
        .wrap(Wrap { trim: false });
    f.render_widget(text, chunks[1]);

    // 커서 표시 (Insert 모드)
    if model.editor_state.mode == EditorMode::Insert {
        // 커서 위치 계산
        let cursor_x = chunks[1].x + model.editor_state.cursor_col as u16;
        let cursor_y = chunks[1].y + model.editor_state.cursor_line as u16;
        f.set_cursor(cursor_x, cursor_y);
    }

    // 하단바: 모드 표시
    let mode_text = match &model.editor_state.mode {
        EditorMode::Normal => "-- NORMAL --".to_string(),
        EditorMode::Insert => "-- INSERT --".to_string(),
        EditorMode::Command(cmd) => format!(":{}", cmd),
    };
    let statusbar = Paragraph::new(mode_text)
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(statusbar, chunks[2]);
}
```

**Step 2: 빌드 확인**

Run: `cargo build`
Expected: 성공

**Step 3: 커밋**

```bash
git add src/view.rs
git commit -m "feat(view): Add editor screen rendering

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 15: Main 이벤트 루프 구현

**Files:**
- Modify: `src/main.rs`

**Step 1: Main 함수 구현**

`src/main.rs`:
```rust
use ratatui_diary::{Model, Msg, update, view, storage::Storage};
use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Storage 초기화
    let storage = Storage::new()?;
    let entries = storage.scan_entries()?;

    // Model 초기화
    let mut model = Model::new(entries);

    // Terminal 초기화
    let mut terminal = ratatui::init();
    terminal.clear()?;

    // 이벤트 루프
    let result = run_app(&mut terminal, &mut model, &storage);

    // Terminal 복원
    ratatui::restore();

    result
}

fn run_app(
    terminal: &mut DefaultTerminal,
    model: &mut Model,
    storage: &Storage,
) -> std::io::Result<()> {
    loop {
        // 렌더링
        terminal.draw(|f| view::view(f, model))?;

        // 이벤트 처리
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let Some(msg) = handle_key(key, model) {
                    // Quit 메시지 처리
                    if matches!(msg, Msg::Quit) {
                        break;
                    }

                    // Update 호출
                    if let Some(cmd) = update::update(model, msg) {
                        execute_command(cmd, model, storage)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_key(key: KeyEvent, model: &Model) -> Option<Msg> {
    use crate::model::{Screen, EditorMode};

    match model.screen {
        Screen::Calendar => handle_calendar_key(key),
        Screen::Editor => handle_editor_key(key, &model.editor_state.mode),
    }
}

fn handle_calendar_key(key: KeyEvent) -> Option<Msg> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => Some(Msg::Quit),
        (KeyCode::Char('h'), _) => Some(Msg::CalendarMoveLeft),
        (KeyCode::Char('l'), _) => Some(Msg::CalendarMoveRight),
        (KeyCode::Char('j'), _) => Some(Msg::CalendarMoveDown),
        (KeyCode::Char('k'), _) => Some(Msg::CalendarMoveUp),
        (KeyCode::Char('H'), KeyModifiers::SHIFT) => Some(Msg::CalendarPrevYear),
        (KeyCode::Char('L'), KeyModifiers::SHIFT) => Some(Msg::CalendarNextYear),
        (KeyCode::Enter, _) => Some(Msg::CalendarSelectDate),
        _ => None,
    }
}

fn handle_editor_key(key: KeyEvent, mode: &EditorMode) -> Option<Msg> {
    match mode {
        EditorMode::Normal => match key.code {
            KeyCode::Char('i') => Some(Msg::EditorEnterInsertMode),
            KeyCode::Char(':') => Some(Msg::EditorStartCommand),
            KeyCode::Char('d') => Some(Msg::EditorDeleteLine), // dd는 두 번 누르기
            KeyCode::Esc => Some(Msg::EditorBack),
            _ => None,
        },
        EditorMode::Insert => match key.code {
            KeyCode::Esc => Some(Msg::EditorEnterNormalMode),
            KeyCode::Char(c) => Some(Msg::EditorInsertChar(c)),
            KeyCode::Backspace => Some(Msg::EditorBackspace),
            KeyCode::Enter => Some(Msg::EditorNewLine),
            _ => None,
        },
        EditorMode::Command(_) => match key.code {
            KeyCode::Char(c) => Some(Msg::EditorCommandChar(c)),
            KeyCode::Enter => Some(Msg::EditorExecuteCommand),
            KeyCode::Esc => Some(Msg::EditorEnterNormalMode),
            KeyCode::Backspace => Some(Msg::EditorCommandChar('\x08')), // TODO: proper backspace
            _ => None,
        },
    }
}

fn execute_command(
    cmd: update::Command,
    model: &mut Model,
    storage: &Storage,
) -> std::io::Result<()> {
    use update::Command;

    match cmd {
        Command::LoadDiary(date) => {
            match storage.load(date) {
                Ok(content) => {
                    update::update(model, Msg::LoadDiarySuccess(date, content));
                }
                Err(e) => {
                    update::update(model, Msg::LoadDiaryFailed(e.to_string()));
                }
            }
        }
        Command::SaveDiary(date, content) => {
            match storage.save(date, &content) {
                Ok(_) => {
                    model.diary_entries.entries.insert(date);
                    update::update(model, Msg::SaveDiarySuccess);
                }
                Err(e) => {
                    update::update(model, Msg::SaveDiaryFailed(e.to_string()));
                }
            }
        }
        Command::DeleteDiary(date) => {
            match storage.delete(date) {
                Ok(_) => {
                    update::update(model, Msg::DeleteDiarySuccess(date));
                }
                Err(e) => {
                    update::update(model, Msg::SaveDiaryFailed(e.to_string()));
                }
            }
        }
    }

    Ok(())
}
```

필요한 import를 `src/lib.rs`에 추가:
```rust
pub use update::update;
pub use view::view;
pub use storage::Storage;
```

**Step 2: 빌드 및 실행 테스트**

Run: `cargo build`
Expected: 성공

Run: `cargo run`
Expected: 앱 실행, 달력 화면 표시, 'q'로 종료 가능

**Step 3: 커밋**

```bash
git add src/main.rs src/lib.rs
git commit -m "feat(main): Add event loop and key handling

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 16: 달력 커서 이동 구현

**Files:**
- Modify: `src/model.rs`
- Modify: `src/update.rs`

**Step 1: CalendarState 커서 이동 메서드**

`src/model.rs`의 `CalendarState impl`에 추가:
```rust
impl CalendarState {
    pub fn move_cursor_left(&mut self) {
        self.selected_date = self.selected_date.pred_opt().unwrap_or(self.selected_date);
    }

    pub fn move_cursor_right(&mut self) {
        self.selected_date = self.selected_date.succ_opt().unwrap_or(self.selected_date);
    }

    pub fn move_cursor_up(&mut self) {
        self.selected_date = self.selected_date
            .checked_sub_days(chrono::Days::new(7))
            .unwrap_or(self.selected_date);
    }

    pub fn move_cursor_down(&mut self) {
        self.selected_date = self.selected_date
            .checked_add_days(chrono::Days::new(7))
            .unwrap_or(self.selected_date);
    }
}
```

**Step 2: Update 함수에 적용**

`src/update.rs`의 `update` 함수에서 TODO 부분 구현:
```rust
Msg::CalendarMoveLeft => {
    if model.screen == Screen::Calendar {
        model.calendar_state.move_cursor_left();
    }
}
Msg::CalendarMoveRight => {
    if model.screen == Screen::Calendar {
        model.calendar_state.move_cursor_right();
    }
}
Msg::CalendarMoveUp => {
    if model.screen == Screen::Calendar {
        model.calendar_state.move_cursor_up();
    }
}
Msg::CalendarMoveDown => {
    if model.screen == Screen::Calendar {
        model.calendar_state.move_cursor_down();
    }
}
```

**Step 3: 빌드 및 테스트**

Run: `cargo build`
Expected: 성공

Run: `cargo run`
Expected: h/j/k/l로 날짜 선택 가능

**Step 4: 커밋**

```bash
git add src/model.rs src/update.rs
git commit -m "feat(calendar): Add cursor navigation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 17: 에러 팝업 렌더링

**Files:**
- Modify: `src/view.rs`

**Step 1: 에러 팝업 추가**

`src/view.rs`의 `view` 함수 끝에 추가:
```rust
pub fn view(f: &mut Frame, model: &Model) {
    match model.screen {
        Screen::Calendar => render_calendar(f, model),
        Screen::Editor => render_editor(f, model),
    }

    // 에러 팝업
    if model.show_error_popup {
        render_error_popup(f, model);
    }
}

fn render_error_popup(f: &mut Frame, model: &Model) {
    use ratatui::widgets::Clear;

    let area = centered_rect(60, 20, f.size());

    let error_msg = model.error_message.as_deref().unwrap_or("알 수 없는 에러");
    let popup = Paragraph::new(error_msg)
        .block(Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)))
        .style(Style::default().bg(Color::Black))
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
```

**Step 2: 에러 팝업 닫기 키 핸들링**

`src/main.rs`의 `handle_key` 함수 시작 부분에 추가:
```rust
fn handle_key(key: KeyEvent, model: &Model) -> Option<Msg> {
    // 에러 팝업이 표시 중이면 Esc로 닫기
    if model.show_error_popup && key.code == KeyCode::Esc {
        return Some(Msg::DismissError);
    }

    use crate::model::{Screen, EditorMode};
    // ... 기존 코드
}
```

**Step 3: 빌드 및 테스트**

Run: `cargo build`
Expected: 성공

**Step 4: 커밋**

```bash
git add src/view.rs src/main.rs
git commit -m "feat(view): Add error popup rendering

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 18: README 작성

**Files:**
- Create: `README.md`

**Step 1: README 작성**

`README.md`:
```markdown
# Ratatui Diary

터미널 기반 다이어리 애플리케이션 (Rust + Ratatui)

## 기능

- 📅 월간 달력 뷰
- ✍️ Vi 모드 텍스트 에디터
- 💾 Markdown 파일 자동 저장
- 🎨 다이어리 작성 유무 시각적 표시

## 설치

```bash
cargo build --release
cargo install --path .
```

## 사용법

```bash
ratatui-diary
```

### 달력 화면

| 키 | 동작 |
|---|---|
| `h/j/k/l` | 날짜 이동 |
| `H/L` | 연도 이동 |
| `Enter` | 다이어리 작성/편집 |
| `q` | 종료 |

### 에디터 화면

**Normal 모드:**
- `i`: Insert 모드
- `:w`: 저장
- `:q`: 나가기
- `:wq`: 저장 후 나가기
- `dd`: 다이어리 삭제
- `Esc`: 달력으로 돌아가기

**Insert 모드:**
- 텍스트 입력
- `Esc`: Normal 모드

## 데이터 저장

다이어리는 `~/.local/share/ratatui-diary/entries/` 디렉토리에 Markdown 파일로 저장됩니다.

파일명 형식: `YYYY-MM-DD.md`

## 아키텍처

ELM (Model-Update-View) 패턴 기반

- **Model**: 앱 상태
- **Update**: 상태 업데이트 로직
- **View**: UI 렌더링

## 개발

```bash
# 테스트 실행
cargo test

# 개발 모드 실행
cargo run
```

## 라이선스

MIT
```

**Step 2: 커밋**

```bash
git add README.md
git commit -m "docs: Add README with usage instructions

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 19: 통합 테스트 및 버그 수정

**Step 1: 전체 테스트 실행**

Run: `cargo test`
Expected: 모든 테스트 통과

**Step 2: 수동 테스트 체크리스트**

1. [ ] 앱 시작 및 달력 표시
2. [ ] 달력 네비게이션 (h/j/k/l, H/L)
3. [ ] 다이어리 작성 (Enter → Insert 모드 → 입력 → :wq)
4. [ ] 저장된 다이어리 표시 (녹색 ●)
5. [ ] 다이어리 수정 (기존 날짜 선택 → 수정 → :w)
6. [ ] 다이어리 삭제 (dd)
7. [ ] 에러 핸들링 (잘못된 명령)

**Step 3: 발견된 버그 수정**

수동 테스트 중 발견된 버그를 수정하고 각각 커밋

**Step 4: 최종 확인**

Run: `cargo test && cargo run`
Expected: 모든 기능 정상 동작

---

## 구현 완료

모든 태스크가 완료되면:

1. 최종 빌드: `cargo build --release`
2. 테스트 실행: `cargo test`
3. 문서 검토
4. 설계 문서와 비교하여 누락된 기능 확인

---

## 다음 단계 (Phase 2)

- 월별/연별 통계
- 단어 빈도 분석
- 검색 기능
- 그래프 시각화
