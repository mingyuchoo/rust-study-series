# SPEC-025: PromptSet — DB 기반 도메인 프롬프트 번들 + 버전관리

## 메타데이터
- SPEC ID: SPEC-025
- 관련 PRD: PRD-025
- 작성일: 2026-04-11
- 상태: Draft

## 추적 정보

### 정방향 추적 (이 SPEC 이 구현하는 요구사항)
| PRD | FR ID | 요구사항 (요약) |
|-----|-------|----------------|
| PRD-025 | FR-1 | `prompt_sets` 테이블 — 도메인 × 버전 × 4 템플릿 번들 |
| PRD-025 | FR-2 | 기존 하드코딩 문구를 v1 bootstrap 으로 자동 시드 |
| PRD-025 | FR-3 | 런타임 `LlmClient` 가 활성 PromptSet 조회해 렌더링 |
| PRD-025 | FR-4 | 슬롯 규약 및 `str::replace` 기반 렌더러 |
| PRD-025 | FR-5 | 저장 시 필수 슬롯 검증 |
| PRD-025 | FR-6 | REST CRUD 엔드포인트 5종 |
| PRD-025 | FR-7 | 활성/bootstrap 삭제 금지 + 활성 전환 원자성 |
| PRD-025 | FR-8 | Trajectory 에 `prompt_set_id` 기록, 테이블 컬럼 추가 |
| PRD-025 | FR-9 | Web UI Domains 탭 "Prompts" 서브 섹션 + 궤적 배지 |

### 역방향 추적 (이 SPEC 을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | v6→v7 마이그레이션 멱등 | NFR-1 | (Phase 3) | Draft |
| TC-2  | bootstrap 시드가 기존 하드코딩 문구와 바이트-동등 | FR-2 | (Phase 3) | Draft |
| TC-3  | 재기동 시 bootstrap 재-시드 건너뜀 | NFR-2 | (Phase 3) | Draft |
| TC-4  | 렌더러 슬롯 치환 (정상) | FR-4 | (Phase 3) | Draft |
| TC-5  | 필수 슬롯 누락 검증 — policy_user 에서 `{tools}` 제거 → 400 + missing 목록 | FR-5 | (Phase 3) | Draft |
| TC-6  | POST 새 버전 → auto-increment version 반환 | FR-1, FR-6 | (Phase 3) | Draft |
| TC-7  | PUT activate 트랜잭션: 기존 활성 해제 + 새 활성 설정이 원자적 | FR-3, FR-7, NFR-4 | (Phase 3) | Draft |
| TC-8  | DELETE 활성 버전 → 409 | FR-7 | (Phase 3) | Draft |
| TC-9  | DELETE bootstrap 버전 → 409 | FR-7 | (Phase 3) | Draft |
| TC-10 | domain CASCADE → PromptSet 들도 삭제 | FR-1 | (Phase 3) | Draft |
| TC-11 | `LlmClient::create_perceive_prompt` 가 활성 PromptSet 을 사용 + id 반환 | FR-3 | (Phase 3) | Draft |
| TC-12 | `LlmClient::create_policy_prompt` 가 활성 PromptSet + tools 슬롯 렌더 | FR-3, FR-4 | (Phase 3) | Draft |
| TC-13 | Polling 도메인에 활성 PromptSet 없으면 `general` 폴백, 그 다음 컴파일 상수 폴백 | FR-3 | (Phase 3) | Draft |
| TC-14 | `PpaAgent::execute_task` 후 Trajectory.prompt_set_id 가 활성 버전의 id 와 일치 | FR-8 | (Phase 3) | Draft |
| TC-15 | `Trajectory` 의 `prompt_set_id` 필드가 `#[serde(default)]` 로 과거 JSON 파싱 호환 | FR-8 | (Phase 3) | Draft |
| TC-16 | REST GET 목록이 최신 버전 먼저 정렬 | FR-6 | (Phase 3) | Draft |
| TC-17 | 회귀: 기존 `customer_service`, `financial` 시나리오 실행 결과가 bootstrap 시드 사용해 SPEC-025 이전과 동일 | FR-2 | (Phase 3) | Draft |
| TC-18 | `index.html` body 에 `#prompts-section`, textarea id 4종(`ps-perc-sys`, `ps-perc-user`, `ps-pol-sys`, `ps-pol-user`), `savePromptSetNewVersion` 함수명이 포함 | FR-9 | (Phase 3) | Draft |

