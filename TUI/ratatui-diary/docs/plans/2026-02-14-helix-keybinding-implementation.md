# Helix 키바인딩 구현 계획

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Ratatui Diary 앱의 키바인딩을 Helix Editor 스타일로 전환하여 selection-action 모델 구현

**Architecture:** ELM 패턴 유지하면서 EditorState에 selection, history, clipboard 추가. 키 핸들러를 서브모드 기반으로 재작성. TDD로 각 기능 검증.

**Tech Stack:** Rust, Ratatui, Crossterm, Chrono

---

## Task 1: Selection 구조체 및 기본 타입 정의

**Files:**
- Modify: `src/model.rs:1-196`
- Test: `tests/model_tests.rs`

**Step 1: Write the failing test**

`tests/model_tests.rs`에 추가:

```rust
#[cfg(test)]
mod selection_tests {
    use super::*;
    use ratatui_diary::model::{EditorState, Selection};
    use chrono::NaiveDate;

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
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test selection_tests
```

Expected: FAIL - "Selection" type not found

**Step 3: Write minimal implementation**

`src/model.rs`의 EditorState 위에 추가:

```rust
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
```

EditorState에 필드 추가:

```rust
pub struct EditorState {
    pub mode: EditorMode,
    pub date: NaiveDate,
    pub content: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_modified: bool,
    // 새로 추가
    pub selection: Option<Selection>,
    pub edit_history: Vec<EditorSnapshot>,
    pub history_index: usize,
    pub clipboard: String,
    pub submode: Option<EditorSubMode>,
    pub search_pattern: String,
    pub search_matches: Vec<(usize, usize)>,
    pub current_match_index: usize,
}
```

EditorMode 수정 (Command 제거):

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
}
```

CalendarState에 필드 추가:

```rust
pub struct CalendarState {
    pub current_year: i32,
    pub current_month: u32,
    pub selected_date: NaiveDate,
    pub cursor_pos: usize,
    pub submode: Option<CalendarSubMode>, // 새로 추가
}
```

EditorState::new() 업데이트:

```rust
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
}
```

CalendarState::new() 업데이트:

```rust
impl CalendarState {
    pub fn new(year: i32, month: u32) -> Self {
        let selected_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        Self {
            current_year: year,
            current_month: month,
            selected_date,
            cursor_pos: 0,
            submode: None, // 새로 추가
        }
    }
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test selection_tests
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat: Add Selection types and EditorState fields for Helix support"
```

---

## Task 2: Selection 범위 계산 메서드

**Files:**
- Modify: `src/model.rs:128-196`
- Test: `tests/model_tests.rs`

**Step 1: Write the failing test**

```rust
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
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_get_selection
```

Expected: FAIL - method not found

**Step 3: Write minimal implementation**

`src/model.rs`의 `impl EditorState`에 추가:

```rust
impl EditorState {
    // 기존 메서드들...

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

        if start_line == end_line {
            // 같은 줄
            Some(self.content[start_line][start_col..end_col].to_string())
        } else {
            // 여러 줄
            let mut result = String::new();
            result.push_str(&self.content[start_line][start_col..]);
            result.push('\n');
            for line in (start_line + 1)..end_line {
                result.push_str(&self.content[line]);
                result.push('\n');
            }
            result.push_str(&self.content[end_line][..end_col]);
            Some(result)
        }
    }
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_get_selection
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat: Add selection range calculation methods"
```

---

## Task 3: 새로운 메시지 타입 정의

**Files:**
- Create: `src/message.rs:1-100`
- Modify: `src/lib.rs:1-20`
- Test: 컴파일 확인

**Step 1: Write the test (컴파일 테스트)**

컴파일이 성공하면 타입이 올바르게 정의된 것

**Step 2: Run test to verify it fails**

```bash
cargo build
```

Expected: 새 타입들이 없어서 실패

**Step 3: Write minimal implementation**

`src/message.rs` 생성:

```rust
use chrono::NaiveDate;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Quit,
    DismissError,

    // 에디터 - 네비게이션
    EditorMoveLeft,
    EditorMoveRight,
    EditorMoveUp,
    EditorMoveDown,
    EditorWordNext,
    EditorWordPrev,
    EditorWordEnd,

    // 에디터 - Goto 모드
    EditorEnterGotoMode,
    EditorGotoDocStart,
    EditorGotoDocEnd,
    EditorGotoLineStart,
    EditorGotoLineEnd,
    EditorExitSubMode,

    // 에디터 - Insert 모드
    EditorEnterInsert(InsertPosition),
    EditorEnterNormalMode,
    EditorInsertChar(char),
    EditorBackspace,
    EditorNewLine,

    // 에디터 - Selection
    EditorToggleSelection,
    EditorSelectLine,

    // 에디터 - 편집
    EditorDelete,
    EditorChange,
    EditorYank,
    EditorPasteAfter,
    EditorPasteBefore,

    // 에디터 - Undo/Redo
    EditorUndo,
    EditorRedo,

    // 에디터 - 검색
    EditorEnterSearchMode,
    EditorSearchChar(char),
    EditorSearchBackspace,
    EditorSearchNext,
    EditorSearchPrev,
    EditorExecuteSearch,

    // 에디터 - Space 명령
    EditorEnterSpaceMode,
    EditorSpaceSave,
    EditorSpaceQuit,
    EditorSpaceSaveQuit,
    EditorBack,

