# SPEC-019: 시나리오/골든셋 CRUD 관리 기능

## 메타데이터
- SPEC ID: SPEC-019
- PRD: PRD-019
- 작성일: 2026-04-11
- 상태: Draft

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-019 | FR-1 | 평가 시나리오 REST CRUD |
| PRD-019 | FR-2 | 골든셋 엔트리 REST CRUD |
| PRD-019 | FR-3 | DB 단독 쓰기(파일 역동기화 금지) |
| PRD-019 | FR-4 | 시나리오 삭제 시 골든셋 cascade |
| PRD-019 | FR-5 | 409/404/400 에러 매핑 |
| PRD-019 | FR-6 | 웹 UI 관리 탭 |
| PRD-019 | FR-7 | read-after-write 일관성 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | `insert_scenario` 가 신규 시나리오를 저장하고 `load_all_domains` 로 조회된다 | FR-1 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-2  | 동일 `(domain, id)` 로 `insert_scenario` 재호출 시 `StoreError::Conflict` 반환 | FR-5 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-3  | `update_scenario` 가 존재하는 행의 task/environment 를 갱신한다 | FR-1 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-4  | `update_scenario` 가 없는 행이면 `StoreError::NotFound` | FR-5 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-5  | `delete_scenario` 는 연결된 golden_set 엔트리까지 cascade 로 제거한다 | FR-4 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-6  | `insert_golden_entry` 가 신규 엔트리를 저장하고 `load_golden_sets_by_domain` 에 포함된다 | FR-2 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-7  | `update_golden_entry` / `delete_golden_entry` 의 CRUD 라운드트립과 NotFound 처리 | FR-2, FR-5 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-8  | 마이그레이션 v2: 기존 v1 DB 에 FK 가 추가되어도 데이터 손실이 없다 | FR-4 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-9  | `POST /api/eval-scenarios` 가 201 + 본문 반환, 재호출 시 409 | FR-1, FR-5 | crates/eval-harness/src/web/api_crud.rs | Draft |
| TC-10 | `PUT /api/eval-scenarios/:domain/:id` 200 / 404 분기 | FR-1, FR-5 | crates/eval-harness/src/web/api_crud.rs | Draft |
| TC-11 | `DELETE /api/eval-scenarios/:domain/:id` 204 + 연결된 골든셋 API 조회 시 404 | FR-1, FR-4 | crates/eval-harness/src/web/api_crud.rs | Draft |
| TC-12 | `POST/PUT/DELETE /api/golden-sets/:domain[/:scenario_id]` 전체 사이클 200/201/204/404 | FR-2, FR-5 | crates/eval-harness/src/web/api_crud.rs | Draft |
| TC-13 | CRUD 호출이 `eval_data/eval_scenarios/*.yaml` 과 `eval_data/golden_sets/*.json` 의 mtime 을 변경하지 않는다 | FR-3 | crates/eval-harness/src/web/api_crud.rs | Draft |
| TC-14 | 필수 필드 누락 본문에 대해 400 Bad Request + 에러 JSON 반환 | FR-5 | crates/eval-harness/src/web/api_crud.rs | Draft |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|--------|
| crates/data-scenarios | src/sqlite_store.rs | `SqliteStore::insert_scenario`, `update_scenario`, `delete_scenario`, `insert_golden_entry`, `update_golden_entry`, `delete_golden_entry`, `migrate_v2_cascade`, `StoreError::{Conflict,NotFound}` | FR-1, FR-2, FR-4, FR-5 |
| crates/eval-harness | src/web/api_crud.rs (신규) | `create_scenario`, `update_scenario_handler`, `delete_scenario_handler`, `create_golden_entry`, `update_golden_entry_handler`, `delete_golden_entry_handler`, `CrudError`, `EvalScenarioUpsert`, `GoldenEntryUpsert` | FR-1, FR-2, FR-5, FR-7 |
| crates/eval-harness | src/web/mod.rs | `AppState.store: Arc<SqliteStore>`, `Router` 에 CRUD 라우트 7개 추가 | FR-1, FR-2, FR-7 |
| crates/eval-harness | src/web/index.html | "관리" 탭, 도메인/시나리오/골든셋 리스트, JSON 편집기, fetch 호출 | FR-6 |

