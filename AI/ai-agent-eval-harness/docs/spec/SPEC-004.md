# SPEC-004: crates 4-10 실행용 HTTP 엔드포인트

## 메타데이터
- SPEC ID: SPEC-004
- PRD: PRD-004
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-004 | FR-1 | POST /api/scenarios/:domain/:id/run |
| PRD-004 | FR-2 | POST /api/agents/:name/execute |
| PRD-004 | FR-3 | POST /api/tools/:name/invoke |
| PRD-004 | FR-4 | GET /api/golden-sets/:domain/:scenario_id |
| PRD-004 | FR-5 | POST /api/score |
| PRD-004 | FR-6 | POST /api/tools/:name/simulate-fault |
| PRD-004 | FR-7 | GET /api/trajectories[/:name] |
| PRD-004 | FR-8 | POST /api/agents/:name/execute (domain 필드) |

### 역방향 추적
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|------------|------|
| TC-1  | run_scenario_impl 성공 | FR-1 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-2  | 없는 시나리오 404 | FR-1 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-3  | agent_execute_impl passthrough | FR-2 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-4  | tool_invoke_impl 성공 | FR-3 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-5  | 알 수 없는 도구 거부 | FR-3 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-6  | golden entry 조회 성공 | FR-4 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-7  | golden entry 404 | FR-4 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-8  | score_impl 빈 궤적 처리 | FR-5 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-9  | fault sim 결과 반환 | FR-6 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-10 | list_trajectories 스캔 | FR-7 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-11 | get_trajectory 경로 이탈 거부 | FR-7 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-12 | agent_execute_impl + customer_service 도메인으로 도구 로드 성공 | FR-8 | crates/eval-harness/src/web/api_exec.rs | Draft |
| TC-13 | agent_execute_impl + 알 수 없는 도메인 Err 반환 | FR-8 | crates/eval-harness/src/web/api_exec.rs | Draft |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/web/api_exec.rs | run_scenario_impl, agent_execute_impl, tool_invoke_impl, golden_entry_impl, score_impl, fault_sim_impl, list_trajectories_impl, get_trajectory_impl, build_full_tool_registry | FR-1~FR-7 |
| eval-harness | src/web/mod.rs | build_router (라우트 8개 추가) | FR-1~FR-7 |
| eval-harness | src/main.rs | Commands::Serve (trajectories_dir 추가) | FR-7 |

## 기술 설계

### 의존성 추가
`eval-harness/Cargo.toml`에 `scoring`, `reporting`, `execution-fault-injection` 워크스페이스 크레이트 추가.

### 핵심 순수 함수
```rust
pub fn build_full_tool_registry() -> ToolRegistry;

pub fn run_scenario_impl(scen_dir: &Path, reps_dir: &Path,
    domain: &str, id: &str, agent_name: &str) -> Result<EvaluationResult, String>;

pub fn agent_execute_impl(scen_dir: &Path, agent_name: &str, task: &str,
    env: Option<HashMap<String, Value>>, domain: Option<&str>) -> Result<Trajectory, String>;

pub fn tool_invoke_impl(name: &str, params: &HashMap<String, Value>)
    -> Result<HashMap<String, Value>, String>;

pub fn golden_entry_impl(dir: &Path, domain: &str, sid: &str) -> Option<GoldenSetEntry>;

pub fn score_impl(trajectory: Trajectory) -> EvaluationResult;

pub fn fault_sim_impl(name: &str, params: &HashMap<String, Value>,
    config: FaultInjectionConfig) -> Result<HashMap<String, Value>, String>;

pub fn list_trajectories_impl(dir: &Path) -> Vec<String>;
pub fn get_trajectory_impl(dir: &Path, name: &str) -> Option<serde_json::Value>;
```

### 실행/안전 규칙
- 모든 impl 함수는 sync; 핸들러에서 `tokio::task::spawn_blocking` 으로 감싼다.
- 경로/이름 파라미터는 `handlers::is_safe_name` 통과 필수.
- `AppState`에 `trajectories_dir: PathBuf` 추가 (기본 `reporting_trajectories`).
- `run_scenario_impl`은 `ScenarioConfig -> Scenario` 변환 후 `HarnessRunner::run_scenario` 호출.
- `agent_execute_impl`은 `build_agent_registry`에서 에이전트 조회 후 `execute_task` 직접 호출.

## 테스트 시나리오 (요약)
| TC | 입력 | 기대 |
|----|------|------|
| TC-1 | `customer_service`,`cs_001`,`passthrough` | Ok(eval) |
| TC-2 | 없는 id | Err |
| TC-3 | `passthrough`,"hello",None | Ok(trajectory) |
| TC-4 | `classify_inquiry` + 적절 params | Ok(map) |
| TC-5 | `nonexistent_tool_xyz` | Err |
| TC-6 | 임시 goldenset dir + 일치 id | Some(entry) |
| TC-7 | 없는 id | None |
| TC-8 | 빈 Trajectory | EvaluationResult 반환 (panic 없음) |
| TC-9 | classify_inquiry + disabled config | Ok(map) |
| TC-10 | 임시 dir에 .json 3개 | 3개 반환 |
| TC-11 | `../evil` 이름 | None |
| TC-12 | scen_dir + `passthrough` + task + domain=`customer_service` | Ok(trajectory), 에러 없음 |
| TC-13 | scen_dir + `passthrough` + task + domain=`bogus_xxx` | Err("domain not found: ...") |

## 완료 정의
- 모든 TC 통과
- 실제 서버에서 `POST /api/scenarios/customer_service/cs_001/run`, `POST /api/tools/classify_inquiry/invoke` 정상 응답
- verify_trace COMPLETE
- README 업데이트