    // 달력
    CalendarMoveLeft,
    CalendarMoveRight,
    CalendarMoveUp,
    CalendarMoveDown,
    CalendarSelectDate,
    CalendarEnterSpaceMode,
    CalendarExitSubMode,
    CalendarSpaceQuit,
    CalendarSpaceNextMonth,
    CalendarSpacePrevMonth,
    CalendarSpaceNextYear,
    CalendarSpacePrevYear,

    // 파일 I/O 결과
    LoadDiarySuccess(NaiveDate, String),
    LoadDiaryFailed(String),
    SaveDiarySuccess,
    SaveDiaryFailed(String),
    DeleteDiarySuccess(NaiveDate),
    RefreshIndex(HashSet<NaiveDate>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    BeforeCursor,
    AfterCursor,
    LineBelow,
    LineAbove,
}
```

`src/lib.rs` 수정:

```rust
pub mod message;
pub mod model;
pub mod storage;
pub mod update;
pub mod view;
pub mod markdown;

pub use message::Msg;
pub use model::Model;
```

**Step 4: Run test to verify it passes**

```bash
cargo build
```

Expected: BUILD SUCCESS

**Step 5: Commit**

```bash
git add src/message.rs src/lib.rs
git commit -m "feat: Add new Helix-style message types"
```

---

## Task 4: Selection 토글 및 라인 선택 구현

**Files:**
- Modify: `src/update.rs:1-168`
- Test: `tests/model_tests.rs`

**Step 1: Write the failing test**

```rust
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
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_selection_toggle
```

Expected: FAIL - pattern not matched

**Step 3: Write minimal implementation**

`src/update.rs`의 `update` 함수에 추가:

```rust
pub fn update(model: &mut Model, msg: Msg) -> Option<Command> {
    match msg {
        Msg::Quit => {
            // Handled by main loop
        }

        Msg::DismissError => {
            model.show_error_popup = false;
            model.error_message = None;
        }

        // 에디터 - Selection
        Msg::EditorToggleSelection => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                if state.selection.is_some() {
                    state.selection = None;
                } else {
                    state.selection = Some(Selection {
                        anchor_line: state.cursor_line,
                        anchor_col: state.cursor_col,
                        cursor_line: state.cursor_line,
                        cursor_col: state.cursor_col,
                    });
                }
            }
        }

        Msg::EditorSelectLine => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                let line_len = if state.cursor_line < state.content.len() {
                    state.content[state.cursor_line].len()
                } else {
                    0
                };
                state.selection = Some(Selection {
                    anchor_line: state.cursor_line,
                    anchor_col: 0,
                    cursor_line: state.cursor_line,
                    cursor_col: line_len,
                });
            }
        }

        // 기존 메시지들...
        _ => {}
    }

    None
}
```

`src/update.rs`의 import에 Selection 추가:

```rust
use crate::{
    message::Msg,
    model::{EditorMode, EditorState, Model, Screen, Selection},
};
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_selection_toggle
cargo test test_select_line
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/update.rs tests/model_tests.rs
git commit -m "feat: Implement selection toggle and line selection"
```

---

## Task 5: Delete 선택 영역 메서드

**Files:**
- Modify: `src/model.rs:128-196`
- Test: `tests/model_tests.rs`

**Step 1: Write the failing test**

```rust
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
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_delete_selection
```

Expected: FAIL - method not found

**Step 3: Write minimal implementation**

`src/model.rs`의 `impl EditorState`에 추가:

```rust
pub fn delete_selection(&mut self) {
    let ((start_line, start_col), (end_line, end_col)) = match self.get_selection_range() {
        Some(range) => range,
        None => return,
    };

    if start_line == end_line {
        // 같은 줄에서 삭제
        self.content[start_line].replace_range(start_col..end_col, "");
        self.cursor_line = start_line;
        self.cursor_col = start_col;
    } else {
        // 여러 줄 삭제
        let before = self.content[start_line][..start_col].to_string();
        let after = self.content[end_line][end_col..].to_string();

        // 중간 줄들 제거
        self.content.drain(start_line..=end_line);

        // 합친 줄 삽입
        self.content.insert(start_line, before + &after);
        self.cursor_line = start_line;
        self.cursor_col = start_col;
    }
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_delete_selection
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat: Implement delete_selection method"
```

---

## Task 6: Undo/Redo 시스템 구현

**Files:**
- Modify: `src/model.rs:128-196`
- Test: `tests/model_tests.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_undo_restores_previous_state() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["Original".to_string()];
    state.save_snapshot(); // 스냅샷 1

    state.content = vec!["Modified".to_string()];
    state.save_snapshot(); // 스냅샷 2

    state.undo();

    assert_eq!(state.content[0], "Original");
}

#[test]
fn test_redo_after_undo() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["First".to_string()];
    state.save_snapshot();

    state.content = vec!["Second".to_string()];
    state.save_snapshot();

    state.undo();
    assert_eq!(state.content[0], "First");

    state.redo();
    assert_eq!(state.content[0], "Second");
}

#[test]
fn test_new_edit_clears_redo_history() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["First".to_string()];
    state.save_snapshot();

    state.content = vec!["Second".to_string()];
    state.save_snapshot();

    state.undo();

    // 새 편집
    state.content = vec!["Third".to_string()];
    state.save_snapshot();

    // redo 불가능해야 함
    let before_redo = state.content.clone();
    state.redo();
    assert_eq!(state.content, before_redo);
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_undo
cargo test test_redo
```

Expected: FAIL - methods not found

**Step 3: Write minimal implementation**

`src/model.rs`의 `impl EditorState`에 추가:

```rust
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
        self.edit_history.drain(0..1);
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
```

`load_content` 메서드 수정:

```rust
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
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_undo
cargo test test_redo
cargo test test_new_edit_clears
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat: Implement undo/redo system with history management"
```

---

## Task 7: 단어 이동 구현

**Files:**
- Modify: `src/model.rs:128-196`
- Test: `tests/model_tests.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_word_next() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["Hello World Test".to_string()];
    state.cursor_line = 0;
    state.cursor_col = 0;

    state.move_word_next();
    assert_eq!(state.cursor_col, 6); // "World"의 시작

    state.move_word_next();
    assert_eq!(state.cursor_col, 12); // "Test"의 시작
}