### 구현 추적 (이 SPEC 을 구현하는 코드)
| 패키지 | 파일 | 심볼 (함수/클래스) | 관련 FR |
|--------|------|-------------------|--------|
| `crates/data-scenarios` | `src/sqlite_store.rs` | `SCHEMA_VERSION=7`, `migrate_to_v7`, `PromptSetRow`, `list_prompt_sets`, `get_active_prompt_set`, `get_prompt_set`, `insert_prompt_set`, `activate_prompt_set`, `delete_prompt_set`, `seed_bootstrap_prompt_sets` | FR-1, FR-2, FR-6, FR-7, NFR-1, NFR-2, NFR-4 |
| `crates/agent-core` | `src/llm_client.rs` | `BOOTSTRAP_PERCEIVE_SYSTEM`, `BOOTSTRAP_PERCEIVE_USER`, `BOOTSTRAP_POLICY_SYSTEM`, `BOOTSTRAP_POLICY_USER`, `render_template`, `validate_required_slots`, `create_perceive_prompt (re-signed)`, `create_policy_prompt (re-signed)` | FR-3, FR-4, FR-5 |
| `crates/agent-core` | `src/agent.rs` | `PpaAgent::perceive_step`, `PpaAgent::policy_step`, `PpaAgent::execute_task` (도메인 해석 + Trajectory 기록) | FR-3, FR-8 |
| `crates/agent-models` | `src/models.rs` | `Trajectory::prompt_set_id: Option<i64>` (`#[serde(default)]`) | FR-8 |
| `crates/eval-harness` | `src/web/api_crud.rs` | `list_prompt_sets_handler`, `get_prompt_set_handler`, `create_prompt_set_handler`, `activate_prompt_set_handler`, `delete_prompt_set_handler`, `PromptSetCreatePayload`, slot validation | FR-5, FR-6, FR-7 |
| `crates/eval-harness` | `src/web/mod.rs` | 라우트 5종 추가 | FR-6 |
| `crates/eval-harness` | `src/web/index.html` | Domains 탭 "Prompts" 서브 섹션 HTML + JS 함수군(`refreshPromptSets`, `savePromptSetNewVersion`, `activatePromptSet`, `deletePromptSet`), 궤적 패널 배지 렌더링 | FR-9 |
| `crates/eval-harness` | `src/web/db_query.rs` | `Trajectory` JSON 반환 시 `prompt_set_id` 유지 확인 (없으면 추가) | FR-8 |

## 개요

PRD-025 의 9개 FR 을 다음 6 개 영역으로 분해해 구현:

1. **스키마 마이그레이션** — SqliteStore v7: `prompt_sets` 테이블 + `trajectories`/`evaluations` 에 `prompt_set_id` 컬럼 추가
2. **스토어 API** — `PromptSetRow` + 7개 CRUD/seed 메서드
3. **런타임 연결** — `LlmClient` 시그니처 확장(`domain` 인자), 렌더러 도입, Bootstrap 상수 분리, Trajectory 에 id 기록
4. **REST API** — axum 라우트 5개 + 슬롯 검증 로직 + payload 구조체
5. **Web UI** — Domains 탭 서브 섹션 + 궤적 배지
6. **테스트** — 단위/통합 17개 TC

## 대상 패키지

- `crates/data-scenarios` — v7 마이그레이션, PromptSet CRUD, bootstrap 시드
- `crates/agent-core` — 렌더러, LlmClient 시그니처 확장, PpaAgent 의 domain 전달 및 Trajectory 기록
- `crates/agent-models` — Trajectory 필드 추가
- `crates/eval-harness` — REST 핸들러, 라우트, Web UI
- `crates/domains` — 변경 없음 (검증만)
- `crates/execution` — `PpaAgent::execute_task` 호출부가 domain 컨텍스트를 넘기는지 점검 (기존 라우터가 결정한 domain 이 그대로 흐르는지)