## 개요

SPEC-017 에서 구축된 `SqliteStore` 를 확장하여 쓰기 메서드(INSERT/UPDATE/DELETE)를 추가하고, axum 웹 계층에 REST CRUD 엔드포인트를 노출한다. 웹 UI 에는 "관리(Manage)" 탭을 추가하여 폼 기반 편집 경험을 제공한다.

쓰기는 **DB 단독**이며, 기존 `eval_data/eval_scenarios/*.yaml`, `eval_data/golden_sets/*.json` 파일은 초기 seed 소스로만 사용된다. 파일 역동기화는 수행하지 않는다.

```
[Web UI 관리 탭]
  └─ fetch POST /api/eval-scenarios ──┐
                                     ▼
                          [api_crud handlers]
                                     │  validate → SqliteStore::insert_scenario(&tx, ...)
                                     ▼
                          [SqliteStore (sqlx Pool)]
                                     │  INSERT / UPDATE / DELETE
                                     ▼
                          eval_data/eval_harness.db  (단일 Source of Truth)
```

## 기술 설계

### DB 스키마 마이그레이션 v2

SPEC-017 의 v1 스키마는 `golden_sets` 에 FK 가 없다. v2 에서 cascade 를 보장하기 위해 테이블을 재생성한다:

```sql
-- schema_migrations version=2
PRAGMA foreign_keys = ON;

BEGIN;

CREATE TABLE IF NOT EXISTS golden_sets_v2 (
    domain            TEXT NOT NULL,
    scenario_id       TEXT NOT NULL,
    version           TEXT NOT NULL DEFAULT '1.0',
    task              TEXT NOT NULL,
    input_environment TEXT NOT NULL,
    tool_sequence     TEXT NOT NULL,
    tool_results      TEXT NOT NULL,
    tolerance         REAL NOT NULL DEFAULT 0.01,
    PRIMARY KEY (domain, scenario_id),
    FOREIGN KEY (domain, scenario_id)
        REFERENCES eval_scenarios(domain, id)
        ON DELETE CASCADE
);

INSERT INTO golden_sets_v2
  SELECT domain, scenario_id, version, task, input_environment,
         tool_sequence, tool_results, tolerance
  FROM golden_sets;

DROP TABLE golden_sets;
ALTER TABLE golden_sets_v2 RENAME TO golden_sets;

CREATE INDEX IF NOT EXISTS idx_golden_sets_domain ON golden_sets(domain);

INSERT INTO schema_migrations (version, applied_at) VALUES (2, datetime('now'));

COMMIT;
```

마이그레이션은 `SqliteStore::init_schema` 내부에서 `schema_migrations` 조회 후 현재 버전 < 2 이면 위 블록을 실행한다. 신규 DB 는 처음부터 v2 스키마로 생성한다(v1 블록을 실행한 뒤 v2 가 실행되어도 이미 같은 결과가 되도록 `IF NOT EXISTS` 와 `INSERT OR IGNORE` 로 멱등 처리). `PRAGMA foreign_keys = ON` 은 모든 커넥션에서 활성화해야 하므로 `SqlitePoolOptions::after_connect` 훅에서 설정한다.

### `SqliteStore` CRUD API

```rust
// crates/data-scenarios/src/sqlite_store.rs

impl SqliteStore {
    pub async fn insert_scenario(
        &self,
        domain: &str,
        scenario: &EvalScenario,
    ) -> Result<(), StoreError>;

    pub async fn update_scenario(
        &self,
        domain: &str,
        id: &str,
        scenario: &EvalScenario,
    ) -> Result<(), StoreError>;

    pub async fn delete_scenario(
        &self,
        domain: &str,
        id: &str,
    ) -> Result<(), StoreError>;

    pub async fn insert_golden_entry(
        &self,
        domain: &str,
        entry: &GoldenSetEntry,
    ) -> Result<(), StoreError>;

    pub async fn update_golden_entry(
        &self,
        domain: &str,
        scenario_id: &str,
        entry: &GoldenSetEntry,
    ) -> Result<(), StoreError>;

    pub async fn delete_golden_entry(
        &self,
        domain: &str,
        scenario_id: &str,
    ) -> Result<(), StoreError>;
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    // ... 기존 variants
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("not found: {0}")]
    NotFound(String),
}
```

