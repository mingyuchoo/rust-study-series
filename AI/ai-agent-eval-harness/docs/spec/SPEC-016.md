# SPEC-016: eval-harness.toml [evaluation] 섹션 파싱

## 메타데이터
- SPEC ID: SPEC-016
- PRD: PRD-016
- 작성일: 2026-04-11
- 상태: Draft

## 추적 정보

### 정방향 추적
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-016 | FR-1 | `[evaluation]` 섹션에서 `max_iterations`, `early_stop_threshold` 읽기 |
| PRD-016 | FR-2 | 섹션/키 누락 시 기본값 fallback |
| PRD-016 | FR-3 | CLI 와 웹 진입점이 동일 해석 로직 공유 |

### 역방향 추적
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|------------|------|
| TC-1  | 두 키 모두 있는 TOML 로드 | FR-1 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-2  | 섹션 전체 누락 → 기본값 fallback | FR-2 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-3  | 일부 키만 있는 경우 해당 키만 override | FR-1, FR-2 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-4  | `max_iterations = 0` 거부 | NFR-1 | crates/eval-harness/src/data_paths.rs | Draft |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/data_paths.rs | `ConfigFile`, `ConfigEvaluation`, `load_evaluation_config` | FR-1, FR-2 |
| eval-harness | src/main.rs | `build_registry` (기본값 → 설정 로드로 교체) | FR-3 |
| eval-harness | src/web/api.rs | `build_agent_registry` (기본값 → 설정 로드로 교체) | FR-3 |
| agent-core   | src/config.rs | `EvaluationConfig` (변경 없음) | - |

## 기술 설계

### 핵심 타입 및 함수
```rust
// crates/eval-harness/src/data_paths.rs
#[derive(Debug, serde::Deserialize, Default)]
struct ConfigEvaluation {
    max_iterations: Option<u32>,
    early_stop_threshold: Option<u32>,
}

pub fn load_evaluation_config(base: &Path)
    -> Result<agent_core::config::EvaluationConfig, DataPathsError>;
```

`load_evaluation_config` 는 `DEFAULT_CONFIG_FILENAME` 을 읽어 `EvaluationConfig::default()` 에 `[evaluation]` 섹션 값을 overlay 한다. 파일이 없으면 기본값을 그대로 반환한다.

### 검증 규칙
- `max_iterations == 0` 또는 `early_stop_threshold == 0` 은 `DataPathsError::Invalid` 로 거부한다.
- 타입 불일치는 `toml::de::Error` 로 이미 잡히므로 `DataPathsError::Parse` 로 래핑.

### 진입점 변경
- `main.rs::build_registry()` 와 `web/api.rs::build_agent_registry()` 양쪽에서
  `EvaluationConfig::default()` 호출을 `load_evaluation_config(&cwd).unwrap_or_default()` 로 교체한다.
- 파일/파싱 에러는 경고 메시지 후 기본값으로 fallback 하여 기존 동작을 깨지 않는다.

## 테스트 시나리오
| TC | 입력 | 기대 |
|----|------|------|
| TC-1 | `[evaluation]\nmax_iterations=5\nearly_stop_threshold=4` | Ok(cfg) with (5, 4) |
| TC-2 | 파일에 `[evaluation]` 섹션 없음 | Ok(cfg) == default (3, 3) |
| TC-3 | `[evaluation]\nmax_iterations=7` | Ok(cfg) with (7, 3) |
| TC-4 | `[evaluation]\nmax_iterations=0` | Err |

## 완료 정의
- 4개 TC 통과
- 설정 파일에 `max_iterations=5` 지정 시 CLI 및 웹 UI 실행이 실제로 5 회까지 반복
- README 에 `[evaluation]` 섹션 예제 추가
