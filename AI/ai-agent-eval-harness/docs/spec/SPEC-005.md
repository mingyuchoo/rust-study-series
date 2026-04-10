# SPEC-005: CLI-Web 동등화 확장

## 메타데이터
- SPEC ID: SPEC-005
- PRD: PRD-005
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-005 | FR-1 | POST /api/run output |
| PRD-005 | FR-2 | POST /api/compare output |
| PRD-005 | FR-3 | GET /api/list 집계 |

### 역방향 추적
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|------------|------|
| TC-1 | run_suite_impl 기본 저장 | FR-1 | crates/eval-harness/src/web/api.rs | Draft |
| TC-2 | run_suite_impl output 지정 저장 | FR-1 | crates/eval-harness/src/web/api.rs | Draft |
| TC-3 | run output 경로 이탈 거부 | FR-1 | crates/eval-harness/src/web/api.rs | Draft |
| TC-4 | compare_impl output 저장 | FR-2 | crates/eval-harness/src/web/api.rs | Draft |
| TC-5 | compare output 없을 때 저장 안 함 | FR-2 | crates/eval-harness/src/web/api.rs | Draft |
| TC-6 | compare output 경로 이탈 거부 | FR-2 | crates/eval-harness/src/web/api.rs | Draft |
| TC-7 | list_all_impl 집계 | FR-3 | crates/eval-harness/src/web/api.rs | Draft |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/web/api.rs | run_suite_with_save_impl, compare_with_save_impl, list_all_impl, RunResponse, CompareResponse, ListAllResponse | FR-1~FR-3 |
| eval-harness | src/web/mod.rs | build_router (`/api/list` 추가) | FR-3 |

## 기술 설계

### 응답 래퍼
```rust
#[derive(Serialize)]
pub struct RunResponse {
    pub report: EvaluationReport,
    pub saved_to: String,  // 절대 경로 문자열
}

#[derive(Serialize)]
pub struct CompareResponse {
    pub result: ComparisonResult,
    pub saved_to: Option<String>,
}

#[derive(Serialize)]
pub struct ListAllResponse {
    pub domains: Vec<DomainSummary>,
    pub agents: Vec<String>,
}
```

### 확장 요청 구조체
```rust
#[derive(Deserialize)]
pub struct RunRequest {
    pub suite: String,
    pub agent: String,
    #[serde(default)]
    pub output: Option<String>,  // 신규
}

#[derive(Deserialize)]
pub struct CompareRequest {
    pub baseline: String,
    pub current: String,
    #[serde(default = "default_threshold")]
    pub threshold: f64,
    #[serde(default)]
    pub output: Option<String>,  // 신규
}
```

### 순수 함수
```rust
pub fn run_suite_with_save_impl(
    suite: &str, agent_name: &str, scenarios_dir: &Path, reports_dir: &Path,
    output: Option<&str>,
) -> Result<(EvaluationReport, String), String>;

pub fn compare_with_save_impl(
    baseline: &str, current: &str, threshold: f64, reports_dir: &Path,
    output: Option<&str>,
) -> Result<(ComparisonResult, Option<String>), String>;

pub fn list_all_impl(scen_dir: &Path) -> ListAllResponse;
```

### 핸들러 업데이트
기존 `run_suite`, `compare_reports` 핸들러를 확장된 impl로 변경. 응답 타입을 `Json<RunResponse>`, `Json<CompareResponse>`로 교체.

### 경로 안전성
- `run`: `output`이 Some 이면 `is_safe_name` 통과 확인. 실패 시 400.
- `compare`: 동일.
- 저장 경로: `reports_dir.join(output)`.

### 기존 FR-5/FR-6 재사용
기존 `run_suite_impl`, `compare_impl`의 로직을 재사용하고 저장 단계만 추가. 중복 구현을 피하기 위해 내부에서 기존 impl을 호출한다.

## 테스트 시나리오 (요약)
| TC | 입력 | 기대 |
|----|------|------|
| TC-1 | output=None | 기본 파일명(`evaluation_report_*.json`) 디렉토리에 생성 |
| TC-2 | output="my.json" | `reports_dir/my.json` 생성 |
| TC-3 | output="../evil" | Err("invalid output name") |
| TC-4 | output="cmp.json" | 저장 경로 Some, 파일 존재 |
| TC-5 | output=None | 저장 경로 None, 추가 파일 없음 |
| TC-6 | output="../evil" | Err |
| TC-7 | - | `domains` 비어있지 않음 && `agents` 에 `"passthrough"` 포함 |

## 완료 정의
- 모든 TC 통과
- 라이브 스모크: `curl POST /api/run`에 `output` 지정 시 파일 생성 확인
- README 에 `/api/list`, 확장 `output` 필드 문서화
- verify_trace COMPLETE
