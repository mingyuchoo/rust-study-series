# Helix 키바인딩 디자인

**작성일**: 2026-02-14
**목적**: Ratatui Diary 앱의 키바인딩을 Helix Editor 스타일로 전환

## 개요

현재 Vi 스타일 키바인딩을 Helix Editor의 "selection-action" 모델로 전환하여 더 직관적이고 강력한 편집 경험을 제공합니다.

## 요구사항

### 구현 범위

1. **핵심 편집 키바인딩**
   - 기본 이동: h/j/k/l, w/b/e (단어 이동), g 계열 (goto)
   - 편집: i/a/o/O (삽입), d (삭제), c (변경), y/p (복사/붙여넣기)
   - 실행 취소: u/U (undo/redo)
   - 선택: v (select), x (select line)

2. **기본 선택 기능**
   - v: 문자 단위 선택 모드
   - x: 현재 줄 선택
   - d/c/y: 선택 영역에 대해 동작
   - 선택 없이 동작 시 현재 라인이 암묵적 선택

3. **Helix 스타일 명령**
   - `:` Command 모드 제거
   - Space 키 기반 명령으로 대체
   - 에디터: Space + w/q/x
   - 달력: Space + q/n/p/y/Y

4. **검색 기능**
   - `/`: 검색 모드 진입
   - `n`: 다음 검색 결과
   - `N`: 이전 검색 결과

5. **전체 앱 통일**
   - 달력과 에디터 모두 Helix 스타일로 일관성 유지

## 아키텍처 개요

### 유지되는 부분

- ELM 패턴 (Model-Update-View)
- Storage 시스템
- 화면 구조 (Calendar/Editor)
- View 렌더링 기본 구조

### 변경되는 부분

1. **EditorState 확장**
   - selection, edit_history, clipboard 필드 추가
   - submode 필드로 Goto, Space, Search 모드 관리

2. **메시지 타입 재구성**
   - Helix 키바인딩에 맞게 Msg enum 재설계
   - InsertPosition enum 추가

3. **키 핸들링 분리**
   - 서브모드별 키 핸들러 분리
   - 상태 기반 키 해석

4. **상태 관리**
   - 모든 편집 동작이 history에 기록
   - undo/redo 지원

### 핵심 철학

- Helix의 "selection-action" 모델 구현
- 선택이 없으면 현재 커서 위치/라인이 암묵적 선택
- 모든 편집 명령은 선택에 대해 동작

## 상태 구조

### EditorState 확장

```rust
pub struct EditorState {
    pub mode: EditorMode,
    pub date: NaiveDate,
    pub content: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_modified: bool,

    // 새로 추가되는 필드들
    pub selection: Option<Selection>,      // 선택 영역
    pub edit_history: Vec<EditorSnapshot>, // undo/redo 히스토리
    pub history_index: usize,              // 현재 히스토리 위치
    pub clipboard: String,                 // 내부 클립보드
    pub submode: Option<EditorSubMode>,    // Goto, Space 등
    pub search_pattern: String,            // 검색어
    pub search_matches: Vec<(usize, usize)>, // 검색 결과 위치들
    pub current_match_index: usize,        // 현재 검색 매치 인덱스
}

pub struct Selection {
    pub anchor_line: usize,    // 선택 시작 줄
    pub anchor_col: usize,     // 선택 시작 열
    pub cursor_line: usize,    // 현재 커서 줄 (끝)
    pub cursor_col: usize,     // 현재 커서 열 (끝)
}

pub struct EditorSnapshot {
    pub content: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub selection: Option<Selection>,
}

pub enum EditorSubMode {
    Goto,           // g 눌렀을 때
    SpaceCommand,   // Space 눌렀을 때
    Search,         // / 눌렀을 때 (입력 중)
}
```

### EditorMode 변경

```rust
pub enum EditorMode {
    Normal,
    Insert,
    // Command 모드 제거 (Space 명령으로 대체)
}
```

### CalendarState 확장

```rust
pub struct CalendarState {
    pub current_year: i32,
    pub current_month: u32,
    pub selected_date: NaiveDate,
    pub cursor_pos: usize,
    pub submode: Option<CalendarSubMode>, // 새로 추가
}

pub enum CalendarSubMode {
    Space, // Space 명령 대기 중
}
```

