# SPEC-015: eval_data 경로 외부 설정화

## 메타데이터
- SPEC ID: SPEC-015
- PRD: PRD-015
- 작성일: 2026-04-10
- 상태: Implemented

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-015 | FR-1 | TOML 설정 파일에서 데이터 경로 읽기 |
| PRD-015 | FR-2 | 상대 경로는 설정 파일 디렉토리 기준 해석 |
| PRD-015 | FR-3 | 환경변수 override 지원 |
| PRD-015 | FR-4 | CLI 인자가 최우선 |
| PRD-015 | FR-5 | 내장 기본값 fallback |
| PRD-015 | FR-6 | desktop 진입점도 동일 로직 사용 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1 | 설정 파일에서 두 경로를 읽는다 | FR-1 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-2 | 일부 키 누락 시 누락된 키만 기본값 fallback | FR-1, FR-5 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-3 | 설정 파일의 상대 경로는 설정 파일 디렉토리 기준 | FR-2 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-4 | 설정 파일의 절대 경로는 그대로 사용 | FR-2 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-5 | 환경변수가 설정 파일을 override | FR-3 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-6 | CLI 인자가 환경변수와 설정 파일을 모두 override | FR-4 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-7 | 설정 파일/ENV/CLI 모두 없으면 내장 기본값 | FR-5 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-8 | desktop 용 `resolve_for_root` 가 워크스페이스 루트 기준으로 동작 | FR-6 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-9 | 잘못된 TOML 은 명확한 에러로 실패 | NFR-2 | crates/eval-harness/src/data_paths.rs | Draft |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 패키지 | 파일 | 심볼 (함수/클래스) | 관련 FR |
|--------|------|-------------------|--------|
| crates/eval-harness | crates/eval-harness/src/data_paths.rs | `DataPaths`, `DataPaths::default`, `DataPaths::load`, `DataPaths::resolve_for_root`, `DataPaths::apply_env`, `DataPaths::with_overrides`, `resolve_relative` | FR-1~FR-6 |
| crates/eval-harness | crates/eval-harness/src/lib.rs | `pub mod data_paths;` | FR-1 |
| crates/eval-harness | crates/eval-harness/src/main.rs | `Run`/`List`/`Tui`/`Serve` 핸들러에서 `DataPaths::with_overrides` 호출 | FR-4, FR-5 |
| desktop | desktop/src/main.rs | `DataPaths::resolve_for_root(&root)` 호출 | FR-6 |

## 개요

`eval-harness` 의 데이터 디렉토리(`scenarios_dir`, `golden_sets_dir`) 해석을 단일 모듈 `data_paths` 로 중앙화한다. 이 모듈은 다음 4 단계 우선순위로 경로를 결정한다:

1. **CLI override** (가장 높음) — `--scenarios-dir`, `--golden-sets-dir`
2. **환경변수** — `EVAL_HARNESS_SCENARIOS_DIR`, `EVAL_HARNESS_GOLDEN_SETS_DIR`
3. **TOML 설정 파일** — `eval-harness.toml` (기본은 CWD 또는 명시적으로 지정한 루트)
4. **내장 기본값** — `eval_data/scenarios`, `eval_data/golden_sets`

상대 경로 해석 기준은 출처에 따라 다르다:
- 설정 파일에서 온 값 → 설정 파일이 위치한 디렉토리 기준
- 환경변수/CLI 에서 온 값 → 그대로 사용 (호출자가 CWD 기준으로 처리한다고 가정)
- 내장 기본값 → CLI 진입점에서는 CWD 기준, desktop 에서는 워크스페이스 루트 기준

## 기술 설계

### 모듈 구조 (`crates/eval-harness/src/data_paths.rs`)

```rust
use std::path::{Path, PathBuf};

pub const DEFAULT_SCENARIOS_DIR: &str = "eval_data/scenarios";
pub const DEFAULT_GOLDEN_SETS_DIR: &str = "eval_data/golden_sets";
pub const DEFAULT_CONFIG_FILENAME: &str = "eval-harness.toml";
pub const ENV_SCENARIOS_DIR: &str = "EVAL_HARNESS_SCENARIOS_DIR";
pub const ENV_GOLDEN_SETS_DIR: &str = "EVAL_HARNESS_GOLDEN_SETS_DIR";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPaths {
    pub scenarios_dir:   PathBuf,
    pub golden_sets_dir: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum DataPathsError {
    #[error("설정 파일 읽기 실패 ({path}): {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("설정 파일 TOML 파싱 실패 ({path}): {source}")]
    Parse { path: PathBuf, source: toml::de::Error },
}

impl DataPaths {
    /// 내장 기본값. 모든 경로는 호출자 CWD 기준의 상대 경로.
    pub fn default() -> Self { ... }

    /// 주어진 base 디렉토리에서 `eval-harness.toml` 을 찾고,
    /// 환경변수 + 내장 기본값과 병합하여 반환한다.
    /// 설정 파일이 없으면 그냥 ENV + 기본값만 적용.
    pub fn load(base: &Path) -> Result<Self, DataPathsError> { ... }

    /// desktop 진입점용. 워크스페이스 루트를 base 로 사용하며,
    /// 내장 기본값도 root 기준으로 join 한다.
    pub fn resolve_for_root(root: &Path) -> Result<Self, DataPathsError> { ... }

    /// 환경변수가 있으면 self 의 필드를 덮어쓴다.
    /// 환경변수 값이 상대 경로면 그대로(호출자 CWD 기준).
    pub fn apply_env(&mut self) { ... }

    /// CLI 인자(`Option<&str>`)가 Some 이면 덮어쓴다.
    pub fn with_overrides(
        mut self,
        scenarios: Option<&str>,
        golden_sets: Option<&str>,
    ) -> Self { ... }
}

/// 경로가 절대면 그대로, 상대면 base 기준으로 결합.
fn resolve_relative(base: &Path, p: &str) -> PathBuf { ... }
```

