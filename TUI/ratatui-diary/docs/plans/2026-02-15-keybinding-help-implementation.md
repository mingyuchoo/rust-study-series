# 키바인딩 도움말 시스템 구현 계획

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 달력과 에디터 화면에 현재 모드/서브모드에 맞는 키바인딩 도움말을 화면 하단에 동적으로 표시

**Architecture:** 순수 View 레이어 기능으로 구현. Model의 상태를 읽어 적절한 키바인딩 텍스트를 생성하는 빌더 함수들을 추가하고, 기존 렌더링 함수에서 이를 호출하여 상태바에 표시.

**Tech Stack:** Rust, ratatui 0.27, TDD with cargo test

---

## Task 1: 달력 키바인딩 빌더 함수 - Normal 모드

**Files:**
- Modify: `src/view.rs` (새 함수 추가)
- Test: `tests/view_tests.rs`

**Step 1: Write the failing test**

`tests/view_tests.rs`에 다음 테스트 추가:

```rust
#[cfg(test)]
mod keybinding_tests {
    use ratatui_diary::model::{CalendarState, CalendarSubMode};
    use ratatui_diary::view::build_calendar_keybindings;

    #[test]
    fn test_build_calendar_keybindings_normal() {
        let state = CalendarState::new(2026, 2);
        let result = build_calendar_keybindings(&state);
        assert_eq!(result, "hjkl:이동 | e:편집 | space:명령 | q:종료");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_build_calendar_keybindings_normal`

Expected: FAIL - "cannot find function `build_calendar_keybindings` in module `ratatui_diary::view`"

**Step 3: Write minimal implementation**

`src/view.rs` 파일 상단 (imports 아래, view 함수 위)에 추가:

```rust
/// 달력 화면의 현재 모드에 맞는 키바인딩 도움말 텍스트 생성
pub fn build_calendar_keybindings(state: &CalendarState) -> String {
    match state.submode {
        None => "hjkl:이동 | e:편집 | space:명령 | q:종료".to_string(),
        Some(CalendarSubMode::Space) => "n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소".to_string(),
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_build_calendar_keybindings_normal`

Expected: PASS

**Step 5: Commit**

```bash
git add src/view.rs tests/view_tests.rs
git commit -m "test: add calendar keybindings builder for normal mode

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: 달력 키바인딩 빌더 함수 - Space 모드

**Files:**
- Test: `tests/view_tests.rs`

**Step 1: Write the failing test**

`tests/view_tests.rs`의 `keybinding_tests` 모듈에 추가:

```rust
#[test]
fn test_build_calendar_keybindings_space_mode() {
    let mut state = CalendarState::new(2026, 2);
    state.submode = Some(CalendarSubMode::Space);
    let result = build_calendar_keybindings(&state);
    assert_eq!(result, "n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소");
}
```

**Step 2: Run test to verify it passes**

Run: `cargo test test_build_calendar_keybindings_space_mode`

Expected: PASS (구현이 이미 Space 모드를 포함하고 있음)

**Step 3: Commit**

```bash
git add tests/view_tests.rs
git commit -m "test: add calendar keybindings test for space mode

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: 에디터 키바인딩 빌더 함수 - Normal 모드

**Files:**
- Modify: `src/view.rs` (새 함수 추가)
- Test: `tests/view_tests.rs`

**Step 1: Write the failing test**

`tests/view_tests.rs`의 `keybinding_tests` 모듈에 추가:

```rust
use ratatui_diary::model::{EditorState, EditorMode, EditorSubMode};
use ratatui_diary::view::build_editor_keybindings;
use chrono::NaiveDate;

#[test]
fn test_build_editor_keybindings_normal() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    let state = EditorState::new(date);
    let result = build_editor_keybindings(&state);
    assert_eq!(result, "hjkl:이동 | w/b/e:단어 | i/a/o/O:입력 | v/x:선택 | d/c/y/p:편집 | u/U:실행취소 | g/space//:모드 | Esc:뒤로");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_build_editor_keybindings_normal`

