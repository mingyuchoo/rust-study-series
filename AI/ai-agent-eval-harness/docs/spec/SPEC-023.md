# SPEC-023: HttpCallTool — 외부 HTTP 도구 동적 등록

## 메타데이터
- SPEC ID: SPEC-023
- 관련 PRD: PRD-023
- 작성일: 2026-04-11
- 상태: Draft

## 개요
PRD-023 의 7개 FR 을 충족하는 설계:
1. SQLite v6 마이그레이션: `external_tools` 테이블 + CRUD 메서드
2. `crates/execution-tools/src/http_tool.rs` 신규: `HttpCallTool` struct
3. `crates/agent-core/src/agent.rs::PpaAgent::load_all_tools` 끝에 `register_external_tools_from_db()` 헬퍼 호출
4. REST 엔드포인트 5종 + URL allowlist 검증
5. Web UI Domains 탭에 external tools 섹션

## 데이터 모델

### v6 테이블
```sql
CREATE TABLE IF NOT EXISTS external_tools (
    name           TEXT NOT NULL,
    domain         TEXT NOT NULL,
    description    TEXT NOT NULL DEFAULT '',
    method         TEXT NOT NULL DEFAULT 'POST',
    url            TEXT NOT NULL,
    headers_json   TEXT,
    body_template  TEXT NOT NULL,
    params_schema  TEXT NOT NULL,
    timeout_ms     INTEGER NOT NULL DEFAULT 10000,
    created_at     TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (domain, name),
    FOREIGN KEY (domain) REFERENCES domains(name) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_external_tools_domain ON external_tools(domain);
```

`SCHEMA_VERSION = 6`. `CREATE TABLE IF NOT EXISTS` 만 추가하면 멱등 마이그레이션 가능.

### Rust API

```rust
// crates/data-scenarios/src/sqlite_store.rs
pub async fn list_external_tools(&self) -> Result<Vec<ExternalToolRow>, StoreError>;
pub async fn list_external_tools_by_domain(&self, domain: &str) -> Result<Vec<ExternalToolRow>, StoreError>;
pub async fn upsert_external_tool(&self, row: &ExternalToolRow) -> Result<(), StoreError>;
pub async fn delete_external_tool(&self, domain: &str, name: &str) -> Result<(), StoreError>;

pub struct ExternalToolRow {
    pub name: String,
    pub domain: String,
    pub description: String,
    pub method: String,
    pub url: String,
    pub headers_json: Option<String>,
    pub body_template: String,
    pub params_schema: String,
    pub timeout_ms: i64,
}
```

```rust
// crates/execution-tools/src/http_tool.rs
pub struct HttpCallTool {
    metadata: ToolMetadata,
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body_template: String,
    timeout_ms: u64,
}

impl HttpCallTool {
    pub fn from_row(row: &ExternalToolRow) -> Result<Self, String>;
}

impl BaseTool for HttpCallTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }
    fn execute(&self, params: &HashMap<String, Value>) -> HashMap<String, Value> { ... }
}
```

`HttpCallTool::execute` 알고리즘:
1. `body_template` 의 `{{key}}` → `params[key]` 치환 (직렬화: number/bool/string)
2. `reqwest::blocking::Client::builder().timeout(Duration::from_millis(timeout_ms)).build()`
3. `client.request(Method::from_bytes(...), &url).headers(...).body(rendered).send()`
4. `status.is_success() && content-type starts with application/json` →  `serde_json::from_str` 로 응답 파싱
5. 응답이 JSON 객체면 그 키들을 그대로 결과 HashMap 에 펼침 + `success: true`
6. 그 외 → `{success: false, error: "..."}`

### 등록 헬퍼