## 기술 설계

### 데이터 모델 (v7 마이그레이션)

```sql
-- 1) PromptSet 번들
CREATE TABLE IF NOT EXISTS prompt_sets (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_name     TEXT NOT NULL,
    version         INTEGER NOT NULL,
    perceive_system TEXT NOT NULL,
    perceive_user   TEXT NOT NULL,
    policy_system   TEXT NOT NULL,
    policy_user     TEXT NOT NULL,
    notes           TEXT,
    is_active       INTEGER NOT NULL DEFAULT 0,
    is_bootstrap    INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (domain_name, version),
    FOREIGN KEY (domain_name) REFERENCES domains(name) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_prompt_sets_active_per_domain
    ON prompt_sets(domain_name) WHERE is_active = 1;

CREATE INDEX IF NOT EXISTS idx_prompt_sets_domain
    ON prompt_sets(domain_name);

-- 2) 궤적/평가 추적 컬럼
ALTER TABLE trajectories ADD COLUMN prompt_set_id INTEGER;
ALTER TABLE evaluations  ADD COLUMN prompt_set_id INTEGER;
```

멱등성 확보:
- `SCHEMA_VERSION` 을 현재 6 → 7 로 올리고, `migrate(current, target)` 에서 6→7 구간에만 실행
- `ALTER TABLE ADD COLUMN` 은 `PRAGMA table_info` 로 존재 여부 확인 후 수행 (SQLite 는 `IF NOT EXISTS` 미지원)

### Rust API

```rust
// crates/data-scenarios/src/sqlite_store.rs
pub struct PromptSetRow {
    pub id: i64,
    pub domain_name: String,
    pub version: i64,
    pub perceive_system: String,
    pub perceive_user: String,
    pub policy_system: String,
    pub policy_user: String,
    pub notes: Option<String>,
    pub is_active: bool,
    pub is_bootstrap: bool,
    pub created_at: String,
}

pub struct PromptSetInsert<'a> {
    pub domain_name: &'a str,
    pub perceive_system: &'a str,
    pub perceive_user: &'a str,
    pub policy_system: &'a str,
    pub policy_user: &'a str,
    pub notes: Option<&'a str>,
    pub is_bootstrap: bool,
}

impl SqliteStore {
    pub async fn list_prompt_sets(&self, domain: &str) -> Result<Vec<PromptSetRow>, StoreError>;
    pub async fn get_prompt_set(&self, domain: &str, version: i64) -> Result<Option<PromptSetRow>, StoreError>;
    pub async fn get_active_prompt_set(&self, domain: &str) -> Result<Option<PromptSetRow>, StoreError>;
    pub async fn insert_prompt_set(&self, row: PromptSetInsert<'_>) -> Result<i64 /* new version */, StoreError>;
    pub async fn activate_prompt_set(&self, domain: &str, version: i64) -> Result<(), StoreError>;
    pub async fn delete_prompt_set(&self, domain: &str, version: i64) -> Result<(), StoreError>;
    pub async fn seed_bootstrap_prompt_sets(&self) -> Result<(), StoreError>;
}
```

`insert_prompt_set` 알고리즘:
1. 트랜잭션 시작
2. `SELECT COALESCE(MAX(version),0)+1 FROM prompt_sets WHERE domain_name=?` 로 next version 계산
3. 첫 삽입이면 `is_active=1` 자동 부여, 그 외는 0
4. `INSERT` 수행 및 commit
5. 새 version 반환

`activate_prompt_set` 알고리즘:
1. 트랜잭션 시작
2. `UPDATE prompt_sets SET is_active=0 WHERE domain_name=? AND is_active=1`
3. `UPDATE prompt_sets SET is_active=1 WHERE domain_name=? AND version=?`
   - 0 rows affected → rollback + `NotFound`