**구현 규칙:**
- `insert_*` 은 `INSERT INTO ... VALUES (...)` 를 사용하며, sqlx 의 `sqlite::SqliteError::code() == Some("2067")` (UNIQUE constraint) 를 `StoreError::Conflict` 로 매핑한다. (`1555` 도 허용)
- `update_*` / `delete_*` 는 `rows_affected() == 0` 이면 `StoreError::NotFound` 를 반환한다.
- 동적 필드(`initial_environment`, `success_criteria`, `input.environment`, `expected_output.tool_results`)는 SPEC-017 과 동일하게 `serde_json::to_string` 으로 TEXT 컬럼에 저장한다.
- 모든 쓰기는 단일 statement 이므로 명시적 트랜잭션을 사용하지 않는다 (sqlx 의 auto-commit 에 의존). 다만 마이그레이션 v2 는 `BEGIN...COMMIT` 으로 래핑.

### Web 레이어: `api_crud.rs`

```rust
// crates/eval-harness/src/web/api_crud.rs
use axum::{extract::{Path, State, Json as JsonExt},
           http::StatusCode,
           response::{IntoResponse, Json as JsonOut}};
use data_scenarios::sqlite_store::{SqliteStore, StoreError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct EvalScenarioUpsert {
    pub domain: String,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub task_description: String,
    pub initial_environment: serde_json::Value,
    pub expected_tools: Vec<String>,
    pub success_criteria: serde_json::Value,
    pub difficulty: Option<String>,
    pub position: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GoldenEntryUpsert {
    pub scenario_id: String,
    pub version: Option<String>,
    pub task: String,
    pub input_environment: serde_json::Value,
    pub tool_sequence: Vec<String>,
    pub tool_results: serde_json::Value,
    pub tolerance: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CrudError {
    pub error: &'static str,
    pub detail: String,
}

impl From<StoreError> for (StatusCode, JsonOut<CrudError>) {
    fn from(err: StoreError) -> Self {
        use StoreError::*;
        let (status, kind) = match &err {
            Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            _           => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        };
        (status, JsonOut(CrudError { error: kind, detail: err.to_string() }))
    }
}

pub async fn create_scenario(
    State(store): State<Arc<SqliteStore>>,
    JsonExt(body): JsonExt<EvalScenarioUpsert>,
) -> impl IntoResponse { /* validate → store.insert_scenario → 201 */ }

pub async fn update_scenario_handler(
    State(store): State<Arc<SqliteStore>>,
    Path((domain, id)): Path<(String, String)>,
    JsonExt(body): JsonExt<EvalScenarioUpsert>,
) -> impl IntoResponse { /* store.update_scenario → 200 / 404 */ }

pub async fn delete_scenario_handler(
    State(store): State<Arc<SqliteStore>>,
    Path((domain, id)): Path<(String, String)>,
) -> impl IntoResponse { /* store.delete_scenario → 204 / 404 */ }

// golden-set 핸들러 3개 동일 패턴
```

**검증 규칙 (핸들러 진입 직후):**
- `domain`/`id`/`scenario_id` 는 정규식 `^[a-z0-9_\-]+$` 및 1..=64 바이트.
- `task_description` / `task` 는 1..=4096 바이트, 비어있으면 400.
- `expected_tools` / `tool_sequence` 는 빈 배열 허용 가능 (명시적으로 허용).
- JSON 파싱 실패 시 axum 자동 400.

### Router 변경

```rust
// crates/eval-harness/src/web/mod.rs (build_router 내부)
.route("/api/eval-scenarios",                post(api_crud::create_scenario))
.route("/api/eval-scenarios/:domain/:id",    put(api_crud::update_scenario_handler)
                                            .delete(api_crud::delete_scenario_handler))
.route("/api/golden-sets/:domain",           post(api_crud::create_golden_entry))
.route("/api/golden-sets/:domain/:scenario_id",
                                             put(api_crud::update_golden_entry_handler)
                                            .delete(api_crud::delete_golden_entry_handler))
```

