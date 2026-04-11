# SPEC-022: 동적 도메인 CRUD + 도구 카탈로그

## 메타데이터
- SPEC ID: SPEC-022
- 관련 PRD: PRD-022
- 작성일: 2026-04-11
- 상태: Draft

## 개요
PRD-022 의 7개 FR 을 충족시키기 위한 설계:
1. SQLite v5 마이그레이션: `domain_keywords` 테이블 신규
2. `SqliteStore` 에 도메인 단위 CRUD 메서드 + 키워드 관리 API
3. `domain_router` 의 const 상수를 RwLock 캐시 + DB lazy load 로 교체
4. REST 엔드포인트 4종 + 도구 카탈로그 API
5. Web UI Manage 탭에 Domains 섹션
6. 부트스트랩 도메인 보호 정책

## 데이터 모델

### 신규 테이블 (v5)

```sql
CREATE TABLE IF NOT EXISTS domain_keywords (
    domain  TEXT NOT NULL,
    keyword TEXT NOT NULL,
    PRIMARY KEY (domain, keyword),
    FOREIGN KEY (domain) REFERENCES domains(name) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_domain_keywords_domain ON domain_keywords(domain);
```

`SCHEMA_VERSION` 을 5로 올린다. 마이그레이션은 `CREATE TABLE IF NOT EXISTS` + 기본 키워드 시드.

### Rust API (`SqliteStore`)

```rust
pub async fn insert_domain(&self, name: &str, description: &str) -> Result<(), StoreError>;
pub async fn update_domain(&self, name: &str, description: &str) -> Result<(), StoreError>;
pub async fn delete_domain(&self, name: &str) -> Result<(), StoreError>;
pub async fn list_domain_summaries(&self) -> Result<Vec<DomainSummary>, StoreError>;
pub async fn get_domain_summary(&self, name: &str) -> Result<Option<DomainSummary>, StoreError>;

pub async fn replace_domain_tools(&self, domain: &str, tool_class_names: &[String]) -> Result<(), StoreError>;
pub async fn replace_domain_keywords(&self, domain: &str, keywords: &[String]) -> Result<(), StoreError>;
pub async fn list_all_domain_keywords(&self) -> Result<HashMap<String, Vec<String>>, StoreError>;
```

```rust
pub struct DomainSummary {
    pub name: String,
    pub description: String,
    pub tool_class_names: Vec<String>,
    pub keywords: Vec<String>,
    pub scenario_count: i64,
    pub is_bootstrap: bool,
}
```

### 라우터 캐시

```rust
// crates/agent-core/src/domain_router.rs
static KEYWORD_CACHE: RwLock<Option<HashMap<String, Vec<String>>>> = RwLock::new(None);

pub fn select_domains(task_description: &str, top_k: usize) -> Vec<String> { ... }
pub fn invalidate_cache() { /* write lock → set None */ }
```

CRUD 핸들러가 도메인/키워드를 변경할 때마다 `invalidate_cache()` 호출.

### REST 엔드포인트

| Method | Path | Body / Response |
|--------|------|---------|
| GET | `/api/domains` | `Vec<DomainSummaryDto>` |
| GET | `/api/domains/:name` | `DomainSummaryDto` |
| POST | `/api/domains` | body: `DomainUpsert { name, description, tool_class_names, keywords }` |
| PUT | `/api/domains/:name` | body: `DomainUpsert` (name 무시) |
| DELETE | `/api/domains/:name` | 부트스트랩이면 409 |
| GET | `/api/tools/catalog` | `Vec<serde_json::Value>` (이미 있는 `list_tools_impl()` 결과 그대로) |

### Web UI

Manage 탭 좌측 panel 에 "도메인" 섹션 추가:
- 목록(부트스트랩은 자물쇠 아이콘)
- 신규/편집 폼: 이름·설명·도구 multi-select·키워드 chip 입력
- 저장 시 POST/PUT 호출 후 라우터 캐시는 서버 측에서 자동 invalidate

## 테스트 계획

| TC ID | 시나리오 | 입력 | 기대 | 유형 | FR |
|-------|--------|------|------|------|-----|
| TC-1 | v5 마이그레이션 멱등 | 빈 DB → init_schema 2회 | domain_keywords 테이블, 에러 없음 | unit | NFR-1 |
| TC-2 | 부트스트랩 키워드 시드 | v5 마이그레이션 후 | financial/customer_service 의 키워드 N개 이상 존재 | unit | FR-7 |
| TC-3 | insert_domain + update_domain | "healthcare" → upsert | list 에 포함, description 갱신 반영 | unit | FR-1 |
| TC-4 | replace_domain_tools | tool_class_names 3→2 변경 | domain_tools 테이블 정확히 2행 | unit | FR-1 |
| TC-5 | replace_domain_keywords | 키워드 5→3 변경 | domain_keywords 정확히 3행 | unit | FR-3 |
| TC-6 | delete_domain cascade | "healthcare" 삭제 | scenarios/goldens/tools/keywords 모두 cascade 삭제 | unit | FR-4 |
| TC-7 | delete_domain 부트스트랩 차단 | "financial" 삭제 시도 | StoreError::Conflict | unit | FR-5 |
| TC-8 | list_all_domain_keywords | 다중 도메인 + 다중 키워드 | HashMap 정확히 매핑 | unit | FR-3 |
| TC-9 | domain_router 캐시 lazy load | 첫 select_domains 호출 | DB 에서 키워드 로드 | unit | FR-3 |
| TC-10 | invalidate_cache 후 재로드 | 키워드 변경 → invalidate → select | 새 키워드 반영 | unit | FR-3 |
| TC-11 | REST POST /api/domains | 신규 도메인 body | 200 + 도메인 list 반영 | integration | FR-1 |
| TC-12 | REST DELETE 부트스트랩 | 409 | integration | FR-5 |
| TC-13 | GET /api/tools/catalog | 호출 | Rust 컴파일 도구 ≥6 | unit | FR-2 |

TC-1 ~ TC-13 은 단위/통합 테스트로 본 SPEC 의 GREEN 단계에서 모두 추가.

## 구현 단계

1. **L1-2**: SqliteStore v5 + CRUD 메서드 + DomainSummary struct
2. **L1-3**: 부트스트랩 키워드 자동 시드
3. **L1-4**: domain_router 캐시 리팩터 + invalidate API
4. **L1-5**: REST 핸들러 + 카탈로그 + 부트스트랩 보호
5. **L1-6**: index.html Domains 섹션 + JS 글루
6. **L1-7**: 단위/통합 테스트 + 회귀 통과

각 단계 종료 후 `cargo test --workspace` 0 failures 확인.