4. commit

`delete_prompt_set` 알고리즘:
1. 행 조회 → 없으면 `NotFound`
2. `is_active=1` 또는 `is_bootstrap=1` → `ConstraintViolation("cannot delete active/bootstrap")`
3. `DELETE FROM prompt_sets WHERE domain_name=? AND version=?`

`seed_bootstrap_prompt_sets` 알고리즘:
1. `domains` 테이블의 모든 이름 조회
2. 각 도메인마다 `SELECT COUNT(*) FROM prompt_sets WHERE domain_name=?` 로 존재 여부 확인
3. 0이면 `insert_prompt_set(PromptSetInsert { is_bootstrap: true, .. })` 호출
4. 값 본문은 `agent_core::llm_client::{BOOTSTRAP_PERCEIVE_SYSTEM, BOOTSTRAP_PERCEIVE_USER, BOOTSTRAP_POLICY_SYSTEM, BOOTSTRAP_POLICY_USER}` 를 그대로 사용
   - `data-scenarios` 가 `agent-core` 를 의존하면 순환 위험 → 해결: bootstrap 상수를 `agent-models` 로 옮기거나, `seed_bootstrap_prompt_sets` 가 문자열 파라미터를 받는 형태(`seed_bootstrap_prompt_sets(&self, bundle: &BootstrapBundle)`)로 외부 주입
   - **채택**: 외부 주입. 호출자(`eval-harness` 기동 또는 `PpaAgent::load_all_tools` 옆)에서 `BOOTSTRAP_*` 상수를 모아 `BootstrapBundle` 로 넘김

### 렌더러 (agent-core)

```rust
// crates/agent-core/src/llm_client.rs
pub const BOOTSTRAP_PERCEIVE_SYSTEM: &str = "..."; // 기존 문구 그대로
pub const BOOTSTRAP_PERCEIVE_USER: &str   = "작업: {task_description}\n\n환경 상태:\n{environment_state}{context}\n\n위 정보를 분석하여 JSON 형식으로 출력하세요.";
pub const BOOTSTRAP_POLICY_SYSTEM: &str   = "..."; // 기존 문구 그대로
pub const BOOTSTRAP_POLICY_USER: &str     = "작업: {task_description}\n\n인지된 정보:\n{perceived_info}\n\n사용 가능한 도구:\n{tools}{context}\n\n위 도구 중 적절한 것을 선택하고...";

pub struct BootstrapBundle {
    pub perceive_system: &'static str,
    pub perceive_user:   &'static str,
    pub policy_system:   &'static str,
    pub policy_user:     &'static str,
}
pub const BOOTSTRAP_BUNDLE: BootstrapBundle = BootstrapBundle {
    perceive_system: BOOTSTRAP_PERCEIVE_SYSTEM,
    perceive_user:   BOOTSTRAP_PERCEIVE_USER,
    policy_system:   BOOTSTRAP_POLICY_SYSTEM,
    policy_user:     BOOTSTRAP_POLICY_USER,
};

/// 단순 슬롯 치환. 미사용 슬롯은 그대로 남는다.
pub fn render_template(tmpl: &str, vars: &HashMap<&str, String>) -> String {
    let mut out = tmpl.to_string();
    for (k, v) in vars {
        out = out.replace(&format!("{{{}}}", k), v);
    }
    out
}

/// 필수 슬롯이 모두 존재하는지 확인 (저장 시 검증용).
pub fn validate_required_slots(field: &str, tmpl: &str) -> Vec<String> {
    let required: &[&str] = match field {
        "perceive_user" => &["{task_description}", "{environment_state}"],
        "policy_user"   => &["{task_description}", "{perceived_info}", "{tools}"],
        _ => return Vec::new(), // system 은 필수 슬롯 없음
    };
    required.iter()
        .filter(|slot| !tmpl.contains(*slot))
        .map(|s| (*s).to_string())
        .collect()
}
```

`create_perceive_prompt` / `create_policy_prompt` 시그니처 변경:

```rust
pub async fn create_perceive_prompt(
    domain: &str,                                       // 신규
    task: &str,
    environment_state: &HashMap<String, serde_json::Value>,
    context: Option<&HashMap<String, serde_json::Value>>,
) -> (Vec<Message>, Option<i64> /* prompt_set_id */)
```

알고리즘:
1. `try_installed_store()` → `get_active_prompt_set(domain)`
2. 없으면 `get_active_prompt_set("general")` 폴백
3. 그래도 없으면 `(BOOTSTRAP_BUNDLE.perceive_system, BOOTSTRAP_BUNDLE.perceive_user, id=None)` 사용
4. `vars` 맵 조립:
   - `task_description` → `task`
   - `environment_state` → `format!("{:?}", environment_state)`
   - `context` → context 가 Some 이면 `format!("\n이전 맥락: {:?}", c)` 아니면 빈 문자열
   - `domain_name` → `domain.to_string()`
5. `render_template` 2회 (system, user)
6. `(vec![Message::system(rendered_sys), Message::user(rendered_user)], prompt_set_id)`

`create_policy_prompt` 도 동일 패턴 + `tools` 슬롯(호출자가 사전에 현재 로직으로 조립한 문자열) 주입.

호출부(`PpaAgent::perceive_step`, `policy_step`) 는 반환된 `Option<i64>` 를 Trajectory 에 기록 (첫 스텝의 id 를 유지).

### Trajectory 필드

```rust
// crates/agent-models/src/models.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub task_id: String,
    pub task_description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub steps: Vec<PpaStep>,
    pub final_state: Option<AgentState>,
    pub success: bool,
    pub total_iterations: u32,
    #[serde(default)]
    pub prompt_set_id: Option<i64>,
}
```

`PpaAgent::execute_task` 에서 최초 `create_perceive_prompt` 호출 결과의 `Option<i64>` 를 `self.trajectory.prompt_set_id` 에 저장.

### REST 핸들러 (api_crud.rs)

```rust
#[derive(Deserialize)]
pub struct PromptSetCreatePayload {
    pub perceive_system: String,
    pub perceive_user:   String,
    pub policy_system:   String,
    pub policy_user:     String,
    #[serde(default)]
    pub notes: Option<String>,
}

pub async fn list_prompt_sets_handler(
    Path(domain): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PromptSetRow>>, (StatusCode, Json<ErrorBody>)>;

pub async fn get_prompt_set_handler(
    Path((domain, version)): Path<(String, i64)>,
    State(state): State<AppState>,
) -> Result<Json<PromptSetRow>, (StatusCode, Json<ErrorBody>)>;

pub async fn create_prompt_set_handler(
    Path(domain): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<PromptSetCreatePayload>,
) -> Result<Json<PromptSetRow>, (StatusCode, Json<ErrorBody>)> {
    // 1) 슬롯 검증
    let mut missing = serde_json::Map::new();
    for (field, tmpl) in [
        ("perceive_system", &payload.perceive_system),
        ("perceive_user",   &payload.perceive_user),
        ("policy_system",   &payload.policy_system),
        ("policy_user",     &payload.policy_user),
    ] {
        let m = validate_required_slots(field, tmpl);
        if !m.is_empty() { missing.insert(field.into(), json!(m)); }
    }
    if !missing.is_empty() {
        return Err((BAD_REQUEST, Json(ErrorBody::new_with("missing slots", missing))));
    }
    // 2) insert
    ...
}

pub async fn activate_prompt_set_handler(
    Path((domain, version)): Path<(String, i64)>,
    State(state): State<AppState>,
) -> Result<Json<PromptSetRow>, (StatusCode, Json<ErrorBody>)>;

pub async fn delete_prompt_set_handler(
    Path((domain, version)): Path<(String, i64)>,
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, Json<ErrorBody>)> {
    // 404 없음, 409 active/bootstrap, 아니면 204
}
```

### 라우트 등록 (mod.rs)