기존 `GET` 라우트는 그대로 유지된다. `AppState` 에는 `pub store: Arc<SqliteStore>` 를 추가하고, `run_server` 시그니처에 `store` 파라미터를 추가한다(이미 `SqliteStore` 를 보유한 `main.rs` 에서 전달).

### Web UI (`index.html`)

기존 탭 구조에 `<button data-tab="manage">관리</button>` 를 추가하고, 새 패널을 다음과 같이 구성한다:

```html
<section data-panel="manage" hidden>
  <div class="manage-layout">
    <aside class="manage-domains">
      <h3>도메인</h3>
      <ul id="manage-domain-list"></ul>
    </aside>
    <main class="manage-list">
      <div class="toolbar">
        <button id="new-scenario-btn">+ 새 시나리오</button>
        <button id="new-golden-btn">+ 새 골든셋</button>
      </div>
      <table id="manage-scenario-table"><thead>...</thead><tbody></tbody></table>
      <table id="manage-golden-table"><thead>...</thead><tbody></tbody></table>
    </main>
    <aside class="manage-editor">
      <h3 id="editor-title">편집</h3>
      <textarea id="editor-json" rows="20"></textarea>
      <div class="editor-actions">
        <button id="editor-save">저장</button>
        <button id="editor-delete" class="danger">삭제</button>
      </div>
      <div id="editor-status" role="status"></div>
    </aside>
  </div>
</section>
```

JavaScript (Vanilla, 새 함수들):
```js
async function loadManageDomains() { /* GET /api/scenarios → 도메인 목록 */ }
async function loadManageScenarios(domain) { /* GET /api/eval-scenarios/:domain */ }
async function saveScenario(payload, {isNew, domain, id}) {
  const url = isNew ? '/api/eval-scenarios' : `/api/eval-scenarios/${domain}/${id}`;
  const method = isNew ? 'POST' : 'PUT';
  const res = await fetch(url, { method, headers: {'Content-Type':'application/json'},
                                 body: JSON.stringify(payload) });
  if (!res.ok) throw new Error((await res.json()).detail || res.statusText);
  return res.json();
}
async function deleteScenario(domain, id) { /* DELETE ... */ }
// golden-set 핸들러 동일 패턴
```

테마 토큰(`--bg`, `--panel`, `--accent`)은 기존 값을 재사용한다.

### `AppState`/`main.rs` 수정

```rust
// web/mod.rs
#[derive(Clone)]
pub struct AppState {
    pub scenarios_dir:    PathBuf,
    pub reports_dir:      PathBuf,
    pub golden_sets_dir:  PathBuf,
    pub trajectories_dir: PathBuf,
    pub store:            Arc<SqliteStore>,  // 신규
}

pub fn run_server(
    addr: SocketAddr,
    scenarios_dir: PathBuf,
    reports_dir: PathBuf,
    golden_sets_dir: PathBuf,
    trajectories_dir: PathBuf,
    store: Arc<SqliteStore>,                 // 신규 파라미터
) -> io::Result<()> { ... }
```

`main.rs` 의 `cmd_web` 에서 이미 보유한 `SqliteStore` (SPEC-017) 를 `Arc` 로 감싸 전달한다.

### 검증 전략

- **단위 테스트 (data-scenarios)**: `sqlx::SqlitePool::connect("sqlite::memory:")` + `init_schema` + seed 후 CRUD 호출. `tokio::test`.
- **핸들러 테스트 (eval-harness)**: axum `Router` 를 `build_router(AppState { store: mem_store, ... })` 로 만든 뒤 `tower::ServiceExt::oneshot` 으로 요청을 보내 응답 코드/바디 검증. tempdir 의 더미 YAML/JSON seed 파일 mtime 을 기록하고, CRUD 후 동일한 mtime 을 재확인한다 (TC-13).
- **실패/경계값**: 빈 JSON, 잘못된 domain, 존재하지 않는 엔트리, 중복 insert, delete cascade 확인.