Expected: FAIL - "cannot find function `build_editor_keybindings` in module `ratatui_diary::view`"

**Step 3: Write minimal implementation**

`src/view.rs`의 `build_calendar_keybindings` 함수 아래에 추가:

```rust
/// 에디터 화면의 현재 모드에 맞는 키바인딩 도움말 텍스트 생성
pub fn build_editor_keybindings(state: &EditorState) -> String {
    match state.mode {
        EditorMode::Normal => {
            match &state.submode {
                None => "hjkl:이동 | w/b/e:단어 | i/a/o/O:입력 | v/x:선택 | d/c/y/p:편집 | u/U:실행취소 | g/space//:모드 | Esc:뒤로".to_string(),
                Some(EditorSubMode::Goto) => "g:문서시작 | e:문서끝 | h:줄시작 | l:줄끝 | Esc:취소".to_string(),
                Some(EditorSubMode::SpaceCommand) => "w:저장 | q:뒤로 | x:저장후뒤로 | Esc:취소".to_string(),
                Some(EditorSubMode::Search) => "입력:검색어 | Enter:실행 | n/N:다음/이전 | Esc:취소".to_string(),
            }
        },
        EditorMode::Insert => "입력중... | Enter:새줄 | Backspace:삭제 | Esc:Normal모드".to_string(),
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_build_editor_keybindings_normal`

Expected: PASS

**Step 5: Commit**

```bash
git add src/view.rs tests/view_tests.rs
git commit -m "feat: add editor keybindings builder for normal mode

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: 에디터 키바인딩 빌더 함수 - 모든 모드 테스트

**Files:**
- Test: `tests/view_tests.rs`

**Step 1: Write failing tests for all editor modes**

`tests/view_tests.rs`의 `keybinding_tests` 모듈에 추가:

```rust
#[test]
fn test_build_editor_keybindings_insert() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    let mut state = EditorState::new(date);
    state.mode = EditorMode::Insert;
    let result = build_editor_keybindings(&state);
    assert_eq!(result, "입력중... | Enter:새줄 | Backspace:삭제 | Esc:Normal모드");
}

#[test]
fn test_build_editor_keybindings_goto() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    let mut state = EditorState::new(date);
    state.submode = Some(EditorSubMode::Goto);
    let result = build_editor_keybindings(&state);
    assert_eq!(result, "g:문서시작 | e:문서끝 | h:줄시작 | l:줄끝 | Esc:취소");
}

#[test]
fn test_build_editor_keybindings_space_command() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    let mut state = EditorState::new(date);
    state.submode = Some(EditorSubMode::SpaceCommand);
    let result = build_editor_keybindings(&state);
    assert_eq!(result, "w:저장 | q:뒤로 | x:저장후뒤로 | Esc:취소");
}

#[test]
fn test_build_editor_keybindings_search() {
    let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    let mut state = EditorState::new(date);
    state.submode = Some(EditorSubMode::Search);
    let result = build_editor_keybindings(&state);
    assert_eq!(result, "입력:검색어 | Enter:실행 | n/N:다음/이전 | Esc:취소");
}
```

**Step 2: Run tests to verify they pass**

Run: `cargo test keybinding_tests`

Expected: 모든 테스트 PASS

**Step 3: Commit**

```bash
git add tests/view_tests.rs
git commit -m "test: add comprehensive editor keybindings tests

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: 달력 렌더링에 동적 키바인딩 통합

**Files:**
- Modify: `src/view.rs:69` (render_calendar 함수의 statusbar 부분)

**Step 1: Modify calendar statusbar to use dynamic keybindings**

`src/view.rs`의 `render_calendar` 함수에서 69번 라인을 찾아 수정:

**Before:**
```rust
// 상태바
let statusbar = Paragraph::new("h/l: 달 | H/L: 연도 | Enter: 작성 | q: 종료").alignment(Alignment::Center);
f.render_widget(statusbar, calendar_chunks[2]);
```