```rust
.route("/api/domains/:name/prompts",
    get(api_crud::list_prompt_sets_handler)
    .post(api_crud::create_prompt_set_handler))
.route("/api/domains/:name/prompts/:version",
    get(api_crud::get_prompt_set_handler)
    .delete(api_crud::delete_prompt_set_handler))
.route("/api/domains/:name/prompts/:version/activate",
    put(api_crud::activate_prompt_set_handler))
```

### Web UI

`index.html` Domains 탭의 `#panel-domains` 에 `<div id="prompts-section">` 추가. 구조:

```html
<div class="form-block" id="prompts-section" hidden>
  <h3 data-i18n="prompts.title">Prompt sets</h3>
  <div class="split">
    <div class="list" id="prompts-list"></div>
    <div>
      <div class="form-block">
        <h3 id="prompt-editor-title">version editor</h3>
        <label><span>perceive_system</span>
          <textarea id="ps-perc-sys"></textarea>
        </label>
        <label><span>perceive_user (must contain {task_description}, {environment_state})</span>
          <textarea id="ps-perc-user"></textarea>
        </label>
        <label><span>policy_system</span>
          <textarea id="ps-pol-sys"></textarea>
        </label>
        <label><span>policy_user (must contain {task_description}, {perceived_info}, {tools})</span>
          <textarea id="ps-pol-user"></textarea>
        </label>
        <label><span>notes</span>
          <input id="ps-notes" type="text" />
        </label>
        <div class="row">
          <button class="primary" onclick="savePromptSetNewVersion()">새 버전으로 저장</button>
          <button class="secondary" onclick="activateSelectedPromptSet()">이 버전 활성화</button>
          <button class="danger" onclick="deleteSelectedPromptSet()">삭제</button>
        </div>
        <pre id="ps-out" data-i18n="common.idle">Ready</pre>
      </div>
    </div>
  </div>
</div>
```

JS 함수:
- `refreshPromptSets(domain)` — `GET /api/domains/:name/prompts`, 리스트 구성, 활성 배지 표시 (.active 선택 하이라이트 기존 패턴 재사용)
- `loadPromptSetIntoEditor(row)` — 선택한 버전의 4 필드를 textarea 에 채움. `is_bootstrap` 또는 `is_active` 면 "편집은 항상 새 버전으로" 힌트 표시
- `savePromptSetNewVersion()` — `showPending` + POST → 성공 시 `refreshPromptSets` + 새 version 자동 선택
- `activateSelectedPromptSet()` — PUT activate → 리프레시
- `deleteSelectedPromptSet()` — DELETE, 409 시 에러 표시

도메인 선택(`editDomain(d)`) 시 `#prompts-section` hidden 해제 + `refreshPromptSets(d.name)` 호출.

궤적 패널: `refreshTrajectories` 의 각 항목 클릭 시 응답에 `trajectory.prompt_set_id` 가 있으면 `traj-out` 위에 배지 `<div id="traj-badge">prompt_set #{id}</div>` 렌더.

## 대상 파일 상세

