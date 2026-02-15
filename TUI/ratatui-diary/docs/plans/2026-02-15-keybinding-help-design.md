# 키바인딩 도움말 시스템 설계

## 개요

ratatui-diary의 달력 화면과 에디터 화면에 현재 모드/서브모드에 맞는 키바인딩 도움말을 화면 하단에 항상 표시하는 기능을 추가합니다.

## 사용자 요구사항

- **표시 방식**: 항상 표시 (현재 모드에 맞는 키바인딩을 항상 화면 하단에 표시)
- **상세도**: 주요 키 모두 (10-15개 정도의 모든 주요 키 표시)
- **스타일**: 간결한 텍스트 (예: "h:왼쪽 | l:오른쪽 | e:편집 | q:종료")

## 설계 결정

### 선택된 접근 방식: 동적 단일 줄 키바인딩 바

현재 모드/서브모드에 맞는 키를 동적으로 선택하여 화면 하단 한 줄에 표시합니다.

**선택 이유:**
- 기존 레이아웃 변경 최소 (달력은 2줄, 에디터는 1줄 상태바 활용)
- 모드별로 관련 키만 표시하여 혼란 방지
- 간결한 텍스트 스타일과 자연스럽게 어울림
- 구현이 상대적으로 단순

**대안 (고려했으나 채택하지 않음):**
- 2줄 키바인딩 바: 화면 공간을 더 많이 차지
- 스마트 필터링: "주요 키 모두" 요구사항을 충족하지 못함

## 아키텍처

키바인딩 도움말은 **순수 View 레이어 기능**으로 구현합니다.

### 파일 변경

**`src/view.rs`:**
- 키바인딩 텍스트 생성 함수 추가:
  - `build_calendar_keybindings(state: &CalendarState) -> String`
  - `build_editor_keybindings(state: &EditorState) -> String`
- 기존 렌더링 함수 수정:
  - `render_calendar()`: 69번 라인의 하드코딩된 statusbar 교체
  - `render_editor()`: 221-222번 라인의 statusbar에 키바인딩 추가

### 설계 원칙

- Model/Message/Update는 변경하지 않음 (순수 표시 기능)
- 각 모드/서브모드별로 관련 키만 선택하여 표시
- 기존 상태 정보(CalendarState.submode, EditorState.mode/submode) 활용

## 컴포넌트 설계

### 핵심 함수

#### `build_calendar_keybindings(state: &CalendarState) -> String`

CalendarState의 submode를 확인하여 적절한 키바인딩 텍스트 반환:

- `None`: `"hjkl:이동 | e:편집 | space:명령 | q:종료"`
- `Some(CalendarSubMode::Space)`: `"n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소"`

#### `build_editor_keybindings(state: &EditorState) -> String`

EditorMode와 submode를 모두 확인하여 적절한 키바인딩 텍스트 반환:

- `Normal + None`:
  ```
  "hjkl:이동 | w/b/e:단어 | i/a/o/O:입력 | v/x:선택 | d/c/y/p:편집 | u/U:실행취소 | g/space//:모드 | Esc:뒤로"
  ```

- `Normal + Goto`:
  ```
  "g:문서시작 | e:문서끝 | h:줄시작 | l:줄끝 | Esc:취소"
  ```

- `Normal + SpaceCommand`:
  ```
  "w:저장 | q:뒤로 | x:저장후뒤로 | Esc:취소"
  ```

- `Normal + Search`:
  ```
  "입력:검색어 | Enter:실행 | n/N:다음/이전 | Esc:취소"
  ```

- `Insert`:
  ```
  "입력중... | Enter:새줄 | Backspace:삭제 | Esc:Normal모드"
  ```

### 텍스트 형식 규칙

- **구분자**: `|` (파이프)로 그룹 구분
- **키:설명 형식**: 예: `"h:왼쪽"`, `"hjkl:이동"`
- **관련 키 묶음**: 슬래시로 묶음, 예: `"w/b/e:단어"`, `"n/p:다음/이전달"`

## 데이터 플로우

### 렌더링 파이프라인

```
Model (상태)
  → view::view(f, model)
    → match model.screen

      → Calendar: render_calendar(f, model)
        → build_calendar_keybindings(&model.calendar_state)
          → CalendarState.submode 확인
          → 해당 모드의 키바인딩 텍스트 반환
        → Paragraph::new(keybindings_text).render()

      → Editor: render_editor(f, model)
        → build_editor_keybindings(&model.editor_state)
          → EditorState.mode + submode 확인
          → 해당 모드의 키바인딩 텍스트 반환
        → 모드 정보와 키바인딩을 결합하여 Paragraph 렌더링
```

