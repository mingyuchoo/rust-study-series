# SPEC-003: 전체 크레이트 기능 HTTP 노출

## 메타데이터
- SPEC ID: SPEC-003
- PRD: PRD-003
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-003 | FR-1 | /api/agents |
| PRD-003 | FR-2 | /api/tools |
| PRD-003 | FR-3 | /api/golden-sets |
| PRD-003 | FR-4 | /api/scenarios/:domain/:id |
| PRD-003 | FR-5 | POST /api/run |
| PRD-003 | FR-6 | POST /api/compare |

### 역방향 추적
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|------------|------|
| TC-1  | agents 목록 | FR-1 | crates/eval-harness/src/web/api.rs | Draft |
| TC-2  | tools 메타데이터 | FR-2 | crates/eval-harness/src/web/api.rs | Draft |
| TC-3  | golden-sets 로드 | FR-3 | crates/eval-harness/src/web/api.rs | Draft |
| TC-4  | scenario detail 성공 | FR-4 | crates/eval-harness/src/web/api.rs | Draft |
| TC-5  | scenario detail 404 | FR-4 | crates/eval-harness/src/web/api.rs | Draft |
| TC-6  | run_suite 정상 | FR-5 | crates/eval-harness/src/web/api.rs | Draft |
| TC-7  | 알 수 없는 에이전트 거부 | FR-5 | crates/eval-harness/src/web/api.rs | Draft |
| TC-8  | compare 동일파일 pass | FR-6 | crates/eval-harness/src/web/api.rs | Draft |
| TC-9  | compare 경로 이탈 거부 | FR-6 | crates/eval-harness/src/web/api.rs | Draft |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/web/api.rs | list_agents_impl, list_tools_impl, list_golden_sets_impl, scenario_detail_impl, run_suite_impl, compare_impl, is_safe_report_path | FR-1~FR-6 |
| eval-harness | src/web/mod.rs | build_router (라우트 추가) | FR-1~FR-6 |

## 개요
기존 PRD-002 웹 서버에 6개 엔드포인트를 추가한다. 핸들러는 얇은 래퍼, 내부 로직은 순수 함수(`*_impl`)로 분리해 단위 테스트.

## 기술 설계

### 새 라우트
```
GET  /api/agents                   → Vec<String>
GET  /api/tools                    → Vec<serde_json::Value>
GET  /api/golden-sets              → Vec<GoldenSetFile>
GET  /api/scenarios/:domain/:id    → ScenarioConfig | 404
POST /api/run                      → EvaluationReport
POST /api/compare                  → ComparisonResult
```

### 핵심 시그니처
```rust
pub fn list_agents_impl(reg: &AgentRegistry) -> Vec<String>;
pub fn list_tools_impl() -> Vec<serde_json::Value>;  // 전체 도메인 등록 후 메타데이터
pub fn list_golden_sets_impl(dir: &Path) -> Vec<GoldenSetFile>;
pub fn scenario_detail_impl(dir: &Path, domain: &str, id: &str) -> Option<ScenarioConfig>;
pub fn run_suite_impl(suite: &str, agent_name: &str, scenarios_dir: &Path, reports_dir: &Path)
    -> Result<EvaluationReport, String>;
pub fn compare_impl(baseline: &str, current: &str, threshold: f64, reports_dir: &Path)
    -> Result<ComparisonResult, String>;
pub fn is_safe_report_path(name: &str) -> bool;  // reuse handlers::is_safe_name
```

### AppState 확장
기존 `{scenarios_dir, reports_dir}`에 `golden_sets_dir: PathBuf` 추가 (기본 `eval_data/golden_sets`).

### 에이전트 빌드
`main.rs::build_registry()`를 `web::build_agent_registry()`로 이동/재사용하여 핸들러에서 on-demand 생성.

### 경로 안전성 (FR-6)
`compare_impl`은 baseline/current 이름에 `is_safe_name` 적용. 통과 시 `reports_dir.join(name)`으로만 파일 열기.

### 블로킹
`/api/run`, `/api/compare`는 `tokio::task::spawn_blocking`으로 sync 함수 래핑.

## 테스트 시나리오
| TC ID | 입력 | 기대 | FR |
|-------|------|------|----|
| TC-1  | registry에 passthrough 등록 | `list_agents_impl` → `["passthrough"]` 포함 | FR-1 |
| TC-2  | - | `list_tools_impl`이 비어있지 않음 | FR-2 |
| TC-3  | 임시 dir에 goldenset JSON | `list_golden_sets_impl`이 1개 반환 | FR-3 |
| TC-4  | 실제 `eval_data/eval_scenarios`, `customer_service`, `cs_001` | Some(scenario) | FR-4 |
| TC-5  | 없는 id | None | FR-4 |
| TC-6  | `customer_service`, `passthrough`, 실제 dirs | Ok(report) suite_name 일치 | FR-5 |
| TC-7  | 에이전트 `"unknown"` | Err | FR-5 |
| TC-8  | 동일 파일 비교 | `verdict == "pass"` | FR-6 |
| TC-9  | baseline `"../evil"` | Err | FR-6 |

## 완료 정의
- 모든 TC 통과
- `curl POST /api/run` 정상 동작
- README 업데이트
- verify_trace COMPLETE