```
crates/data-scenarios/src/sqlite_store.rs
  + PromptSetRow / PromptSetInsert 구조체
  + SCHEMA_VERSION 6→7
  + migrate_to_v7 (prompt_sets 테이블 + ALTER trajectories/evaluations)
  + list_prompt_sets / get_prompt_set / get_active_prompt_set / insert_prompt_set
  + activate_prompt_set / delete_prompt_set / seed_bootstrap_prompt_sets

crates/agent-core/src/llm_client.rs
  + BOOTSTRAP_* 상수 4개 + BOOTSTRAP_BUNDLE
  + render_template / validate_required_slots
  * create_perceive_prompt 시그니처 변경: (domain, task, env, ctx) -> (msgs, Option<i64>)
  * create_policy_prompt  시그니처 변경: (domain, task, perceived, tools, ctx) -> (msgs, Option<i64>)

crates/agent-core/src/agent.rs
  * PpaAgent::perceive_step / policy_step 에 domain 인자 전달
  * 첫 perceive 호출 결과의 Option<i64> 를 self.trajectory.prompt_set_id 에 저장
  * domain 해석: 현재 task 의 domain_router 결과를 사용 (없으면 "general")

crates/agent-models/src/models.rs
  + Trajectory::prompt_set_id: Option<i64> (#[serde(default)])

crates/eval-harness/src/web/api_crud.rs
  + PromptSetCreatePayload
  + list/get/create/activate/delete 핸들러 5개

crates/eval-harness/src/web/mod.rs
  + 3개 route() 추가

crates/eval-harness/src/web/index.html
  + #prompts-section 블록
  + JS: refreshPromptSets, loadPromptSetIntoEditor, savePromptSetNewVersion,
        activateSelectedPromptSet, deleteSelectedPromptSet
  + traj-badge 렌더링

crates/eval-harness/src/main.rs (또는 bin 진입점)
  + 기동 시퀀스: store.init_schema() 후 store.seed_bootstrap_prompt_sets(BOOTSTRAP_BUNDLE) 호출
```

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1  | v6→v7 마이그레이션 멱등 | 기존 v6 DB 로드 → init_schema → 재실행 | 에러 없음, prompt_sets 테이블 존재, trajectories.prompt_set_id 컬럼 존재 | unit | NFR-1 |
| TC-2  | bootstrap 시드가 하드코딩 상수와 동등 | `seed_bootstrap_prompt_sets(BOOTSTRAP_BUNDLE)` 후 `get_active_prompt_set("customer_service")` | 4개 필드가 BOOTSTRAP_* 상수와 문자열 동등 + is_bootstrap=true + version=1 | unit | FR-2 |
| TC-3  | 재기동 시 재-시드 건너뜀 | seed 2회 연속 호출 | 도메인당 행 수 1개 유지 | unit | NFR-2 |
| TC-4  | render_template 정상 치환 | `"hi {a} {b}"`, `{a:"1", b:"2"}` | `"hi 1 2"` | unit | FR-4 |
| TC-5  | validate_required_slots | policy_user 에서 `{tools}` 제거 | `["{tools}"]` 반환 | unit | FR-5 |
| TC-6  | POST 새 버전 생성 | 유효 payload | 200, 반환 PromptSetRow.version = 2 (bootstrap 이 v1) | integration | FR-1, FR-6 |
| TC-7  | activate 원자성 | v1 active 상태에서 v2 activate 호출 | v1.is_active=false AND v2.is_active=true, `idx_prompt_sets_active_per_domain` 충돌 없음 | integration | FR-3, FR-7, NFR-4 |
| TC-8  | 활성 버전 DELETE | 활성 v2 에 DELETE | 409 + 에러 메시지 "cannot delete active version" | integration | FR-7 |
| TC-9  | bootstrap DELETE | v1 (bootstrap) 에 DELETE | 409 + "cannot delete bootstrap version" | integration | FR-7 |
| TC-10 | 도메인 CASCADE | 도메인 삭제 | 해당 도메인의 prompt_sets 0행 | unit | FR-1 |
| TC-11 | create_perceive_prompt 활성 사용 | 테스트 store 에 v2 활성, v2.perceive_system="X" | 반환 system 메시지가 "X" + id=v2.id | unit | FR-3 |
| TC-12 | create_policy_prompt tools 슬롯 | v1 bootstrap 사용 | user 메시지 안에 tools 문자열이 정확히 주입 | unit | FR-3, FR-4 |
| TC-13 | 폴백 체인 | `xxx` 도메인 활성 없음, `general` 도 없음 | BOOTSTRAP_BUNDLE 사용, id=None | unit | FR-3 |
| TC-14 | Trajectory.prompt_set_id 기록 | execute_task 완료 후 trajectory 조회 | `prompt_set_id` == 활성 버전의 id | integration | FR-8 |
| TC-15 | 레거시 JSON 파싱 | `prompt_set_id` 없는 구 Trajectory JSON 역직렬화 | Ok, field=None | unit | FR-8 |
| TC-16 | 목록 정렬 | GET list 반환 | version DESC 정렬 | unit | FR-6 |
| TC-17 | 회귀 | customer_service cs_001 실행 전/후 점수 비교 | 동일 (bootstrap 문구 = 하드코딩과 바이트 동등이므로) | integration | FR-2 |
| TC-18 | Web UI 정적 자산 검증 | `include_str!("web/index.html")` 문자열 | `#prompts-section`, `ps-perc-sys`, `ps-perc-user`, `ps-pol-sys`, `ps-pol-user`, `savePromptSetNewVersion` 포함 | unit | FR-9 |