### TOML 스키마

```toml
[data]
scenarios_dir   = "eval_data/scenarios"     # 상대 경로 = 설정 파일 디렉토리 기준
golden_sets_dir = "/var/lib/eval/golden"    # 절대 경로 그대로
```

`[data]` 섹션 자체가 없거나 키가 없으면 내장 기본값을 사용한다.

### 진입점 통합

**`crates/eval-harness/src/main.rs`**:
- 모든 `default_value = "eval_data/..."` 를 제거하고 `Option<String>` 으로 변경.
- 각 핸들러에서:
  ```rust
  let paths = DataPaths::load(&std::env::current_dir()?)?
      .with_overrides(scenarios_dir.as_deref(), golden_sets_dir.as_deref());
  ```

**`desktop/src/main.rs`**:
- `root.join("eval_data/...")` 직접 결합 제거.
- `DataPaths::resolve_for_root(&root)?` 호출로 대체.

### 의존성 추가
- 워크스페이스 `toml = "0.8"` (workspace.dependencies 에 추가)
- 워크스페이스 `thiserror` 는 이미 존재 → eval-harness Cargo.toml 의 dependencies 에 `thiserror`, `toml` 추가.

### 검증 전략
- 단위 테스트는 `tempfile::tempdir` 로 격리 디렉토리에 `eval-harness.toml` 을 생성하여 검증.
- 환경변수 테스트는 동일 프로세스 내 다른 테스트와 충돌을 피하기 위해 `apply_env_from_map` 같은 주입 가능 형태도 함께 제공 (또는 환경변수 직접 set 후 unset).
- 상대 경로 테스트는 결과 PathBuf 가 base 디렉토리로 시작하는지 확인.

## 대상 패키지
- `crates/eval-harness`: 신규 `data_paths` 모듈 + main.rs/lib.rs 통합
- `desktop`: `data_paths` 호출

## 테스트 시나리오
| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | 두 키 모두 있는 TOML 로드 | tempdir 에 두 키 모두 있는 `eval-harness.toml` | scenarios_dir, golden_sets_dir 가 base 와 join 된 경로 | 단위 | FR-1 |
| TC-2 | 일부 키 누락 | scenarios_dir 만 있는 TOML | scenarios_dir 는 파일 값, golden_sets_dir 는 base/eval_data/golden_sets | 단위 | FR-1, FR-5 |
| TC-3 | 설정 파일의 상대 경로 | `scenarios_dir = "rel/scen"` | base.join("rel/scen") | 단위 | FR-2 |
| TC-4 | 설정 파일의 절대 경로 | `scenarios_dir = "/abs/scen"` | PathBuf::from("/abs/scen") | 단위 | FR-2 |
| TC-5 | 환경변수 override | TOML 에 값 + env 에 다른 값 | env 값이 우선 | 단위 | FR-3 |
| TC-6 | CLI override | TOML + env + CLI 모두 다른 값 | CLI 값이 우선 | 단위 | FR-4 |
| TC-7 | 모두 없음 | 빈 base, env unset, CLI None | DEFAULT_SCENARIOS_DIR / DEFAULT_GOLDEN_SETS_DIR | 단위 | FR-5 |
| TC-8 | resolve_for_root | 임의 root + 설정 파일 없음 | root.join(DEFAULT_*) | 단위 | FR-6 |
| TC-9 | 잘못된 TOML | `eval-harness.toml = "garbage = ="` | `DataPathsError::Parse` 반환 | 단위 | NFR-2 |

## 구현 가이드
- 대상 패키지: `crates/eval-harness`
- 파일 위치:
  - `crates/eval-harness/src/data_paths.rs` — 신규 모듈 (구현 + `#[cfg(test)] mod tests`)
  - `crates/eval-harness/src/lib.rs` — `pub mod data_paths;` 추가
  - `crates/eval-harness/src/main.rs` — CLI 인자 변경 + DataPaths 통합
  - `desktop/src/main.rs` — DataPaths::resolve_for_root 호출
  - `Cargo.toml` (workspace) — `toml = "0.8"` 추가
  - `crates/eval-harness/Cargo.toml` — `toml`, `thiserror` dep 추가
- 주의사항:
  - 환경변수 단위 테스트는 한 개의 `#[test]` 함수 내에서 set→assert→unset 순으로 닫는다(병렬 테스트 충돌 회피를 위해 `serial_test` 도입은 지양; 대신 환경변수 이름을 테스트 전용으로 분리하지 말고 set/unset 을 보장).
  - 더 안전한 방법으로 `apply_env_from_map(&BTreeMap<&str,&str>)` 헬퍼를 별도 제공하여 단위 테스트는 이 헬퍼를 호출, 프로덕션 코드만 `std::env::var` 를 호출하도록 분리.

## 완료 정의 (Definition of Done)
- [ ] 9 개 TC 모두 통과 (`cargo test -p eval-harness data_paths`)
- [ ] 전체 워크스페이스 빌드 통과 (`cargo build`)
- [ ] `cargo run -- list` 가 설정 파일 없이도 동작 (기존 동작 호환)
- [ ] `eval-harness.toml` 만 두고 CLI 인자 없이 `cargo run -- list` 가 동작
- [ ] 추적성 검증 통과 (`verify_trace.py`)
- [ ] README 업데이트
