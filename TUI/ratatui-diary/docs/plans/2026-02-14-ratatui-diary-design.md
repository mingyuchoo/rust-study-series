# Ratatui 다이어리 설계 문서

**작성일**: 2026-02-14
**버전**: MVP 1.0
**아키텍처**: ELM (Model-Update-View)

---

## 1. 프로젝트 개요

### 목적
Rust와 Ratatui를 사용한 터미널 기반 다이어리 애플리케이션

### MVP 범위
- ✅ 달력 화면 (월간 뷰)
- ✅ 다이어리 작성 유무 표시
- ✅ Vi 모드 에디터
- ✅ Markdown 파일 저장
- ❌ 단어 빈도/관계 그래프 (향후 추가)

### 핵심 기능
1. **달력 화면**: 월간 달력 표시, 다이어리 작성된 날짜 시각적 표시
2. **에디터 화면**: Vi 모드 텍스트 에디터 (Normal/Insert/Command 모드)
3. **파일 저장**: `~/.local/share/ratatui-diary/entries/YYYY-MM-DD.md`
4. **네비게이션**: Vi 스타일 키바인딩 (h/j/k/l, H/L)

---

## 2. 아키텍처

### ELM 아키텍처 패턴

```
사용자 입력 → Message → Update → Model → View → UI 렌더링
                ↑                    ↓
                └─── Command ───────┘
                   (Side Effects)
```

### 디렉토리 구조

```
ratatui-diary/
├── Cargo.toml
├── src/
│   ├── main.rs           # 앱 진입점, 이벤트 루프
│   ├── model.rs          # 상태 정의
│   ├── message.rs        # 이벤트 메시지
│   ├── update.rs         # 상태 업데이트 로직
│   ├── view.rs           # UI 렌더링
│   ├── storage.rs        # 파일 I/O
│   └── lib.rs            # 모듈 선언
├── tests/
│   └── storage_tests.rs
└── docs/
    └── plans/
```

### 핵심 원칙
- **불변성**: Model은 불변하게 업데이트
- **순수 함수**: View는 상태를 읽기만 함
- **명시적 Side Effect**: 파일 I/O는 Command로 분리
- **단방향 데이터 흐름**: Message → Update → Model → View

---

## 3. 데이터 모델

### 앱 상태

```rust
struct Model {
    screen: Screen,
    calendar_state: CalendarState,
    editor_state: EditorState,
    diary_entries: DiaryIndex,
    error_message: Option<String>,
    show_error_popup: bool,
}

enum Screen {
    Calendar,
    Editor,
}
```

### 달력 상태

```rust
struct CalendarState {
    current_year: i32,
    current_month: u32,
    selected_date: NaiveDate,
    cursor_pos: usize,
}
```

### 에디터 상태

```rust
struct EditorState {
    mode: EditorMode,
    date: NaiveDate,
    content: Vec<String>,
    cursor_line: usize,
    cursor_col: usize,
    is_modified: bool,
}

enum EditorMode {
    Normal,
    Insert,
    Command(String),
}
```

### 다이어리 인덱스

```rust
struct DiaryIndex {
    entries: HashSet<NaiveDate>,
}
```

---

## 4. 메시지 시스템

### 메시지 정의

```rust
enum Msg {
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
```

---

## 5. 업데이트 로직

### update 함수

```rust
fn update(model: &mut Model, msg: Msg) -> Option<Command>
```

### 상태 전환
- **Calendar → Editor**: `CalendarSelectDate` → `Command::LoadDiary`
- **Editor → Calendar**: `:q` 또는 `:wq` 실행
- **Normal ↔ Insert**: `i` / `Esc`
- **Normal → Command**: `:`

### Side Effect 처리

```rust
enum Command {
    LoadDiary(NaiveDate),
    SaveDiary(NaiveDate, String),
    DeleteDiary(NaiveDate),
    SaveAndBack(NaiveDate, String),
}
```

Command는 `main.rs`에서 실행 → 결과를 Msg로 변환 → update 호출

---

## 6. 뷰 렌더링

### 달력 화면 레이아웃

```
┌─────────────────────────────────┐
│         2026년 2월              │ (헤더)
├─────────────────────────────────┤
│ 일  월  화  수  목  금  토      │
│                     1   2   3   │
│  4   5   6   7   8   9  10      │
│ 11  12  13★ 14  15  16  17      │ (달력 그리드)
│ 18  19  20  21  22  23  24      │
│ 25  26  27  28                  │
├─────────────────────────────────┤
│ h/l: 달 | H/L: 연도 | q: 종료  │ (상태바)
└─────────────────────────────────┘
```

**표시 규칙:**
- ★ = 다이어리 있음 (녹색, Bold)
- 선택된 날짜 = 파란 배경
- 오늘 = 밑줄

### 에디터 화면 레이아웃

```
┌─────────────────────────────────┐
│ 2026-02-14                      │ (날짜 헤더)
├─────────────────────────────────┤
│                                 │
│ 오늘은 Rust로...                │
│                                 │ (에디터 영역)
│ [커서]                          │
│                                 │
├─────────────────────────────────┤
│ -- INSERT --                    │ (모드 표시)
└─────────────────────────────────┘
```

