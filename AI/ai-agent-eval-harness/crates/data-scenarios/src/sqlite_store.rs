// =============================================================================
// @trace SPEC-017
// @trace SPEC-019
// @trace PRD: PRD-017, PRD-019
// @trace FR: PRD-017/FR-1, PRD-017/FR-2, PRD-017/FR-3, PRD-017/FR-6,
// PRD-017/FR-7, PRD-019/FR-1, PRD-019/FR-2, PRD-019/FR-3, PRD-019/FR-4,
// PRD-019/FR-5, PRD-019/FR-7
// @trace file-type: impl
// =============================================================================

use crate::models::{GoldenSetEntry,
                    GoldenSetExpectedOutput,
                    GoldenSetFile,
                    GoldenSetInput};
use agent_models::domain_config::{DomainConfig,
                                  ScenarioConfig,
                                  ToolConfig};
use serde_json::Value;
use sqlx::{Row,
           SqlitePool,
           sqlite::{SqliteConnectOptions,
                    SqlitePoolOptions}};
use std::{collections::HashMap,
          path::{Path,
                 PathBuf}};
use thiserror::Error;

const SCHEMA_VERSION: i64 = 7;

/// sqlx UNIQUE 제약 위반을 `StoreError::Conflict` 로 매핑.
fn map_unique_violation(err: sqlx::Error, subject: String) -> StoreError {
    if let sqlx::Error::Database(db) = &err {
        // SQLite UNIQUE 제약 오류 코드: 1555 (PRIMARY KEY), 2067 (UNIQUE)
        if let Some(code) = db.code() {
            if code == "1555" || code == "2067" || code == "19" {
                return StoreError::Conflict(format!("{subject} already exists"));
            }
        }
        // 메시지 기반 fallback
        let msg = db.message();
        if msg.contains("UNIQUE constraint failed") {
            return StoreError::Conflict(format!("{subject} already exists"));
        }
    }
    StoreError::Sqlx(err)
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("io error ({path}): {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("yaml error ({path}): {source}")]
    Yaml {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SeedReport {
    pub domains_inserted: usize,
    pub scenarios_inserted: usize,
    pub golden_sets_inserted: usize,
}

/// SQLite 기반 평가 데이터 저장소.
///
/// @trace SPEC: SPEC-017
/// @trace FR: PRD-017/FR-1, PRD-017/FR-6
pub struct SqliteStore {
    pool: SqlitePool,
}

impl SqliteStore {
    /// 파일 경로로 DB 를 열거나 생성한다. 상위 디렉토리가 없으면 생성.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-1, PRD-017/FR-6
    pub async fn open(db_path: &Path) -> Result<Self, StoreError> {
        if let Some(parent) = db_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| StoreError::Io {
                    path: parent.to_path_buf(),
                    source: e,
                })?;
            }
        }
        let opts = SqliteConnectOptions::new().filename(db_path).create_if_missing(true).foreign_keys(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .after_connect(|conn, _| {
                Box::pin(async move {
                    sqlx::query("PRAGMA foreign_keys = ON").execute(conn).await?;
                    Ok(())
                })
            })
            .connect_with(opts)
            .await?;
        let store = Self {
            pool,
        };
        store.init_schema().await?;
        Ok(store)
    }

    /// 인메모리 DB (테스트 전용). 단일 커넥션으로 설정.
    #[cfg(test)]
    pub async fn open_in_memory() -> Result<Self, StoreError> { Self::open_in_memory_for_loader().await }

    /// 인메모리 저장소. `loader` 의 fallback 경로에서 사용.
    pub async fn open_in_memory_for_loader() -> Result<Self, StoreError> {
        use std::str::FromStr;
        let opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap().foreign_keys(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .after_connect(|conn, _| {
                Box::pin(async move {
                    sqlx::query("PRAGMA foreign_keys = ON").execute(conn).await?;
                    Ok(())
                })
            })
            .connect_with(opts)
            .await?;
        let store = Self {
            pool,
        };
        store.init_schema().await?;
        Ok(store)
    }

    pub fn pool(&self) -> &SqlitePool { &self.pool }

    /// CREATE TABLE IF NOT EXISTS (멱등) + v1→v2 마이그레이션.
    ///
    /// @trace SPEC: SPEC-017, SPEC-019
    /// @trace FR: PRD-017/FR-1, PRD-019/FR-4
    pub async fn init_schema(&self) -> Result<(), StoreError> {
        let stmts = [
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version    INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS domains (
                name        TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT ''
            )",
            "CREATE TABLE IF NOT EXISTS domain_tools (
                domain      TEXT NOT NULL,
                class_name  TEXT NOT NULL,
                module_path TEXT NOT NULL,
                position    INTEGER NOT NULL,
                PRIMARY KEY (domain, class_name),
                FOREIGN KEY (domain) REFERENCES domains(name) ON DELETE CASCADE
            )",
            "CREATE TABLE IF NOT EXISTS eval_scenarios (
                domain              TEXT NOT NULL,
                id                  TEXT NOT NULL,
                name                TEXT NOT NULL,
                description         TEXT NOT NULL DEFAULT '',
                task_description    TEXT NOT NULL,
                initial_environment TEXT NOT NULL,
                expected_tools      TEXT NOT NULL,
                success_criteria    TEXT NOT NULL,
                difficulty          TEXT NOT NULL DEFAULT 'medium',
                position            INTEGER NOT NULL,
                PRIMARY KEY (domain, id),
                FOREIGN KEY (domain) REFERENCES domains(name) ON DELETE CASCADE
            )",
            // 신규 DB 는 v3 스키마 (FK + expected_domain) 로 바로 생성.
            "CREATE TABLE IF NOT EXISTS golden_sets (
                domain            TEXT NOT NULL,
                scenario_id       TEXT NOT NULL,
                version           TEXT NOT NULL DEFAULT '1.0',
                task              TEXT NOT NULL,
                input_environment TEXT NOT NULL,
                tool_sequence     TEXT NOT NULL,
                tool_results      TEXT NOT NULL,
                tolerance         REAL NOT NULL DEFAULT 0.01,
                expected_domain   TEXT,
                PRIMARY KEY (domain, scenario_id),
                FOREIGN KEY (domain, scenario_id)
                    REFERENCES eval_scenarios(domain, id)
                    ON DELETE CASCADE
            )",
            "CREATE INDEX IF NOT EXISTS idx_eval_scenarios_domain ON eval_scenarios(domain)",
            "CREATE INDEX IF NOT EXISTS idx_golden_sets_domain ON golden_sets(domain)",
            // SPEC-021 v4: 에이전트 실행 결과(궤적). steps/final_state 는 JSON BLOB.
            "CREATE TABLE IF NOT EXISTS trajectories (
                task_id           TEXT PRIMARY KEY,
                task_description  TEXT NOT NULL DEFAULT '',
                agent_name        TEXT NOT NULL DEFAULT '',
                domain            TEXT,
                scenario_id       TEXT,
                success           INTEGER NOT NULL DEFAULT 0,
                total_iterations  INTEGER NOT NULL DEFAULT 0,
                started_at        TEXT NOT NULL,
                ended_at          TEXT,
                steps_json        TEXT NOT NULL,
                final_state_json  TEXT,
                created_at        TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            "CREATE INDEX IF NOT EXISTS idx_trajectories_agent ON trajectories(agent_name)",
            "CREATE INDEX IF NOT EXISTS idx_trajectories_domain ON trajectories(domain)",
            "CREATE INDEX IF NOT EXISTS idx_trajectories_started ON trajectories(started_at)",
            // SPEC-021 v4: 평가 점수. trajectory 1:1.
            "CREATE TABLE IF NOT EXISTS evaluations (
                task_id                TEXT PRIMARY KEY,
                agent_name             TEXT NOT NULL DEFAULT '',
                domain                 TEXT,
                scenario_id            TEXT,
                success                INTEGER NOT NULL DEFAULT 0,
                criteria_score         REAL,
                tool_sequence_score    REAL,
                domain_routing_score   REAL,
                overall_score          REAL,
                metrics_json           TEXT NOT NULL DEFAULT '{}',
                golden_set_result_json TEXT,
                created_at             TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (task_id) REFERENCES trajectories(task_id) ON DELETE CASCADE
            )",
            "CREATE INDEX IF NOT EXISTS idx_evaluations_agent ON evaluations(agent_name)",
            "CREATE INDEX IF NOT EXISTS idx_evaluations_domain ON evaluations(domain)",
            "CREATE INDEX IF NOT EXISTS idx_evaluations_created ON evaluations(created_at)",
            // SPEC-022 v5: 도메인 라우터 키워드. 기존 const 를 대체.
            "CREATE TABLE IF NOT EXISTS domain_keywords (
                domain  TEXT NOT NULL,
                keyword TEXT NOT NULL,
                PRIMARY KEY (domain, keyword),
                FOREIGN KEY (domain) REFERENCES domains(name) ON DELETE CASCADE
            )",
            "CREATE INDEX IF NOT EXISTS idx_domain_keywords_domain ON domain_keywords(domain)",
            // SPEC-023 v6: 외부 HTTP 도구.
            "CREATE TABLE IF NOT EXISTS external_tools (
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
            )",
            "CREATE INDEX IF NOT EXISTS idx_external_tools_domain ON external_tools(domain)",
            // SPEC-025 v7: 도메인별 PromptSet 번들 (4 템플릿 x N 버전). 불변 편집.
            "CREATE TABLE IF NOT EXISTS prompt_sets (
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
            )",
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_prompt_sets_active_per_domain
                ON prompt_sets(domain_name) WHERE is_active = 1",
            "CREATE INDEX IF NOT EXISTS idx_prompt_sets_domain ON prompt_sets(domain_name)",
        ];
        for sql in stmts.iter() {
            sqlx::query(sql).execute(&self.pool).await?;
        }

        // v1 DB 마이그레이션: 기존 golden_sets 에 FK 가 없는 경우 재생성.
        // `CREATE TABLE IF NOT EXISTS` 는 기존 스키마를 건드리지 않으므로,
        // `schema_migrations` 의 현재 버전을 확인하여 필요 시 rebuild 한다.
        let current: Option<i64> = sqlx::query_scalar("SELECT MAX(version) FROM schema_migrations").fetch_one(&self.pool).await?;
        let current = current.unwrap_or(0);
        if current < 2 {
            self.migrate_v2_cascade().await?;
        }
        if current < 3 {
            self.migrate_v3_expected_domain().await?;
        }
        // SPEC-025 v7: trajectories / evaluations 에 prompt_set_id 컬럼 추가.
        // ALTER TABLE ADD COLUMN 은 IF NOT EXISTS 미지원 → PRAGMA 사전 검사.
        self.migrate_v7_prompt_set_id().await?;

        sqlx::query("INSERT OR IGNORE INTO schema_migrations (version, applied_at) VALUES (?, datetime('now'))")
            .bind(SCHEMA_VERSION)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// v1 → v2: `golden_sets` 에 `(domain, scenario_id) → eval_scenarios` FK
    /// 추가. 무손실 table-rename 방식.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-4
    async fn migrate_v2_cascade(&self) -> Result<(), StoreError> {
        // FK 가 이미 있는지 검사: PRAGMA foreign_key_list('golden_sets')
        let fks = sqlx::query("PRAGMA foreign_key_list('golden_sets')").fetch_all(&self.pool).await?;
        if !fks.is_empty() {
            return Ok(());
        }
        let stmts = [
            "CREATE TABLE golden_sets_v2 (
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
            )",
            "INSERT INTO golden_sets_v2
             SELECT domain, scenario_id, version, task, input_environment,
                    tool_sequence, tool_results, tolerance
             FROM golden_sets",
            "DROP TABLE golden_sets",
            "ALTER TABLE golden_sets_v2 RENAME TO golden_sets",
            "CREATE INDEX IF NOT EXISTS idx_golden_sets_domain ON golden_sets(domain)",
        ];
        for sql in stmts.iter() {
            sqlx::query(sql).execute(&self.pool).await?;
        }
        Ok(())
    }

    /// v2 → v3: `golden_sets` 에 `expected_domain TEXT` 컬럼 추가.
    ///
    /// @trace SPEC: SPEC-020
    /// @trace FR: PRD-020/FR-2
    async fn migrate_v3_expected_domain(&self) -> Result<(), StoreError> {
        let cols = sqlx::query("PRAGMA table_info('golden_sets')").fetch_all(&self.pool).await?;
        let has = cols.iter().any(|r| {
            let n: String = r.get("name");
            n == "expected_domain"
        });
        if has {
            return Ok(());
        }
        sqlx::query("ALTER TABLE golden_sets ADD COLUMN expected_domain TEXT")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// SPEC-025 v7: `trajectories` / `evaluations` 에 `prompt_set_id INTEGER
    /// NULL` 추가. `ALTER TABLE ADD COLUMN` 은 IF NOT EXISTS 미지원이므로
    /// PRAGMA 로 사전 검사.
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-8
    async fn migrate_v7_prompt_set_id(&self) -> Result<(), StoreError> {
        for table in ["trajectories", "evaluations"] {
            let cols = sqlx::query(&format!("PRAGMA table_info('{table}')")).fetch_all(&self.pool).await?;
            let has = cols.iter().any(|r| {
                let n: String = r.get("name");
                n == "prompt_set_id"
            });
            if !has {
                sqlx::query(&format!("ALTER TABLE {table} ADD COLUMN prompt_set_id INTEGER"))
                    .execute(&self.pool)
                    .await?;
            }
        }
        Ok(())
    }

    /// eval_scenarios 테이블이 비어 있는지.
    pub async fn is_empty(&self) -> Result<bool, StoreError> {
        let row = sqlx::query("SELECT COUNT(*) AS cnt FROM eval_scenarios").fetch_one(&self.pool).await?;
        let cnt: i64 = row.get("cnt");
        Ok(cnt == 0)
    }

    /// SPEC-019 후속 버그픽스: scenarios OR golden_sets 중 어느 하나가 비어
    /// 있으면 초기 시드가 완전히 적용되지 않은 것으로 간주. `INSERT OR
    /// IGNORE` 기반 시드 는 멱등이므로 안전하게 재실행할 수 있다. 다만
    /// 사용자가 모든 goldens 를 CRUD 로 삭제한 경우에도 이 조건이 참이 되어
    /// 재시드가 일어날 수 있는데, 그 경우 사용자는 domain 단위 의도적
    /// 초기화로 해석한다(범위 외 시나리오).
    pub async fn needs_seed(&self) -> Result<bool, StoreError> {
        let s: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM eval_scenarios").fetch_one(&self.pool).await?;
        let g: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM golden_sets").fetch_one(&self.pool).await?;
        Ok(s == 0 || g == 0)
    }

    /// 내장(embedded) 기본 시드에서 DB 를 채운다. 외부 파일 의존 없이 `INSERT
    /// OR IGNORE` 로 멱등 적재.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-2, PRD-017/FR-7
    pub async fn seed_from_embedded(&self) -> Result<SeedReport, StoreError> {
        use crate::seed_embedded::{EMBEDDED_GOLDEN_JSONS,
                                   EMBEDDED_SCENARIO_YAMLS};
        let scenarios: Vec<(String, String)> = EMBEDDED_SCENARIO_YAMLS
            .iter()
            .map(|(stem, body)| ((*stem).to_string(), (*body).to_string()))
            .collect();
        let goldens: Vec<(String, String)> = EMBEDDED_GOLDEN_JSONS
            .iter()
            .map(|(stem, body)| ((*stem).to_string(), (*body).to_string()))
            .collect();
        self.seed_from_sources(&scenarios, &goldens).await
    }

    /// YAML/JSON 파일 디렉토리에서 읽어 DB 에 적재. 테스트에서 임시 디렉토리
    /// 기반 시드가 필요할 때만 사용.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-2, PRD-017/FR-7
    pub async fn seed_from_fs(&self, scenarios_dir: &Path, golden_sets_dir: &Path) -> Result<SeedReport, StoreError> {
        let mut scenarios: Vec<(String, String)> = Vec::new();
        if scenarios_dir.exists() {
            let mut yaml_files: Vec<_> = std::fs::read_dir(scenarios_dir)
                .map_err(|e| StoreError::Io {
                    path: scenarios_dir.to_path_buf(),
                    source: e,
                })?
                .flatten()
                .filter(|e| e.path().extension().map(|x| x == "yaml").unwrap_or(false))
                .collect();
            yaml_files.sort_by_key(|e| e.path());
            for entry in yaml_files {
                let p = entry.path();
                let body = std::fs::read_to_string(&p).map_err(|e| StoreError::Io {
                    path: p.clone(),
                    source: e,
                })?;
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();
                scenarios.push((stem, body));
            }
        }

        let mut goldens: Vec<(String, String)> = Vec::new();
        if golden_sets_dir.exists() {
            let mut json_files: Vec<_> = std::fs::read_dir(golden_sets_dir)
                .map_err(|e| StoreError::Io {
                    path: golden_sets_dir.to_path_buf(),
                    source: e,
                })?
                .flatten()
                .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
                .collect();
            json_files.sort_by_key(|e| e.path());
            for entry in json_files {
                let p = entry.path();
                let body = std::fs::read_to_string(&p).map_err(|e| StoreError::Io {
                    path: p.clone(),
                    source: e,
                })?;
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();
                goldens.push((stem, body));
            }
        }
        self.seed_from_sources(&scenarios, &goldens).await
    }

    /// 공통 시드 헬퍼. `(stem, body)` 페어 리스트를 받아 트랜잭션으로 적재.
    async fn seed_from_sources(&self, scenarios: &[(String, String)], goldens: &[(String, String)]) -> Result<SeedReport, StoreError> {
        let mut report = SeedReport::default();
        let mut tx = self.pool.begin().await?;

        for (stem, body) in scenarios {
            let cfg: DomainConfig = serde_yaml::from_str(body).map_err(|e| StoreError::Yaml {
                path: PathBuf::from(format!("<embedded:{stem}.yaml>")),
                source: e,
            })?;

            let r = sqlx::query("INSERT OR IGNORE INTO domains (name, description) VALUES (?, ?)")
                .bind(&cfg.name)
                .bind(&cfg.description)
                .execute(&mut *tx)
                .await?;
            if r.rows_affected() > 0 {
                report.domains_inserted += 1;
            }

            for (idx, tool) in cfg.tools.iter().enumerate() {
                sqlx::query("INSERT OR IGNORE INTO domain_tools (domain, class_name, module_path, position) VALUES (?, ?, ?, ?)")
                    .bind(&cfg.name)
                    .bind(&tool.class_name)
                    .bind(&tool.module_path)
                    .bind(idx as i64)
                    .execute(&mut *tx)
                    .await?;
            }

            for (idx, scen) in cfg.scenarios.iter().enumerate() {
                let env_json = serde_json::to_string(&scen.initial_environment)?;
                let tools_json = serde_json::to_string(&scen.expected_tools)?;
                let crit_json = serde_json::to_string(&scen.success_criteria)?;
                let r = sqlx::query(
                    "INSERT OR IGNORE INTO eval_scenarios
                     (domain, id, name, description, task_description,
                      initial_environment, expected_tools, success_criteria,
                      difficulty, position)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&cfg.name)
                .bind(&scen.id)
                .bind(&scen.name)
                .bind(&scen.description)
                .bind(&scen.task_description)
                .bind(env_json)
                .bind(tools_json)
                .bind(crit_json)
                .bind(&scen.difficulty)
                .bind(idx as i64)
                .execute(&mut *tx)
                .await?;
                if r.rows_affected() > 0 {
                    report.scenarios_inserted += 1;
                }
            }
        }

        for (_stem, body) in goldens {
            let gs: GoldenSetFile = serde_json::from_str(body)?;
            for g in gs.golden_sets.iter() {
                let env_json = serde_json::to_string(&g.input.environment)?;
                let seq_json = serde_json::to_string(&g.expected_output.tool_sequence)?;
                let res_json = serde_json::to_string(&g.expected_output.tool_results)?;
                let r = sqlx::query(
                    "INSERT OR IGNORE INTO golden_sets
                     (domain, scenario_id, version, task,
                      input_environment, tool_sequence, tool_results, tolerance, expected_domain)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&gs.domain)
                .bind(&g.scenario_id)
                .bind(&gs.version)
                .bind(&g.input.task)
                .bind(env_json)
                .bind(seq_json)
                .bind(res_json)
                .bind(g.expected_output.tolerance)
                .bind(&g.expected_output.expected_domain)
                .execute(&mut *tx)
                .await?;
                if r.rows_affected() > 0 {
                    report.golden_sets_inserted += 1;
                }
            }
        }

        tx.commit().await?;
        Ok(report)
    }

    /// 편의 헬퍼: open → (scenarios 또는 golden_sets 가 비어 있으면) 내장 시드
    /// 적용. 항상 멱등.
    pub async fn open_and_seed(db_path: &Path) -> Result<(Self, SeedReport), StoreError> {
        let store = Self::open(db_path).await?;
        let mut report = SeedReport::default();
        if store.needs_seed().await? {
            report = store.seed_from_embedded().await?;
        }
        Ok((store, report))
    }

    /// 모든 도메인 설정을 반환 (position 보존).
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-3, PRD-017/FR-7
    pub async fn load_all_domains(&self) -> Result<Vec<DomainConfig>, StoreError> {
        let domain_rows = sqlx::query("SELECT name, description FROM domains ORDER BY name").fetch_all(&self.pool).await?;

        let mut out = Vec::with_capacity(domain_rows.len());
        for row in domain_rows {
            let name: String = row.get("name");
            let description: String = row.get("description");

            let tool_rows = sqlx::query("SELECT class_name, module_path FROM domain_tools WHERE domain = ? ORDER BY position")
                .bind(&name)
                .fetch_all(&self.pool)
                .await?;
            let tools: Vec<ToolConfig> = tool_rows
                .into_iter()
                .map(|r| ToolConfig {
                    class_name: r.get("class_name"),
                    module_path: r.get("module_path"),
                })
                .collect();

            let scen_rows = sqlx::query(
                "SELECT id, name, description, task_description,
                        initial_environment, expected_tools, success_criteria, difficulty
                 FROM eval_scenarios
                 WHERE domain = ?
                 ORDER BY position",
            )
            .bind(&name)
            .fetch_all(&self.pool)
            .await?;

            let mut scenarios = Vec::with_capacity(scen_rows.len());
            for r in scen_rows {
                let env_json: String = r.get("initial_environment");
                let tools_json: String = r.get("expected_tools");
                let crit_json: String = r.get("success_criteria");
                let initial_environment: HashMap<String, Value> = serde_json::from_str(&env_json)?;
                let expected_tools: Vec<String> = serde_json::from_str(&tools_json)?;
                let success_criteria: HashMap<String, Value> = serde_json::from_str(&crit_json)?;
                scenarios.push(ScenarioConfig {
                    id: r.get("id"),
                    name: r.get("name"),
                    description: r.get("description"),
                    task_description: r.get("task_description"),
                    initial_environment,
                    expected_tools,
                    success_criteria,
                    difficulty: r.get("difficulty"),
                });
            }

            out.push(DomainConfig {
                name,
                description,
                tools,
                scenarios,
            });
        }
        Ok(out)
    }

    /// 특정 도메인의 골든셋을 `GoldenSetFile` 형태로 조회.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-3, PRD-017/FR-7
    pub async fn load_golden_sets_by_domain(&self, domain: &str) -> Result<GoldenSetFile, StoreError> {
        let rows = sqlx::query(
            "SELECT scenario_id, version, task, input_environment, tool_sequence, tool_results, tolerance, expected_domain
             FROM golden_sets WHERE domain = ? ORDER BY scenario_id",
        )
        .bind(domain)
        .fetch_all(&self.pool)
        .await?;

        let mut version = "1.0".to_string();
        let mut entries = Vec::with_capacity(rows.len());
        for r in rows {
            version = r.get("version");
            let env_json: String = r.get("input_environment");
            let seq_json: String = r.get("tool_sequence");
            let res_json: String = r.get("tool_results");
            let expected_domain: Option<String> = r.try_get("expected_domain").ok();
            entries.push(GoldenSetEntry {
                scenario_id: r.get("scenario_id"),
                input: GoldenSetInput {
                    task: r.get("task"),
                    environment: serde_json::from_str(&env_json)?,
                },
                expected_output: GoldenSetExpectedOutput {
                    tool_sequence: serde_json::from_str(&seq_json)?,
                    tool_results: serde_json::from_str(&res_json)?,
                    tolerance: r.get("tolerance"),
                    expected_domain,
                },
            });
        }
        Ok(GoldenSetFile {
            domain: domain.to_string(),
            version,
            golden_sets: entries,
        })
    }

    // =========================================================================
    // CRUD: 시나리오/골든셋 쓰기 API
    //
    // @trace SPEC: SPEC-019
    // @trace FR: PRD-019/FR-1, PRD-019/FR-2, PRD-019/FR-3, PRD-019/FR-4,
    // PRD-019/FR-5
    // =========================================================================

    /// 신규 시나리오 INSERT. 동일 `(domain, id)` 존재 시
    /// `StoreError::Conflict`.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-1, PRD-019/FR-5
    pub async fn insert_scenario(&self, domain: &str, scenario: &ScenarioConfig, position: i64) -> Result<(), StoreError> {
        // 상위 도메인이 없으면 생성 (seed 단계가 아닐 때 대비).
        sqlx::query("INSERT OR IGNORE INTO domains (name, description) VALUES (?, '')")
            .bind(domain)
            .execute(&self.pool)
            .await?;

        let env_json = serde_json::to_string(&scenario.initial_environment)?;
        let tools_json = serde_json::to_string(&scenario.expected_tools)?;
        let crit_json = serde_json::to_string(&scenario.success_criteria)?;

        let res = sqlx::query(
            "INSERT INTO eval_scenarios
             (domain, id, name, description, task_description,
              initial_environment, expected_tools, success_criteria,
              difficulty, position)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(domain)
        .bind(&scenario.id)
        .bind(&scenario.name)
        .bind(&scenario.description)
        .bind(&scenario.task_description)
        .bind(env_json)
        .bind(tools_json)
        .bind(crit_json)
        .bind(&scenario.difficulty)
        .bind(position)
        .execute(&self.pool)
        .await;
        match res {
            | Ok(_) => Ok(()),
            | Err(e) => Err(map_unique_violation(e, format!("scenario ({domain}, {})", scenario.id))),
        }
    }

    /// 기존 시나리오 UPDATE. 없으면 `StoreError::NotFound`.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-1, PRD-019/FR-5
    pub async fn update_scenario(&self, domain: &str, id: &str, scenario: &ScenarioConfig) -> Result<(), StoreError> {
        let env_json = serde_json::to_string(&scenario.initial_environment)?;
        let tools_json = serde_json::to_string(&scenario.expected_tools)?;
        let crit_json = serde_json::to_string(&scenario.success_criteria)?;
        let res = sqlx::query(
            "UPDATE eval_scenarios
             SET name = ?, description = ?, task_description = ?,
                 initial_environment = ?, expected_tools = ?, success_criteria = ?,
                 difficulty = ?
             WHERE domain = ? AND id = ?",
        )
        .bind(&scenario.name)
        .bind(&scenario.description)
        .bind(&scenario.task_description)
        .bind(env_json)
        .bind(tools_json)
        .bind(crit_json)
        .bind(&scenario.difficulty)
        .bind(domain)
        .bind(id)
        .execute(&self.pool)
        .await?;
        if res.rows_affected() == 0 {
            return Err(StoreError::NotFound(format!("scenario ({domain}, {id})")));
        }
        Ok(())
    }

    /// 시나리오 DELETE. 없으면 `StoreError::NotFound`. 연결된 golden_set 는
    /// FK cascade 로 함께 삭제된다.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-1, PRD-019/FR-4
    pub async fn delete_scenario(&self, domain: &str, id: &str) -> Result<(), StoreError> {
        let res = sqlx::query("DELETE FROM eval_scenarios WHERE domain = ? AND id = ?")
            .bind(domain)
            .bind(id)
            .execute(&self.pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(StoreError::NotFound(format!("scenario ({domain}, {id})")));
        }
        Ok(())
    }

    /// 신규 골든셋 엔트리 INSERT.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-2
    pub async fn insert_golden_entry(&self, domain: &str, version: &str, entry: &GoldenSetEntry) -> Result<(), StoreError> {
        let env_json = serde_json::to_string(&entry.input.environment)?;
        let seq_json = serde_json::to_string(&entry.expected_output.tool_sequence)?;
        let res_json = serde_json::to_string(&entry.expected_output.tool_results)?;
        let res = sqlx::query(
            "INSERT INTO golden_sets
             (domain, scenario_id, version, task,
              input_environment, tool_sequence, tool_results, tolerance, expected_domain)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(domain)
        .bind(&entry.scenario_id)
        .bind(version)
        .bind(&entry.input.task)
        .bind(env_json)
        .bind(seq_json)
        .bind(res_json)
        .bind(entry.expected_output.tolerance)
        .bind(&entry.expected_output.expected_domain)
        .execute(&self.pool)
        .await;
        match res {
            | Ok(_) => Ok(()),
            | Err(e) => Err(map_unique_violation(e, format!("golden_entry ({domain}, {})", entry.scenario_id))),
        }
    }

    /// 골든셋 엔트리 UPDATE.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-2, PRD-019/FR-5
    pub async fn update_golden_entry(&self, domain: &str, scenario_id: &str, entry: &GoldenSetEntry) -> Result<(), StoreError> {
        let env_json = serde_json::to_string(&entry.input.environment)?;
        let seq_json = serde_json::to_string(&entry.expected_output.tool_sequence)?;
        let res_json = serde_json::to_string(&entry.expected_output.tool_results)?;
        let res = sqlx::query(
            "UPDATE golden_sets
             SET task = ?, input_environment = ?, tool_sequence = ?,
                 tool_results = ?, tolerance = ?, expected_domain = ?
             WHERE domain = ? AND scenario_id = ?",
        )
        .bind(&entry.input.task)
        .bind(env_json)
        .bind(seq_json)
        .bind(res_json)
        .bind(entry.expected_output.tolerance)
        .bind(&entry.expected_output.expected_domain)
        .bind(domain)
        .bind(scenario_id)
        .execute(&self.pool)
        .await?;
        if res.rows_affected() == 0 {
            return Err(StoreError::NotFound(format!("golden_entry ({domain}, {scenario_id})")));
        }
        Ok(())
    }

    /// 골든셋 엔트리 DELETE.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-2, PRD-019/FR-5
    pub async fn delete_golden_entry(&self, domain: &str, scenario_id: &str) -> Result<(), StoreError> {
        let res = sqlx::query("DELETE FROM golden_sets WHERE domain = ? AND scenario_id = ?")
            .bind(domain)
            .bind(scenario_id)
            .execute(&self.pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(StoreError::NotFound(format!("golden_entry ({domain}, {scenario_id})")));
        }
        Ok(())
    }

    /// 모든 도메인의 골든셋을 반환.
    pub async fn load_all_golden_sets(&self) -> Result<Vec<GoldenSetFile>, StoreError> {
        let rows = sqlx::query("SELECT DISTINCT domain FROM golden_sets ORDER BY domain")
            .fetch_all(&self.pool)
            .await?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            let d: String = r.get("domain");
            out.push(self.load_golden_sets_by_domain(&d).await?);
        }
        Ok(out)
    }

    // =========================================================================
    // SPEC-021: trajectories / evaluations CRUD
    // =========================================================================

    /// 궤적 1행 INSERT OR REPLACE. `task_id` 가 PK 이므로 동일 ID 재기록은
    /// 덮어쓴다. `steps_json` 은 호출자가 직렬화한 JSON 텍스트.
    ///
    /// @trace SPEC: SPEC-021
    /// @trace FR: PRD-021/FR-1
    #[allow(clippy::too_many_arguments)]
    pub async fn upsert_trajectory(
        &self,
        task_id: &str,
        task_description: &str,
        agent_name: &str,
        domain: Option<&str>,
        scenario_id: Option<&str>,
        success: bool,
        total_iterations: i64,
        started_at: &str,
        ended_at: Option<&str>,
        steps_json: &str,
        final_state_json: Option<&str>,
        prompt_set_id: Option<i64>,
    ) -> Result<(), StoreError> {
        sqlx::query(
            "INSERT OR REPLACE INTO trajectories
             (task_id, task_description, agent_name, domain, scenario_id,
              success, total_iterations, started_at, ended_at,
              steps_json, final_state_json, prompt_set_id, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))",
        )
        .bind(task_id)
        .bind(task_description)
        .bind(agent_name)
        .bind(domain)
        .bind(scenario_id)
        .bind(success as i64)
        .bind(total_iterations)
        .bind(started_at)
        .bind(ended_at)
        .bind(steps_json)
        .bind(final_state_json)
        .bind(prompt_set_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 평가 1행 INSERT OR REPLACE. trajectory 가 먼저 존재해야 FK 충족.
    ///
    /// @trace SPEC: SPEC-021
    /// @trace FR: PRD-021/FR-2
    #[allow(clippy::too_many_arguments)]
    pub async fn upsert_evaluation(
        &self,
        task_id: &str,
        agent_name: &str,
        domain: Option<&str>,
        scenario_id: Option<&str>,
        success: bool,
        criteria_score: Option<f64>,
        tool_sequence_score: Option<f64>,
        domain_routing_score: Option<f64>,
        overall_score: Option<f64>,
        metrics_json: &str,
        golden_set_result_json: Option<&str>,
    ) -> Result<(), StoreError> {
        sqlx::query(
            "INSERT OR REPLACE INTO evaluations
             (task_id, agent_name, domain, scenario_id, success,
              criteria_score, tool_sequence_score, domain_routing_score, overall_score,
              metrics_json, golden_set_result_json, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))",
        )
        .bind(task_id)
        .bind(agent_name)
        .bind(domain)
        .bind(scenario_id)
        .bind(success as i64)
        .bind(criteria_score)
        .bind(tool_sequence_score)
        .bind(domain_routing_score)
        .bind(overall_score)
        .bind(metrics_json)
        .bind(golden_set_result_json)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 궤적 task_id 목록을 created_at 내림차순으로 반환.
    ///
    /// @trace SPEC: SPEC-021
    /// @trace FR: PRD-021/FR-4
    pub async fn list_trajectory_ids(&self) -> Result<Vec<TrajectoryListRow>, StoreError> {
        let rows = sqlx::query(
            "SELECT task_id, started_at, agent_name, domain, scenario_id, success
             FROM trajectories ORDER BY started_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| TrajectoryListRow {
                task_id: r.get("task_id"),
                started_at: r.get("started_at"),
                agent_name: r.get("agent_name"),
                domain: r.try_get("domain").ok(),
                scenario_id: r.try_get("scenario_id").ok(),
                success: r.get::<i64, _>("success") != 0,
            })
            .collect())
    }

    /// 단건 trajectory 를 raw JSON 으로 반환. task_id 미존재 시 None.
    ///
    /// @trace SPEC: SPEC-021
    /// @trace FR: PRD-021/FR-4
    pub async fn get_trajectory_json(&self, task_id: &str) -> Result<Option<serde_json::Value>, StoreError> {
        let row = sqlx::query(
            "SELECT task_id, task_description, agent_name, domain, scenario_id,
                    success, total_iterations, started_at, ended_at,
                    steps_json, final_state_json, prompt_set_id
             FROM trajectories WHERE task_id = ?",
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await?;
        let Some(r) = row else {
            return Ok(None);
        };
        let steps: serde_json::Value = serde_json::from_str(&r.get::<String, _>("steps_json"))?;
        let final_state: Option<serde_json::Value> = r
            .try_get::<Option<String>, _>("final_state_json")
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok());
        let success: i64 = r.get("success");
        Ok(Some(serde_json::json!({
            "task_id": r.get::<String, _>("task_id"),
            "task_description": r.get::<String, _>("task_description"),
            "agent_name": r.get::<String, _>("agent_name"),
            "domain": r.try_get::<Option<String>, _>("domain").ok().flatten(),
            "scenario_id": r.try_get::<Option<String>, _>("scenario_id").ok().flatten(),
            "success": success != 0,
            "total_iterations": r.get::<i64, _>("total_iterations"),
            "start_time": r.get::<String, _>("started_at"),
            "end_time": r.try_get::<Option<String>, _>("ended_at").ok().flatten(),
            "steps": steps,
            "final_state": final_state,
            "prompt_set_id": r.try_get::<Option<i64>, _>("prompt_set_id").ok().flatten(),
        })))
    }

    /// 평가 task_id 목록을 created_at 내림차순.
    ///
    /// @trace SPEC: SPEC-021
    /// @trace FR: PRD-021/FR-4
    pub async fn list_evaluation_ids(&self) -> Result<Vec<EvaluationListRow>, StoreError> {
        let rows = sqlx::query(
            "SELECT task_id, created_at, agent_name, domain, scenario_id, success, overall_score
             FROM evaluations ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| EvaluationListRow {
                task_id: r.get("task_id"),
                created_at: r.get("created_at"),
                agent_name: r.get("agent_name"),
                domain: r.try_get("domain").ok(),
                scenario_id: r.try_get("scenario_id").ok(),
                success: r.get::<i64, _>("success") != 0,
                overall_score: r.try_get("overall_score").ok(),
            })
            .collect())
    }

    /// 단건 evaluation 을 raw JSON 으로.
    ///
    /// @trace SPEC: SPEC-021
    /// @trace FR: PRD-021/FR-4
    pub async fn get_evaluation_json(&self, task_id: &str) -> Result<Option<serde_json::Value>, StoreError> {
        let row = sqlx::query(
            "SELECT e.task_id, e.agent_name, e.domain, e.scenario_id, e.success,
                    e.criteria_score, e.tool_sequence_score, e.domain_routing_score, e.overall_score,
                    e.metrics_json, e.golden_set_result_json,
                    t.task_description, t.total_iterations, t.started_at, t.ended_at,
                    t.steps_json, t.final_state_json
             FROM evaluations e LEFT JOIN trajectories t ON t.task_id = e.task_id
             WHERE e.task_id = ?",
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await?;
        let Some(r) = row else {
            return Ok(None);
        };
        let metrics: serde_json::Value = serde_json::from_str(&r.get::<String, _>("metrics_json"))?;
        let golden_set_result: Option<serde_json::Value> = r
            .try_get::<Option<String>, _>("golden_set_result_json")
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok());
        let steps: Option<serde_json::Value> = r
            .try_get::<Option<String>, _>("steps_json")
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok());
        let final_state: Option<serde_json::Value> = r
            .try_get::<Option<String>, _>("final_state_json")
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok());
        let success: i64 = r.get("success");
        Ok(Some(serde_json::json!({
            "trajectory": {
                "task_id": r.get::<String, _>("task_id"),
                "task_description": r.try_get::<Option<String>, _>("task_description").ok().flatten().unwrap_or_default(),
                "agent_name": r.get::<String, _>("agent_name"),
                "success": success != 0,
                "total_iterations": r.try_get::<Option<i64>, _>("total_iterations").ok().flatten().unwrap_or(0),
                "start_time": r.try_get::<Option<String>, _>("started_at").ok().flatten(),
                "end_time": r.try_get::<Option<String>, _>("ended_at").ok().flatten(),
                "steps": steps.unwrap_or(serde_json::json!([])),
                "final_state": final_state,
            },
            "metrics": metrics,
            "golden_set_result": golden_set_result,
            "scores": {
                "criteria_score": r.try_get::<Option<f64>, _>("criteria_score").ok().flatten(),
                "tool_sequence_score": r.try_get::<Option<f64>, _>("tool_sequence_score").ok().flatten(),
                "domain_routing_score": r.try_get::<Option<f64>, _>("domain_routing_score").ok().flatten(),
                "overall_score": r.try_get::<Option<f64>, _>("overall_score").ok().flatten(),
            },
        })))
    }

    /// `compare` FR-6 의 시간 범위 평균. 지정 agent + 시간 범위의 평가 결과
    /// 평균 메트릭을 반환. 매칭 행이 없으면 빈 HashMap.
    ///
    /// @trace SPEC: SPEC-021
    /// @trace FR: PRD-021/FR-6
    pub async fn evaluation_window_average(&self, agent_name: &str, since: &str, until: &str) -> Result<EvaluationWindow, StoreError> {
        let row = sqlx::query(
            "SELECT COUNT(*) AS cnt,
                    AVG(criteria_score) AS criteria,
                    AVG(tool_sequence_score) AS tool_seq,
                    AVG(domain_routing_score) AS routing,
                    AVG(overall_score) AS overall,
                    SUM(CASE WHEN success != 0 THEN 1 ELSE 0 END) AS successes
             FROM evaluations
             WHERE agent_name = ? AND created_at >= ? AND created_at <= ?",
        )
        .bind(agent_name)
        .bind(since)
        .bind(until)
        .fetch_one(&self.pool)
        .await?;
        Ok(EvaluationWindow {
            count: row.get::<i64, _>("cnt"),
            successes: row.try_get::<Option<i64>, _>("successes").ok().flatten().unwrap_or(0),
            criteria_score: row.try_get::<Option<f64>, _>("criteria").ok().flatten(),
            tool_sequence_score: row.try_get::<Option<f64>, _>("tool_seq").ok().flatten(),
            domain_routing_score: row.try_get::<Option<f64>, _>("routing").ok().flatten(),
            overall_score: row.try_get::<Option<f64>, _>("overall").ok().flatten(),
        })
    }

    // =========================================================================
    // SPEC-022: 도메인 CRUD + 키워드/도구 매핑
    // =========================================================================

    /// 신규 도메인 INSERT. 동일 이름 존재 시 `Conflict`.
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-1
    pub async fn insert_domain(&self, name: &str, description: &str) -> Result<(), StoreError> {
        let res = sqlx::query("INSERT INTO domains (name, description) VALUES (?, ?)")
            .bind(name)
            .bind(description)
            .execute(&self.pool)
            .await;
        match res {
            | Ok(_) => Ok(()),
            | Err(e) => Err(map_unique_violation(e, format!("domain ({name})"))),
        }
    }

    /// 도메인 description 갱신. 미존재 시 `NotFound`.
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-1
    pub async fn update_domain(&self, name: &str, description: &str) -> Result<(), StoreError> {
        let res = sqlx::query("UPDATE domains SET description = ? WHERE name = ?")
            .bind(description)
            .bind(name)
            .execute(&self.pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(StoreError::NotFound(format!("domain ({name})")));
        }
        Ok(())
    }

    /// 도메인 삭제. cascade 로 scenarios/goldens/tools/keywords 모두 함께 삭제.
    /// 미존재 시 `NotFound`. 부트스트랩 보호는 호출자 책임(SqliteStore 는
    /// 라이브러리이므로 정책을 모름).
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-1, PRD-022/FR-4
    pub async fn delete_domain(&self, name: &str) -> Result<(), StoreError> {
        let res = sqlx::query("DELETE FROM domains WHERE name = ?").bind(name).execute(&self.pool).await?;
        if res.rows_affected() == 0 {
            return Err(StoreError::NotFound(format!("domain ({name})")));
        }
        Ok(())
    }

    /// 모든 도메인의 요약 정보(설명·도구·키워드·시나리오 개수). UI 목록용.
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-1
    pub async fn list_domain_summaries(&self) -> Result<Vec<DomainSummary>, StoreError> {
        let rows = sqlx::query("SELECT name, description FROM domains ORDER BY name").fetch_all(&self.pool).await?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            let name: String = r.get("name");
            let description: String = r.get("description");
            out.push(self.build_domain_summary(name, description).await?);
        }
        Ok(out)
    }

    /// 단일 도메인 요약. 미존재 시 None.
    pub async fn get_domain_summary(&self, name: &str) -> Result<Option<DomainSummary>, StoreError> {
        let row = sqlx::query("SELECT name, description FROM domains WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;
        let Some(r) = row else {
            return Ok(None);
        };
        let n: String = r.get("name");
        let d: String = r.get("description");
        Ok(Some(self.build_domain_summary(n, d).await?))
    }

    async fn build_domain_summary(&self, name: String, description: String) -> Result<DomainSummary, StoreError> {
        let tool_class_names: Vec<String> = sqlx::query_scalar("SELECT class_name FROM domain_tools WHERE domain = ? ORDER BY position, class_name")
            .bind(&name)
            .fetch_all(&self.pool)
            .await?;
        let keywords: Vec<String> = sqlx::query_scalar("SELECT keyword FROM domain_keywords WHERE domain = ? ORDER BY keyword")
            .bind(&name)
            .fetch_all(&self.pool)
            .await?;
        let scenario_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM eval_scenarios WHERE domain = ?")
            .bind(&name)
            .fetch_one(&self.pool)
            .await?;
        Ok(DomainSummary {
            name,
            description,
            tool_class_names,
            keywords,
            scenario_count,
        })
    }

    /// 도메인의 도구 목록을 통째로 교체. 트랜잭션 안에서 DELETE → INSERT.
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-1
    pub async fn replace_domain_tools(&self, domain: &str, tool_class_names: &[String]) -> Result<(), StoreError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM domain_tools WHERE domain = ?").bind(domain).execute(&mut *tx).await?;
        for (idx, name) in tool_class_names.iter().enumerate() {
            sqlx::query("INSERT INTO domain_tools (domain, class_name, module_path, position) VALUES (?, ?, ?, ?)")
                .bind(domain)
                .bind(name)
                .bind("") // module_path: SPEC-022 이후 의미 없음. 후속 정리.
                .bind(idx as i64)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// 도메인의 라우터 키워드 목록을 통째로 교체.
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-3
    pub async fn replace_domain_keywords(&self, domain: &str, keywords: &[String]) -> Result<(), StoreError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM domain_keywords WHERE domain = ?")
            .bind(domain)
            .execute(&mut *tx)
            .await?;
        for kw in keywords {
            let trimmed = kw.trim();
            if trimmed.is_empty() {
                continue;
            }
            sqlx::query("INSERT OR IGNORE INTO domain_keywords (domain, keyword) VALUES (?, ?)")
                .bind(domain)
                .bind(trimmed)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// 모든 도메인의 키워드를 `domain → Vec<keyword>` 매핑으로 반환.
    /// `domain_router` 의 캐시 채우기에 사용.
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-3
    pub async fn list_all_domain_keywords(&self) -> Result<HashMap<String, Vec<String>>, StoreError> {
        let rows = sqlx::query("SELECT domain, keyword FROM domain_keywords ORDER BY domain, keyword")
            .fetch_all(&self.pool)
            .await?;
        let mut out: HashMap<String, Vec<String>> = HashMap::new();
        for r in rows {
            let d: String = r.get("domain");
            let k: String = r.get("keyword");
            out.entry(d).or_default().push(k);
        }
        Ok(out)
    }

    /// SPEC-022 v5 시드: 부트스트랩 도메인의 기본 키워드를 INSERT OR IGNORE.
    /// 호출자가 (domain, keyword) 페어 리스트를 전달.
    ///
    /// @trace SPEC: SPEC-022
    /// @trace FR: PRD-022/FR-7
    pub async fn seed_domain_keywords(&self, pairs: &[(String, String)]) -> Result<usize, StoreError> {
        let mut inserted = 0usize;
        let mut tx = self.pool.begin().await?;
        for (d, k) in pairs {
            let res = sqlx::query("INSERT OR IGNORE INTO domain_keywords (domain, keyword) VALUES (?, ?)")
                .bind(d)
                .bind(k)
                .execute(&mut *tx)
                .await?;
            if res.rows_affected() > 0 {
                inserted += 1;
            }
        }
        tx.commit().await?;
        Ok(inserted)
    }

    // =========================================================================
    // SPEC-023: external_tools CRUD
    // =========================================================================

    /// 모든 external tool 행을 반환. domain, name 정렬.
    ///
    /// @trace SPEC: SPEC-023
    /// @trace FR: PRD-023/FR-1, PRD-023/FR-2
    pub async fn list_external_tools(&self) -> Result<Vec<ExternalToolRow>, StoreError> {
        let rows = sqlx::query(
            "SELECT name, domain, description, method, url, headers_json, body_template, params_schema, timeout_ms
             FROM external_tools ORDER BY domain, name",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(row_to_external_tool).collect())
    }

    /// 특정 도메인의 external tool 행 반환.
    pub async fn list_external_tools_by_domain(&self, domain: &str) -> Result<Vec<ExternalToolRow>, StoreError> {
        let rows = sqlx::query(
            "SELECT name, domain, description, method, url, headers_json, body_template, params_schema, timeout_ms
             FROM external_tools WHERE domain = ? ORDER BY name",
        )
        .bind(domain)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(row_to_external_tool).collect())
    }

    /// external tool 1행 INSERT OR REPLACE.
    ///
    /// @trace SPEC: SPEC-023
    /// @trace FR: PRD-023/FR-1
    pub async fn upsert_external_tool(&self, row: &ExternalToolRow) -> Result<(), StoreError> {
        sqlx::query(
            "INSERT OR REPLACE INTO external_tools
             (name, domain, description, method, url, headers_json, body_template, params_schema, timeout_ms, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, COALESCE((SELECT created_at FROM external_tools WHERE domain = ? AND name = ?), datetime('now')))",
        )
        .bind(&row.name)
        .bind(&row.domain)
        .bind(&row.description)
        .bind(&row.method)
        .bind(&row.url)
        .bind(&row.headers_json)
        .bind(&row.body_template)
        .bind(&row.params_schema)
        .bind(row.timeout_ms)
        .bind(&row.domain)
        .bind(&row.name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// external tool 삭제. 미존재 시 NotFound.
    pub async fn delete_external_tool(&self, domain: &str, name: &str) -> Result<(), StoreError> {
        let res = sqlx::query("DELETE FROM external_tools WHERE domain = ? AND name = ?")
            .bind(domain)
            .bind(name)
            .execute(&self.pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(StoreError::NotFound(format!("external_tool ({domain}, {name})")));
        }
        Ok(())
    }

    // =========================================================================
    // SPEC-025: prompt_sets CRUD (도메인별 프롬프트 번들 + 불변 버전관리)
    // =========================================================================

    /// 도메인의 모든 PromptSet 버전을 `version DESC` 정렬로 반환.
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-6
    pub async fn list_prompt_sets(&self, domain: &str) -> Result<Vec<PromptSetRow>, StoreError> {
        let rows = sqlx::query(PROMPT_SET_SELECT_COLS).bind(domain).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(row_to_prompt_set).collect())
    }

    /// 단일 버전 조회.
    pub async fn get_prompt_set(&self, domain: &str, version: i64) -> Result<Option<PromptSetRow>, StoreError> {
        let row = sqlx::query(
            "SELECT id, domain_name, version, perceive_system, perceive_user, policy_system, policy_user,
                    notes, is_active, is_bootstrap, created_at
             FROM prompt_sets WHERE domain_name = ? AND version = ?",
        )
        .bind(domain)
        .bind(version)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(row_to_prompt_set))
    }

    /// 도메인의 활성 PromptSet (없으면 None).
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-3
    pub async fn get_active_prompt_set(&self, domain: &str) -> Result<Option<PromptSetRow>, StoreError> {
        let row = sqlx::query(
            "SELECT id, domain_name, version, perceive_system, perceive_user, policy_system, policy_user,
                    notes, is_active, is_bootstrap, created_at
             FROM prompt_sets WHERE domain_name = ? AND is_active = 1 LIMIT 1",
        )
        .bind(domain)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(row_to_prompt_set))
    }

    /// 새 버전 삽입. version 은 `MAX(version)+1` 로 자동 증가. 도메인의 첫
    /// 삽입이면 자동으로 `is_active=1` 부여.
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-1, PRD-025/FR-6
    pub async fn insert_prompt_set(&self, row: PromptSetInsert<'_>) -> Result<PromptSetRow, StoreError> {
        let mut tx = self.pool.begin().await?;
        let next_version: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(version), 0) + 1 FROM prompt_sets WHERE domain_name = ?")
            .bind(row.domain_name)
            .fetch_one(&mut *tx)
            .await?;
        let any_existing: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prompt_sets WHERE domain_name = ?")
            .bind(row.domain_name)
            .fetch_one(&mut *tx)
            .await?;
        let is_active = if any_existing == 0 { 1_i64 } else { 0_i64 };
        let is_bootstrap = if row.is_bootstrap { 1_i64 } else { 0_i64 };
        sqlx::query(
            "INSERT INTO prompt_sets
             (domain_name, version, perceive_system, perceive_user, policy_system, policy_user,
              notes, is_active, is_bootstrap, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))",
        )
        .bind(row.domain_name)
        .bind(next_version)
        .bind(row.perceive_system)
        .bind(row.perceive_user)
        .bind(row.policy_system)
        .bind(row.policy_user)
        .bind(row.notes)
        .bind(is_active)
        .bind(is_bootstrap)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        // 삽입된 행을 다시 읽어 반환 (id 포함)
        self.get_prompt_set(row.domain_name, next_version)
            .await?
            .ok_or_else(|| StoreError::NotFound(format!("prompt_set just inserted: {}/v{}", row.domain_name, next_version)))
    }

    /// 활성 버전 전환: 기존 활성 해제 + 새 활성 설정을 단일 트랜잭션으로.
    /// 대상 버전이 없으면 NotFound.
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-3, PRD-025/FR-7
    pub async fn activate_prompt_set(&self, domain: &str, version: i64) -> Result<(), StoreError> {
        let mut tx = self.pool.begin().await?;
        // 존재 확인
        let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM prompt_sets WHERE domain_name = ? AND version = ?")
            .bind(domain)
            .bind(version)
            .fetch_optional(&mut *tx)
            .await?;
        if exists.is_none() {
            return Err(StoreError::NotFound(format!("prompt_set {domain}/v{version}")));
        }
        sqlx::query("UPDATE prompt_sets SET is_active = 0 WHERE domain_name = ? AND is_active = 1")
            .bind(domain)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE prompt_sets SET is_active = 1 WHERE domain_name = ? AND version = ?")
            .bind(domain)
            .bind(version)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    /// 삭제. 활성/bootstrap 은 Conflict.
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-7
    pub async fn delete_prompt_set(&self, domain: &str, version: i64) -> Result<(), StoreError> {
        let row = self.get_prompt_set(domain, version).await?;
        let Some(row) = row else {
            return Err(StoreError::NotFound(format!("prompt_set {domain}/v{version}")));
        };
        if row.is_active {
            return Err(StoreError::Conflict("cannot delete active version".into()));
        }
        if row.is_bootstrap {
            return Err(StoreError::Conflict("cannot delete bootstrap version".into()));
        }
        sqlx::query("DELETE FROM prompt_sets WHERE domain_name = ? AND version = ?")
            .bind(domain)
            .bind(version)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 모든 도메인에 대해 PromptSet 이 하나도 없으면 주어진 bootstrap 번들을 v1
    /// 로 삽입. 기존 PromptSet 이 이미 있으면 해당 도메인은 skip (NFR-2
    /// 재시드 금지).
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-2
    pub async fn seed_bootstrap_prompt_sets(&self, bundle: &BootstrapBundleRef<'_>) -> Result<usize, StoreError> {
        let domains: Vec<String> = sqlx::query_scalar("SELECT name FROM domains ORDER BY name").fetch_all(&self.pool).await?;
        let mut inserted = 0usize;
        for d in domains {
            let cnt: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prompt_sets WHERE domain_name = ?")
                .bind(&d)
                .fetch_one(&self.pool)
                .await?;
            if cnt > 0 {
                continue;
            }
            self.insert_prompt_set(PromptSetInsert {
                domain_name: &d,
                perceive_system: bundle.perceive_system,
                perceive_user: bundle.perceive_user,
                policy_system: bundle.policy_system,
                policy_user: bundle.policy_user,
                notes: Some("bootstrap seed"),
                is_bootstrap: true,
            })
            .await?;
            inserted += 1;
        }
        Ok(inserted)
    }
}

const PROMPT_SET_SELECT_COLS: &str = "SELECT id, domain_name, version, perceive_system, perceive_user, policy_system, policy_user,
        notes, is_active, is_bootstrap, created_at
 FROM prompt_sets WHERE domain_name = ? ORDER BY version DESC";

fn row_to_prompt_set(r: sqlx::sqlite::SqliteRow) -> PromptSetRow {
    PromptSetRow {
        id: r.get("id"),
        domain_name: r.get("domain_name"),
        version: r.get("version"),
        perceive_system: r.get("perceive_system"),
        perceive_user: r.get("perceive_user"),
        policy_system: r.get("policy_system"),
        policy_user: r.get("policy_user"),
        notes: r.try_get("notes").ok(),
        is_active: {
            let v: i64 = r.get("is_active");
            v != 0
        },
        is_bootstrap: {
            let v: i64 = r.get("is_bootstrap");
            v != 0
        },
        created_at: r.get("created_at"),
    }
}

/// SPEC-025: PromptSet 행(DB 표현).
#[derive(Debug, Clone)]
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

/// SPEC-025: PromptSet 삽입 페이로드.
#[derive(Debug, Clone, Copy)]
pub struct PromptSetInsert<'a> {
    pub domain_name: &'a str,
    pub perceive_system: &'a str,
    pub perceive_user: &'a str,
    pub policy_system: &'a str,
    pub policy_user: &'a str,
    pub notes: Option<&'a str>,
    pub is_bootstrap: bool,
}

