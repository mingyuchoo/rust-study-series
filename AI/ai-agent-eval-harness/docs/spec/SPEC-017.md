# SPEC-017: eval_data SQLite 저장소 전환

## 메타데이터
- SPEC ID: SPEC-017
- PRD: PRD-017
- 작성일: 2026-04-11
- 상태: Implemented

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-017 | FR-1 | SQLite 단일 DB 에 시나리오/골든셋 저장 |
| PRD-017 | FR-2 | 기존 파일을 seed 소스로 유지, idempotent 자동 적재 |
| PRD-017 | FR-3 | `ScenarioLoader` 를 DB 조회로 전환, 호출부 시그니처 유지 |
| PRD-017 | FR-4 | `eval-harness.toml` 의 `db_path` 설정 |
| PRD-017 | FR-5 | 진입점 공통 `EvalDataStore::open_and_seed` API |
| PRD-017 | FR-6 | `sqlx` SQLite + tokio 비동기, 동기 브릿지 |
| PRD-017 | FR-7 | 동적 필드는 JSON TEXT 컬럼 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1 | 빈 DB 에서 seed 후 두 도메인 모두 적재된다 | FR-1, FR-2 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-2 | seed 재실행 시 중복 insert 가 발생하지 않는다 (idempotent) | FR-2 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-3 | `load_all_domains` 가 `financial`, `customer_service` 를 id 순으로 반환 | FR-3 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-4 | 특정 도메인의 골든셋을 `scenario_id` 로 조회할 수 있다 | FR-3, FR-7 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-5 | 시나리오의 `initial_environment` JSON 이 round-trip 된다 | FR-7 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-6 | 골든셋의 `tool_sequence`/`tool_results` 가 round-trip 된다 | FR-7 | crates/data-scenarios/src/sqlite_store.rs | Draft |
| TC-7 | `db_path` TOML 키가 `DataPaths` 로 해석되어 상대 경로가 설정 파일 기준으로 결합된다 | FR-4 | crates/eval-harness/src/data_paths.rs | Draft |
| TC-8 | `ScenarioLoader::new()` 와 `load_all_domains` 호출부 API 가 기존과 호환 | FR-3 | crates/data-scenarios/src/loader.rs | Draft |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|--------|
| crates/data-scenarios | src/sqlite_store.rs | `SqliteStore`, `SqliteStore::open`, `SqliteStore::init_schema`, `SqliteStore::seed_from_fs`, `SqliteStore::load_all_domains`, `SqliteStore::load_golden_sets_by_domain` | FR-1, FR-2, FR-3, FR-6, FR-7 |
| crates/data-scenarios | src/loader.rs | `ScenarioLoader::load_all_domains`, `ScenarioLoader::load_golden_sets`, `ScenarioLoader::load_all_golden_sets` | FR-3 |
| crates/data-scenarios | src/lib.rs | `pub mod sqlite_store;` | FR-1 |
| crates/data-scenarios | Cargo.toml | `sqlx`, `tokio` deps | FR-6 |
| crates/eval-harness | src/data_paths.rs | `DataPaths::db_path`, `DEFAULT_DB_PATH`, `ENV_DB_PATH`, toml `db_path` 파싱 | FR-4 |
| crates/eval-harness | src/main.rs | 기동 시 `SqliteStore::open_and_seed` 호출 | FR-5 |
| crates/eval-harness | src/web/mod.rs | `AppState` 에 store 핸들 주입 | FR-5 |

## 개요

`eval_data/` 하위 YAML/JSON 파일의 로딩을 SQLite 기반 저장소 `SqliteStore` 로 교체한다. `data-scenarios` 크레이트 내부에 `sqlite_store` 모듈을 신설하고, 기존 `ScenarioLoader` 는 얇은 래퍼로 남겨 호출부 영향을 최소화한다.

동작 흐름:

```
[기동]
  ↓
DataPaths::load  →  db_path, scenarios_dir(seed), golden_sets_dir(seed)
  ↓
SqliteStore::open(db_path)         ← 파일 없으면 생성
  ↓
SqliteStore::init_schema()          ← 멱등 CREATE TABLE IF NOT EXISTS
  ↓
is_empty() ?  → yes → seed_from_fs(scenarios_dir, golden_sets_dir)
  ↓
ScenarioLoader::new(store)          ← 기존 인터페이스 제공
  ↓
[CLI/TUI/Web/Desktop 기존 로직 그대로]
```

## 기술 설계

### DB 스키마