```rust
// crates/domains/src/lib.rs (또는 agent-core)
pub fn register_external_tools_from_db(registry: &mut ToolRegistry) {
    let Some(store) = data_scenarios::loader::try_installed_store() else { return; };
    let rows = match block_on(store.list_external_tools()) {
        Ok(r) => r,
        Err(e) => { eprintln!("[warn] external_tools 로드 실패: {e}"); return; }
    };
    for row in rows {
        match HttpCallTool::from_row(&row) {
            Ok(tool) => registry.register_with_domain(Arc::new(tool), &row.domain),
            Err(e) => eprintln!("[warn] HttpCallTool '{}' 등록 실패: {e}", row.name),
        }
    }
}
```

`PpaAgent::load_all_tools()` 의 마지막 줄에 호출 추가.

### URL allowlist

```rust
fn url_allowed(url: &str) -> bool {
    let Ok(allow) = std::env::var("EVAL_HARNESS_HTTP_TOOL_ALLOWLIST") else { return true; };
    if allow.trim().is_empty() { return true; }
    allow.split(',').map(str::trim).any(|prefix| url.starts_with(prefix))
}
```

REST 핸들러(create/update)에서 호출, 실패 시 400.

### REST

| Method | Path | 비고 |
|--------|------|------|
| GET | `/api/external-tools` | 전체 |
| GET | `/api/external-tools/:domain` | 도메인별 |
| POST | `/api/external-tools/:domain` | 신규 |
| PUT | `/api/external-tools/:domain/:name` | 갱신 |
| DELETE | `/api/external-tools/:domain/:name` | 삭제 |

### Web UI

Domains 탭의 도메인 편집기 하단에 "External Tools" 섹션 추가:
- 도메인 선택 시 해당 도메인의 external tools 목록 자동 로드
- 신규/편집 폼 (name, description, method, url, headers JSON, body template, params schema JSON, timeout)
- 저장 시 POST/PUT, 삭제 버튼

## 테스트 계획

| TC ID | 시나리오 | 입력 | 기대 | 유형 | FR |
|-------|--------|------|------|------|-----|
| TC-1 | v6 마이그레이션 멱등 | init_schema 2회 | external_tools 1개, 에러 없음 | unit | NFR-1 |
| TC-2 | upsert + list | 신규 row 삽입 → list | 1행 반환 | unit | FR-1 |
| TC-3 | delete cascade by domain | 도메인 삭제 | 해당 external_tools 도 삭제 | unit | FR-4 |
| TC-4 | HttpCallTool body 치환 | `{"q":"{{topic}}"}` + params={topic: "AI"} | 렌더된 body 가 `{"q":"AI"}` | unit | FR-3 |
| TC-5 | HttpCallTool 응답 파싱 (mock) | mock 200 + `{"x":1}` | result 에 `x: 1` + success: true | unit | FR-3 |
| TC-6 | HttpCallTool 4xx 응답 | mock 400 | success: false + error 메시지 | unit | FR-3 |
| TC-7 | URL allowlist 검증 | env=`https://api.x/`, url=`http://evil/` | url_allowed false | unit | FR-5 |
| TC-8 | URL allowlist 빈 환경 | env unset | 모든 URL 허용 | unit | FR-5 |
| TC-9 | register_external_tools_from_db | DB에 row 2개 + register | registry 에 2개 추가 | unit | FR-2 |

TC-1 ~ TC-4, TC-7, TC-8 은 외부 의존 없이 단위 테스트로 구현.
TC-5, TC-6 은 mock HTTP server (예: `wiremock` 또는 `tokio::net` 으로 작은 listener) 가 필요하므로 본 SPEC 단계에서는 단순한 동기 mock 으로 대체하거나 통합 테스트로 미룬다.
TC-9 는 in-memory store + register 흐름으로 검증.

## 구현 단계

1. **L2-2**: SqliteStore v6 + ExternalToolRow + CRUD 메서드
2. **L2-3**: `crates/execution-tools/src/http_tool.rs::HttpCallTool` 구현
3. **L2-4**: `register_external_tools_from_db` 헬퍼 + `agent.rs::load_all_tools` 통합
4. **L2-5**: REST 핸들러 + URL allowlist + Web UI
5. **L2-6**: 단위 테스트 + 회귀 통과

각 단계 종료 후 `cargo test --workspace` 0 failures.