/// SPEC-025: 기동 시점 bootstrap 번들 주입용. `agent-core` 의 상수를 참조하여
/// `data-scenarios` → `agent-core` 순환 의존을 피한다.
#[derive(Debug, Clone, Copy)]
pub struct BootstrapBundleRef<'a> {
    pub perceive_system: &'a str,
    pub perceive_user: &'a str,
    pub policy_system: &'a str,
    pub policy_user: &'a str,
}

fn row_to_external_tool(r: sqlx::sqlite::SqliteRow) -> ExternalToolRow {
    ExternalToolRow {
        name: r.get("name"),
        domain: r.get("domain"),
        description: r.get("description"),
        method: r.get("method"),
        url: r.get("url"),
        headers_json: r.try_get("headers_json").ok(),
        body_template: r.get("body_template"),
        params_schema: r.get("params_schema"),
        timeout_ms: r.get("timeout_ms"),
    }
}

/// SPEC-022: 도메인 단위 요약 정보. UI/REST 응답용.
#[derive(Debug, Clone)]
pub struct DomainSummary {
    pub name: String,
    pub description: String,
    pub tool_class_names: Vec<String>,
    pub keywords: Vec<String>,
    pub scenario_count: i64,
}

/// SPEC-023: 외부 HTTP 도구 등록 행. `HttpCallTool::from_row` 가 이 값으로
/// 인스턴스를 만든다.
#[derive(Debug, Clone)]
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

