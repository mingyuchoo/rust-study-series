# SPEC-002: Axum 기반 웹 서버 및 정적 SPA

## 메타데이터
- SPEC ID: SPEC-002
- PRD: PRD-002
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-002 | FR-1 | serve 서브커맨드 |
| PRD-002 | FR-2 | GET /api/scenarios |
| PRD-002 | FR-3 | GET /api/reports |
| PRD-002 | FR-4 | GET /api/reports/:name |
| PRD-002 | FR-5 | GET / (index.html) |

### 역방향 추적
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|------------|------|
| TC-1  | serve 서브커맨드 파싱 | FR-1 | crates/eval-harness/src/web/mod.rs | Draft |
| TC-2  | scenarios 핸들러가 YAML 로드 | FR-2 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-3  | reports 핸들러가 .json 파일 나열 | FR-3 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-4  | report content 반환 | FR-4 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-5  | 경로 순회 거부 | FR-4 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-6  | index.html 임베드 반환 | FR-5 | crates/eval-harness/src/web/handlers.rs | Draft |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/web/mod.rs | run_server, build_router, AppState | FR-1 |
| eval-harness | src/web/handlers.rs | list_scenarios, list_reports, get_report, index, is_safe_name | FR-2~FR-5 |
| eval-harness | src/web/index.html | 정적 SPA | FR-5 |
| eval-harness | src/main.rs | Commands::Serve | FR-1 |

## 개요

`eval-harness` CLI에 `serve` 서브커맨드 추가. Axum으로 HTTP 서버를 띄우고, 4개 JSON API + 1개 정적 HTML을 제공한다.

## 기술 설계

### 의존성 (워크스페이스 공용)
- `axum = "0.7"`
- `tokio` (이미 있음)
- `tower-http` (서빙 헬퍼, 이미 있음)

### 라우트
```
GET  /                        → index.html (include_str!)
GET  /api/scenarios           → Vec<DomainSummary>
GET  /api/reports             → Vec<String>  (파일명)
GET  /api/reports/:name       → serde_json::Value (파일 원문) | 404
```

### 데이터 모델
```rust
#[derive(Clone)]
pub struct AppState {
    pub scenarios_dir: PathBuf,
    pub reports_dir: PathBuf,
}

#[derive(Serialize)]
pub struct DomainSummary {
    pub name: String,
    pub description: String,
    pub scenarios: Vec<ScenarioSummary>,
}

#[derive(Serialize)]
pub struct ScenarioSummary {
    pub id: String,
    pub name: String,
    pub difficulty: String,
}
```

### 경로 안전성
`is_safe_name(s: &str) -> bool`: `s`가 `/`, `\`, `..` 를 포함하지 않고 비어있지 않으면 true. 불통과 시 404.

### 테스트 전략
- 핸들러 함수를 순수 함수로 분리해 `State`/`Path` 추출 없이 내부 로직(`list_reports_impl`, `get_report_impl`, `is_safe_name`)을 직접 호출하여 단위 테스트.
- 통합 테스트(axum 라우터) 는 생략 (단위로 충분).

## 대상 패키지
- `crates/eval-harness/`

## 테스트 시나리오
| TC ID | 시나리오 | 입력 | 기대 결과 | FR |
|-------|---------|------|----------|----|
| TC-1  | serve 서브커맨드 파싱 | `["eval-harness","serve"]` | `Commands::Serve` (compile) | FR-1 |
| TC-2  | scenarios 핸들러 로딩 | 임시 YAML 디렉토리 | DomainSummary 리스트 반환 | FR-2 |
| TC-3  | reports 목록 | 임시 dir에 .json 3개, .txt 1개 | 3개 반환 | FR-3 |
| TC-4  | report content | 존재 파일 | Some(Value) | FR-4 |
| TC-5  | 경로 순회 | `"../etc/passwd"` | is_safe_name=false | FR-4 |
| TC-6  | index.html 비어있지 않음 | - | include_str!이 non-empty | FR-5 |

## 완료 정의
- 모든 TC 통과
- `curl http://127.0.0.1:8080/api/scenarios` 동작
- README 업데이트
- verify_trace 통과