**After:**
```rust
// 상태바 - 동적 키바인딩
let keybindings = build_calendar_keybindings(&model.calendar_state);
let statusbar = Paragraph::new(keybindings).alignment(Alignment::Center);
f.render_widget(statusbar, calendar_chunks[2]);
```

**Step 2: Build and run the application**

Run: `cargo build && cargo run`

Expected: 빌드 성공, 애플리케이션 실행 시 달력 화면 하단에 "hjkl:이동 | e:편집 | space:명령 | q:종료" 표시

**Step 3: Test space mode transition**

1. 애플리케이션 실행
2. Space 키 입력
3. 키바인딩이 "n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소"로 변경되는지 확인
4. Esc 키로 Normal 모드로 복귀 시 원래 키바인딩으로 돌아오는지 확인

Expected: 모드 전환에 따라 키바인딩이 올바르게 변경됨

**Step 4: Commit**

```bash
git add src/view.rs
git commit -m "feat: integrate dynamic keybindings into calendar view

달력 화면의 하드코딩된 상태바를 동적 키바인딩으로 교체
모드 전환 시 자동으로 키바인딩 도움말 업데이트

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: 에디터 렌더링에 동적 키바인딩 통합

**Files:**
- Modify: `src/view.rs:220-223` (render_editor 함수의 statusbar 부분)

**Step 1: Modify editor statusbar to include keybindings**

`src/view.rs`의 `render_editor` 함수에서 220-223번 라인 부근을 찾아 수정:

**Before:**
```rust
// 하단바: 모드와 submode 표시
let mode_text = build_status_text(&model.editor_state);
let statusbar = Paragraph::new(mode_text).style(Style::default().add_modifier(Modifier::BOLD));
f.render_widget(statusbar, editor_chunks[2]);
```

**After:**
```rust
// 하단바: 모드 정보와 키바인딩 표시
let mode_text = build_status_text(&model.editor_state);
let keybindings = build_editor_keybindings(&model.editor_state);
let status_text = format!("{} | {}", mode_text, keybindings);
let statusbar = Paragraph::new(status_text).style(Style::default().add_modifier(Modifier::BOLD));
f.render_widget(statusbar, editor_chunks[2]);
```

**Step 2: Build and run the application**

Run: `cargo build && cargo run`

Expected: 빌드 성공

**Step 3: Test editor keybindings display**

1. 애플리케이션 실행
2. 달력에서 'e' 키로 에디터 진입
3. Normal 모드에서 전체 키바인딩이 표시되는지 확인
4. 'i' 키로 Insert 모드 진입 → Insert 모드 키바인딩 확인
5. Esc로 Normal 복귀 → Normal 키바인딩 확인
6. 'g' 키로 Goto 모드 진입 → Goto 키바인딩 확인
7. Esc로 복귀
8. Space 키로 SpaceCommand 모드 진입 → Space 키바인딩 확인
9. Esc로 복귀
10. '/' 키로 Search 모드 진입 → Search 키바인딩 확인

Expected: 모든 모드에서 올바른 키바인딩이 표시됨

**Step 4: Commit**

```bash
git add src/view.rs
git commit -m "feat: integrate dynamic keybindings into editor view

에디터 화면 하단바에 모드 정보와 함께 동적 키바인딩 표시
모든 모드/서브모드 전환 시 자동으로 키바인딩 업데이트

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: 전체 테스트 및 검증

**Files:**
- None (테스트 실행만)

**Step 1: Run all unit tests**

Run: `cargo test`

Expected: 모든 테스트 PASS

**Step 2: Manual integration testing**

전체 기능 흐름 테스트:

