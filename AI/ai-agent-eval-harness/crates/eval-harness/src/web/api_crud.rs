// =============================================================================
// @trace SPEC-019
// @trace PRD: PRD-019
// @trace FR: PRD-019/FR-1, PRD-019/FR-2, PRD-019/FR-3, PRD-019/FR-4,
// PRD-019/FR-5, PRD-019/FR-7
// @trace file-type: impl
// =============================================================================

use agent_models::domain_config::ScenarioConfig;
use axum::{extract::{Json as JsonExt,
                     Path as AxPath,
                     State},
           http::StatusCode,
           response::{IntoResponse,
                      Json as JsonOut}};
use data_scenarios::{models::{GoldenSetEntry,
                              GoldenSetExpectedOutput,
                              GoldenSetInput},
                     sqlite_store::{SqliteStore,
                                    StoreError}};
use serde::{Deserialize,
            Serialize};
use std::{collections::HashMap,
          sync::Arc};

use super::AppState;

// --------------------------------------------------------------------------
// Request / response DTOs
// --------------------------------------------------------------------------

/// 시나리오 생성/수정 요청 본문.
#[derive(Debug, Deserialize)]
pub struct EvalScenarioUpsert {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub task_description: String,
    #[serde(default)]
    pub initial_environment: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub expected_tools: Vec<String>,
    #[serde(default)]
    pub success_criteria: HashMap<String, serde_json::Value>,
    #[serde(default = "default_difficulty")]
    pub difficulty: String,
    #[serde(default)]
    pub position: Option<i64>,
}

fn default_difficulty() -> String { "medium".into() }

/// 골든셋 엔트리 생성/수정 요청 본문.
#[derive(Debug, Deserialize)]
pub struct GoldenEntryUpsert {
    pub scenario_id: String,
    #[serde(default = "default_version")]
    pub version: String,
    pub task: String,
    #[serde(default)]
    pub environment: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tool_sequence: Vec<String>,
    #[serde(default)]
    pub tool_results: HashMap<String, serde_json::Value>,
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,
}

fn default_version() -> String { "1.0".into() }
fn default_tolerance() -> f64 { 0.01 }

#[derive(Debug, Serialize)]
pub struct CrudError {
    pub error: &'static str,
    pub detail: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CrudFailure {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("internal: {0}")]
    Internal(String),
}

impl CrudFailure {
    fn status_and_kind(&self) -> (StatusCode, &'static str) {
        match self {
            | Self::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            | Self::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            | Self::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            | Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        }
    }
}

impl From<StoreError> for CrudFailure {
    fn from(err: StoreError) -> Self {
        match err {
            | StoreError::Conflict(m) => Self::Conflict(m),
            | StoreError::NotFound(m) => Self::NotFound(m),
            | other => Self::Internal(other.to_string()),
        }
    }
}

impl IntoResponse for CrudFailure {
    fn into_response(self) -> axum::response::Response {
        let (status, kind) = self.status_and_kind();
        let body = JsonOut(CrudError {
            error: kind,
            detail: self.to_string(),
        });
        (status, body).into_response()
    }
}

// --------------------------------------------------------------------------
// ID / 필드 검증
// --------------------------------------------------------------------------

fn validate_id(kind: &str, value: &str) -> Result<(), CrudFailure> {
    if value.is_empty() || value.len() > 64 {
        return Err(CrudFailure::BadRequest(format!("{kind} length must be 1..=64")));
    }
    if !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err(CrudFailure::BadRequest(format!("{kind} must match ^[A-Za-z0-9_-]+$")));
    }
    Ok(())
}

fn validate_non_empty(kind: &str, value: &str) -> Result<(), CrudFailure> {
    if value.trim().is_empty() {
        return Err(CrudFailure::BadRequest(format!("{kind} must not be empty")));
    }
    Ok(())
}

// --------------------------------------------------------------------------
// *_impl: 스토어 직접 호출 (테스트 용이)
// --------------------------------------------------------------------------

/// 신규 시나리오 생성.
///
/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1, PRD-019/FR-5
pub async fn create_scenario_impl(store: &SqliteStore, domain: &str, body: EvalScenarioUpsert) -> Result<ScenarioConfig, CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("id", &body.id)?;
    validate_non_empty("name", &body.name)?;
    validate_non_empty("task_description", &body.task_description)?;

    let scen = ScenarioConfig {
        id: body.id,
        name: body.name,
        description: body.description,
        task_description: body.task_description,
        initial_environment: body.initial_environment,
        expected_tools: body.expected_tools,
        success_criteria: body.success_criteria,
        difficulty: body.difficulty,
    };
    let position = body.position.unwrap_or(9999);
    store.insert_scenario(domain, &scen, position).await?;
    Ok(scen)
}