## 메시지 타입 설계

```rust
pub enum Msg {
    Quit,
    DismissError,

    // 에디터 - 네비게이션
    EditorMoveLeft,
    EditorMoveRight,
    EditorMoveUp,
    EditorMoveDown,
    EditorWordNext,        // w
    EditorWordPrev,        // b
    EditorWordEnd,         // e

    // 에디터 - Goto 모드
    EditorEnterGotoMode,   // g 키
    EditorGotoDocStart,    // gg
    EditorGotoDocEnd,      // ge
    EditorGotoLineStart,   // gh
    EditorGotoLineEnd,     // gl
    EditorExitSubMode,     // Esc

    // 에디터 - Insert 모드
    EditorEnterInsert(InsertPosition),
    EditorEnterNormalMode,
    EditorInsertChar(char),
    EditorBackspace,
    EditorNewLine,

    // 에디터 - Selection
    EditorToggleSelection, // v
    EditorSelectLine,      // x

    // 에디터 - 편집
    EditorDelete,          // d
    EditorChange,          // c
    EditorYank,            // y
    EditorPasteAfter,      // p
    EditorPasteBefore,     // P

    // 에디터 - Undo/Redo
    EditorUndo,            // u
    EditorRedo,            // U

    // 에디터 - 검색
    EditorEnterSearchMode, // /
    EditorSearchChar(char),
    EditorSearchBackspace,
    EditorSearchNext,      // n
    EditorSearchPrev,      // N
    EditorExecuteSearch,

    // 에디터 - Space 명령
    EditorEnterSpaceMode,  // Space
    EditorSpaceSave,       // Space + w
    EditorSpaceQuit,       // Space + q
    EditorSpaceSaveQuit,   // Space + x

    // 달력 - 기존 + Space 명령
    CalendarMoveLeft,      // h
    CalendarMoveRight,     // l
    CalendarMoveUp,        // k
    CalendarMoveDown,      // j
    CalendarSelectDate,    // Enter
    CalendarEnterSpaceMode, // Space
    CalendarExitSubMode,   // Esc
    CalendarSpaceQuit,     // Space + q
    CalendarSpaceNextMonth,  // Space + n
    CalendarSpacePrevMonth,  // Space + p
    CalendarSpaceNextYear,   // Space + y
    CalendarSpacePrevYear,   // Space + Y

    // 파일 I/O 결과 (기존 유지)
    LoadDiarySuccess(NaiveDate, String),
    LoadDiaryFailed(String),
    SaveDiarySuccess,
    SaveDiaryFailed(String),
    DeleteDiarySuccess(NaiveDate),
    RefreshIndex(HashSet<NaiveDate>),
}

pub enum InsertPosition {
    BeforeCursor,  // i
    AfterCursor,   // a
    LineBelow,     // o
    LineAbove,     // O
}
```

## 키바인딩 매핑

### 달력 키 핸들러

```rust
fn handle_calendar_key(key: KeyEvent, state: &CalendarState) -> Option<Msg> {
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

### 에디터 키 핸들러

```rust
fn handle_editor_key(key: KeyEvent, state: &EditorState) -> Option<Msg> {
    match state.mode {
        EditorMode::Normal => handle_editor_normal_key(key, state),
        EditorMode::Insert => handle_editor_insert_key(key),
    }
}

