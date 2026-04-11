# SPEC-001: TUI 모듈 (ratatui 기반)

## 메타데이터
- SPEC ID: SPEC-001
- PRD: PRD-001
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-001 | FR-1 | `cargo run -- tui` 서브커맨드 |
| PRD-001 | FR-2 | 시나리오 목록 표시 및 키보드 탐색 |
| PRD-001 | FR-3 | 저장된 리포트 파일 목록 표시 |
| PRD-001 | FR-4 | q/Esc 종료 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|------------|------|
| TC-1  | tui 서브커맨드 파싱 | FR-1 | crates/eval-harness/src/tui/mod.rs | Draft |
| TC-2  | 시나리오 목록 로딩 | FR-2 | crates/eval-harness/src/tui/state.rs | Draft |
| TC-3  | 선택 인덱스 이동 (next/prev) | FR-2 | crates/eval-harness/src/tui/state.rs | Draft |
| TC-4  | 리포트 파일 목록 로딩 | FR-3 | crates/eval-harness/src/tui/state.rs | Draft |
| TC-5  | Tab으로 포커스 전환 | FR-3 | crates/eval-harness/src/tui/state.rs | Draft |
| TC-6  | q/Esc 입력 시 should_quit=true | FR-4 | crates/eval-harness/src/tui/state.rs | Draft |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | crates/eval-harness/src/tui/mod.rs | run_tui | FR-1 |
| eval-harness | crates/eval-harness/src/tui/state.rs | TuiState, Focus | FR-2, FR-3, FR-4 |
| eval-harness | crates/eval-harness/src/tui/view.rs | draw | FR-2, FR-3 |
| eval-harness | crates/eval-harness/src/main.rs | Commands::Tui | FR-1 |

## 개요

`eval-harness` CLI에 `tui` 서브커맨드를 추가하고, `ratatui` + `crossterm` 기반 2-패널 TUI를 구현한다. 좌측 패널은 시나리오 목록, 우측 패널은 저장된 리포트 파일 목록을 보여준다. Tab으로 포커스 전환, ↑/↓/j/k로 항목 이동, q/Esc로 종료.

## 기술 설계

### 의존성
- `ratatui = "0.28"` (TUI 렌더링)
- `crossterm = "0.28"` (터미널 backend, 이벤트)

### 모듈 구조
```
crates/eval-harness/src/
├── main.rs              # Commands::Tui 분기 추가
└── tui/
    ├── mod.rs           # run_tui() 진입점
    ├── state.rs         # TuiState, Focus, 이벤트 핸들러
    └── view.rs          # draw() 렌더 함수
```

### 데이터 모델
```rust
pub enum Focus { Scenarios, Reports }

pub struct TuiState {
    pub scenarios: Vec<String>,   // 시나리오 ID 목록 (예: ["cs_001", ...])
    pub reports: Vec<String>,     // 리포트 파일명 목록
    pub scenario_idx: usize,
    pub report_idx: usize,
    pub focus: Focus,
    pub should_quit: bool,
}
```

### API
- `TuiState::new(scenarios_dir, reports_dir) -> io::Result<Self>`: 초기 상태 로드
- `TuiState::handle_key(&mut self, key: KeyCode)`: 키 이벤트 적용
- `TuiState::next(&mut self)`, `prev(&mut self)`: 포커스된 리스트 이동
- `run_tui(scenarios_dir, reports_dir) -> io::Result<()>`: 터미널 setup/teardown 포함 메인 루프

## 대상 패키지
- `crates/eval-harness/`: CLI 바이너리, TUI 모듈 추가

## 테스트 시나리오
| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1  | tui 서브커맨드 파싱 | `Cli::parse_from(["eval-harness","tui"])` | `Commands::Tui` 분기 | unit | FR-1 |
| TC-2  | 시나리오 목록 로딩 | eval_data/eval_scenarios 디렉토리 | scenarios가 비어있지 않음 | unit | FR-2 |
| TC-3  | 선택 인덱스 이동 | next() 호출 | scenario_idx가 1 증가 | unit | FR-2 |
| TC-4  | 리포트 파일 목록 로딩 | 임시 dir에 .json 파일 배치 | reports 목록에 포함 | unit | FR-3 |
| TC-5  | Tab으로 포커스 전환 | handle_key(Tab) | focus가 Reports로 바뀜 | unit | FR-3 |
| TC-6  | q 입력 시 종료 플래그 | handle_key('q') | should_quit=true | unit | FR-4 |

## 구현 가이드
- 대상 패키지: `crates/eval-harness`
- 의존성 추가: `Cargo.toml`에 ratatui, crossterm
- 주의사항: 터미널 raw mode 복원 보장 (Drop 패턴 또는 명시적 cleanup)
- 테스트 가능성을 위해 상태 로직(`TuiState`)과 렌더 로직(`view::draw`)을 분리. 테스트는 상태 로직만 검증.

## 완료 정의 (Definition of Done)
- 모든 TC-1~TC-6 통과
- `cargo run -- tui` 실행 시 에러 없이 TUI 표시
- `verify_trace.py` 추적성 검증 통과
- README.md에 TUI 사용법 섹션 추가