```sql
CREATE TABLE IF NOT EXISTS schema_migrations (
    version     INTEGER PRIMARY KEY,
    applied_at  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS domains (
    name        TEXT PRIMARY KEY,
    description TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS domain_tools (
    domain      TEXT NOT NULL,
    class_name  TEXT NOT NULL,
    module_path TEXT NOT NULL,
    position    INTEGER NOT NULL,
    PRIMARY KEY (domain, class_name),
    FOREIGN KEY (domain) REFERENCES domains(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS eval_scenarios (
    domain              TEXT NOT NULL,
    id                  TEXT NOT NULL,
    name                TEXT NOT NULL,
    description         TEXT NOT NULL DEFAULT '',
    task_description    TEXT NOT NULL,
    initial_environment TEXT NOT NULL,   -- JSON object
    expected_tools      TEXT NOT NULL,   -- JSON array of strings
    success_criteria    TEXT NOT NULL,   -- JSON object
    difficulty          TEXT NOT NULL DEFAULT 'medium',
    position            INTEGER NOT NULL,
    PRIMARY KEY (domain, id),
    FOREIGN KEY (domain) REFERENCES domains(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS golden_sets (
    domain            TEXT NOT NULL,
    scenario_id       TEXT NOT NULL,
    version           TEXT NOT NULL DEFAULT '1.0',
    task              TEXT NOT NULL,
    input_environment TEXT NOT NULL,    -- JSON object
    tool_sequence     TEXT NOT NULL,    -- JSON array of strings
    tool_results      TEXT NOT NULL,    -- JSON object
    tolerance         REAL NOT NULL DEFAULT 0.01,
    PRIMARY KEY (domain, scenario_id)
);

CREATE INDEX IF NOT EXISTS idx_eval_scenarios_domain ON eval_scenarios(domain);
CREATE INDEX IF NOT EXISTS idx_golden_sets_domain ON golden_sets(domain);
```

### `SqliteStore` API

```rust
// crates/data-scenarios/src/sqlite_store.rs
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub struct SqliteStore {
    pool: SqlitePool,
}

impl SqliteStore {
    /// 파일 경로로 DB 를 열거나 생성한다. 필요한 상위 디렉토리도 생성.
    pub async fn open(db_path: &Path) -> Result<Self, StoreError>;

    /// 스키마 생성(멱등). schema_migrations 에 현재 버전 기록.
    pub async fn init_schema(&self) -> Result<(), StoreError>;

    /// eval_scenarios 테이블이 비어 있는지 확인.
    pub async fn is_empty(&self) -> Result<bool, StoreError>;

    /// YAML/JSON 파일에서 읽어 DB 에 적재 (INSERT OR IGNORE 로 idempotent).
    pub async fn seed_from_fs(
        &self,
        scenarios_dir: &Path,
        golden_sets_dir: &Path,
    ) -> Result<SeedReport, StoreError>;

    /// 모든 도메인 설정을 position 순으로 반환.
    pub async fn load_all_domains(&self) -> Result<Vec<DomainConfig>, StoreError>;

    /// 도메인별 골든셋 파일 형태로 반환.
    pub async fn load_golden_sets_by_domain(
        &self,
        domain: &str,
    ) -> Result<GoldenSetFile, StoreError>;

    pub async fn load_all_golden_sets(&self) -> Result<Vec<GoldenSetFile>, StoreError>;

    /// 기동 편의 헬퍼: open → init_schema → is_empty 이면 seed.
    pub async fn open_and_seed(
        db_path: &Path,
        scenarios_dir: &Path,
        golden_sets_dir: &Path,
    ) -> Result<Self, StoreError>;
}

#[derive(Debug, Default)]
pub struct SeedReport {
    pub domains_inserted: usize,
    pub scenarios_inserted: usize,
    pub golden_sets_inserted: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("io error ({path}): {source}")]
    Io { path: PathBuf, #[source] source: std::io::Error },
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}
```

### `ScenarioLoader` 브릿지

기존 동기 API 를 유지한다. 내부에서 `SqliteStore` 를 `tokio::runtime::Handle::current` 또는 임시 `Runtime::new` 로 블로킹 호출한다.

```rust
pub struct ScenarioLoader {
    store: Arc<SqliteStore>,
    runtime: Option<Arc<Runtime>>,  // async 컨텍스트가 없을 때만 사용
}

impl ScenarioLoader {
    /// DB 경로 + seed 소스를 받아 초기화. 기동 시 1회 호출.
    pub fn open_blocking(
        db_path: &Path,
        scenarios_dir: &Path,
        golden_sets_dir: &Path,
    ) -> Result<Self>;

    /// 호환: 기존 코드는 `new()` 호출 후 `load_all_domains(dir)` 을 부른다.
    /// 전역 기본 DB(`eval_data/eval_harness.db`) + 디렉토리 seed 로 동작.
    pub fn new() -> Self;

    pub fn load_all_domains(&self, _directory: &str) -> Result<Vec<DomainConfig>>;
    pub fn load_golden_sets(&self, _filepath: &str) -> Result<GoldenSetFile>;
    pub fn load_all_golden_sets(&self, _directory: &str) -> Result<Vec<GoldenSetFile>>;
}
```

> 인자의 `_directory`/`_filepath` 는 하위호환을 위해 남기되 내부 로직은 store 기반이다. 추후 별도 PRD 에서 인자를 제거할 수 있다.

### `DataPaths` 확장

```rust
pub const DEFAULT_DB_PATH: &str = "eval_data/eval_harness.db";
pub const ENV_DB_PATH: &str = "EVAL_HARNESS_DB_PATH";

pub struct DataPaths {
    pub scenarios_dir:   PathBuf,
    pub golden_sets_dir: PathBuf,
    pub db_path:         PathBuf,  // 신규
}
```