#[test]
fn test_word_prev() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["Hello World Test".to_string()];
    state.cursor_line = 0;
    state.cursor_col = 12;

    state.move_word_prev();
    assert_eq!(state.cursor_col, 6); // "World"의 시작

    state.move_word_prev();
    assert_eq!(state.cursor_col, 0); // "Hello"의 시작
}

#[test]
fn test_word_end() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["Hello World Test".to_string()];
    state.cursor_line = 0;
    state.cursor_col = 0;

    state.move_word_end();
    assert_eq!(state.cursor_col, 4); // "Hello"의 끝 (마지막 문자 인덱스)

    state.move_word_end();
    assert_eq!(state.cursor_col, 10); // "World"의 끝
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_word_
```

Expected: FAIL - methods not found

**Step 3: Write minimal implementation**

`src/model.rs`의 `impl EditorState`에 추가:

```rust
pub fn move_word_next(&mut self) {
    if self.cursor_line >= self.content.len() {
        return;
    }

    let line = &self.content[self.cursor_line];
    let mut chars = line[self.cursor_col..].char_indices().peekable();

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
        self.cursor_col += idx;
    } else {
        // 줄 끝
        self.cursor_col = line.len();
    }
}

pub fn move_word_prev(&mut self) {
    if self.cursor_line >= self.content.len() || self.cursor_col == 0 {
        return;
    }

    let line = &self.content[self.cursor_line];
    let before = &line[..self.cursor_col];
    let mut pos = before.len();

    // 현재 위치 뒤로 이동
    if pos > 0 {
        pos -= 1;
    }

    // 공백 건너뛰기
    while pos > 0 && before.chars().nth(pos).map_or(false, |c| c.is_whitespace()) {
        pos -= 1;
    }

    // 단어 시작까지 이동
    while pos > 0 && before.chars().nth(pos - 1).map_or(false, |c| !c.is_whitespace()) {
        pos -= 1;
    }

    self.cursor_col = pos;
}

pub fn move_word_end(&mut self) {
    if self.cursor_line >= self.content.len() {
        return;
    }

    let line = &self.content[self.cursor_line];
    if self.cursor_col >= line.len() {
        return;
    }

    let mut pos = self.cursor_col;

    // 현재 공백이면 건너뛰기
    while pos < line.len() && line.chars().nth(pos).map_or(false, |c| c.is_whitespace()) {
        pos += 1;
    }

    // 단어 끝까지 이동
    while pos < line.len() && line.chars().nth(pos).map_or(false, |c| !c.is_whitespace()) {
        pos += 1;
    }

    // 마지막 문자 위치로 (끝이 아니라 마지막 문자)
    if pos > 0 {
        pos -= 1;
    }

    self.cursor_col = pos;
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_word_
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat: Implement word navigation (w/b/e)"
```

---

## Task 8: 검색 시스템 구현

**Files:**
- Modify: `src/model.rs:128-196`
- Test: `tests/model_tests.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_search_finds_all_matches() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec![
        "Hello world".to_string(),
        "World of Rust".to_string(),
        "world again".to_string(),
    ];
    state.search_pattern = "world".to_string();

    state.execute_search();

    assert_eq!(state.search_matches.len(), 2); // 대소문자 구분
    assert_eq!(state.search_matches[0], (0, 6));
    assert_eq!(state.search_matches[1], (2, 0));
}

#[test]
fn test_search_next_wraps() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["test test test".to_string()];
    state.search_pattern = "test".to_string();
    state.execute_search();

    assert_eq!(state.current_match_index, 0);

    state.search_next();
    assert_eq!(state.current_match_index, 1);

    state.search_next();
    assert_eq!(state.current_match_index, 2);

    state.search_next(); // wrap around
    assert_eq!(state.current_match_index, 0);
}

