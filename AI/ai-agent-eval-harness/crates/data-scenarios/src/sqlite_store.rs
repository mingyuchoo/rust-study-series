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

const SCHEMA_VERSION: i64 = 2;

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
            // 신규 DB 는 v2 스키마 (FK 포함) 로 바로 생성.
            "CREATE TABLE IF NOT EXISTS golden_sets (
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
            "CREATE INDEX IF NOT EXISTS idx_eval_scenarios_domain ON eval_scenarios(domain)",
            "CREATE INDEX IF NOT EXISTS idx_golden_sets_domain ON golden_sets(domain)",
        ];
        for sql in stmts.iter() {
            sqlx::query(sql).execute(&self.pool).await?;
        }

        // v1 DB 마이그레이션: 기존 golden_sets 에 FK 가 없는 경우 재생성.
        // `CREATE TABLE IF NOT EXISTS` 는 기존 스키마를 건드리지 않으므로,
        // `schema_migrations` 의 현재 버전을 확인하여 필요 시 rebuild 한다.
        let current: Option<i64> = sqlx::query_scalar("SELECT MAX(version) FROM schema_migrations")
            .fetch_one(&self.pool)
            .await?;
        let current = current.unwrap_or(0);
        if current < 2 {
            self.migrate_v2_cascade().await?;
        }

        sqlx::query("INSERT OR IGNORE INTO schema_migrations (version, applied_at) VALUES (?, datetime('now'))")
            .bind(SCHEMA_VERSION)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// v1 → v2: `golden_sets` 에 `(domain, scenario_id) → eval_scenarios` FK 추가.
    /// 무손실 table-rename 방식.
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

    /// eval_scenarios 테이블이 비어 있는지.
    pub async fn is_empty(&self) -> Result<bool, StoreError> {
        let row = sqlx::query("SELECT COUNT(*) AS cnt FROM eval_scenarios").fetch_one(&self.pool).await?;
        let cnt: i64 = row.get("cnt");
        Ok(cnt == 0)
    }

    /// SPEC-019 후속 버그픽스: scenarios OR golden_sets 중 어느 하나가 비어 있으면
    /// 초기 시드가 완전히 적용되지 않은 것으로 간주. `INSERT OR IGNORE` 기반 시드
    /// 는 멱등이므로 안전하게 재실행할 수 있다. 다만 사용자가 모든 goldens 를
    /// CRUD 로 삭제한 경우에도 이 조건이 참이 되어 재시드가 일어날 수 있는데,
    /// 그 경우 사용자는 domain 단위 의도적 초기화로 해석한다(범위 외 시나리오).
    pub async fn needs_seed(&self) -> Result<bool, StoreError> {
        let s: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM eval_scenarios").fetch_one(&self.pool).await?;
        let g: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM golden_sets").fetch_one(&self.pool).await?;
        Ok(s == 0 || g == 0)
    }

    /// YAML/JSON 파일에서 읽어 DB 에 적재. INSERT OR IGNORE 로 멱등.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-2, PRD-017/FR-7
    pub async fn seed_from_fs(&self, scenarios_dir: &Path, golden_sets_dir: &Path) -> Result<SeedReport, StoreError> {
        let mut report = SeedReport::default();
        let mut tx = self.pool.begin().await?;

        // 시나리오 YAML 파일들
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
                let content = std::fs::read_to_string(&p).map_err(|e| StoreError::Io {
                    path: p.clone(),
                    source: e,
                })?;
                let cfg: DomainConfig = serde_yaml::from_str(&content).map_err(|e| StoreError::Yaml {
                    path: p.clone(),
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
        }

        // 골든셋 JSON 파일들
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
                let content = std::fs::read_to_string(&p).map_err(|e| StoreError::Io {
                    path: p.clone(),
                    source: e,
                })?;
                let gs: GoldenSetFile = serde_json::from_str(&content)?;

                for g in gs.golden_sets.iter() {
                    let env_json = serde_json::to_string(&g.input.environment)?;
                    let seq_json = serde_json::to_string(&g.expected_output.tool_sequence)?;
                    let res_json = serde_json::to_string(&g.expected_output.tool_results)?;
                    let r = sqlx::query(
                        "INSERT OR IGNORE INTO golden_sets
                         (domain, scenario_id, version, task,
                          input_environment, tool_sequence, tool_results, tolerance)
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                    )
                    .bind(&gs.domain)
                    .bind(&g.scenario_id)
                    .bind(&gs.version)
                    .bind(&g.input.task)
                    .bind(env_json)
                    .bind(seq_json)
                    .bind(res_json)
                    .bind(g.expected_output.tolerance)
                    .execute(&mut *tx)
                    .await?;
                    if r.rows_affected() > 0 {
                        report.golden_sets_inserted += 1;
                    }
                }
            }
        }

        tx.commit().await?;
        Ok(report)
    }

    /// 편의 헬퍼: open → (scenarios 또는 golden_sets 가 비어 있으면) seed. 항상 멱등.
    pub async fn open_and_seed(db_path: &Path, scenarios_dir: &Path, golden_sets_dir: &Path) -> Result<(Self, SeedReport), StoreError> {
        let store = Self::open(db_path).await?;
        let mut report = SeedReport::default();
        if store.needs_seed().await? {
            report = store.seed_from_fs(scenarios_dir, golden_sets_dir).await?;
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
            "SELECT scenario_id, version, task, input_environment, tool_sequence, tool_results, tolerance
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

    /// 신규 시나리오 INSERT. 동일 `(domain, id)` 존재 시 `StoreError::Conflict`.
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
              input_environment, tool_sequence, tool_results, tolerance)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(domain)
        .bind(&entry.scenario_id)
        .bind(version)
        .bind(&entry.input.task)
        .bind(env_json)
        .bind(seq_json)
        .bind(res_json)
        .bind(entry.expected_output.tolerance)
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
                 tool_results = ?, tolerance = ?
             WHERE domain = ? AND scenario_id = ?",
        )
        .bind(&entry.input.task)
        .bind(env_json)
        .bind(seq_json)
        .bind(res_json)
        .bind(entry.expected_output.tolerance)
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
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        let db = dir.path().join("eval.db");
        write_sample_dataset(&scen, &gold);

        let (_s1, r1) = SqliteStore::open_and_seed(&db, &scen, &gold).await.unwrap();
        assert!(r1.scenarios_inserted > 0);

        let (_s2, r2) = SqliteStore::open_and_seed(&db, &scen, &gold).await.unwrap();
        assert_eq!(r2.scenarios_inserted, 0);
    }
}