## 대상 패키지
- `crates/data-scenarios`: `sqlite_store.rs` CRUD 메서드 + StoreError variants + 마이그레이션 v2
- `crates/eval-harness`: `web/api_crud.rs` 신규 + `web/mod.rs` 라우트/AppState + `web/index.html` 관리 탭 + `main.rs` store 전달

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | FR |
|-------|---------|------|----------|------|----|
| TC-1  | insert_scenario 신규 | 신규 `EvalScenario` | `load_all_domains` 결과에 포함 | 단위 | FR-1 |
| TC-2  | insert 중복 | 동일 `(domain,id)` 2회 | 두 번째 `StoreError::Conflict` | 단위 | FR-5 |
| TC-3  | update_scenario 정상 | 기존 행 + 새 task | 조회 시 새 task 반영 | 단위 | FR-1 |
| TC-4  | update 없는 행 | 존재하지 않는 id | `StoreError::NotFound` | 단위 | FR-5 |
| TC-5  | delete cascade | scenario 삭제 | 연결된 golden_set 도 삭제됨 | 단위 | FR-4 |
| TC-6  | insert_golden_entry | 신규 entry | `load_golden_sets_by_domain` 에 포함 | 단위 | FR-2 |
| TC-7  | golden entry update/delete | CRUD 사이클 | 각 단계 일관된 결과, 없는 건 NotFound | 단위 | FR-2, FR-5 |
| TC-8  | 마이그레이션 v2 | v1 스키마 DB + 기존 데이터 | 마이그 후 FK 활성화, 데이터 보존 | 단위 | FR-4 |
| TC-9  | POST /api/eval-scenarios | 유효 body | 201 + body echo, 재호출 409 | 통합(핸들러) | FR-1, FR-5 |
| TC-10 | PUT /api/eval-scenarios/:d/:i | 유효 body | 200 / 없는 건 404 | 통합 | FR-1, FR-5 |
| TC-11 | DELETE /api/eval-scenarios | 기존 id | 204, cascade 로 golden 404 | 통합 | FR-1, FR-4 |
| TC-12 | golden CRUD API 사이클 | POST→PUT→DELETE | 201/200/204/404 | 통합 | FR-2, FR-5 |
| TC-13 | 파일 mtime 불변 | 모든 CRUD 시퀀스 | seed 파일 mtime 변화 없음 | 통합 | FR-3 |
| TC-14 | 잘못된 body | 필수 필드 누락 | 400 + `{error:"bad_request"}` | 통합 | FR-5 |

## 구현 가이드

- 파일:
  - `crates/data-scenarios/src/sqlite_store.rs` (수정 — CRUD + 마이그레이션 v2)
  - `crates/eval-harness/src/web/api_crud.rs` (신규)
  - `crates/eval-harness/src/web/mod.rs` (라우트/AppState/run_server 시그니처)
  - `crates/eval-harness/src/web/index.html` (관리 탭)
  - `crates/eval-harness/src/main.rs` (cmd_web 에서 store 전달)

- 순서:
  1. `sqlite_store.rs` 에 `StoreError::Conflict/NotFound` 추가 + CRUD 메서드 스텁 → RED 테스트(TC-1~8) 작성.
  2. CRUD 메서드 구현 + 마이그레이션 v2 구현 → GREEN.
  3. `api_crud.rs` 스텁 + 핸들러 테스트(TC-9~14) 작성 → RED.
  4. 핸들러 구현 → GREEN.
  5. `web/mod.rs` 라우트 연결 + `AppState.store` 필드 + `main.rs` 전달.
  6. `index.html` 관리 탭 + JS fetch 호출.
  7. `cargo test --workspace` 전체 통과.
  8. 수동 E2E: `cargo run -- web` → 관리 탭에서 CRUD 플로우 확인.

## 완료 정의 (Definition of Done)
- [ ] TC-1 ~ TC-14 모두 통과
- [ ] `cargo build --workspace` 통과
- [ ] `cargo test --workspace` 통과
- [ ] 수동 E2E: 웹 UI 에서 시나리오/골든셋 CRUD 플로우 동작
- [ ] 시나리오 삭제 시 연결된 골든셋 자동 삭제 확인
- [ ] 서버 재기동 후 변경 유지 확인 (파일 seed 로 되돌아가지 않음)
- [ ] 추적성 매트릭스에 SPEC-019 등록
- [ ] README 에 CRUD API 섹션 추가