#[test]
fn test_search_prev_wraps() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut state = EditorState::new(date);
    state.content = vec!["test test test".to_string()];
    state.search_pattern = "test".to_string();
    state.execute_search();

    state.search_prev(); // wrap around to end
    assert_eq!(state.current_match_index, 2);
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_search_
```

Expected: FAIL - methods not found

**Step 3: Write minimal implementation**

`src/model.rs`의 `impl EditorState`에 추가:

```rust
pub fn execute_search(&mut self) {
    if self.search_pattern.is_empty() {
        return;
    }

    // 전체 문서에서 검색
    self.search_matches.clear();
    for (line_idx, line) in self.content.iter().enumerate() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(&self.search_pattern) {
            self.search_matches.push((line_idx, start + pos));
            start += pos + 1;
        }
    }

    // 현재 커서 이후 첫 매치로 이동
    if !self.search_matches.is_empty() {
        self.current_match_index = self
            .search_matches
            .iter()
            .position(|(line, col)| {
                *line > self.cursor_line
                    || (*line == self.cursor_line && *col >= self.cursor_col)
            })
            .unwrap_or(0);

        let (line, col) = self.search_matches[self.current_match_index];
        self.cursor_line = line;
        self.cursor_col = col;

        // 검색어 길이만큼 선택
        self.selection = Some(Selection {
            anchor_line: line,
            anchor_col: col,
            cursor_line: line,
            cursor_col: col + self.search_pattern.len(),
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

    // 검색어 선택
    self.selection = Some(Selection {
        anchor_line: line,
        anchor_col: col,
        cursor_line: line,
        cursor_col: col + self.search_pattern.len(),
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

    // 검색어 선택
    self.selection = Some(Selection {
        anchor_line: line,
        anchor_col: col,
        cursor_line: line,
        cursor_col: col + self.search_pattern.len(),
    });
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_search_
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/model.rs tests/model_tests.rs
git commit -m "feat: Implement search system with next/prev navigation"
```

---

## Task 9: 키 핸들러 재작성 - 에디터 Normal 모드

**Files:**
- Modify: `src/main.rs:74-128`
- Test: 수동 테스트

**Step 1: Manual test plan**

다음 키들이 올바르게 동작하는지 확인:
- h/j/k/l (이동)
- w/b/e (단어 이동)
- g (goto 모드 진입)
- v/x (선택)
- i/a/o/O (insert 모드)
- Space (명령 모드)
- / (검색 모드)

**Step 2: Verify current implementation doesn't work**

기존 handle_editor_key는 Vi 스타일이므로 Helix 키들이 동작하지 않음

**Step 3: Write implementation**

`src/main.rs`의 `handle_editor_key` 함수를 완전히 재작성:

```rust
fn handle_editor_key(key: KeyEvent, state: &ratatui_diary::model::EditorState) -> Option<Msg> {
    use ratatui_diary::model::{EditorMode, EditorSubMode};

    match state.mode {
        EditorMode::Normal => handle_editor_normal_key(key, state),
        EditorMode::Insert => handle_editor_insert_key(key),
    }
}

fn handle_editor_normal_key(
    key: KeyEvent,
    state: &ratatui_diary::model::EditorState,
) -> Option<Msg> {
    use ratatui_diary::model::EditorSubMode;

    // 서브모드 처리
    match &state.submode {
        Some(EditorSubMode::Goto) => {
            return match key.code {
                KeyCode::Char('g') => Some(Msg::EditorGotoDocStart),
                KeyCode::Char('e') => Some(Msg::EditorGotoDocEnd),
                KeyCode::Char('h') => Some(Msg::EditorGotoLineStart),
                KeyCode::Char('l') => Some(Msg::EditorGotoLineEnd),
                KeyCode::Esc => Some(Msg::EditorExitSubMode),
                _ => None,
            };
        }
        Some(EditorSubMode::SpaceCommand) => {
            return match key.code {
                KeyCode::Char('w') => Some(Msg::EditorSpaceSave),
                KeyCode::Char('q') => Some(Msg::EditorSpaceQuit),
                KeyCode::Char('x') => Some(Msg::EditorSpaceSaveQuit),
                KeyCode::Esc => Some(Msg::EditorExitSubMode),
                _ => None,
            };
        }
        Some(EditorSubMode::Search) => {
            return match key.code {
                KeyCode::Char(c) => Some(Msg::EditorSearchChar(c)),
                KeyCode::Enter => Some(Msg::EditorExecuteSearch),
                KeyCode::Esc => Some(Msg::EditorExitSubMode),
                KeyCode::Backspace => Some(Msg::EditorSearchBackspace),
                _ => None,
            };
        }
        None => {}
    }

    // Normal 모드 키
    match key.code {
        // 이동
        KeyCode::Char('h') => Some(Msg::EditorMoveLeft),
        KeyCode::Char('l') => Some(Msg::EditorMoveRight),
        KeyCode::Char('k') => Some(Msg::EditorMoveUp),
        KeyCode::Char('j') => Some(Msg::EditorMoveDown),
        KeyCode::Char('w') => Some(Msg::EditorWordNext),
        KeyCode::Char('b') => Some(Msg::EditorWordPrev),
        KeyCode::Char('e') => Some(Msg::EditorWordEnd),

        // 서브모드 진입
        KeyCode::Char('g') => Some(Msg::EditorEnterGotoMode),
        KeyCode::Char(' ') => Some(Msg::EditorEnterSpaceMode),
        KeyCode::Char('/') => Some(Msg::EditorEnterSearchMode),

        // Insert
        KeyCode::Char('i') => {
            Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::BeforeCursor))
        }
        KeyCode::Char('a') => {
            Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::AfterCursor))
        }
        KeyCode::Char('o') => {
            Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::LineBelow))
        }
        KeyCode::Char('O') => {
            Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::LineAbove))
        }

        // Selection
        KeyCode::Char('v') => Some(Msg::EditorToggleSelection),
        KeyCode::Char('x') => Some(Msg::EditorSelectLine),

        // 편집
        KeyCode::Char('d') => Some(Msg::EditorDelete),
        KeyCode::Char('c') => Some(Msg::EditorChange),
        KeyCode::Char('y') => Some(Msg::EditorYank),
        KeyCode::Char('p') => Some(Msg::EditorPasteAfter),
        KeyCode::Char('P') => Some(Msg::EditorPasteBefore),

        // Undo/Redo
        KeyCode::Char('u') => Some(Msg::EditorUndo),
        KeyCode::Char('U') => Some(Msg::EditorRedo),

        // 검색 네비게이션
        KeyCode::Char('n') => Some(Msg::EditorSearchNext),
        KeyCode::Char('N') => Some(Msg::EditorSearchPrev),

        // 기타
        KeyCode::Esc => Some(Msg::EditorBack),
        _ => None,
    }
}