/// SPEC-021: trajectories 목록 조회 행.
#[derive(Debug, Clone)]
pub struct TrajectoryListRow {
    pub task_id: String,
    pub started_at: String,
    pub agent_name: String,
    pub domain: Option<String>,
    pub scenario_id: Option<String>,
    pub success: bool,
}

/// SPEC-021: evaluations 목록 조회 행.
#[derive(Debug, Clone)]
pub struct EvaluationListRow {
    pub task_id: String,
    pub created_at: String,
    pub agent_name: String,
    pub domain: Option<String>,
    pub scenario_id: Option<String>,
    pub success: bool,
    pub overall_score: Option<f64>,
}

/// SPEC-021: 시간 범위 평균 결과(`compare --agent --since --until`).
#[derive(Debug, Clone, Default)]
pub struct EvaluationWindow {
    pub count: i64,
    pub successes: i64,
    pub criteria_score: Option<f64>,
    pub tool_sequence_score: Option<f64>,
    pub domain_routing_score: Option<f64>,
    pub overall_score: Option<f64>,
}

// =============================================================================
// Tests
// =============================================================================
//
// @trace SPEC-017
// @trace PRD: PRD-017
// @trace TC: SPEC-017/TC-1, TC-2, TC-3, TC-4, TC-5, TC-6
// @trace file-type: test
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_sample_dataset(scen_dir: &Path, gold_dir: &Path) {
        fs::create_dir_all(scen_dir).unwrap();
        fs::create_dir_all(gold_dir).unwrap();

        let fin_yaml = r#"
name: financial
description: 금융 도메인
tools:
  - class_name: SimpleInterestCalculatorTool
    module_path: domains.financial.tools
scenarios:
  - id: fin_001
    name: 단리 이자 계산
    description: 단리
    task_description: 단리 이자 계산
    initial_environment:
      customer_id: C001
      deposit_amount: 1000000
      interest_rate: 0.05
    expected_tools:
      - calculate_simple_interest
    success_criteria:
      interest: 100000.0
    difficulty: easy
  - id: fin_002
    name: 복리 이자 계산
    description: 복리
    task_description: 복리
    initial_environment:
      customer_id: C002
    expected_tools:
      - calculate_compound_interest
    success_criteria:
      comparison_done: true
    difficulty: medium
"#;
        fs::write(scen_dir.join("financial.yaml"), fin_yaml).unwrap();

        let cs_yaml = r#"
name: customer_service
description: 고객 서비스 도메인
tools:
  - class_name: InquiryClassifierTool
    module_path: domains.customer_service.tools
scenarios:
  - id: cs_001
    name: 고객 문의 분류
    description: 분류
    task_description: 문의 분류
    initial_environment:
      inquiry_text: "환불 요청"
    expected_tools:
      - classify_inquiry
    success_criteria:
      category: refund
    difficulty: easy
"#;
        fs::write(scen_dir.join("customer_service.yaml"), cs_yaml).unwrap();

        let fin_gs = r#"{
  "domain": "financial",
  "version": "1.0",
  "golden_sets": [
    {
      "scenario_id": "fin_001",
      "input": {
        "task": "단리",
        "environment": {"customer_id": "C001", "deposit_amount": 1000000}
      },
      "expected_output": {
        "tool_sequence": ["calculate_simple_interest"],
        "tool_results": {"interest": 100000.0, "total_amount": 1100000.0},
        "tolerance": 0.01
      }
    }
  ]
}"#;
        fs::write(gold_dir.join("financial.json"), fin_gs).unwrap();

        let cs_gs = r#"{
  "domain": "customer_service",
  "version": "1.0",
  "golden_sets": [
    {
      "scenario_id": "cs_001",
      "input": {
        "task": "분류",
        "environment": {"inquiry_text": "환불"}
      },
      "expected_output": {
        "tool_sequence": ["classify_inquiry"],
        "tool_results": {"category": "refund"},
        "tolerance": 0.01
      }
    }
  ]
}"#;
        fs::write(gold_dir.join("customer_service.json"), cs_gs).unwrap();
    }

    /// @trace TC: SPEC-017/TC-1
    /// @trace FR: PRD-017/FR-1, PRD-017/FR-2
    #[tokio::test]
    async fn tc_1_seed_empty_db_inserts_all() {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_sample_dataset(&scen, &gold);

        let store = SqliteStore::open_in_memory().await.unwrap();
        assert!(store.is_empty().await.unwrap());

        let report = store.seed_from_fs(&scen, &gold).await.unwrap();
        assert_eq!(report.domains_inserted, 2);
        assert_eq!(report.scenarios_inserted, 3);
        assert_eq!(report.golden_sets_inserted, 2);
        assert!(!store.is_empty().await.unwrap());
    }

    /// @trace TC: SPEC-017/TC-2
    /// @trace FR: PRD-017/FR-2
    #[tokio::test]
    async fn tc_2_seed_is_idempotent() {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_sample_dataset(&scen, &gold);

        let store = SqliteStore::open_in_memory().await.unwrap();
        let first = store.seed_from_fs(&scen, &gold).await.unwrap();
        assert!(first.scenarios_inserted > 0);
        let second = store.seed_from_fs(&scen, &gold).await.unwrap();
        assert_eq!(second.domains_inserted, 0);
        assert_eq!(second.scenarios_inserted, 0);
        assert_eq!(second.golden_sets_inserted, 0);
    }

    /// @trace TC: SPEC-017/TC-3
    /// @trace FR: PRD-017/FR-3
    #[tokio::test]
    async fn tc_3_load_all_domains_returns_both() {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_sample_dataset(&scen, &gold);

        let store = SqliteStore::open_in_memory().await.unwrap();
        store.seed_from_fs(&scen, &gold).await.unwrap();

        let domains = store.load_all_domains().await.unwrap();
        assert_eq!(domains.len(), 2);
        let names: Vec<&str> = domains.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"financial"));
        assert!(names.contains(&"customer_service"));

        let fin = domains.iter().find(|d| d.name == "financial").unwrap();
        assert_eq!(fin.scenarios.len(), 2);
        assert_eq!(fin.scenarios[0].id, "fin_001");
        assert_eq!(fin.scenarios[1].id, "fin_002");
        assert_eq!(fin.tools.len(), 1);
    }

    /// @trace TC: SPEC-017/TC-4
    /// @trace FR: PRD-017/FR-3, PRD-017/FR-7
    #[tokio::test]
    async fn tc_4_load_golden_sets_by_domain() {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_sample_dataset(&scen, &gold);

        let store = SqliteStore::open_in_memory().await.unwrap();
        store.seed_from_fs(&scen, &gold).await.unwrap();

        let gs = store.load_golden_sets_by_domain("financial").await.unwrap();
        assert_eq!(gs.domain, "financial");
        assert_eq!(gs.golden_sets.len(), 1);
        assert_eq!(gs.golden_sets[0].scenario_id, "fin_001");
        assert_eq!(gs.golden_sets[0].expected_output.tool_sequence, vec!["calculate_simple_interest".to_string()]);
    }

    /// @trace TC: SPEC-017/TC-5
    /// @trace FR: PRD-017/FR-7
    #[tokio::test]
    async fn tc_5_scenario_environment_roundtrip() {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_sample_dataset(&scen, &gold);

        let store = SqliteStore::open_in_memory().await.unwrap();
        store.seed_from_fs(&scen, &gold).await.unwrap();

        let domains = store.load_all_domains().await.unwrap();
        let fin = domains.iter().find(|d| d.name == "financial").unwrap();
        let s = fin.scenarios.iter().find(|s| s.id == "fin_001").unwrap();
        assert_eq!(s.initial_environment.get("customer_id").and_then(|v| v.as_str()), Some("C001"));
        assert_eq!(s.initial_environment.get("deposit_amount").and_then(|v| v.as_i64()), Some(1_000_000));
        assert_eq!(s.initial_environment.get("interest_rate").and_then(|v| v.as_f64()), Some(0.05));
    }

    /// @trace TC: SPEC-017/TC-6
    /// @trace FR: PRD-017/FR-7
    #[tokio::test]
    async fn tc_6_golden_set_tool_results_roundtrip() {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_sample_dataset(&scen, &gold);

        let store = SqliteStore::open_in_memory().await.unwrap();
        store.seed_from_fs(&scen, &gold).await.unwrap();

        let gs = store.load_golden_sets_by_domain("financial").await.unwrap();
        let entry = &gs.golden_sets[0];
        assert_eq!(entry.expected_output.tool_results.get("interest").and_then(|v| v.as_f64()), Some(100000.0));
        assert_eq!(entry.expected_output.tool_results.get("total_amount").and_then(|v| v.as_f64()), Some(1100000.0));
        assert_eq!(entry.expected_output.tolerance, 0.01);
    }

    // =========================================================================
    // SPEC-019 CRUD 테스트
    //
    // @trace SPEC-019
    // @trace PRD: PRD-019
    // @trace TC: SPEC-019/TC-1, TC-2, TC-3, TC-4, TC-5, TC-6, TC-7, TC-8, TC-15
    // @trace file-type: test
    // =========================================================================

    fn sample_scenario(id: &str, task: &str) -> ScenarioConfig {
        let mut env = HashMap::new();
        env.insert("x".to_string(), Value::from(1_i64));
        let mut crit = HashMap::new();
        crit.insert("ok".to_string(), Value::Bool(true));
        ScenarioConfig {
            id: id.to_string(),
            name: format!("시나리오 {id}"),
            description: "테스트".to_string(),
            task_description: task.to_string(),
            initial_environment: env,
            expected_tools: vec!["tool_a".to_string()],
            success_criteria: crit,
            difficulty: "easy".to_string(),
        }
    }

    fn sample_entry(scenario_id: &str) -> GoldenSetEntry {
        let mut env = HashMap::new();
        env.insert("y".to_string(), Value::from(2_i64));
        let mut results = HashMap::new();
        results.insert("result".to_string(), Value::from(42_i64));
        GoldenSetEntry {
            scenario_id: scenario_id.to_string(),
            input: GoldenSetInput {
                task: "golden task".to_string(),
                environment: env,
            },
            expected_output: GoldenSetExpectedOutput {
                tool_sequence: vec!["tool_a".to_string()],
                tool_results: results,
                tolerance: 0.01,
                expected_domain: None,
            },
        }
    }

    async fn seeded_store() -> SqliteStore {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_sample_dataset(&scen, &gold);
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.seed_from_fs(&scen, &gold).await.unwrap();
        // tempdir 은 drop 되지만 DB 는 in-memory 이므로 무관.
        store
    }

    /// @trace TC: SPEC-019/TC-1
    /// @trace FR: PRD-019/FR-1
    #[tokio::test]
    async fn spec019_tc_1_insert_scenario_appears_in_load_all() {
        let store = seeded_store().await;
        let scen = sample_scenario("fin_new", "새 시나리오");
        store.insert_scenario("financial", &scen, 99).await.expect("insert should succeed");

        let domains = store.load_all_domains().await.unwrap();
        let fin = domains.iter().find(|d| d.name == "financial").unwrap();
        assert!(fin.scenarios.iter().any(|s| s.id == "fin_new"));
    }

    /// @trace TC: SPEC-019/TC-2
    /// @trace FR: PRD-019/FR-5
    #[tokio::test]
    async fn spec019_tc_2_insert_scenario_duplicate_returns_conflict() {
        let store = seeded_store().await;
        // fin_001 은 seed 에 이미 있음
        let dup = sample_scenario("fin_001", "duplicate");
        let err = store.insert_scenario("financial", &dup, 10).await.unwrap_err();
        assert!(matches!(err, StoreError::Conflict(_)), "expected Conflict, got {err:?}");
    }

    /// @trace TC: SPEC-019/TC-3
    /// @trace FR: PRD-019/FR-1
    #[tokio::test]
    async fn spec019_tc_3_update_scenario_changes_task() {
        let store = seeded_store().await;
        let mut s = sample_scenario("fin_001", "변경된 task");
        s.name = "변경된 이름".into();
        store.update_scenario("financial", "fin_001", &s).await.expect("update should succeed");

        let domains = store.load_all_domains().await.unwrap();
        let fin = domains.iter().find(|d| d.name == "financial").unwrap();
        let got = fin.scenarios.iter().find(|x| x.id == "fin_001").unwrap();
        assert_eq!(got.task_description, "변경된 task");
        assert_eq!(got.name, "변경된 이름");
    }

    /// @trace TC: SPEC-019/TC-4
    /// @trace FR: PRD-019/FR-5
    #[tokio::test]
    async fn spec019_tc_4_update_scenario_missing_returns_not_found() {
        let store = seeded_store().await;
        let s = sample_scenario("nonexistent", "x");
        let err = store.update_scenario("financial", "nonexistent", &s).await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(_)), "expected NotFound, got {err:?}");
    }

    /// @trace TC: SPEC-019/TC-5
    /// @trace FR: PRD-019/FR-4
    #[tokio::test]
    async fn spec019_tc_5_delete_scenario_cascades_golden_set() {
        let store = seeded_store().await;
        // seed 상태: financial/fin_001 에 golden entry 있음
        let before = store.load_golden_sets_by_domain("financial").await.unwrap();
        assert!(before.golden_sets.iter().any(|e| e.scenario_id == "fin_001"));

        store.delete_scenario("financial", "fin_001").await.expect("delete should succeed");

        let after = store.load_golden_sets_by_domain("financial").await.unwrap();
        assert!(
            !after.golden_sets.iter().any(|e| e.scenario_id == "fin_001"),
            "cascade delete: golden_set for fin_001 must be gone"
        );

        let domains = store.load_all_domains().await.unwrap();
        let fin = domains.iter().find(|d| d.name == "financial").unwrap();
        assert!(!fin.scenarios.iter().any(|s| s.id == "fin_001"));
    }

    /// @trace TC: SPEC-019/TC-6
    /// @trace FR: PRD-019/FR-2
    #[tokio::test]
    async fn spec019_tc_6_insert_golden_entry_appears_in_load() {
        let store = seeded_store().await;
        // fin_002 는 seed 에 시나리오는 있으나 golden entry 는 없음
        let entry = sample_entry("fin_002");
        store.insert_golden_entry("financial", "1.0", &entry).await.expect("insert should succeed");

        let gs = store.load_golden_sets_by_domain("financial").await.unwrap();
        assert!(gs.golden_sets.iter().any(|e| e.scenario_id == "fin_002"));
    }

    /// @trace TC: SPEC-019/TC-7
    /// @trace FR: PRD-019/FR-2, PRD-019/FR-5
    #[tokio::test]
    async fn spec019_tc_7_golden_entry_update_delete_cycle() {
        let store = seeded_store().await;
        // update 없는 것 → NotFound
        let missing = sample_entry("nope");
        let err = store.update_golden_entry("financial", "nope", &missing).await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(_)));

        // 기존 fin_001 업데이트
        let mut upd = sample_entry("fin_001");
        upd.input.task = "업데이트된 task".into();
        store.update_golden_entry("financial", "fin_001", &upd).await.expect("update should succeed");
        let gs = store.load_golden_sets_by_domain("financial").await.unwrap();
        let got = gs.golden_sets.iter().find(|e| e.scenario_id == "fin_001").unwrap();
        assert_eq!(got.input.task, "업데이트된 task");

        // delete
        store.delete_golden_entry("financial", "fin_001").await.expect("delete should succeed");
        let after = store.load_golden_sets_by_domain("financial").await.unwrap();
        assert!(!after.golden_sets.iter().any(|e| e.scenario_id == "fin_001"));

        // 없는 것 delete → NotFound
        let err = store.delete_golden_entry("financial", "fin_001").await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(_)));
    }

    /// @trace TC: SPEC-019/TC-8
    /// @trace FR: PRD-019/FR-4
    #[tokio::test]
    async fn spec019_tc_8_migration_v2_preserves_data() {
        // init_schema 이후 golden_sets 에 FK 가 활성화되어야 하고,
        // PRAGMA foreign_keys = ON 에서 cascade 동작이 유효해야 한다.
        let store = seeded_store().await;
        // FK 활성 확인: fin_001 삭제 시 golden_set 도 사라져야 함
        store.delete_scenario("financial", "fin_001").await.expect("delete ok");
        let gs = store.load_golden_sets_by_domain("financial").await.unwrap();
        assert!(!gs.golden_sets.iter().any(|e| e.scenario_id == "fin_001"));

        // 마이그레이션이 기존 fin_001 이외의 데이터를 보존했는지
        let domains = store.load_all_domains().await.unwrap();
        let fin = domains.iter().find(|d| d.name == "financial").unwrap();
        assert!(fin.scenarios.iter().any(|s| s.id == "fin_002"));
    }

    /// @trace TC: SPEC-019/TC-15
    /// @trace FR: PRD-019/FR-7
    #[tokio::test]
    async fn spec019_tc_15_loader_golden_sets_via_db() {
        // ScenarioLoader 가 DB 를 조회하는지 (파일이 아닌) 확인하기 위해,
        // DB 에만 존재하고 파일에는 없는 엔트리를 삽입한 뒤 로더 결과에
        // 포함되는지 본다. 이 TC 는 loader.rs 의 리와이어링 이후 GREEN.
        //
        // NOTE: 현재 ScenarioLoader 는 전역 싱글톤이고 파일을 직접 읽으므로
        // 본 테스트는 `SqliteStore::load_all_golden_sets` 만 직접 검증한다.
        // loader 경로 자체는 integration 레벨에서 재검증한다.
        let store = seeded_store().await;
        let entry = sample_entry("fin_002");
        store.insert_golden_entry("financial", "1.0", &entry).await.expect("insert ok");

        let all = store.load_all_golden_sets().await.unwrap();
        let fin = all.iter().find(|f| f.domain == "financial").unwrap();
        assert!(
            fin.golden_sets.iter().any(|e| e.scenario_id == "fin_002"),
            "store.load_all_golden_sets must reflect DB writes"
        );
    }

    /// @trace TC: SPEC-017/TC-1, SPEC-017/TC-2
    /// @trace FR: PRD-017/FR-2
    #[tokio::test]
    async fn open_and_seed_is_idempotent_across_opens() {
        let dir = tempdir().unwrap();
        let db = dir.path().join("eval.db");

        let (_s1, r1) = SqliteStore::open_and_seed(&db).await.unwrap();
        assert!(r1.scenarios_inserted > 0);

        let (_s2, r2) = SqliteStore::open_and_seed(&db).await.unwrap();
        assert_eq!(r2.scenarios_inserted, 0);
    }

    // -------- SPEC-021 --------

    /// @trace TC: SPEC-021/TC-2, SPEC-021/TC-4
    /// @trace FR: PRD-021/FR-1
    #[tokio::test]
    async fn spec021_tc_2_trajectory_upsert_round_trip() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        let task_id = "550e8400-e29b-41d4-a716-446655440001";
        store
            .upsert_trajectory(
                task_id,
                "단리 이자 계산",
                "ppa",
                Some("financial"),
                Some("fin_001"),
                true,
                3,
                "2026-04-11T10:23:45Z",
                Some("2026-04-11T10:23:50Z"),
                "[{\"stage\":\"perceive\"}]",
                Some("{\"k\":\"v\"}"),
                Some(7),
            )
            .await
            .unwrap();

        let rows = store.list_trajectory_ids().await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].task_id, task_id);
        assert_eq!(rows[0].domain.as_deref(), Some("financial"));
        assert!(rows[0].success);

        // 동일 task_id 재기록 → 행 1개 유지(REPLACE)
        store
            .upsert_trajectory(
                task_id,
                "단리 이자 계산 v2",
                "ppa",
                Some("financial"),
                Some("fin_001"),
                false,
                4,
                "2026-04-11T10:30:00Z",
                Some("2026-04-11T10:30:10Z"),
                "[]",
                None,
                None,
            )
            .await
            .unwrap();
        let rows2 = store.list_trajectory_ids().await.unwrap();
        assert_eq!(rows2.len(), 1, "REPLACE 가 새 행을 만들면 안 된다");
        assert!(!rows2[0].success);

        let json = store.get_trajectory_json(task_id).await.unwrap().unwrap();
        assert_eq!(json["task_description"], "단리 이자 계산 v2");
        assert_eq!(json["total_iterations"], 4);
    }

    /// @trace TC: SPEC-021/TC-3
    /// @trace FR: PRD-021/FR-2
    #[tokio::test]
    async fn spec021_tc_3_evaluation_upsert_with_fk() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        let task_id = "550e8400-e29b-41d4-a716-446655440002";
        store
            .upsert_trajectory(
                task_id,
                "task",
                "ppa",
                Some("financial"),
                Some("fin_001"),
                true,
                2,
                "2026-04-11T10:00:00Z",
                Some("2026-04-11T10:00:05Z"),
                "[]",
                None,
                None,
            )
            .await
            .unwrap();
        store
            .upsert_evaluation(
                task_id,
                "ppa",
                Some("financial"),
                Some("fin_001"),
                true,
                Some(0.9),
                Some(1.0),
                Some(1.0),
                Some(0.93),
                "{\"latency\":0.5}",
                Some("{\"criteria_score\":0.9}"),
            )
            .await
            .unwrap();
        let rows = store.list_evaluation_ids().await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].task_id, task_id);
        assert_eq!(rows[0].overall_score, Some(0.93));

        let json = store.get_evaluation_json(task_id).await.unwrap().unwrap();
        assert_eq!(json["scores"]["domain_routing_score"], 1.0);
    }

    /// @trace TC: SPEC-021/TC-10
    /// @trace FR: PRD-021/FR-6
    #[tokio::test]
    async fn spec021_tc_10_evaluation_window_average() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        for (i, score) in [0.6_f64, 0.8, 1.0].iter().enumerate() {
            let task_id = format!("550e8400-e29b-41d4-a716-44665544000{i}");
            store
                .upsert_trajectory(
                    &task_id,
                    "t",
                    "ppa",
                    Some("financial"),
                    None,
                    true,
                    1,
                    "2026-04-11T10:00:00Z",
                    None,
                    "[]",
                    None,
                    None,
                )
                .await
                .unwrap();
            store
                .upsert_evaluation(
                    &task_id,
                    "ppa",
                    Some("financial"),
                    None,
                    true,
                    Some(*score),
                    Some(*score),
                    Some(1.0),
                    Some(*score),
                    "{}",
                    None,
                )
                .await
                .unwrap();
        }
        let win = store
            .evaluation_window_average("ppa", "1900-01-01 00:00:00", "2099-12-31 23:59:59")
            .await
            .unwrap();
        assert_eq!(win.count, 3);
        assert_eq!(win.successes, 3);
        let overall = win.overall_score.unwrap();
        assert!((overall - 0.8).abs() < 1e-9, "expected average 0.8, got {overall}");
    }

    // -------- SPEC-022 --------

    /// @trace TC: SPEC-022/TC-3
    /// @trace FR: PRD-022/FR-1
    #[tokio::test]
    async fn spec022_tc_3_insert_update_domain() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("healthcare", "의료 도메인").await.unwrap();
        let s = store.get_domain_summary("healthcare").await.unwrap().unwrap();
        assert_eq!(s.name, "healthcare");
        assert_eq!(s.description, "의료 도메인");
        assert_eq!(s.scenario_count, 0);

        store.update_domain("healthcare", "갱신된 설명").await.unwrap();
        let s2 = store.get_domain_summary("healthcare").await.unwrap().unwrap();
        assert_eq!(s2.description, "갱신된 설명");

        // 미존재 갱신 → NotFound
        let err = store.update_domain("nope", "x").await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(_)));
    }

    /// @trace TC: SPEC-022/TC-4
    /// @trace FR: PRD-022/FR-1
    #[tokio::test]
    async fn spec022_tc_4_replace_domain_tools() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("legal", "법률").await.unwrap();
        store
            .replace_domain_tools("legal", &["read_file".to_string(), "write_file".to_string(), "list_directory".to_string()])
            .await
            .unwrap();
        let s = store.get_domain_summary("legal").await.unwrap().unwrap();
        assert_eq!(s.tool_class_names.len(), 3);

        // 통째로 교체 → 2개로 줄어듦
        store
            .replace_domain_tools("legal", &["read_file".to_string(), "write_file".to_string()])
            .await
            .unwrap();
        let s2 = store.get_domain_summary("legal").await.unwrap().unwrap();
        assert_eq!(s2.tool_class_names.len(), 2);
    }

    /// @trace TC: SPEC-022/TC-5
    /// @trace FR: PRD-022/FR-3
    #[tokio::test]
    async fn spec022_tc_5_replace_domain_keywords() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("healthcare", "").await.unwrap();
        store
            .replace_domain_keywords("healthcare", &["환자".to_string(), "처방".to_string(), "치료".to_string(), "  ".to_string()])
            .await
            .unwrap();
        let s = store.get_domain_summary("healthcare").await.unwrap().unwrap();
        assert_eq!(s.keywords.len(), 3, "공백 키워드는 무시됨");
        assert!(s.keywords.contains(&"환자".to_string()));
    }

    /// @trace TC: SPEC-022/TC-6
    /// @trace FR: PRD-022/FR-4
    #[tokio::test]
    async fn spec022_tc_6_delete_cascade() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("healthcare", "").await.unwrap();
        store.replace_domain_tools("healthcare", &["read_file".to_string()]).await.unwrap();
        store.replace_domain_keywords("healthcare", &["환자".to_string()]).await.unwrap();

        store.delete_domain("healthcare").await.unwrap();
        assert!(store.get_domain_summary("healthcare").await.unwrap().is_none());

        // cascade 확인: domain_keywords 와 domain_tools 도 0
        let kw_map = store.list_all_domain_keywords().await.unwrap();
        assert!(!kw_map.contains_key("healthcare"));
    }

    /// @trace TC: SPEC-022/TC-8
    /// @trace FR: PRD-022/FR-3
    #[tokio::test]
    async fn spec022_tc_8_list_all_domain_keywords() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("a", "").await.unwrap();
        store.insert_domain("b", "").await.unwrap();
        store.replace_domain_keywords("a", &["x".to_string(), "y".to_string()]).await.unwrap();
        store.replace_domain_keywords("b", &["z".to_string()]).await.unwrap();
        let map = store.list_all_domain_keywords().await.unwrap();
        assert_eq!(map.get("a").unwrap().len(), 2);
        assert_eq!(map.get("b").unwrap().len(), 1);
    }

    /// @trace TC: SPEC-022/TC-2
    /// @trace FR: PRD-022/FR-7
    #[tokio::test]
    async fn spec022_tc_2_seed_domain_keywords_idempotent() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("financial", "").await.unwrap();
        let pairs = vec![("financial".to_string(), "이자".to_string()), ("financial".to_string(), "대출".to_string())];
        let n1 = store.seed_domain_keywords(&pairs).await.unwrap();
        assert_eq!(n1, 2);
        let n2 = store.seed_domain_keywords(&pairs).await.unwrap();
        assert_eq!(n2, 0, "재호출은 INSERT OR IGNORE 로 0건");
    }

    // -------- SPEC-023 --------

    fn sample_external_row(name: &str, domain: &str) -> ExternalToolRow {
        ExternalToolRow {
            name: name.to_string(),
            domain: domain.to_string(),
            description: "테스트 도구".into(),
            method: "POST".into(),
            url: "http://localhost:9000/q".into(),
            headers_json: Some(r#"{"X-Auth":"abc"}"#.to_string()),
            body_template: r#"{"q":"{{topic}}"}"#.to_string(),
            params_schema: r#"{"type":"object"}"#.to_string(),
            timeout_ms: 5000,
        }
    }

    /// @trace TC: SPEC-023/TC-2
    /// @trace FR: PRD-023/FR-1
    #[tokio::test]
    async fn spec023_tc_2_upsert_and_list_external_tools() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("healthcare", "").await.unwrap();
        store.upsert_external_tool(&sample_external_row("search_patient", "healthcare")).await.unwrap();
        store.upsert_external_tool(&sample_external_row("get_record", "healthcare")).await.unwrap();
        let rows = store.list_external_tools().await.unwrap();
        assert_eq!(rows.len(), 2);
        let by_dom = store.list_external_tools_by_domain("healthcare").await.unwrap();
        assert_eq!(by_dom.len(), 2);

        // upsert (REPLACE) → 행 수 유지
        let mut updated = sample_external_row("search_patient", "healthcare");
        updated.description = "갱신".into();
        store.upsert_external_tool(&updated).await.unwrap();
        let rows2 = store.list_external_tools().await.unwrap();
        assert_eq!(rows2.len(), 2);
        assert!(rows2.iter().any(|r| r.name == "search_patient" && r.description == "갱신"));
    }

    /// @trace TC: SPEC-023/TC-3
    /// @trace FR: PRD-023/FR-4
    #[tokio::test]
    async fn spec023_tc_3_delete_domain_cascades_external_tools() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("healthcare", "").await.unwrap();
        store.upsert_external_tool(&sample_external_row("search_patient", "healthcare")).await.unwrap();
        store.delete_domain("healthcare").await.unwrap();
        let rows = store.list_external_tools().await.unwrap();
        assert!(rows.is_empty(), "도메인 삭제 시 external_tools cascade 삭제");
    }

    #[tokio::test]
    async fn spec023_delete_external_tool_not_found() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("healthcare", "").await.unwrap();
        let err = store.delete_external_tool("healthcare", "nope").await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(_)));
    }

    // -------- SPEC-025 --------

    fn sample_bundle() -> BootstrapBundleRef<'static> {
        BootstrapBundleRef {
            perceive_system: "SYS-PER {domain_name}",
            perceive_user: "작업: {task_description}\n환경: {environment_state}{context}",
            policy_system: "SYS-POL",
            policy_user: "작업: {task_description}\n인지: {perceived_info}\n도구: {tools}{context}",
        }
    }

    /// @trace TC: SPEC-025/TC-1
    /// @trace FR: PRD-025/NFR-1
    #[tokio::test]
    async fn spec025_tc_1_migration_idempotent() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        // 재호출 멱등
        store.init_schema().await.unwrap();
        // prompt_sets 테이블 존재
        let tbl: Option<String> = sqlx::query_scalar("SELECT name FROM sqlite_master WHERE type='table' AND name='prompt_sets'")
            .fetch_optional(&store.pool)
            .await
            .unwrap();
        assert_eq!(tbl.as_deref(), Some("prompt_sets"));
        // trajectories/evaluations 에 prompt_set_id 컬럼 존재
        for t in ["trajectories", "evaluations"] {
            let cols = sqlx::query(&format!("PRAGMA table_info('{t}')")).fetch_all(&store.pool).await.unwrap();
            let has = cols.iter().any(|r| {
                let n: String = r.get("name");
                n == "prompt_set_id"
            });
            assert!(has, "{t} 에 prompt_set_id 컬럼 필요");
        }
    }

    /// @trace TC: SPEC-025/TC-2
    /// @trace FR: PRD-025/FR-2
    #[tokio::test]
    async fn spec025_tc_2_bootstrap_seed_matches_bundle() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        store.insert_domain("financial", "").await.unwrap();
        let b = sample_bundle();
        let n = store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        assert_eq!(n, 2);
        let active = store.get_active_prompt_set("customer_service").await.unwrap().unwrap();
        assert_eq!(active.version, 1);
        assert!(active.is_bootstrap);
        assert!(active.is_active);
        assert_eq!(active.perceive_system, b.perceive_system);
        assert_eq!(active.perceive_user, b.perceive_user);
        assert_eq!(active.policy_system, b.policy_system);
        assert_eq!(active.policy_user, b.policy_user);
    }

    /// @trace TC: SPEC-025/TC-3
    /// @trace FR: PRD-025/NFR-2
    #[tokio::test]
    async fn spec025_tc_3_bootstrap_seed_idempotent() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let b = sample_bundle();
        assert_eq!(store.seed_bootstrap_prompt_sets(&b).await.unwrap(), 1);
        assert_eq!(store.seed_bootstrap_prompt_sets(&b).await.unwrap(), 0, "재호출은 도메인당 행이 이미 있어 skip");
        let all = store.list_prompt_sets("customer_service").await.unwrap();
        assert_eq!(all.len(), 1);
    }

    /// @trace TC: SPEC-025/TC-6, SPEC-025/TC-16
    /// @trace FR: PRD-025/FR-1, PRD-025/FR-6
    #[tokio::test]
    async fn spec025_tc_6_insert_auto_version_and_list_desc() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let b = sample_bundle();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        // v2 생성
        let v2 = store
            .insert_prompt_set(PromptSetInsert {
                domain_name: "customer_service",
                perceive_system: "v2 per sys",
                perceive_user: "{task_description} / {environment_state}",
                policy_system: "v2 pol sys",
                policy_user: "{task_description} / {perceived_info} / {tools}",
                notes: Some("second"),
                is_bootstrap: false,
            })
            .await
            .unwrap();
        assert_eq!(v2.version, 2);
        assert!(!v2.is_active, "새 버전은 기본적으로 비활성");
        assert!(!v2.is_bootstrap);
        // list 는 version DESC
        let list = store.list_prompt_sets("customer_service").await.unwrap();
        assert_eq!(list.iter().map(|r| r.version).collect::<Vec<_>>(), vec![2, 1]);
    }

    /// @trace TC: SPEC-025/TC-7
    /// @trace FR: PRD-025/FR-3, PRD-025/FR-7, PRD-025/NFR-4
    #[tokio::test]
    async fn spec025_tc_7_activate_atomic_toggle() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let b = sample_bundle();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        store
            .insert_prompt_set(PromptSetInsert {
                domain_name: "customer_service",
                perceive_system: "v2",
                perceive_user: "{task_description} {environment_state}",
                policy_system: "v2",
                policy_user: "{task_description} {perceived_info} {tools}",
                notes: None,
                is_bootstrap: false,
            })
            .await
            .unwrap();
        // 활성 전환
        store.activate_prompt_set("customer_service", 2).await.unwrap();
        let active = store.get_active_prompt_set("customer_service").await.unwrap().unwrap();
        assert_eq!(active.version, 2);
        // 기존 v1 은 비활성
        let v1 = store.get_prompt_set("customer_service", 1).await.unwrap().unwrap();
        assert!(!v1.is_active);
        // partial unique index 확인: 활성 행이 정확히 1개
        let active_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prompt_sets WHERE domain_name='customer_service' AND is_active=1")
            .fetch_one(&store.pool)
            .await
            .unwrap();
        assert_eq!(active_count, 1);
    }

    /// @trace TC: SPEC-025/TC-8
    /// @trace FR: PRD-025/FR-7
    #[tokio::test]
    async fn spec025_tc_8_cannot_delete_active_version() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let b = sample_bundle();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        // v2 생성 후 활성화
        store
            .insert_prompt_set(PromptSetInsert {
                domain_name: "customer_service",
                perceive_system: "v2",
                perceive_user: "{task_description} {environment_state}",
                policy_system: "v2",
                policy_user: "{task_description} {perceived_info} {tools}",
                notes: None,
                is_bootstrap: false,
            })
            .await
            .unwrap();
        store.activate_prompt_set("customer_service", 2).await.unwrap();
        let err = store.delete_prompt_set("customer_service", 2).await.unwrap_err();
        assert!(matches!(err, StoreError::Conflict(msg) if msg.contains("active")));
    }

    /// @trace TC: SPEC-025/TC-9
    /// @trace FR: PRD-025/FR-7
    #[tokio::test]
    async fn spec025_tc_9_cannot_delete_bootstrap_version() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let b = sample_bundle();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        // v2 생성해 v1 을 비활성 상태로 만든 뒤 v1(bootstrap) 삭제 시도
        store
            .insert_prompt_set(PromptSetInsert {
                domain_name: "customer_service",
                perceive_system: "v2",
                perceive_user: "{task_description} {environment_state}",
                policy_system: "v2",
                policy_user: "{task_description} {perceived_info} {tools}",
                notes: None,
                is_bootstrap: false,
            })
            .await
            .unwrap();
        store.activate_prompt_set("customer_service", 2).await.unwrap();
        let err = store.delete_prompt_set("customer_service", 1).await.unwrap_err();
        assert!(matches!(err, StoreError::Conflict(msg) if msg.contains("bootstrap")));
    }

    /// @trace TC: SPEC-025/TC-10
    /// @trace FR: PRD-025/FR-1
    #[tokio::test]
    async fn spec025_tc_10_domain_cascade_drops_prompt_sets() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let b = sample_bundle();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        assert_eq!(store.list_prompt_sets("customer_service").await.unwrap().len(), 1);
        store.delete_domain("customer_service").await.unwrap();
        let rows = store.list_prompt_sets("customer_service").await.unwrap();
        assert!(rows.is_empty(), "도메인 CASCADE 로 prompt_sets 도 삭제");
    }

    #[tokio::test]
    async fn spec025_activate_nonexistent_version_not_found() {
        let store = SqliteStore::open_in_memory().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let err = store.activate_prompt_set("customer_service", 99).await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(_)));
    }
}
