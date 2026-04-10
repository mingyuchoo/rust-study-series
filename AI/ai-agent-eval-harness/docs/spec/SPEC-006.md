# SPEC-006: 탭 기반 단일 파일 SPA

## 메타데이터
- SPEC ID: SPEC-006
- PRD: PRD-006
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-006 | FR-1 | 탭 네비게이션 |
| PRD-006 | FR-2 | Run 탭 |
| PRD-006 | FR-3 | Scenarios 탭 단일 실행 |
| PRD-006 | FR-4 | Tools 탭 invoke/fault |
| PRD-006 | FR-5 | Agents 탭 execute |
| PRD-006 | FR-6 | Reports 탭 compare |
| PRD-006 | FR-7 | Trajectories 탭 score |
| PRD-006 | FR-8 | Goldens 탭 |

### 역방향 추적
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|------------|------|
| TC-1 | 탭 버튼 7개 존재 | FR-1 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-2 | run 폼 요소 존재 | FR-2 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-3 | scenario run 함수 존재 | FR-3 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-4 | tool invoke 함수 존재 | FR-4 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-5 | agent execute 함수 존재 | FR-5 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-6 | compare 폼 요소 존재 | FR-6 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-7 | trajectories fetch 존재 | FR-7 | crates/eval-harness/src/web/handlers.rs | Draft |
| TC-8 | goldens fetch 존재 | FR-8 | crates/eval-harness/src/web/handlers.rs | Draft |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/web/index.html | tab 네비 + 8개 패널 JS | FR-1~FR-8 |
| eval-harness | src/web/handlers.rs | index_html_body + smoke tests | FR-1~FR-8 |

## 기술 설계

### 전체 구조
```
<header>
<nav class="tabs">  Run | Scenarios | Tools | Agents | Reports | Trajectories | Goldens
<main>
  <section id="tab-run"      class="panel">
  <section id="tab-scenarios" class="panel">
  ... (총 7 패널)
<footer>  (결과 영역 공용 또는 패널별 내장)
```

### JS 구조 (바닐라)
```js
const API = { get: (p) => fetch(p).then(r=>r.json()),
               post: (p,body) => fetch(p,{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify(body)}).then(r=>r.json()) };
const tabs = ['run','scenarios','tools','agents','reports','trajectories','goldens'];
function showTab(name) { ... }
async function initRun() { /* suite/agent 옵션 채움 */ }
async function initScenarios() { /* /api/list 로드 */ }
async function initTools() { /* /api/tools 로드 */ }
// ... 각 탭 초기화 함수
```

### 탭별 상세

**Run 탭**: `select[suite]` (customer_service/financial/all) + `select[agent]` (fetch /api/agents) + `input[output]` + `button Run` → `POST /api/run` → `<pre>` JSON 출력.

**Scenarios 탭**: `/api/list` 호출 → 좌측 트리(도메인>시나리오), 우측 상세 + `select[agent]` + `button Run this scenario` → `POST /api/scenarios/:d/:id/run`.

**Tools 탭**: `/api/tools` → 좌측 목록, 우측 선택된 도구 메타/스키마 + `textarea[params JSON]` + `button Invoke` (POST /api/tools/:n/invoke) + `button Simulate fault` (POST /api/tools/:n/simulate-fault).

**Agents 탭**: `/api/agents` → 좌측 목록, 우측 `textarea[task]` + `textarea[env JSON]` + `button Execute` → `POST /api/agents/:n/execute`.

**Reports 탭**: `/api/reports` 좌측 + 선택 상세 + **Compare** 서브패널: `select[baseline]` `select[current]` `input[threshold]` `button Compare` → `POST /api/compare`.

**Trajectories 탭**: `/api/trajectories` 좌측 + 선택 상세 + `button Score this trajectory` → 선택된 궤적 JSON을 body로 `POST /api/score`.

**Goldens 탭**: `/api/golden-sets` 전체 파일 + `input[domain]` `input[scenario_id]` + `button Fetch entry` → `GET /api/golden-sets/:d/:sid`.

### 테스트 전략
`handlers::index_html_body()`가 반환하는 문자열에 8개 탭의 핵심 키워드 존재를 확인하는 smoke test. 실제 DOM/브라우저 동작은 수동 검증.

## 테스트 시나리오
| TC | 확인 키워드 |
|----|------------|
| TC-1 | `data-tab="run"`, `data-tab="scenarios"`, `data-tab="tools"`, `data-tab="agents"`, `data-tab="reports"`, `data-tab="trajectories"`, `data-tab="goldens"` |
| TC-2 | `id="run-form"`, `/api/run` |
| TC-3 | `runScenario(`, `/api/scenarios/` + `/run` |
| TC-4 | `invokeTool(`, `/api/tools/` |
| TC-5 | `executeAgent(`, `/api/agents/` |
| TC-6 | `compareReports(`, `/api/compare` |
| TC-7 | `scoreTrajectory(`, `/api/score`, `/api/trajectories` |
| TC-8 | `fetchGolden(`, `/api/golden-sets/` |

## 완료 정의
- 모든 TC 통과
- `cargo run -- serve` 후 브라우저에서 7개 탭 모두 동작
- verify_trace COMPLETE
- README 간단 갱신