### 핵심 원칙

- 상태는 Model에 이미 존재 (CalendarState.submode, EditorState.mode/submode)
- 키바인딩 함수는 순수 함수 (상태 → 텍스트 변환만)
- 실시간으로 모드 변경에 따라 자동 업데이트 (기존 렌더링 루프 활용)

### 예시 플로우

1. 사용자가 달력 화면에서 `space` 입력
2. `update()` → `CalendarState.submode = Some(Space)`
3. 다음 렌더링 사이클에서 `build_calendar_keybindings()` 호출
4. Space 모드 키바인딩 반환: `"n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소"`
5. 화면 하단에 새로운 키바인딩 표시

## 에러 핸들링

이 기능은 순수 View 로직이므로 런타임 에러가 발생할 가능성이 거의 없습니다.

- **에러 가능성 없음**: 키바인딩 함수는 상태를 읽기만 하고 문자열을 반환
- **방어적 코딩**: match 문에서 모든 모드/서브모드 케이스 명시적으로 처리
- **컴파일 타임 안전성**: 새로운 모드 추가 시 match exhaustiveness로 컴파일 에러 발생

## 테스트 계획

### 단위 테스트

`tests/view_tests.rs`에 추가할 테스트:

```rust
#[test]
fn test_build_calendar_keybindings_normal() {
    // CalendarState.submode = None일 때 정상 키바인딩 반환
}

#[test]
fn test_build_calendar_keybindings_space_mode() {
    // CalendarState.submode = Some(Space)일 때 Space 모드 키바인딩 반환
}

#[test]
fn test_build_editor_keybindings_normal() {
    // EditorMode::Normal, submode = None
}

#[test]
fn test_build_editor_keybindings_insert() {
    // EditorMode::Insert
}

#[test]
fn test_build_editor_keybindings_goto() {
    // Normal + Goto 서브모드
}

#[test]
fn test_build_editor_keybindings_search() {
    // Normal + Search 서브모드
}

#[test]
fn test_build_editor_keybindings_space() {
    // Normal + SpaceCommand 서브모드
}
```

### BDD 형식 테스트 명세

```gherkin
Feature: 키바인딩 도움말 표시

Scenario: 달력 Normal 모드에서 키바인딩 표시
  Given 사용자가 달력 화면에 있을 때
  When 아무 서브모드도 활성화되지 않았다면
  Then "hjkl:이동 | e:편집 | space:명령 | q:종료"가 표시된다

Scenario: 달력 Space 모드에서 키바인딩 표시
  Given 사용자가 달력 화면에서 space를 눌렀을 때
  When CalendarSubMode::Space가 활성화되면
  Then "n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소"가 표시된다

Scenario: 에디터 Normal 모드에서 키바인딩 표시
  Given 사용자가 에디터 화면에 있을 때
  When Normal 모드이고 서브모드가 없다면
  Then 모든 주요 키바인딩이 표시된다

Scenario: 에디터 Insert 모드에서 키바인딩 표시
  Given 사용자가 에디터 화면에서 'i'를 눌렀을 때
  When Insert 모드로 전환되면
  Then "입력중... | Enter:새줄 | Backspace:삭제 | Esc:Normal모드"가 표시된다

Scenario: 에디터 Goto 서브모드에서 키바인딩 표시
  Given 사용자가 에디터 Normal 모드에서 'g'를 눌렀을 때
  When Goto 서브모드로 진입하면
  Then "g:문서시작 | e:문서끝 | h:줄시작 | l:줄끝 | Esc:취소"가 표시된다
```

### 통합 테스트

수동으로 각 모드 전환하며 키바인딩 표시 확인:
1. 달력 화면 → space 모드 진입/종료
2. 에디터 화면 → Normal/Insert 모드 전환
3. 에디터 Normal → Goto/Space/Search 서브모드 진입/종료

## 구현 순서

1. `src/view.rs`에 키바인딩 빌더 함수 추가
2. 달력 렌더링에 동적 키바인딩 적용
3. 에디터 렌더링에 동적 키바인딩 적용
4. 단위 테스트 작성
5. 수동 통합 테스트 수행

## 개선 가능한 점 (미래 작업)

- 키바인딩 설정 파일로 커스터마이징 가능하게
- 다국어 지원 (현재는 한국어만)
- 화면 너비에 따라 자동으로 축약/확장