TOML 파싱, `apply_env`, `with_overrides` 에 `db_path` 처리 추가.

### 진입점 통합

- `crates/eval-harness/src/main.rs`: `resolve_data_paths` 직후 `SqliteStore::open_and_seed_blocking(&paths)` 를 호출하여 로더를 초기화. `ScenarioLoader::new()` 호출 지점들은 내부적으로 전역 store 핸들을 공유한다.
- `crates/eval-harness/src/web/mod.rs`: `AppState` 에 `Arc<SqliteStore>` 추가.
- `crates/execution/src/runner.rs`: `loader.load_all_domains(scenarios_dir)` 호출은 그대로. loader 구현만 바뀐다.

### 의존성

```toml
# crates/data-scenarios/Cargo.toml
sqlx = { version = "0.8", default-features = false, features = [
    "runtime-tokio-rustls",
    "sqlite",
    "macros"   # 선택(런타임 쿼리만 사용 시 생략 가능)
] }
tokio = { workspace = true }
thiserror = { workspace = true }
```

### 검증 전략

- 단위 테스트는 `sqlx::SqlitePool::connect("sqlite::memory:")` 인메모리 DB 로 빠르게 수행.
- seed 테스트는 `tempdir` 에 작은 YAML/JSON 샘플을 생성 후 `seed_from_fs` 호출.
- round-trip 테스트는 복잡한 nested JSON 을 넣고 꺼내서 `serde_json::Value` 로 비교.
- `tokio::test` 사용.

## 대상 패키지
- `crates/data-scenarios`: `sqlite_store` 신규 모듈 + `loader` 리팩토링 + Cargo.toml
- `crates/eval-harness`: `data_paths` 확장 + main/web 초기화 연결

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | FR |
|-------|---------|------|----------|------|----|
| TC-1 | 빈 in-memory DB 에 init_schema + seed | 2개 도메인 YAML + 2개 골든셋 JSON | SeedReport: domains=2, scenarios≥6, golden_sets≥6 | 단위 | FR-1, FR-2 |
| TC-2 | seed 2회 호출 | 동일 파일 | 두 번째 호출에서 inserted=0 (idempotent) | 단위 | FR-2 |
| TC-3 | load_all_domains | seed 된 DB | Vec<DomainConfig> 길이=2, 이름 정렬 | 단위 | FR-3 |
| TC-4 | load_golden_sets_by_domain("financial") | seed 된 DB | fin_001 등 포함 GoldenSetFile | 단위 | FR-3, FR-7 |
| TC-5 | Scenario JSON round-trip | `initial_environment = {customer_id: "C001", deposit_amount: 1000000}` | 조회 결과가 입력과 동일 | 단위 | FR-7 |
| TC-6 | GoldenSet JSON round-trip | `tool_sequence=["t1","t2"], tool_results={ok:true}` | 동일 | 단위 | FR-7 |
| TC-7 | DataPaths 가 TOML 의 db_path 를 해석 | `db_path = "eval_data/eval.db"` | `base.join("eval_data/eval.db")` | 단위 | FR-4 |
| TC-8 | ScenarioLoader 호환 API | `loader.load_all_domains("legacy/dir")` | DB 기반 결과 반환 | 단위 | FR-3 |

## 구현 가이드

- 파일:
  - `crates/data-scenarios/src/sqlite_store.rs` (신규)
  - `crates/data-scenarios/src/loader.rs` (수정)
  - `crates/data-scenarios/src/lib.rs` (pub mod)
  - `crates/data-scenarios/Cargo.toml` (deps)
  - `crates/eval-harness/src/data_paths.rs` (db_path 필드)
  - `crates/eval-harness/src/main.rs` (기동 시 store 초기화)
  - `crates/eval-harness/src/web/mod.rs` (AppState 주입)
  - `eval-harness.toml` (샘플 키 추가)
  - `Cargo.toml` (workspace: `sqlx` 추가)

- 순서:
  1. `data-scenarios` Cargo 의존성 추가 → `cargo check` 로 빌드 확인.
  2. `sqlite_store.rs` 스켈레톤 + 테스트(RED) 작성.
  3. 스키마/쿼리 구현(GREEN).
  4. `loader.rs` 를 store 래퍼로 재구성.
  5. `data_paths.rs` 에 `db_path` 필드/TOML 파싱/기본값 추가.
  6. `main.rs`/`web/mod.rs` 에서 store 초기화 & 주입.
  7. 전체 `cargo test` + `cargo run -- list` 로 통합 검증.

## 완료 정의 (Definition of Done)
- [ ] TC-1 ~ TC-8 모두 통과
- [ ] `cargo build` 전체 통과
- [ ] `cargo test --workspace` 통과
- [ ] `cargo run -- list` 가 seed 된 DB 로부터 도메인 2 개를 출력
- [ ] 재실행 시 inserted=0 로그 확인 (idempotent)
- [ ] 추적성 매트릭스에 SPEC-017 등록
- [ ] README 에 SQLite 저장소 섹션 추가