/// 시나리오 수정.
///
/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1, PRD-019/FR-5
pub async fn update_scenario_impl(store: &SqliteStore, domain: &str, id: &str, body: EvalScenarioUpsert) -> Result<ScenarioConfig, CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("id", id)?;
    validate_non_empty("name", &body.name)?;
    validate_non_empty("task_description", &body.task_description)?;

    let scen = ScenarioConfig {
        id: id.to_string(),
        name: body.name,
        description: body.description,
        task_description: body.task_description,
        initial_environment: body.initial_environment,
        expected_tools: body.expected_tools,
        success_criteria: body.success_criteria,
        difficulty: body.difficulty,
    };
    store.update_scenario(domain, id, &scen).await?;
    Ok(scen)
}

/// 시나리오 삭제.
///
/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1, PRD-019/FR-4
pub async fn delete_scenario_impl(store: &SqliteStore, domain: &str, id: &str) -> Result<(), CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("id", id)?;
    store.delete_scenario(domain, id).await?;
    Ok(())
}

/// 골든셋 엔트리 생성.
///
/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2, PRD-019/FR-5
pub async fn create_golden_entry_impl(store: &SqliteStore, domain: &str, body: GoldenEntryUpsert) -> Result<GoldenSetEntry, CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("scenario_id", &body.scenario_id)?;
    validate_non_empty("task", &body.task)?;

    let version = body.version.clone();
    let entry = GoldenSetEntry {
        scenario_id: body.scenario_id,
        input: GoldenSetInput {
            task: body.task,
            environment: body.environment,
        },
        expected_output: GoldenSetExpectedOutput {
            tool_sequence: body.tool_sequence,
            tool_results: body.tool_results,
            tolerance: body.tolerance,
        },
    };
    store.insert_golden_entry(domain, &version, &entry).await?;
    Ok(entry)
}

/// 골든셋 엔트리 수정.
///
/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2, PRD-019/FR-5
pub async fn update_golden_entry_impl(store: &SqliteStore, domain: &str, scenario_id: &str, body: GoldenEntryUpsert) -> Result<GoldenSetEntry, CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("scenario_id", scenario_id)?;
    validate_non_empty("task", &body.task)?;

    let entry = GoldenSetEntry {
        scenario_id: scenario_id.to_string(),
        input: GoldenSetInput {
            task: body.task,
            environment: body.environment,
        },
        expected_output: GoldenSetExpectedOutput {
            tool_sequence: body.tool_sequence,
            tool_results: body.tool_results,
            tolerance: body.tolerance,
        },
    };
    store.update_golden_entry(domain, scenario_id, &entry).await?;
    Ok(entry)
}

/// 골든셋 엔트리 삭제.
///
/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2, PRD-019/FR-5
pub async fn delete_golden_entry_impl(store: &SqliteStore, domain: &str, scenario_id: &str) -> Result<(), CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("scenario_id", scenario_id)?;
    store.delete_golden_entry(domain, scenario_id).await?;
    Ok(())
}

// --------------------------------------------------------------------------
// Axum 핸들러 (AppState.store 를 꺼내 *_impl 호출)
// --------------------------------------------------------------------------