TC-1~TC-5, TC-10, TC-13, TC-15, TC-16, TC-18 은 외부 의존 없는 단위 테스트.
TC-6~TC-9, TC-11, TC-12, TC-14, TC-17 은 in-memory SQLite store + 테스트 store install 패턴으로 통합.

## 구현 단계

1. **L1**: SqliteStore v7 마이그레이션 + PromptSetRow + 7 CRUD/seed 메서드 (TC-1, 2, 3, 10, 16)
2. **L2**: agent-core 의 BOOTSTRAP_* 상수 분리 + render_template + validate_required_slots (TC-4, 5)
3. **L3**: LlmClient 시그니처 변경 + 폴백 로직 + 호출부(`PpaAgent::perceive_step`, `policy_step`) 수정 (TC-11, 12, 13)
4. **L4**: Trajectory.prompt_set_id 필드 + PpaAgent 기록 + web/db_query.rs 전파 (TC-14, 15)
5. **L5**: REST 핸들러 5개 + 슬롯 검증 + 라우트 등록 (TC-6, 7, 8, 9)
6. **L6**: Web UI Prompts 서브 섹션 + 궤적 배지
7. **L7**: 회귀 테스트 + `cargo test --workspace` 0 failures (TC-17)

각 단계 종료 후 `cargo test --workspace --no-fail-fast`. L6 은 브라우저 수동 확인 포함.

## 위험 및 완화

| 위험 | 완화 |
|-----|------|
| `data-scenarios` ↔ `agent-core` 순환 의존 가능성 | bootstrap 상수는 `agent-core` 에만 두고, `data-scenarios::seed_bootstrap_prompt_sets` 가 외부 주입 파라미터로 받음 (위 설계 채택) |
| `ALTER TABLE ADD COLUMN` 이 이미 존재할 때 에러 | `PRAGMA table_info` 로 사전 체크 + 조건 실행 |
| 기존 Trajectory JSON 파일(파일 기반 로더)에 `prompt_set_id` 키 없음 | `#[serde(default)]` 로 해결 |
| 렌더러 무한 루프(슬롯 값이 다른 슬롯 토큰을 포함) | `str::replace` 는 원본에서만 치환하므로 이미 안전. 치환된 값 안의 `{...}` 는 다시 치환되지 않음 |
| 활성 토글 경합(두 클라이언트가 동시에 activate) | 단일 트랜잭션 + partial unique index 로 두 번째가 실패. 호출자가 재시도 |
| 궤적 배지 UI 가 도메인-버전 매핑을 알아야 함 | Trajectory 응답에 `prompt_set_id` 만 있어도 `GET /api/domains/:name/prompts/:version` 한 번의 조회로 표시 가능. 또는 execute 응답에 version 을 즉시 반환하는 후속 개선 |

## 완료 정의 (Definition of Done)

- 17개 TC 모두 통과
- `cargo test --workspace` 0 failures
- 브라우저 수동 확인: Domains 탭 Prompts 섹션에서 버전 생성 → 활성화 → 시나리오 실행 → 궤적 탭에 버전 배지 표시
- `verify_trace.py` 로 추적성 검증 통과
- 기존 하드코딩 `create_perceive_prompt` / `create_policy_prompt` 의 body 는 제거되고 렌더러 경유
- `BOOTSTRAP_BUNDLE` 이 `llm_client.rs` 에 남아 폴백 용도로 사용됨 (삭제 금지)