1. **달력 화면 테스트:**
   - 애플리케이션 시작 → Normal 모드 키바인딩 확인
   - Space 입력 → Space 모드 키바인딩 확인
   - 'n', 'p', 'y', 'Y' 키로 달/년 이동 → 키바인딩이 Normal로 복귀하는지 확인
   - Space → 'q' 입력 → 종료 확인

2. **에디터 화면 테스트:**
   - 달력에서 'e'로 에디터 진입 → Normal 모드 키바인딩 확인
   - 'i' → Insert 모드 키바인딩 확인 → 텍스트 입력 → Esc
   - 'g' → Goto 모드 키바인딩 확인 → 'g'로 문서 시작 이동 → Goto 종료 확인
   - Space → SpaceCommand 모드 키바인딩 확인 → 'w'로 저장 → SpaceCommand 종료 확인
   - '/' → Search 모드 키바인딩 확인 → 검색어 입력 → Enter → 검색 결과 확인

3. **경계 케이스 테스트:**
   - 빈 문서에서 에디터 키바인딩 표시
   - 긴 키바인딩 텍스트가 화면에 잘 표시되는지 확인
   - 빠른 모드 전환 시 키바인딩 업데이트 확인

Expected: 모든 시나리오에서 올바른 키바인딩 표시, 기능 정상 동작

**Step 3: Final verification**

Run: `cargo clippy`

Expected: No warnings or errors

Run: `cargo fmt --check`

Expected: All files properly formatted

**Step 4: Final commit (if any fixes needed)**

```bash
git add .
git commit -m "chore: final cleanup and verification

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: 문서 업데이트 및 최종 정리

**Files:**
- Create: `docs/features/keybinding-help.md` (선택적)

**Step 1: Create feature documentation (선택적)**

`docs/features/keybinding-help.md` 파일 생성:

```markdown
# 키바인딩 도움말 시스템

## 개요

화면 하단에 현재 모드/서브모드에 맞는 키바인딩 도움말을 동적으로 표시합니다.

## 기능

### 달력 화면

- **Normal 모드**: `hjkl:이동 | e:편집 | space:명령 | q:종료`
- **Space 모드**: `n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소`

### 에디터 화면

- **Normal 모드**: 모든 주요 키바인딩 표시
- **Insert 모드**: Insert 모드 관련 키만 표시
- **Goto 모드**: Goto 명령 키 표시
- **Space 모드**: Space 명령 키 표시
- **Search 모드**: 검색 관련 키 표시

## 구현

- 파일: `src/view.rs`
- 함수: `build_calendar_keybindings()`, `build_editor_keybindings()`
- 테스트: `tests/view_tests.rs`
```

**Step 2: Update main README if needed**

프로젝트 루트의 README.md에 새 기능 언급 (이미 있다면):

```markdown
## 기능

- 달력 기반 일기 관리
- Helix 스타일 키바인딩
- **컨텍스트 기반 키바인딩 도움말** ← 추가
- Markdown 미리보기
```

**Step 3: Final commit**

```bash
git add docs/features/keybinding-help.md README.md
git commit -m "docs: add keybinding help feature documentation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Summary

이 구현 계획은 TDD 방식으로 키바인딩 도움말 시스템을 구축합니다:

1. **Task 1-2**: 달력 키바인딩 빌더 함수와 테스트
2. **Task 3-4**: 에디터 키바인딩 빌더 함수와 전체 모드 테스트
3. **Task 5**: 달력 렌더링 통합
4. **Task 6**: 에디터 렌더링 통합
5. **Task 7**: 전체 테스트 및 검증
6. **Task 8**: 문서화

각 단계는 2-5분 내에 완료 가능하며, 빈번한 커밋으로 진행 상황을 추적합니다.

**원칙:**
- **TDD**: 테스트 먼저, 구현 나중
- **DRY**: 키바인딩 로직을 재사용 가능한 함수로 추출
- **YAGNI**: 현재 필요한 기능만 구현 (커스터마이징, 다국어 등은 미래 작업)
- **Frequent commits**: 각 의미있는 변경마다 커밋