---

## 7. 파일 저장소

### 디렉토리 구조

```
~/.local/share/ratatui-diary/
└── entries/
    ├── 2026-01-15.md
    ├── 2026-02-14.md
    └── 2026-02-20.md
```

### Storage API

```rust
pub struct Storage {
    entries_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self>
    pub fn load(&self, date: NaiveDate) -> Result<String>
    pub fn save(&self, date: NaiveDate, content: &str) -> Result<()>
    pub fn delete(&self, date: NaiveDate) -> Result<()>
    pub fn scan_entries(&self) -> Result<HashSet<NaiveDate>>
}
```

### Markdown 파일 형식

```markdown
오늘은 Rust로 다이어리 프로그램을 만들었다.

Ratatui가 생각보다 사용하기 편했고, ELM 아키텍처로 구조화하니 깔끔했다.

내일은 그래프 기능을 추가해봐야겠다.
```

- 순수 Markdown (메타데이터 없음)
- 파일명이 곧 날짜

---

## 8. 키 바인딩

### 달력 화면

| 키 | 동작 |
|---|---|
| `h` | 이전 날짜 |
| `l` | 다음 날짜 |
| `j` | 다음 주 (아래) |
| `k` | 이전 주 (위) |
| `H` | 이전 연도 |
| `L` | 다음 연도 |
| `Enter` | 다이어리 작성/편집 |
| `q` | 종료 |

### 에디터 화면 - Normal 모드

| 키 | 동작 |
|---|---|
| `i` | Insert 모드 진입 |
| `h/j/k/l` | 커서 이동 |
| `dd` | 다이어리 전체 삭제 |
| `:` | Command 모드 진입 |
| `Esc` | 달력으로 돌아가기 (저장 안됨) |

### 에디터 화면 - Insert 모드

| 키 | 동작 |
|---|---|
| `Esc` | Normal 모드 복귀 |
| 문자 입력 | 텍스트 삽입 |
| `Enter` | 줄바꿈 |
| `Backspace` | 삭제 |

### 에디터 화면 - Command 모드

| 명령 | 동작 |
|---|---|
| `:w` | 저장 |
| `:q` | 저장 안하고 나가기 |
| `:wq` | 저장 후 나가기 |

---

## 9. 에러 처리

### 에러 타입

```rust
pub enum DiaryError {
    Io(io::Error),
    DateParse(chrono::ParseError),
    NoDataDir,
}
```

### 처리 전략

| 상황 | 동작 |
|---|---|
| 파일 없음 (새 다이어리) | 빈 에디터로 시작 |
| 저장 실패 | 에러 팝업, 내용 유지 |
| 디렉토리 생성 실패 | 앱 시작 실패, 터미널 에러 출력 |
| 삭제 실패 | 에러 팝업, 상태 유지 |

### UI 에러 표시

```
┌─────── Error ───────┐
│                     │
│ 저장 실패: 디스크   │
│ 공간 부족           │
│                     │
│ [Esc로 닫기]        │
└─────────────────────┘
```

---

## 10. 테스팅 전략

### 단위 테스트

- **Model 로직**: 달력 네비게이션, 날짜 계산
- **Update 로직**: 메시지 처리, 상태 전환, Command 생성
- **EditorState**: 텍스트 삽입, 삭제, 커서 이동

### 통합 테스트

- **Storage**: 저장, 로드, 삭제, 스캔
- 임시 디렉토리로 격리

### TDD 접근

- Red → Green → Refactor
- 각 기능 구현 전 테스트 작성

### 테스트 범위 목표

- Model/Update: 90%+
- Storage: 80%+
- View: 수동 테스트

---

## 11. 의존성

### 필수 Crates

```toml
[dependencies]
ratatui = "0.27"
crossterm = "0.27"
chrono = "0.4"
dirs = "5.0"

[dev-dependencies]
tempfile = "3.8"
```

---

## 12. 구현 순서

1. ✅ 설계 문서 작성
2. 프로젝트 초기화 및 의존성 설정
3. Storage 구현 및 테스트
4. Model/Message 정의
5. Update 로직 구현 (TDD)
6. View - 달력 화면 구현
7. View - 에디터 화면 구현
8. 통합 및 수동 테스트
9. 에러 처리 개선
10. 문서화 및 README 작성

---

## 13. 향후 확장

### Phase 2: 기본 분석 기능
- 월별/연별 작성 통계
- 단어 빈도 분석

### Phase 3: 그래프 시각화
- 단어 관계 그래프
- 시간별 트렌드 차트
- 감정 분석 (선택적)

---

## 14. 설계 결정 사항

| 항목 | 선택 | 이유 |
|---|---|---|
| 아키텍처 | ELM | 명확한 데이터 흐름, 테스트 용이 |
| 키바인딩 | Vi 스타일 | 효율성, 일관성 |
| 저장 위치 | `~/.local/share` | OS 표준, 어디서든 접근 |
| 파일 형식 | Markdown | 단순, 호환성 |
| 편집 기능 | 수정/삭제 | 완전한 기능 |
| MVP 범위 | 그래프 제외 | 핵심 기능 우선 |

---

**문서 끝**