fn store_from(state: &AppState) -> Result<Arc<SqliteStore>, CrudFailure> {
    state.store.clone().ok_or_else(|| CrudFailure::Internal("SqliteStore not installed in AppState".into()))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1
pub async fn create_scenario(State(st): State<AppState>, AxPath(domain): AxPath<String>, JsonExt(body): JsonExt<EvalScenarioUpsert>) -> Result<(StatusCode, JsonOut<ScenarioConfig>), CrudFailure> {
    let store = store_from(&st)?;
    let scen = create_scenario_impl(store.as_ref(), &domain, body).await?;
    Ok((StatusCode::CREATED, JsonOut(scen)))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1
pub async fn update_scenario_handler(State(st): State<AppState>, AxPath((domain, id)): AxPath<(String, String)>, JsonExt(body): JsonExt<EvalScenarioUpsert>) -> Result<JsonOut<ScenarioConfig>, CrudFailure> {
    let store = store_from(&st)?;
    let scen = update_scenario_impl(store.as_ref(), &domain, &id, body).await?;
    Ok(JsonOut(scen))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1, PRD-019/FR-4
pub async fn delete_scenario_handler(State(st): State<AppState>, AxPath((domain, id)): AxPath<(String, String)>) -> Result<StatusCode, CrudFailure> {
    let store = store_from(&st)?;
    delete_scenario_impl(store.as_ref(), &domain, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2
pub async fn create_golden_entry(State(st): State<AppState>, AxPath(domain): AxPath<String>, JsonExt(body): JsonExt<GoldenEntryUpsert>) -> Result<(StatusCode, JsonOut<GoldenSetEntry>), CrudFailure> {
    let store = store_from(&st)?;
    let entry = create_golden_entry_impl(store.as_ref(), &domain, body).await?;
    Ok((StatusCode::CREATED, JsonOut(entry)))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2
pub async fn update_golden_entry_handler(State(st): State<AppState>, AxPath((domain, scenario_id)): AxPath<(String, String)>, JsonExt(body): JsonExt<GoldenEntryUpsert>) -> Result<JsonOut<GoldenSetEntry>, CrudFailure> {
    let store = store_from(&st)?;
    let entry = update_golden_entry_impl(store.as_ref(), &domain, &scenario_id, body).await?;
    Ok(JsonOut(entry))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2
pub async fn delete_golden_entry_handler(State(st): State<AppState>, AxPath((domain, scenario_id)): AxPath<(String, String)>) -> Result<StatusCode, CrudFailure> {
    let store = store_from(&st)?;
    delete_golden_entry_impl(store.as_ref(), &domain, &scenario_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// Tests
//
// @trace SPEC-019
// @trace PRD: PRD-019
// @trace TC: SPEC-019/TC-9, TC-10, TC-11, TC-12, TC-13, TC-14
// @trace file-type: test
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use data_scenarios::sqlite_store::SqliteStore;
    use std::{fs,
              path::Path,
              time::SystemTime};
    use tempfile::tempdir;

    fn write_seed(scen_dir: &Path, gold_dir: &Path) {
        fs::create_dir_all(scen_dir).unwrap();
        fs::create_dir_all(gold_dir).unwrap();
        let fin_yaml = r#"
name: financial
description: 금융
tools:
  - class_name: T
    module_path: m
scenarios:
  - id: fin_001
    name: 시나리오1
    description: d
    task_description: 단리
    initial_environment:
      x: 1
    expected_tools:
      - t
    success_criteria:
      ok: true
    difficulty: easy
  - id: fin_002
    name: 시나리오2
    description: d
    task_description: 복리
    initial_environment:
      y: 2
    expected_tools:
      - t
    success_criteria:
      ok: true
    difficulty: easy
"#;
        fs::write(scen_dir.join("financial.yaml"), fin_yaml).unwrap();
        let fin_gs = r#"{"domain":"financial","version":"1.0","golden_sets":[
            {"scenario_id":"fin_001","input":{"task":"t","environment":{}},
             "expected_output":{"tool_sequence":["t"],"tool_results":{},"tolerance":0.01}}]}"#;
        fs::write(gold_dir.join("financial.json"), fin_gs).unwrap();
    }

    async fn make_store() -> SqliteStore {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_seed(&scen, &gold);
        let store = SqliteStore::open_in_memory_for_loader().await.unwrap();
        store.seed_from_fs(&scen, &gold).await.unwrap();
        store
    }

    fn upsert_body(id: &str, task: &str) -> EvalScenarioUpsert {
        EvalScenarioUpsert {
            id: id.to_string(),
            name: format!("이름-{id}"),
            description: "d".into(),
            task_description: task.into(),
            initial_environment: HashMap::new(),
            expected_tools: vec![],
            success_criteria: HashMap::new(),
            difficulty: "easy".into(),
            position: Some(1),
        }
    }

    fn golden_body(scenario_id: &str) -> GoldenEntryUpsert {
        GoldenEntryUpsert {
            scenario_id: scenario_id.to_string(),
            version: "1.0".into(),
            task: "t".into(),
            environment: HashMap::new(),
            tool_sequence: vec!["t".into()],
            tool_results: HashMap::new(),
            tolerance: 0.01,
        }
    }

    /// @trace TC: SPEC-019/TC-9
    /// @trace FR: PRD-019/FR-1, PRD-019/FR-5
    #[tokio::test]
    async fn tc_9_create_scenario_then_conflict() {
        let store = make_store().await;
        // 성공
        let scen = create_scenario_impl(&store, "financial", upsert_body("fin_new", "t")).await.expect("create ok");
        assert_eq!(scen.id, "fin_new");
        // 중복 → Conflict
        let err = create_scenario_impl(&store, "financial", upsert_body("fin_new", "t")).await.unwrap_err();
        assert!(matches!(err, CrudFailure::Conflict(_)));
        let (status, kind) = err.status_and_kind();
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(kind, "conflict");
    }

    /// @trace TC: SPEC-019/TC-10
    /// @trace FR: PRD-019/FR-1, PRD-019/FR-5
    #[tokio::test]
    async fn tc_10_update_scenario_ok_and_not_found() {
        let store = make_store().await;
        let updated = update_scenario_impl(&store, "financial", "fin_001", upsert_body("fin_001", "새 task")).await.expect("update ok");
        assert_eq!(updated.task_description, "새 task");

        let err = update_scenario_impl(&store, "financial", "no_such", upsert_body("no_such", "x")).await.unwrap_err();
        assert!(matches!(err, CrudFailure::NotFound(_)));
        let (status, _) = err.status_and_kind();
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    /// @trace TC: SPEC-019/TC-11
    /// @trace FR: PRD-019/FR-1, PRD-019/FR-4
    #[tokio::test]
    async fn tc_11_delete_scenario_cascades_golden() {
        let store = make_store().await;
        delete_scenario_impl(&store, "financial", "fin_001").await.expect("delete ok");

        let gs = store.load_golden_sets_by_domain("financial").await.unwrap();
        assert!(!gs.golden_sets.iter().any(|e| e.scenario_id == "fin_001"));

        let err = delete_scenario_impl(&store, "financial", "fin_001").await.unwrap_err();
        assert!(matches!(err, CrudFailure::NotFound(_)));
    }

    /// @trace TC: SPEC-019/TC-12
    /// @trace FR: PRD-019/FR-2, PRD-019/FR-5
    #[tokio::test]
    async fn tc_12_golden_crud_cycle() {
        let store = make_store().await;
        let created = create_golden_entry_impl(&store, "financial", golden_body("fin_002")).await.expect("create ok");
        assert_eq!(created.scenario_id, "fin_002");

        let mut upd = golden_body("fin_002");
        upd.task = "updated".into();
        let updated = update_golden_entry_impl(&store, "financial", "fin_002", upd).await.expect("update ok");
        assert_eq!(updated.input.task, "updated");

        delete_golden_entry_impl(&store, "financial", "fin_002").await.expect("delete ok");

        let err = delete_golden_entry_impl(&store, "financial", "fin_002").await.unwrap_err();
        assert!(matches!(err, CrudFailure::NotFound(_)));
    }

    /// @trace TC: SPEC-019/TC-13
    /// @trace FR: PRD-019/FR-3
    #[tokio::test]
    async fn tc_13_files_mtime_unchanged_after_crud() {
        let dir = tempdir().unwrap();
        let scen = dir.path().join("scen");
        let gold = dir.path().join("gold");
        write_seed(&scen, &gold);

        let scen_path = scen.join("financial.yaml");
        let gold_path = gold.join("financial.json");
        let scen_mtime_before = fs::metadata(&scen_path).unwrap().modified().unwrap();
        let gold_mtime_before = fs::metadata(&gold_path).unwrap().modified().unwrap();

        let store = SqliteStore::open_in_memory_for_loader().await.unwrap();
        store.seed_from_fs(&scen, &gold).await.unwrap();

        // 전체 CRUD 사이클
        create_scenario_impl(&store, "financial", upsert_body("fin_new", "t")).await.unwrap();
        update_scenario_impl(&store, "financial", "fin_001", upsert_body("fin_001", "변경")).await.unwrap();
        create_golden_entry_impl(&store, "financial", golden_body("fin_002")).await.unwrap();
        delete_golden_entry_impl(&store, "financial", "fin_001").await.unwrap();
        delete_scenario_impl(&store, "financial", "fin_new").await.unwrap();

        let scen_mtime_after = fs::metadata(&scen_path).unwrap().modified().unwrap();
        let gold_mtime_after = fs::metadata(&gold_path).unwrap().modified().unwrap();
        assert_eq!(scen_mtime_before, scen_mtime_after, "seed YAML 파일 mtime 변경 금지");
        assert_eq!(gold_mtime_before, gold_mtime_after, "seed JSON 파일 mtime 변경 금지");
    }

    /// @trace TC: SPEC-019/TC-14
    /// @trace FR: PRD-019/FR-5
    #[tokio::test]
    async fn tc_14_bad_request_on_invalid_input() {
        let store = make_store().await;
        // 빈 name
        let mut body = upsert_body("fin_x", "t");
        body.name = "".into();
        let err = create_scenario_impl(&store, "financial", body).await.unwrap_err();
        assert!(matches!(err, CrudFailure::BadRequest(_)));

        // 잘못된 id 문자
        let mut body = upsert_body("bad id!", "t");
        body.name = "ok".into();
        let err = create_scenario_impl(&store, "financial", body).await.unwrap_err();
        assert!(matches!(err, CrudFailure::BadRequest(_)));
        let (status, kind) = err.status_and_kind();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(kind, "bad_request");

        // 빈 task_description
        let mut body = upsert_body("fin_y", "");
        body.task_description = "   ".into();
        let err = create_scenario_impl(&store, "financial", body).await.unwrap_err();
        assert!(matches!(err, CrudFailure::BadRequest(_)));
    }

    // suppress unused import warning when tests compile without them
    #[allow(dead_code)]
    fn _unused_anchor(_t: SystemTime) {}
}