fn handle_editor_normal_key(key: KeyEvent, state: &EditorState) -> Option<Msg> {
    // 서브모드 처리
    match state.submode {
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
        KeyCode::Char('i') => Some(Msg::EditorEnterInsert(InsertPosition::BeforeCursor)),
        KeyCode::Char('a') => Some(Msg::EditorEnterInsert(InsertPosition::AfterCursor)),
        KeyCode::Char('o') => Some(Msg::EditorEnterInsert(InsertPosition::LineBelow)),
        KeyCode::Char('O') => Some(Msg::EditorEnterInsert(InsertPosition::LineAbove)),

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

## 선택(Selection) 시스템

### 핵심 동작

1. **선택 시작 (v)**
   - selection이 None이면 현재 커서를 anchor로 설정
   - selection이 Some이면 해제

2. **줄 선택 (x)**
   - 현재 줄 전체를 선택 (0 ~ 줄 끝)

3. **선택 확장**
   - 선택 활성 상태에서 이동 키 누르면 anchor 고정, cursor만 이동

4. **암묵적 선택**
   - d/c/y 실행 시 selection이 None이면 현재 줄을 자동 선택

### 주요 메서드

```rust
impl EditorState {
    fn get_selection_range(&self) -> Option<((usize, usize), (usize, usize))> {
        // anchor와 cursor 중 작은 것이 start, 큰 것이 end
        // (start_line, start_col), (end_line, end_col) 반환
    }

    fn get_selected_text(&self) -> Option<String> {
        // 선택 영역의 텍스트 추출
        // 한 줄 vs 여러 줄 처리
    }

    fn delete_selection(&mut self) {
        // 선택 영역 삭제
        // 커서를 삭제 시작 위치로 이동
    }
}
```

## 편집 동작

### Delete (d)

1. 선택이 없으면 현재 줄 선택
2. 히스토리에 스냅샷 저장
3. 선택 텍스트를 클립보드에 저장
4. 선택 영역 삭제
5. 선택 해제

### Change (c)

1. Delete와 동일하게 삭제
2. Insert 모드로 전환

### Yank (y)

1. 선택이 없으면 현재 줄 선택
2. 선택 텍스트를 클립보드에 복사
3. 선택 해제

### Paste After (p)

1. 클립보드 내용 확인
2. 히스토리에 스냅샷 저장
3. 줄 단위 vs 문자 단위 판단 (개행 포함 여부)
4. 줄 단위: 현재 줄 다음에 삽입
5. 문자 단위: 커서 다음에 삽입

### Paste Before (P)

1. p와 유사하지만 현재 위치/줄에 삽입

## Undo/Redo 시스템

### 히스토리 관리

```rust
impl EditorState {
    fn save_snapshot(&mut self) {
        // 현재 index 이후 히스토리 제거 (분기 삭제)
        self.edit_history.truncate(self.history_index + 1);

        // 현재 상태 저장
        let snapshot = EditorSnapshot { ... };
        self.edit_history.push(snapshot);
        self.history_index = self.edit_history.len() - 1;

        // 최대 100개로 제한
        if self.edit_history.len() > 100 {
            self.edit_history.drain(0..1);
            self.history_index -= 1;
        }
    }

    fn restore_snapshot(&mut self, index: usize) {
        // 히스토리의 index 위치 상태로 복원
    }
}
```

### 스냅샷 저장 시점

- Insert 모드에서 Normal 모드로 전환 시
- d/c/p/P 실행 시
- 검색 및 교체 시

### Undo (u)

- history_index를 감소시키고 상태 복원

### Redo (U)

- history_index를 증가시키고 상태 복원

## 검색 시스템

### 검색 프로세스

1. **검색 모드 진입 (/)** - submode를 Search로 설정
2. **검색어 입력** - search_pattern에 문자 추가
3. **검색 실행 (Enter)**
   - 전체 문서에서 패턴 검색
   - 매치 위치를 search_matches에 저장
   - 현재 커서 이후 첫 매치로 이동
   - 매치 영역을 선택으로 표시
4. **다음 검색 (n)** - current_match_index 증가, 순환
5. **이전 검색 (N)** - current_match_index 감소, 순환

### 검색 결과 표시

- View에서 search_matches를 하이라이트
- 현재 매치는 다른 색상
- 화면 하단에 "Match 3/15" 표시

## Space 명령 시스템

### 에디터 명령

- **Space + w**: 저장
- **Space + q**: 종료 (달력으로)
- **Space + x**: 저장 후 종료

### 달력 명령

- **Space + q**: 앱 종료
- **Space + n**: 다음 달
- **Space + p**: 이전 달
- **Space + y**: 다음 해
- **Space + Y**: 이전 해

### UI 표시

Space 모드 진입 시 화면 하단에 사용 가능한 명령 표시:
- 에디터: `[w] save  [q] quit  [x] save & quit`
- 달력: `[q] quit  [n] next month  [p] prev month  [y] next year  [Y] prev year`

## 테스트 전략

### 단위 테스트 (tests/model_tests.rs)

**Selection 테스트**
- test_selection_toggle
- test_selection_line
- test_selection_range_calculation
- test_get_selected_text_single_line
- test_get_selected_text_multi_line

**편집 동작 테스트**
- test_delete_with_selection
- test_delete_without_selection
- test_change_enters_insert_mode
- test_yank_copies_to_clipboard
- test_paste_after
- test_paste_before

**Undo/Redo 테스트**
- test_undo_single_action
- test_redo_after_undo
- test_undo_redo_chain
- test_new_edit_clears_redo_history

**검색 테스트**
- test_search_finds_all_matches
- test_search_next_navigation
- test_search_prev_navigation
- test_search_wraps_around

**단어 이동 테스트**
- test_word_next
- test_word_prev
- test_word_end

**Goto 테스트**
- test_goto_doc_start
- test_goto_doc_end
- test_goto_line_start
- test_goto_line_end

**Insert 모드 테스트**
- test_insert_before_cursor
- test_insert_after_cursor
- test_insert_line_below
- test_insert_line_above

### 통합 테스트 (tests/integration_tests.rs)

**워크플로우 시나리오**
- test_edit_workflow: 입력 → 선택 → 복사 → 이동 → 붙여넣기 → Undo → Redo
- test_search_and_replace_workflow: 검색 → 다음 → 변경 → 입력

### 수동 테스트 체크리스트

- [ ] 모든 키바인딩 동작 확인
- [ ] 선택 영역 시각적 하이라이트
- [ ] Space 명령 UI 표시
- [ ] 검색 결과 하이라이트
- [ ] 긴 문서에서 성능 확인
- [ ] undo/redo 100회 이상
- [ ] 엣지 케이스 (빈 문서, 한 줄 문서, 빈 줄)

### 리그레션 방지

기존 기능 확인:
- [ ] 달력 네비게이션
- [ ] 다이어리 로드/저장
- [ ] Markdown 미리보기
- [ ] 에러 팝업

## 구현 접근 방식

**통합 리팩토링 방식**을 채택합니다:

1. EditorState, CalendarState에 새 필드 추가
2. 모든 Helix 키바인딩을 새로운 메시지 타입으로 정의
3. handle_key 함수 전체 재작성
4. update 로직에서 selection 기반 동작 구현
5. View에 선택/검색 하이라이트 추가
6. 테스트 작성 및 검증

### 이점

- 전체 구조를 처음부터 올바르게 설계
- 기능 간 통합이 자연스러움
- 중복 작업 없음
- Helix 철학을 완전히 구현

## 참고 사항

### Helix vs Vi 철학 차이

- **Vi**: action-motion (동작 후 범위)
- **Helix**: selection-action (선택 후 동작)

### 커서 이동 중 선택 확장

선택이 활성화된 상태에서:
- h/j/k/l/w/b/e 등 이동 키를 누르면
- anchor는 고정, cursor만 이동하여 선택 영역 확장

### 클립보드 관리

- 내부 클립보드 (EditorState.clipboard)만 사용
- 시스템 클립보드 통합은 추후 확장 가능

### 성능 고려사항

- 히스토리 크기를 100개로 제한
- 검색은 단순 문자열 매칭 (정규식은 추후)
- 선택 하이라이트는 화면에 보이는 영역만

## 마이그레이션 가이드

### 기존 사용자를 위한 키바인딩 변경사항

| 기능 | 기존 (Vi) | 새로 (Helix) |
|------|-----------|-------------|
| 저장 | `:w` | `Space + w` |
| 종료 | `:q` | `Space + q` |
| 저장 후 종료 | `:wq` | `Space + x` |
| 줄 삭제 | `dd` | `x` + `d` 또는 그냥 `d` (암묵적) |
| 단어 이동 | 없음 | `w`, `b`, `e` |
| 문서 시작 | 없음 | `gg` |
| 문서 끝 | 없음 | `ge` |
| 줄 시작 | 없음 | `gh` |
| 줄 끝 | 없음 | `gl` |
| 선택 | 없음 | `v`, `x` |
| Undo | 없음 | `u` |
| Redo | 없음 | `U` |
| 검색 | 없음 | `/`, `n`, `N` |

## 결론

이 디자인은 Helix Editor의 핵심 철학인 "selection-action" 모델을 충실히 구현하면서도, 다이어리 앱의 단순함을 유지합니다. 통합 리팩토링 방식을 통해 전체 시스템을 일관성 있게 재구성하고, TDD 접근으로 안정성을 보장합니다.