fn handle_editor_insert_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc => Some(Msg::EditorEnterNormalMode),
        KeyCode::Char(c) => Some(Msg::EditorInsertChar(c)),
        KeyCode::Backspace => Some(Msg::EditorBackspace),
        KeyCode::Enter => Some(Msg::EditorNewLine),
        _ => None,
    }
}
```

**Step 4: Manual test**

```bash
cargo run
```

에디터에서 각 키가 올바르게 동작하는지 확인

**Step 5: Commit**

```bash
git add src/main.rs
git commit -m "refactor: Rewrite editor key handlers for Helix style"
```

---

## Task 10: 달력 키 핸들러 재작성

**Files:**
- Modify: `src/main.rs:88-100`
- Test: 수동 테스트

**Step 1: Manual test plan**

- h/j/k/l (날짜 이동)
- Space (명령 모드)
- Space + n/p/y/Y (월/년 이동)
- Space + q (종료)

**Step 2: Write implementation**

`src/main.rs`의 `handle_calendar_key` 함수 재작성:

```rust
fn handle_calendar_key(
    key: KeyEvent,
    state: &ratatui_diary::model::CalendarState,
) -> Option<Msg> {
    use ratatui_diary::model::CalendarSubMode;

    // Space 서브모드 처리
    if let Some(CalendarSubMode::Space) = state.submode {
        return match key.code {
            KeyCode::Char('q') => Some(Msg::Quit),
            KeyCode::Char('n') => Some(Msg::CalendarSpaceNextMonth),
            KeyCode::Char('p') => Some(Msg::CalendarSpacePrevMonth),
            KeyCode::Char('y') => Some(Msg::CalendarSpaceNextYear),
            KeyCode::Char('Y') => Some(Msg::CalendarSpacePrevYear),
            KeyCode::Esc => Some(Msg::CalendarExitSubMode),
            _ => None,
        };
    }

    // Normal 키
    match key.code {
        KeyCode::Char('h') => Some(Msg::CalendarMoveLeft),
        KeyCode::Char('l') => Some(Msg::CalendarMoveRight),
        KeyCode::Char('k') => Some(Msg::CalendarMoveUp),
        KeyCode::Char('j') => Some(Msg::CalendarMoveDown),
        KeyCode::Enter => Some(Msg::CalendarSelectDate),
        KeyCode::Char(' ') => Some(Msg::CalendarEnterSpaceMode),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}
```

**Step 3: Manual test**

```bash
cargo run
```

달력에서 각 키가 올바르게 동작하는지 확인

**Step 4: Commit**

```bash
git add src/main.rs
git commit -m "refactor: Rewrite calendar key handler for Helix style"
```

---

## Task 11: Update 함수에 모든 메시지 핸들러 추가

**Files:**
- Modify: `src/update.rs:14-167`
- Test: 통합 테스트

**Step 1: Write the test**

```rust
// tests/integration_tests.rs (새 파일)
use ratatui_diary::{Model, Msg, message::InsertPosition};
use ratatui_diary::model::{Screen, EditorMode};
use ratatui_diary::storage::Storage;
use std::collections::HashSet;
use chrono::NaiveDate;

#[test]
fn test_helix_workflow() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let mut model = Model::new(HashSet::new(), Storage::new().unwrap());
    model.screen = Screen::Editor;
    model.editor_state.date = date;

    // Insert 모드로 진입
    ratatui_diary::update::update(&mut model, Msg::EditorEnterInsert(InsertPosition::BeforeCursor));
    assert_eq!(model.editor_state.mode, EditorMode::Insert);

    // 텍스트 입력
    for ch in "Hello World".chars() {
        ratatui_diary::update::update(&mut model, Msg::EditorInsertChar(ch));
    }
    assert_eq!(model.editor_state.content[0], "Hello World");

    // Normal 모드로
    ratatui_diary::update::update(&mut model, Msg::EditorEnterNormalMode);
    assert_eq!(model.editor_state.mode, EditorMode::Normal);

    // 줄 선택
    ratatui_diary::update::update(&mut model, Msg::EditorSelectLine);
    assert!(model.editor_state.selection.is_some());

    // 복사
    ratatui_diary::update::update(&mut model, Msg::EditorYank);
    assert_eq!(model.editor_state.clipboard, "Hello World");

    // Undo
    ratatui_diary::update::update(&mut model, Msg::EditorUndo);
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_helix_workflow
```

Expected: FAIL - 대부분의 메시지가 처리되지 않음

**Step 3: Write implementation**

`src/update.rs`의 `update` 함수에 모든 메시지 핸들러 추가:

```rust
use crate::{
    message::{InsertPosition, Msg},
    model::{CalendarSubMode, EditorMode, EditorState, EditorSubMode, Model, Screen, Selection},
};
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

        // ===== 달력 메시지 =====
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
        Msg::CalendarSelectDate => {
            if model.screen == Screen::Calendar {
                let date = model.calendar_state.selected_date;
                model.screen = Screen::Editor;
                model.editor_state.date = date;
                return Some(Command::LoadDiary(date));
            }
        }
        Msg::CalendarEnterSpaceMode => {
            if model.screen == Screen::Calendar {
                model.calendar_state.submode = Some(CalendarSubMode::Space);
            }
        }
        Msg::CalendarExitSubMode => {
            if model.screen == Screen::Calendar {
                model.calendar_state.submode = None;
            }
        }
        Msg::CalendarSpaceQuit => {
            model.calendar_state.submode = None;
            // Quit는 main에서 처리
        }
        Msg::CalendarSpaceNextMonth => {
            model.calendar_state.submode = None;
            model.calendar_state.next_month();
        }
        Msg::CalendarSpacePrevMonth => {
            model.calendar_state.submode = None;
            model.calendar_state.prev_month();
        }
        Msg::CalendarSpaceNextYear => {
            model.calendar_state.submode = None;
            model.calendar_state.next_year();
        }
        Msg::CalendarSpacePrevYear => {
            model.calendar_state.submode = None;
            model.calendar_state.prev_year();
        }

        // ===== 에디터 - 네비게이션 =====
        Msg::EditorMoveLeft => {
            if model.screen == Screen::Editor && model.editor_state.cursor_col > 0 {
                model.editor_state.cursor_col -= 1;
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorMoveRight => {
            if model.screen == Screen::Editor {
                let line_len = model.editor_state.content[model.editor_state.cursor_line].len();
                if model.editor_state.cursor_col < line_len {
                    model.editor_state.cursor_col += 1;
                    update_selection_on_move(&mut model.editor_state);
                }
            }
        }
        Msg::EditorMoveUp => {
            if model.screen == Screen::Editor && model.editor_state.cursor_line > 0 {
                model.editor_state.cursor_line -= 1;
                let line_len = model.editor_state.content[model.editor_state.cursor_line].len();
                model.editor_state.cursor_col = model.editor_state.cursor_col.min(line_len);
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorMoveDown => {
            if model.screen == Screen::Editor
                && model.editor_state.cursor_line < model.editor_state.content.len() - 1
            {
                model.editor_state.cursor_line += 1;
                let line_len = model.editor_state.content[model.editor_state.cursor_line].len();
                model.editor_state.cursor_col = model.editor_state.cursor_col.min(line_len);
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorWordNext => {
            if model.screen == Screen::Editor {
                model.editor_state.move_word_next();
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorWordPrev => {
            if model.screen == Screen::Editor {
                model.editor_state.move_word_prev();
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorWordEnd => {
            if model.screen == Screen::Editor {
                model.editor_state.move_word_end();
                update_selection_on_move(&mut model.editor_state);
            }
        }

        // ===== 에디터 - Goto 모드 =====
        Msg::EditorEnterGotoMode => {
            if model.screen == Screen::Editor {
                model.editor_state.submode = Some(EditorSubMode::Goto);
            }
        }
        Msg::EditorGotoDocStart => {
            if model.screen == Screen::Editor {
                model.editor_state.cursor_line = 0;
                model.editor_state.cursor_col = 0;
                model.editor_state.submode = None;
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorGotoDocEnd => {
            if model.screen == Screen::Editor {
                let last_line = model.editor_state.content.len().saturating_sub(1);
                model.editor_state.cursor_line = last_line;
                model.editor_state.cursor_col = model.editor_state.content[last_line].len();
                model.editor_state.submode = None;
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorGotoLineStart => {
            if model.screen == Screen::Editor {
                model.editor_state.cursor_col = 0;
                model.editor_state.submode = None;
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorGotoLineEnd => {
            if model.screen == Screen::Editor {
                let line_len = model.editor_state.content[model.editor_state.cursor_line].len();
                model.editor_state.cursor_col = line_len;
                model.editor_state.submode = None;
                update_selection_on_move(&mut model.editor_state);
            }
        }
        Msg::EditorExitSubMode => {
            if model.screen == Screen::Editor {
                model.editor_state.submode = None;
            }
        }

        // ===== 에디터 - Insert 모드 =====
        Msg::EditorEnterInsert(pos) => {
            if model.screen == Screen::Editor
                && model.editor_state.mode == EditorMode::Normal
            {
                match pos {
                    InsertPosition::BeforeCursor => {
                        // 커서 위치 유지
                    }
                    InsertPosition::AfterCursor => {
                        let line_len =
                            model.editor_state.content[model.editor_state.cursor_line].len();
                        if model.editor_state.cursor_col < line_len {
                            model.editor_state.cursor_col += 1;
                        }
                    }
                    InsertPosition::LineBelow => {
                        model.editor_state.cursor_line += 1;
                        model
                            .editor_state
                            .content
                            .insert(model.editor_state.cursor_line, String::new());
                        model.editor_state.cursor_col = 0;
                    }
                    InsertPosition::LineAbove => {
                        model
                            .editor_state
                            .content
                            .insert(model.editor_state.cursor_line, String::new());
                        model.editor_state.cursor_col = 0;
                    }
                }
                model.editor_state.mode = EditorMode::Insert;
            }
        }
        Msg::EditorEnterNormalMode => {
            if model.screen == Screen::Editor {
                model.editor_state.mode = EditorMode::Normal;
                model.editor_state.save_snapshot(); // Insert 종료 시 스냅샷
            }
        }
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

        // ===== 에디터 - Selection =====
        Msg::EditorToggleSelection => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                if state.selection.is_some() {
                    state.selection = None;
                } else {
                    state.selection = Some(Selection {
                        anchor_line: state.cursor_line,
                        anchor_col: state.cursor_col,
                        cursor_line: state.cursor_line,
                        cursor_col: state.cursor_col,
                    });
                }
            }
        }
        Msg::EditorSelectLine => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                let line_len = if state.cursor_line < state.content.len() {
                    state.content[state.cursor_line].len()
                } else {
                    0
                };
                state.selection = Some(Selection {
                    anchor_line: state.cursor_line,
                    anchor_col: 0,
                    cursor_line: state.cursor_line,
                    cursor_col: line_len,
                });
            }
        }

        // ===== 에디터 - 편집 =====
        Msg::EditorDelete => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                ensure_selection_for_edit(state);
                if let Some(text) = state.get_selected_text() {
                    state.clipboard = text;
                }
                state.save_snapshot();
                state.delete_selection();
                state.selection = None;
                state.is_modified = true;
            }
        }
        Msg::EditorChange => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                ensure_selection_for_edit(state);
                if let Some(text) = state.get_selected_text() {
                    state.clipboard = text;
                }
                state.save_snapshot();
                state.delete_selection();
                state.selection = None;
                state.mode = EditorMode::Insert;
                state.is_modified = true;
            }
        }
        Msg::EditorYank => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                ensure_selection_for_edit(state);
                if let Some(text) = state.get_selected_text() {
                    state.clipboard = text;
                }
                state.selection = None;
            }
        }
        Msg::EditorPasteAfter => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                if !state.clipboard.is_empty() {
                    state.save_snapshot();
                    paste_clipboard(state, false);
                    state.is_modified = true;
                }
            }
        }
        Msg::EditorPasteBefore => {
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                if !state.clipboard.is_empty() {
                    state.save_snapshot();
                    paste_clipboard(state, true);
                    state.is_modified = true;
                }
            }
        }

        // ===== 에디터 - Undo/Redo =====
        Msg::EditorUndo => {
            if model.screen == Screen::Editor {
                model.editor_state.undo();
            }
        }
        Msg::EditorRedo => {
            if model.screen == Screen::Editor {
                model.editor_state.redo();
            }
        }

        // ===== 에디터 - 검색 =====
        Msg::EditorEnterSearchMode => {
            if model.screen == Screen::Editor {
                model.editor_state.submode = Some(EditorSubMode::Search);
                model.editor_state.search_pattern.clear();
                model.editor_state.search_matches.clear();
            }
        }
        Msg::EditorSearchChar(c) => {
            if let Some(EditorSubMode::Search) = model.editor_state.submode {
                model.editor_state.search_pattern.push(c);
            }
        }
        Msg::EditorSearchBackspace => {
            if let Some(EditorSubMode::Search) = model.editor_state.submode {
                model.editor_state.search_pattern.pop();
            }
        }
        Msg::EditorExecuteSearch => {
            if model.screen == Screen::Editor {
                model.editor_state.submode = None;
                model.editor_state.execute_search();
            }
        }
        Msg::EditorSearchNext => {
            if model.screen == Screen::Editor {
                model.editor_state.search_next();
            }
        }
        Msg::EditorSearchPrev => {
            if model.screen == Screen::Editor {
                model.editor_state.search_prev();
            }
        }

        // ===== 에디터 - Space 명령 =====
        Msg::EditorEnterSpaceMode => {
            if model.screen == Screen::Editor {
                model.editor_state.submode = Some(EditorSubMode::SpaceCommand);
            }
        }
        Msg::EditorSpaceSave => {
            model.editor_state.submode = None;
            let date = model.editor_state.date;
            let content = model.editor_state.get_content();
            return Some(Command::SaveDiary(date, content));
        }
        Msg::EditorSpaceQuit => {
            model.editor_state.submode = None;
            let date = model.editor_state.date;
            model.screen = Screen::Calendar;
            model.editor_state = EditorState::new(date);
        }
        Msg::EditorSpaceSaveQuit => {
            model.editor_state.submode = None;
            let date = model.editor_state.date;
            let content = model.editor_state.get_content();
            model.screen = Screen::Calendar;
            model.editor_state = EditorState::new(date);
            return Some(Command::SaveDiary(date, content));
        }
        Msg::EditorBack => {
            if model.screen == Screen::Editor {
                model.screen = Screen::Calendar;
            }
        }

        // ===== 파일 I/O 결과 =====
        Msg::LoadDiarySuccess(date, content) => {
            if model.screen == Screen::Editor {
                model.editor_state.date = date;
                model.editor_state.load_content(&content);
            }
        }
        Msg::LoadDiaryFailed(error) => {
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
    }

    None
}

// 헬퍼 함수들
fn update_selection_on_move(state: &mut EditorState) {
    if let Some(sel) = &mut state.selection {
        sel.cursor_line = state.cursor_line;
        sel.cursor_col = state.cursor_col;
    }
}

fn ensure_selection_for_edit(state: &mut EditorState) {
    if state.selection.is_none() {
        // 현재 줄을 암묵적으로 선택
        let line_len = if state.cursor_line < state.content.len() {
            state.content[state.cursor_line].len()
        } else {
            0
        };
        state.selection = Some(Selection {
            anchor_line: state.cursor_line,
            anchor_col: 0,
            cursor_line: state.cursor_line,
            cursor_col: line_len,
        });
    }
}

fn paste_clipboard(state: &mut EditorState, before: bool) {
    if state.clipboard.contains('\n') {
        // 줄 단위
        let lines: Vec<String> = state.clipboard.lines().map(String::from).collect();
        let insert_line = if before {
            state.cursor_line
        } else {
            state.cursor_line + 1
        };
        for (i, line) in lines.iter().enumerate() {
            state.content.insert(insert_line + i, line.clone());
        }
    } else {
        // 문자 단위
        let insert_col = if before {
            state.cursor_col
        } else {
            state.cursor_col + 1
        };
        state.content[state.cursor_line].insert_str(insert_col, &state.clipboard);
        state.cursor_col = insert_col + state.clipboard.len() - 1;
    }
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_helix_workflow
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/update.rs tests/integration_tests.rs
git commit -m "feat: Implement all Helix message handlers in update function"
```

---

## Task 12: View에 선택 및 검색 하이라이트 추가

**Files:**
- Modify: `src/view.rs:1-300`
- Test: 수동 테스트

**Step 1: Manual test plan**

- 선택 영역이 반전 색상으로 표시되는지 확인
- 검색 결과가 하이라이트되는지 확인
- 서브모드 상태가 화면 하단에 표시되는지 확인

**Step 2: Write implementation**

`src/view.rs` 수정 (에디터 뷰 부분):

기존 에디터 렌더링 로직에 선택 하이라이트 추가. 정확한 위치는 기존 코드를 읽어야 하므로 생략하고, 개념만 제시:

```rust
// 선택 영역 계산
let selection_range = if let Some(sel) = &model.editor_state.selection {
    model.editor_state.get_selection_range()
} else {
    None
};

// 검색 매치 표시
let search_matches = &model.editor_state.search_matches;
let current_match = model.editor_state.current_match_index;

// 각 라인 렌더링 시:
// - selection_range에 포함되면 반전 스타일
// - search_matches에 포함되면 하이라이트
// - current_match이면 강조 하이라이트

// 화면 하단에 서브모드 표시
let status_line = match &model.editor_state.submode {
    Some(EditorSubMode::Goto) => "-- GOTO --",
    Some(EditorSubMode::SpaceCommand) => "[w] save  [q] quit  [x] save & quit",
    Some(EditorSubMode::Search) => {
        let pattern = &model.editor_state.search_pattern;
        format!("/{}", pattern)
    }
    None => {
        if model.editor_state.mode == EditorMode::Insert {
            "-- INSERT --"
        } else {
            "-- NORMAL --"
        }
    }
};
```

**Step 3: Manual test**

```bash
cargo run
```

선택, 검색, 서브모드 표시가 올바른지 확인

**Step 4: Commit**

```bash
git add src/view.rs
git commit -m "feat: Add selection and search highlighting to view"
```

---

## Task 13: README 업데이트

**Files:**
- Modify: `README.md:1-83`

**Step 1: Write new content**

기존 README의 키바인딩 섹션을 Helix 스타일로 업데이트:

```markdown
### 달력 화면

| 키 | 동작 |
|---|---|
| `h/j/k/l` | 날짜 이동 |
| `Enter` | 다이어리 작성/편집 |
| `Space` | 명령 모드 |
| `Space + q` | 종료 |
| `Space + n` | 다음 달 |
| `Space + p` | 이전 달 |
| `Space + y` | 다음 해 |
| `Space + Y` | 이전 해 |
| `q` | 종료 |

### 에디터 화면

**Normal 모드:**

*이동:*
- `h/j/k/l`: 기본 커서 이동
- `w/b/e`: 단어 이동 (다음/이전/끝)
- `gg`: 문서 시작
- `ge`: 문서 끝
- `gh`: 줄 시작
- `gl`: 줄 끝

*선택:*
- `v`: 선택 토글
- `x`: 현재 줄 선택

*편집:*
- `i`: 커서 앞에서 삽입
- `a`: 커서 뒤에서 삽입
- `o`: 아래 새 줄 생성 후 삽입
- `O`: 위 새 줄 생성 후 삽입
- `d`: 선택 영역 삭제 (선택 없으면 현재 줄)
- `c`: 선택 영역 삭제 후 Insert 모드
- `y`: 선택 영역 복사
- `p`: 커서 뒤에 붙여넣기
- `P`: 커서 앞에 붙여넣기

*실행 취소:*
- `u`: Undo
- `U`: Redo

*검색:*
- `/`: 검색 모드
- `n`: 다음 검색 결과
- `N`: 이전 검색 결과

*명령:*
- `Space + w`: 저장
- `Space + q`: 나가기
- `Space + x`: 저장 후 나가기

*기타:*
- `Esc`: 달력으로 돌아가기

**Insert 모드:**
- 텍스트 입력
- `Esc`: Normal 모드
```

**Step 2: Apply changes**

```bash
# README.md 수정
```

**Step 3: Commit**

```bash
git add README.md
git commit -m "docs: Update README with Helix keybindings"
```

---

## Task 14: 전체 통합 테스트 및 버그 수정

**Files:**
- Test: 모든 기능
- Fix: 발견된 버그

**Step 1: Comprehensive manual testing**

테스트 체크리스트:
- [ ] 달력 네비게이션 (h/j/k/l, Space 명령)
- [ ] 에디터 진입 및 텍스트 입력
- [ ] 선택 (v/x) 및 시각적 피드백
- [ ] 편집 (d/c/y/p)
- [ ] Undo/Redo 여러 번
- [ ] 단어 이동 (w/b/e)
- [ ] Goto (gg/ge/gh/gl)
- [ ] 검색 (/n/N)
- [ ] Space 명령 (저장/종료)
- [ ] Insert 모드 (i/a/o/O)
- [ ] 긴 문서 (100줄 이상)
- [ ] 빈 문서
- [ ] 에러 처리

**Step 2: Fix bugs**

발견된 버그를 각각 수정하고 커밋

**Step 3: Run all tests**

```bash
cargo test
cargo run
```

**Step 4: Final commit**

```bash
git add .
git commit -m "fix: Address bugs found in integration testing"
```

---

## 완료

모든 태스크가 완료되면:

```bash
# 최종 테스트
cargo test
cargo build --release

# 변경사항 확인
git log --oneline
git diff origin/main
```

구현 완료!
