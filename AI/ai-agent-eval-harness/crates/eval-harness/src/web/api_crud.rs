// =============================================================================
// @trace SPEC-019
// @trace PRD: PRD-019
// @trace FR: PRD-019/FR-1, PRD-019/FR-2, PRD-019/FR-3, PRD-019/FR-4,
// PRD-019/FR-5, PRD-019/FR-7
// @trace file-type: impl
// =============================================================================

use super::AppState;
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
    /// SPEC-020: 기대 도메인(auto-routing 평가용). optional.
    #[serde(default)]
    pub expected_domain: Option<String>,
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
            expected_domain: body.expected_domain,
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
            expected_domain: body.expected_domain,
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
    state
        .store
        .clone()
        .ok_or_else(|| CrudFailure::Internal("SqliteStore not installed in AppState".into()))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1
pub async fn create_scenario(
    State(st): State<AppState>,
    AxPath(domain): AxPath<String>,
    JsonExt(body): JsonExt<EvalScenarioUpsert>,
) -> Result<(StatusCode, JsonOut<ScenarioConfig>), CrudFailure> {
    let store = store_from(&st)?;
    let scen = create_scenario_impl(store.as_ref(), &domain, body).await?;
    Ok((StatusCode::CREATED, JsonOut(scen)))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-1
pub async fn update_scenario_handler(
    State(st): State<AppState>,
    AxPath((domain, id)): AxPath<(String, String)>,
    JsonExt(body): JsonExt<EvalScenarioUpsert>,
) -> Result<JsonOut<ScenarioConfig>, CrudFailure> {
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
pub async fn create_golden_entry(
    State(st): State<AppState>,
    AxPath(domain): AxPath<String>,
    JsonExt(body): JsonExt<GoldenEntryUpsert>,
) -> Result<(StatusCode, JsonOut<GoldenSetEntry>), CrudFailure> {
    let store = store_from(&st)?;
    let entry = create_golden_entry_impl(store.as_ref(), &domain, body).await?;
    Ok((StatusCode::CREATED, JsonOut(entry)))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2
pub async fn update_golden_entry_handler(
    State(st): State<AppState>,
    AxPath((domain, scenario_id)): AxPath<(String, String)>,
    JsonExt(body): JsonExt<GoldenEntryUpsert>,
) -> Result<JsonOut<GoldenSetEntry>, CrudFailure> {
    let store = store_from(&st)?;
    let entry = update_golden_entry_impl(store.as_ref(), &domain, &scenario_id, body).await?;
    Ok(JsonOut(entry))
}

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-2
pub async fn delete_golden_entry_handler(
    State(st): State<AppState>,
    AxPath((domain, scenario_id)): AxPath<(String, String)>,
) -> Result<StatusCode, CrudFailure> {
    let store = store_from(&st)?;
    delete_golden_entry_impl(store.as_ref(), &domain, &scenario_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// SPEC-022: 도메인 CRUD
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct DomainUpsert {
    /// POST 시에만 사용. PUT 은 path param 으로 식별.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tool_class_names: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DomainSummaryDto {
    pub name: String,
    pub description: String,
    pub tool_class_names: Vec<String>,
    pub keywords: Vec<String>,
    pub scenario_count: i64,
    pub is_bootstrap: bool,
}

impl DomainSummaryDto {
    fn from_summary(s: data_scenarios::sqlite_store::DomainSummary) -> Self {
        let is_bootstrap = domains::known_domains().iter().any(|k| *k == s.name);
        Self {
            name: s.name,
            description: s.description,
            tool_class_names: s.tool_class_names,
            keywords: s.keywords,
            scenario_count: s.scenario_count,
            is_bootstrap,
        }
    }
}

/// @trace SPEC: SPEC-022
/// @trace FR: PRD-022/FR-1
pub async fn list_domains_impl(store: &SqliteStore) -> Result<Vec<DomainSummaryDto>, CrudFailure> {
    let summaries = store.list_domain_summaries().await?;
    Ok(summaries.into_iter().map(DomainSummaryDto::from_summary).collect())
}

/// @trace SPEC: SPEC-022
/// @trace FR: PRD-022/FR-1
pub async fn get_domain_impl(store: &SqliteStore, name: &str) -> Result<DomainSummaryDto, CrudFailure> {
    validate_id("domain", name)?;
    let summary = store
        .get_domain_summary(name)
        .await?
        .ok_or_else(|| CrudFailure::NotFound(format!("domain ({name})")))?;
    Ok(DomainSummaryDto::from_summary(summary))
}

/// 신규 도메인 + 도구·키워드 일괄 등록. 라우터 캐시 invalidate.
///
/// @trace SPEC: SPEC-022
/// @trace FR: PRD-022/FR-1, PRD-022/FR-3
pub async fn create_domain_impl(store: &SqliteStore, body: DomainUpsert) -> Result<DomainSummaryDto, CrudFailure> {
    let name = body.name.as_deref().ok_or_else(|| CrudFailure::BadRequest("name required".into()))?;
    validate_id("domain", name)?;
    store.insert_domain(name, &body.description).await?;
    if !body.tool_class_names.is_empty() {
        store.replace_domain_tools(name, &body.tool_class_names).await?;
    }
    if !body.keywords.is_empty() {
        store.replace_domain_keywords(name, &body.keywords).await?;
    }
    agent_core::domain_router::invalidate_cache();
    let summary = store
        .get_domain_summary(name)
        .await?
        .ok_or_else(|| CrudFailure::Internal("just created but not found".into()))?;
    Ok(DomainSummaryDto::from_summary(summary))
}

/// 도메인 갱신 (description + tools + keywords). 부트스트랩 도메인도 갱신
/// 가능(name 만 보호).
///
/// @trace SPEC: SPEC-022
/// @trace FR: PRD-022/FR-1
pub async fn update_domain_impl(store: &SqliteStore, name: &str, body: DomainUpsert) -> Result<DomainSummaryDto, CrudFailure> {
    validate_id("domain", name)?;
    store.update_domain(name, &body.description).await?;
    store.replace_domain_tools(name, &body.tool_class_names).await?;
    store.replace_domain_keywords(name, &body.keywords).await?;
    agent_core::domain_router::invalidate_cache();
    let summary = store
        .get_domain_summary(name)
        .await?
        .ok_or_else(|| CrudFailure::NotFound(format!("domain ({name})")))?;
    Ok(DomainSummaryDto::from_summary(summary))
}

/// 도메인 삭제. 부트스트랩 도메인은 409 Conflict.
///
/// @trace SPEC: SPEC-022
/// @trace FR: PRD-022/FR-1, PRD-022/FR-5
pub async fn delete_domain_impl(store: &SqliteStore, name: &str) -> Result<(), CrudFailure> {
    validate_id("domain", name)?;
    if domains::known_domains().iter().any(|k| *k == name) {
        return Err(CrudFailure::Conflict(format!("bootstrap domain '{name}' cannot be deleted")));
    }
    store.delete_domain(name).await?;
    agent_core::domain_router::invalidate_cache();
    Ok(())
}

// ----- axum handlers -----

pub async fn list_domains(State(st): State<AppState>) -> Result<JsonOut<Vec<DomainSummaryDto>>, CrudFailure> {
    let store = store_from(&st)?;
    let dtos = list_domains_impl(store.as_ref()).await?;
    Ok(JsonOut(dtos))
}

pub async fn get_domain(State(st): State<AppState>, AxPath(name): AxPath<String>) -> Result<JsonOut<DomainSummaryDto>, CrudFailure> {
    let store = store_from(&st)?;
    let dto = get_domain_impl(store.as_ref(), &name).await?;
    Ok(JsonOut(dto))
}

pub async fn create_domain(State(st): State<AppState>, JsonExt(body): JsonExt<DomainUpsert>) -> Result<(StatusCode, JsonOut<DomainSummaryDto>), CrudFailure> {
    let store = store_from(&st)?;
    let dto = create_domain_impl(store.as_ref(), body).await?;
    Ok((StatusCode::CREATED, JsonOut(dto)))
}

pub async fn update_domain_handler(
    State(st): State<AppState>,
    AxPath(name): AxPath<String>,
    JsonExt(body): JsonExt<DomainUpsert>,
) -> Result<JsonOut<DomainSummaryDto>, CrudFailure> {
    let store = store_from(&st)?;
    let dto = update_domain_impl(store.as_ref(), &name, body).await?;
    Ok(JsonOut(dto))
}

pub async fn delete_domain_handler(State(st): State<AppState>, AxPath(name): AxPath<String>) -> Result<StatusCode, CrudFailure> {
    let store = store_from(&st)?;
    delete_domain_impl(store.as_ref(), &name).await?;
    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// SPEC-023: external_tools CRUD + URL allowlist
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct ExternalToolUpsert {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_method")]
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers_json: Option<String>,
    pub body_template: String,
    #[serde(default = "default_schema")]
    pub params_schema: String,
    #[serde(default = "default_timeout")]
    pub timeout_ms: i64,
}

fn default_method() -> String { "POST".into() }
fn default_schema() -> String { "{}".into() }
fn default_timeout() -> i64 { 10000 }

#[derive(Debug, Serialize)]
pub struct ExternalToolDto {
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

impl ExternalToolDto {
    fn from_row(r: data_scenarios::sqlite_store::ExternalToolRow) -> Self {
        Self {
            name: r.name,
            domain: r.domain,
            description: r.description,
            method: r.method,
            url: r.url,
            headers_json: r.headers_json,
            body_template: r.body_template,
            params_schema: r.params_schema,
            timeout_ms: r.timeout_ms,
        }
    }
}

/// `EVAL_HARNESS_HTTP_TOOL_ALLOWLIST` 환경변수가 설정되어 있으면 prefix
/// 매칭으로 URL 을 검증한다. 미설정 또는 빈 값이면 모든 URL 허용.
///
/// @trace SPEC: SPEC-023
/// @trace FR: PRD-023/FR-5
fn url_allowed(url: &str) -> bool {
    let Ok(allow) = std::env::var("EVAL_HARNESS_HTTP_TOOL_ALLOWLIST") else {
        return true;
    };
    if allow.trim().is_empty() {
        return true;
    }
    allow.split(',').map(str::trim).any(|prefix| !prefix.is_empty() && url.starts_with(prefix))
}

/// @trace SPEC: SPEC-023
/// @trace FR: PRD-023/FR-1, PRD-023/FR-4
pub async fn list_external_tools_impl(store: &SqliteStore) -> Result<Vec<ExternalToolDto>, CrudFailure> {
    let rows = store.list_external_tools().await?;
    Ok(rows.into_iter().map(ExternalToolDto::from_row).collect())
}

pub async fn list_external_tools_by_domain_impl(store: &SqliteStore, domain: &str) -> Result<Vec<ExternalToolDto>, CrudFailure> {
    validate_id("domain", domain)?;
    let rows = store.list_external_tools_by_domain(domain).await?;
    Ok(rows.into_iter().map(ExternalToolDto::from_row).collect())
}

pub async fn upsert_external_tool_impl(store: &SqliteStore, domain: &str, body: ExternalToolUpsert) -> Result<ExternalToolDto, CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("name", &body.name)?;
    validate_non_empty("url", &body.url)?;
    validate_non_empty("body_template", &body.body_template)?;
    if !url_allowed(&body.url) {
        return Err(CrudFailure::BadRequest(format!("url '{}' not in EVAL_HARNESS_HTTP_TOOL_ALLOWLIST", body.url)));
    }
    if body.timeout_ms < 0 {
        return Err(CrudFailure::BadRequest("timeout_ms must be >= 0".into()));
    }
    let row = data_scenarios::sqlite_store::ExternalToolRow {
        name: body.name,
        domain: domain.to_string(),
        description: body.description,
        method: body.method,
        url: body.url,
        headers_json: body.headers_json,
        body_template: body.body_template,
        params_schema: body.params_schema,
        timeout_ms: body.timeout_ms,
    };
    store.upsert_external_tool(&row).await?;
    Ok(ExternalToolDto::from_row(row))
}

pub async fn delete_external_tool_impl(store: &SqliteStore, domain: &str, name: &str) -> Result<(), CrudFailure> {
    validate_id("domain", domain)?;
    validate_id("name", name)?;
    store.delete_external_tool(domain, name).await?;
    Ok(())
}

// ----- axum handlers -----

pub async fn list_external_tools(State(st): State<AppState>) -> Result<JsonOut<Vec<ExternalToolDto>>, CrudFailure> {
    let store = store_from(&st)?;
    let dtos = list_external_tools_impl(store.as_ref()).await?;
    Ok(JsonOut(dtos))
}

pub async fn list_external_tools_by_domain(State(st): State<AppState>, AxPath(domain): AxPath<String>) -> Result<JsonOut<Vec<ExternalToolDto>>, CrudFailure> {
    let store = store_from(&st)?;
    let dtos = list_external_tools_by_domain_impl(store.as_ref(), &domain).await?;
    Ok(JsonOut(dtos))
}

pub async fn create_external_tool(
    State(st): State<AppState>,
    AxPath(domain): AxPath<String>,
    JsonExt(body): JsonExt<ExternalToolUpsert>,
) -> Result<(StatusCode, JsonOut<ExternalToolDto>), CrudFailure> {
    let store = store_from(&st)?;
    let dto = upsert_external_tool_impl(store.as_ref(), &domain, body).await?;
    Ok((StatusCode::CREATED, JsonOut(dto)))
}

pub async fn update_external_tool_handler(
    State(st): State<AppState>,
    AxPath((domain, name)): AxPath<(String, String)>,
    JsonExt(mut body): JsonExt<ExternalToolUpsert>,
) -> Result<JsonOut<ExternalToolDto>, CrudFailure> {
    body.name = name; // path 우선
    let store = store_from(&st)?;
    let dto = upsert_external_tool_impl(store.as_ref(), &domain, body).await?;
    Ok(JsonOut(dto))
}

pub async fn delete_external_tool_handler(State(st): State<AppState>, AxPath((domain, name)): AxPath<(String, String)>) -> Result<StatusCode, CrudFailure> {
    let store = store_from(&st)?;
    delete_external_tool_impl(store.as_ref(), &domain, &name).await?;
    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// SPEC-025: PromptSet REST CRUD
//
// @trace SPEC: SPEC-025
// @trace FR: PRD-025/FR-5, PRD-025/FR-6, PRD-025/FR-7
// =============================================================================

/// PromptSet 응답 DTO. SqliteStore 의 row 를 그대로 노출.
#[derive(Debug, Serialize)]
pub struct PromptSetDto {
    pub id:              i64,
    pub domain_name:     String,
    pub version:         i64,
    pub perceive_system: String,
    pub perceive_user:   String,
    pub policy_system:   String,
    pub policy_user:     String,
    pub notes:           Option<String>,
    pub is_active:       bool,
    pub is_bootstrap:    bool,
    pub created_at:      String,
}

impl From<data_scenarios::sqlite_store::PromptSetRow> for PromptSetDto {
    fn from(r: data_scenarios::sqlite_store::PromptSetRow) -> Self {
        Self {
            id:              r.id,
            domain_name:     r.domain_name,
            version:         r.version,
            perceive_system: r.perceive_system,
            perceive_user:   r.perceive_user,
            policy_system:   r.policy_system,
            policy_user:     r.policy_user,
            notes:           r.notes,
            is_active:       r.is_active,
            is_bootstrap:    r.is_bootstrap,
            created_at:      r.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PromptSetCreatePayload {
    pub perceive_system: String,
    pub perceive_user:   String,
    pub policy_system:   String,
    pub policy_user:     String,
    #[serde(default)]
    pub notes:           Option<String>,
}

/// 새 버전 생성 시 필수 슬롯 검증. 누락된 슬롯이 있으면
/// `BadRequest` 에 어느 필드의 어느 슬롯이 빠졌는지 포함.
///
/// @trace SPEC: SPEC-025
/// @trace FR: PRD-025/FR-5
fn validate_prompt_set_payload(p: &PromptSetCreatePayload) -> Result<(), CrudFailure> {
    let mut missing: Vec<String> = Vec::new();
    for (field, tmpl) in [
        ("perceive_system", &p.perceive_system),
        ("perceive_user", &p.perceive_user),
        ("policy_system", &p.policy_system),
        ("policy_user", &p.policy_user),
    ] {
        let m = agent_core::llm_client::validate_required_slots(field, tmpl);
        for slot in m {
            missing.push(format!("{field}:{slot}"));
        }
    }
    if !missing.is_empty() {
        return Err(CrudFailure::BadRequest(format!("missing slots: {}", missing.join(", "))));
    }
    Ok(())
}

pub async fn list_prompt_sets_impl(store: &SqliteStore, domain: &str) -> Result<Vec<PromptSetDto>, CrudFailure> {
    validate_id("domain", domain)?;
    let rows = store.list_prompt_sets(domain).await?;
    Ok(rows.into_iter().map(PromptSetDto::from).collect())
}

pub async fn get_prompt_set_impl(store: &SqliteStore, domain: &str, version: i64) -> Result<PromptSetDto, CrudFailure> {
    validate_id("domain", domain)?;
    let row = store
        .get_prompt_set(domain, version)
        .await?
        .ok_or_else(|| CrudFailure::NotFound(format!("prompt_set {domain}/v{version}")))?;
    Ok(PromptSetDto::from(row))
}

pub async fn create_prompt_set_impl(
    store: &SqliteStore,
    domain: &str,
    payload: PromptSetCreatePayload,
) -> Result<PromptSetDto, CrudFailure> {
    validate_id("domain", domain)?;
    validate_prompt_set_payload(&payload)?;
    let row = store
        .insert_prompt_set(data_scenarios::sqlite_store::PromptSetInsert {
            domain_name:     domain,
            perceive_system: &payload.perceive_system,
            perceive_user:   &payload.perceive_user,
            policy_system:   &payload.policy_system,
            policy_user:     &payload.policy_user,
            notes:           payload.notes.as_deref(),
            is_bootstrap:    false,
        })
        .await?;
    Ok(PromptSetDto::from(row))
}

pub async fn activate_prompt_set_impl(store: &SqliteStore, domain: &str, version: i64) -> Result<PromptSetDto, CrudFailure> {
    validate_id("domain", domain)?;
    store.activate_prompt_set(domain, version).await?;
    let row = store
        .get_prompt_set(domain, version)
        .await?
        .ok_or_else(|| CrudFailure::NotFound(format!("prompt_set {domain}/v{version}")))?;
    Ok(PromptSetDto::from(row))
}

pub async fn delete_prompt_set_impl(store: &SqliteStore, domain: &str, version: i64) -> Result<(), CrudFailure> {
    validate_id("domain", domain)?;
    store.delete_prompt_set(domain, version).await?;
    Ok(())
}

// ----- axum handlers -----

pub async fn list_prompt_sets_handler(State(st): State<AppState>, AxPath(domain): AxPath<String>) -> Result<JsonOut<Vec<PromptSetDto>>, CrudFailure> {
    let store = store_from(&st)?;
    let dtos = list_prompt_sets_impl(store.as_ref(), &domain).await?;
    Ok(JsonOut(dtos))
}

pub async fn get_prompt_set_handler(
    State(st): State<AppState>,
    AxPath((domain, version)): AxPath<(String, i64)>,
) -> Result<JsonOut<PromptSetDto>, CrudFailure> {
    let store = store_from(&st)?;
    let dto = get_prompt_set_impl(store.as_ref(), &domain, version).await?;
    Ok(JsonOut(dto))
}

pub async fn create_prompt_set_handler(
    State(st): State<AppState>,
    AxPath(domain): AxPath<String>,
    JsonExt(payload): JsonExt<PromptSetCreatePayload>,
) -> Result<(StatusCode, JsonOut<PromptSetDto>), CrudFailure> {
    let store = store_from(&st)?;
    let dto = create_prompt_set_impl(store.as_ref(), &domain, payload).await?;
    Ok((StatusCode::CREATED, JsonOut(dto)))
}

pub async fn activate_prompt_set_handler(
    State(st): State<AppState>,
    AxPath((domain, version)): AxPath<(String, i64)>,
) -> Result<JsonOut<PromptSetDto>, CrudFailure> {
    let store = store_from(&st)?;
    let dto = activate_prompt_set_impl(store.as_ref(), &domain, version).await?;
    Ok(JsonOut(dto))
}

pub async fn delete_prompt_set_handler(
    State(st): State<AppState>,
    AxPath((domain, version)): AxPath<(String, i64)>,
) -> Result<StatusCode, CrudFailure> {
    let store = store_from(&st)?;
    delete_prompt_set_impl(store.as_ref(), &domain, version).await?;
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
            expected_domain: None,
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
        let updated = update_scenario_impl(&store, "financial", "fin_001", upsert_body("fin_001", "새 task"))
            .await
            .expect("update ok");
        assert_eq!(updated.task_description, "새 task");

        let err = update_scenario_impl(&store, "financial", "no_such", upsert_body("no_such", "x"))
            .await
            .unwrap_err();
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
        update_scenario_impl(&store, "financial", "fin_001", upsert_body("fin_001", "변경"))
            .await
            .unwrap();
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

    // -------- SPEC-023 --------

    /// @trace TC: SPEC-023/TC-7
    /// @trace FR: PRD-023/FR-5
    #[test]
    fn spec023_tc_7_url_allowlist_blocks_unmatched() {
        // SAFETY: 단위 테스트는 단일 스레드 가정. set_var 후 즉시 검증.
        unsafe {
            std::env::set_var("EVAL_HARNESS_HTTP_TOOL_ALLOWLIST", "https://api.allowed.com/,http://localhost:");
        }
        assert!(url_allowed("https://api.allowed.com/v1/q"));
        assert!(url_allowed("http://localhost:9000/x"));
        assert!(!url_allowed("https://evil.example.com/"));
        unsafe {
            std::env::remove_var("EVAL_HARNESS_HTTP_TOOL_ALLOWLIST");
        }
    }

    /// @trace TC: SPEC-023/TC-8
    /// @trace FR: PRD-023/FR-5
    #[test]
    fn spec023_tc_8_url_allowlist_unset_allows_all() {
        unsafe {
            std::env::remove_var("EVAL_HARNESS_HTTP_TOOL_ALLOWLIST");
        }
        assert!(url_allowed("http://anything"));
        assert!(url_allowed("https://anything"));
    }

    // -------- SPEC-025 --------

    fn valid_payload(label: &str) -> PromptSetCreatePayload {
        PromptSetCreatePayload {
            perceive_system: format!("PER-SYS {label}"),
            perceive_user:   "작업: {task_description}\n환경: {environment_state}{context}".into(),
            policy_system:   format!("POL-SYS {label}"),
            policy_user:     "작업: {task_description}\n인지: {perceived_info}\n도구: {tools}{context}".into(),
            notes:           Some("test".into()),
        }
    }

    async fn store_with_domain(name: &str) -> SqliteStore {
        let store = SqliteStore::open_in_memory_for_loader().await.unwrap();
        store.insert_domain(name, "").await.unwrap();
        // bootstrap v1 시드 (CRUD 가 v2 부터 만들도록)
        store
            .seed_bootstrap_prompt_sets(&data_scenarios::sqlite_store::BootstrapBundleRef {
                perceive_system: agent_core::llm_client::BOOTSTRAP_PERCEIVE_SYSTEM,
                perceive_user:   agent_core::llm_client::BOOTSTRAP_PERCEIVE_USER,
                policy_system:   agent_core::llm_client::BOOTSTRAP_POLICY_SYSTEM,
                policy_user:     agent_core::llm_client::BOOTSTRAP_POLICY_USER,
            })
            .await
            .unwrap();
        store
    }

    /// @trace TC: SPEC-025/TC-6
    /// @trace FR: PRD-025/FR-1, PRD-025/FR-6
    #[tokio::test]
    async fn spec025_tc_6_create_returns_v2_after_bootstrap() {
        let store = store_with_domain("customer_service").await;
        let dto = create_prompt_set_impl(&store, "customer_service", valid_payload("v2")).await.expect("create ok");
        assert_eq!(dto.version, 2);
        assert!(!dto.is_active, "새 버전은 비활성으로 생성");
        assert!(!dto.is_bootstrap);
        let list = list_prompt_sets_impl(&store, "customer_service").await.unwrap();
        // version DESC: [v2, v1]
        assert_eq!(list.iter().map(|d| d.version).collect::<Vec<_>>(), vec![2, 1]);
    }

    /// @trace TC: SPEC-025/TC-5
    /// @trace FR: PRD-025/FR-5
    #[tokio::test]
    async fn spec025_tc_5_handler_rejects_missing_slots() {
        let store = store_with_domain("customer_service").await;
        let mut bad = valid_payload("bad");
        // policy_user 에서 {tools} 제거
        bad.policy_user = "작업: {task_description}\n인지: {perceived_info}\n도구 없음".into();
        let err = create_prompt_set_impl(&store, "customer_service", bad).await.unwrap_err();
        let (status, kind) = err.status_and_kind();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(kind, "bad_request");
        assert!(err.to_string().contains("{tools}"));
        assert!(err.to_string().contains("policy_user"));
    }

    /// @trace TC: SPEC-025/TC-7
    /// @trace FR: PRD-025/FR-3, PRD-025/FR-7
    #[tokio::test]
    async fn spec025_tc_7_activate_handler_toggles() {
        let store = store_with_domain("customer_service").await;
        let v2 = create_prompt_set_impl(&store, "customer_service", valid_payload("v2")).await.unwrap();
        assert!(!v2.is_active);
        let activated = activate_prompt_set_impl(&store, "customer_service", v2.version).await.unwrap();
        assert!(activated.is_active);
        assert_eq!(activated.version, 2);
        // v1 은 비활성
        let v1 = get_prompt_set_impl(&store, "customer_service", 1).await.unwrap();
        assert!(!v1.is_active);
    }

    /// @trace TC: SPEC-025/TC-8
    /// @trace FR: PRD-025/FR-7
    #[tokio::test]
    async fn spec025_tc_8_delete_active_returns_409() {
        let store = store_with_domain("customer_service").await;
        // v1 (bootstrap, active) 을 그대로 두고 v2 만들고 활성화 후 삭제 시도
        let v2 = create_prompt_set_impl(&store, "customer_service", valid_payload("v2")).await.unwrap();
        activate_prompt_set_impl(&store, "customer_service", v2.version).await.unwrap();
        let err = delete_prompt_set_impl(&store, "customer_service", v2.version).await.unwrap_err();
        let (status, _) = err.status_and_kind();
        assert_eq!(status, StatusCode::CONFLICT);
        assert!(err.to_string().to_lowercase().contains("active"));
    }

    /// @trace TC: SPEC-025/TC-9
    /// @trace FR: PRD-025/FR-7
    #[tokio::test]
    async fn spec025_tc_9_delete_bootstrap_returns_409() {
        let store = store_with_domain("customer_service").await;
        // bootstrap v1 을 비활성 상태로 만든 뒤 (v2 활성화 후) 삭제 시도
        let v2 = create_prompt_set_impl(&store, "customer_service", valid_payload("v2")).await.unwrap();
        activate_prompt_set_impl(&store, "customer_service", v2.version).await.unwrap();
        let err = delete_prompt_set_impl(&store, "customer_service", 1).await.unwrap_err();
        let (status, _) = err.status_and_kind();
        assert_eq!(status, StatusCode::CONFLICT);
        assert!(err.to_string().to_lowercase().contains("bootstrap"));
    }

    #[tokio::test]
    async fn spec025_get_unknown_version_404() {
        let store = store_with_domain("customer_service").await;
        let err = get_prompt_set_impl(&store, "customer_service", 99).await.unwrap_err();
        let (status, _) = err.status_and_kind();
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    /// @trace TC: SPEC-025/TC-17
    /// @trace FR: PRD-025/FR-2
    /// 회귀 계약: bootstrap 시드가 현재 하드코딩 상수와 "바이트 동등" 이어야
    /// 한다. 이 조건이 유지되는 한, DB 해석 경로(SPEC-025 이후)와 컴파일
    /// 상수 폴백 경로는 동일한 LLM 메시지를 생성하므로 기존 시나리오
    /// 실행 결과가 변하지 않는다.
    #[tokio::test]
    async fn spec025_tc_17_bootstrap_byte_equivalence_with_constants() {
        let store = SqliteStore::open_in_memory_for_loader().await.unwrap();
        for d in ["customer_service", "financial", "general"] {
            store.insert_domain(d, "").await.unwrap();
        }
        store
            .seed_bootstrap_prompt_sets(&data_scenarios::sqlite_store::BootstrapBundleRef {
                perceive_system: agent_core::llm_client::BOOTSTRAP_PERCEIVE_SYSTEM,
                perceive_user:   agent_core::llm_client::BOOTSTRAP_PERCEIVE_USER,
                policy_system:   agent_core::llm_client::BOOTSTRAP_POLICY_SYSTEM,
                policy_user:     agent_core::llm_client::BOOTSTRAP_POLICY_USER,
            })
            .await
            .unwrap();
        for d in ["customer_service", "financial", "general"] {
            let row = store.get_active_prompt_set(d).await.unwrap().expect("active exists");
            assert_eq!(row.version, 1);
            assert!(row.is_bootstrap);
            assert_eq!(row.perceive_system, agent_core::llm_client::BOOTSTRAP_PERCEIVE_SYSTEM);
            assert_eq!(row.perceive_user, agent_core::llm_client::BOOTSTRAP_PERCEIVE_USER);
            assert_eq!(row.policy_system, agent_core::llm_client::BOOTSTRAP_POLICY_SYSTEM);
            assert_eq!(row.policy_user, agent_core::llm_client::BOOTSTRAP_POLICY_USER);
        }
    }
}
