// =============================================================================
// @trace SPEC-003
// @trace PRD: PRD-003
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5, FR-6
// @trace file-type: impl
// =============================================================================

use super::{AppState,
            handlers::{DomainSummary,
                       is_safe_name}};
use agent_models::domain_config::ScenarioConfig;
use axum::{extract::{Path as AxPath,
                     State},
           http::StatusCode,
           response::Json};
use data_scenarios::{loader::ScenarioLoader,
                     models::GoldenSetFile};
use execution::{agent_registry::AgentRegistry,
                base_agent::PassthroughAgent,
                comparator::ReportComparator,
                models::{ComparisonResult,
                         EvaluationReport,
                         ScenarioResult},
                runner::HarnessRunner};
use execution_tools::registry::ToolRegistry;
use serde::{Deserialize,
            Serialize};
use std::{path::{Path,
                 PathBuf},
          sync::Arc};

// --------------------------------------------------------------------------
// Agent registry construction (shared with CLI main)
// --------------------------------------------------------------------------

/// PPA 에이전트 초기화가 실패해도 passthrough만 담은 레지스트리를 반환한다.
///
/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-1
/// @trace FR: PRD-003/FR-1
pub fn build_agent_registry() -> AgentRegistry {
    let mut registry = AgentRegistry::new();
    registry.register("passthrough", Arc::new(PassthroughAgent));
    dotenvy::dotenv().ok();
    if let Ok(llm_config) = agent_core::config::AzureOpenAiConfig::from_env() {
        let base = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let eval_config = match crate::data_paths::load_evaluation_config(&base) {
            | Ok(cfg) => cfg,
            | Err(e) => {
                eprintln!("[warn] eval-harness.toml [evaluation] 파싱 실패: {e} — 기본값 사용");
                agent_core::config::EvaluationConfig::default()
            },
        };
        let llm = agent_core::llm_client::LlmClient::new(llm_config);
        let agent = agent_core::agent::PpaAgent::new(llm, eval_config);
        registry.register("ppa", Arc::new(agent));
    }
    registry
}

// --------------------------------------------------------------------------
// Pure impls (단위 테스트 대상)
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-1
/// @trace FR: PRD-003/FR-1
pub fn list_agents_impl(reg: &AgentRegistry) -> Vec<String> {
    let mut names = reg.get_agent_names();
    names.sort();
    names
}

/// 모든 도메인 도구를 등록한 ToolRegistry에서 메타데이터를 반환한다.
///
/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-2
/// @trace FR: PRD-003/FR-2
pub fn list_tools_impl() -> Vec<serde_json::Value> {
    let mut reg = ToolRegistry::new();
    domains::register_all(&mut reg);
    reg.get_tools_metadata()
}

/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-3
/// @trace FR: PRD-003/FR-3
pub fn list_golden_sets_impl(dir: &Path) -> Vec<GoldenSetFile> {
    let Some(s) = dir.to_str() else {
        return Vec::new();
    };
    ScenarioLoader::new().load_all_golden_sets(s).unwrap_or_default()
}

/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-4, SPEC-003/TC-5
/// @trace FR: PRD-003/FR-4
pub fn scenario_detail_impl(dir: &Path, domain: &str, id: &str) -> Option<ScenarioConfig> {
    if !is_safe_name(domain) || !is_safe_name(id) {
        return None;
    }
    let dir_str = dir.to_str()?;
    let configs = ScenarioLoader::new().load_all_domains(dir_str).ok()?;
    let cfg = configs.into_iter().find(|c| c.name == domain)?;
    cfg.scenarios.into_iter().find(|s| s.id == id)
}

/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-6, SPEC-003/TC-7
/// @trace FR: PRD-003/FR-5
#[cfg(test)]
pub fn run_eval_scenario_impl(eval_scenario: &str, agent_name: &str, scenarios_dir: &Path, reports_dir: &Path) -> Result<EvaluationReport, String> {
    run_eval_scenario_with_save_impl(eval_scenario, agent_name, scenarios_dir, reports_dir, None).map(|(r, _)| r)
}

/// SPEC-005: 실행 후 aggregate report를 디스크에 저장한다.
/// `output` 지정 시 `reports_dir/<output>`, None이면 기본 파일명.
///
/// @trace SPEC: SPEC-005
/// @trace TC: SPEC-005/TC-1, SPEC-005/TC-2, SPEC-005/TC-3
/// @trace FR: PRD-005/FR-1
pub fn run_eval_scenario_with_save_impl(
    eval_scenario: &str,
    agent_name: &str,
    scenarios_dir: &Path,
    reports_dir: &Path,
    output: Option<&str>,
) -> Result<(EvaluationReport, String), String> {
    if !is_safe_name(eval_scenario) || !is_safe_name(agent_name) {
        return Err("invalid eval_scenario/agent name".into());
    }
    if let Some(name) = output {
        if !is_safe_name(name) {
            return Err("invalid output name".into());
        }
    }
    let registry = build_agent_registry();
    let agent = registry.get_agent(agent_name).ok_or_else(|| format!("unknown agent: {}", agent_name))?;
    let reports_str = reports_dir.to_str().ok_or_else(|| "invalid reports_dir".to_string())?;
    let scenarios_str = scenarios_dir.to_str().ok_or_else(|| "invalid scenarios_dir".to_string())?;
    let mut runner = HarnessRunner::new(reports_str);
    let report = runner
        .run_eval_scenario(eval_scenario, agent.as_ref(), scenarios_str)
        .map_err(|e| e.to_string())?;
    let save_path: PathBuf = match output {
        | Some(n) => reports_dir.join(n),
        | None => reports_dir.join(format!("evaluation_report_{}.json", report.timestamp)),
    };
    let save_str = save_path.to_str().ok_or_else(|| "invalid save path".to_string())?.to_string();
    runner.save_report(&report, Some(save_str.as_str())).map_err(|e| e.to_string())?;
    Ok((report, save_str))
}

/// @trace SPEC: SPEC-003
/// @trace TC: SPEC-003/TC-8, SPEC-003/TC-9
/// @trace FR: PRD-003/FR-6
#[cfg(test)]
pub fn compare_impl(baseline: &str, current: &str, threshold: f64, reports_dir: &Path) -> Result<ComparisonResult, String> {
    compare_with_save_impl(baseline, current, threshold, reports_dir, None).map(|(r, _)| r)
}

/// SPEC-005: 비교 결과를 선택적으로 디스크에 저장한다.
///
/// SPEC-021 이후 평가 로그(`evaluation_<task_id>_*.json`)는 DB 에만 존재할
/// 수 있다. 따라서 디스크에 파일이 없으면 `db_query::get_evaluation` 으로
/// 폴백하여 `EvaluationReport` 를 복원한 뒤 in-memory 비교를 수행한다.
///
/// @trace SPEC: SPEC-005, SPEC-021
/// @trace TC: SPEC-005/TC-4, SPEC-005/TC-5, SPEC-005/TC-6
/// @trace FR: PRD-005/FR-2, PRD-021/FR-4
pub fn compare_with_save_impl(
    baseline: &str,
    current: &str,
    threshold: f64,
    reports_dir: &Path,
    output: Option<&str>,
) -> Result<(ComparisonResult, Option<String>), String> {
    if !is_safe_name(baseline) || !is_safe_name(current) {
        return Err("invalid report name".into());
    }
    if let Some(name) = output {
        if !is_safe_name(name) {
            return Err("invalid output name".into());
        }
    }
    let base_report = load_report_by_name(reports_dir, baseline)?;
    let cur_report = load_report_by_name(reports_dir, current)?;
    let comparator = ReportComparator::new(threshold);
    let result = comparator.compare(&base_report, &cur_report);
    let saved_to = if let Some(name) = output {
        let p: PathBuf = reports_dir.join(name);
        let s = p.to_str().ok_or("invalid save path")?.to_string();
        comparator.save_comparison(&result, &s).map_err(|e| e.to_string())?;
        Some(s)
    } else {
        None
    };
    Ok((result, saved_to))
}

/// 이름(파일명 형식)으로 `EvaluationReport` 를 로드한다.
/// 1) `reports_dir/<name>` 파일을 먼저 시도하고,
/// 2) 실패하면 파일명에서 task_id 를 추출해 DB 행을 1-시나리오
///    `EvaluationReport` 로 합성한다. DB 스키마(`trajectory/metrics/scores`)
///    와 집계 리포트 스키마(`timestamp/average_metrics/scenarios`)는 다르므로
///    필드 매핑이 필요하다.
///
/// @trace SPEC: SPEC-021
/// @trace FR: PRD-021/FR-4
fn load_report_by_name(reports_dir: &Path, name: &str) -> Result<EvaluationReport, String> {
    let path = reports_dir.join(name);
    if let Ok(s) = std::fs::read_to_string(&path) {
        return serde_json::from_str::<EvaluationReport>(&s).map_err(|e| format!("parse {name}: {e}"));
    }
    let task_id = super::db_query::parse_task_id_from_filename(name).ok_or_else(|| format!("report not found: {name}"))?;
    let v = super::db_query::get_evaluation(&task_id).ok_or_else(|| format!("report not found in DB: {name}"))?;
    db_row_to_report(name, &v)
}

/// DB `get_evaluation_json` 의 JSON (trajectory/metrics/scores/golden_set_result)
/// 을 1-시나리오 `EvaluationReport` 로 매핑한다. 비교기는 `average_metrics`
/// 만 보므로 scores + metrics 의 숫자 값을 모두 편입한다.
fn db_row_to_report(name: &str, v: &serde_json::Value) -> Result<EvaluationReport, String> {
    use std::collections::HashMap;
    let traj = v.get("trajectory").ok_or_else(|| format!("DB row missing 'trajectory': {name}"))?;
    let task_id = traj.get("task_id").and_then(|x| x.as_str()).unwrap_or("").to_string();
    let task_description = traj.get("task_description").and_then(|x| x.as_str()).unwrap_or("").to_string();
    let agent_name = traj.get("agent_name").and_then(|x| x.as_str()).unwrap_or("ppa").to_string();
    let success = traj.get("success").and_then(|x| x.as_bool()).unwrap_or(false);
    let total_iterations = traj.get("total_iterations").and_then(|x| x.as_u64()).unwrap_or(0) as u32;
    // trajectory 페이로드에는 domain/scenario_id 가 없을 수 있음 → 기본값.
    let domain = traj.get("domain").and_then(|x| x.as_str()).unwrap_or("general").to_string();
    let scenario_id = traj.get("scenario_id").and_then(|x| x.as_str()).unwrap_or("").to_string();

    let mut average_metrics: HashMap<String, f64> = HashMap::new();
    if let Some(obj) = v.get("scores").and_then(|x| x.as_object()) {
        for (k, val) in obj {
            if let Some(f) = val.as_f64() {
                average_metrics.insert(k.clone(), f);
            }
        }
    }
    let mut scen_metrics: HashMap<String, Option<f64>> = HashMap::new();
    if let Some(obj) = v.get("metrics").and_then(|x| x.as_object()) {
        for (k, val) in obj {
            let f = val.as_f64();
            if let Some(x) = f {
                average_metrics.entry(k.clone()).or_insert(x);
            }
            scen_metrics.insert(k.clone(), f);
        }
    }

    let timestamp = timestamp_from_filename(name).unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

    Ok(EvaluationReport {
        version: "1.0".into(),
        timestamp,
        agent_name,
        eval_scenario: "all".into(),
        total_scenarios: 1,
        success_count: if success { 1 } else { 0 },
        success_rate: if success { 1.0 } else { 0.0 },
        average_metrics,
        scenarios: vec![ScenarioResult {
            task_id,
            task_description,
            success,
            total_iterations,
            metrics: scen_metrics,
            domain,
            scenario_id,
        }],
    })
}

/// `<prefix>_<uuid>_YYYYMMDD_HHMMSS.json` 에서 timestamp 를 RFC3339 로 복원.
fn timestamp_from_filename(name: &str) -> Option<String> {
    let stripped = name.strip_suffix(".json")?;
    let (rest, hhmmss) = stripped.rsplit_once('_')?;
    let (_, yyyymmdd) = rest.rsplit_once('_')?;
    if yyyymmdd.len() != 8 || hhmmss.len() != 6 {
        return None;
    }
    if !yyyymmdd.chars().all(|c| c.is_ascii_digit()) || !hhmmss.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(format!(
        "{}-{}-{}T{}:{}:{}Z",
        &yyyymmdd[.. 4],
        &yyyymmdd[4 .. 6],
        &yyyymmdd[6 .. 8],
        &hhmmss[.. 2],
        &hhmmss[2 .. 4],
        &hhmmss[4 .. 6]
    ))
}

/// SPEC-005: 도메인/시나리오와 에이전트 목록을 단일 페이로드로 집계.
///
/// @trace SPEC: SPEC-005
/// @trace TC: SPEC-005/TC-7
/// @trace FR: PRD-005/FR-3
pub fn list_all_impl(scen_dir: &Path) -> ListAllResponse {
    let reg = build_agent_registry();
    ListAllResponse {
        domains: super::handlers::list_scenarios_impl(scen_dir),
        agents: list_agents_impl(&reg),
    }
}

// --------------------------------------------------------------------------
// Axum handlers (얇은 래퍼)
// --------------------------------------------------------------------------

pub async fn list_agents() -> Json<Vec<String>> { Json(list_agents_impl(&build_agent_registry())) }

pub async fn list_tools() -> Json<Vec<serde_json::Value>> { Json(list_tools_impl()) }

/// @trace SPEC: SPEC-019
/// @trace FR: PRD-019/FR-7
pub async fn list_golden_sets(State(st): State<AppState>) -> Json<Vec<GoldenSetFile>> {
    let dir = st.golden_sets_dir.clone();
    // loader 는 내부 runtime 에서 block_on 을 사용하므로, axum worker 스레드에서
    // 직접 호출하면 runtime 중첩 패닉이 발생한다. spawn_blocking 으로 격리.
    let res = tokio::task::spawn_blocking(move || list_golden_sets_impl(&dir)).await.unwrap_or_default();
    Json(res)
}

pub async fn scenario_detail(State(st): State<AppState>, AxPath((domain, id)): AxPath<(String, String)>) -> Result<Json<ScenarioConfig>, StatusCode> {
    let scen = st.scenarios_dir.clone();
    let res = tokio::task::spawn_blocking(move || scenario_detail_impl(&scen, &domain, &id))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match res {
        | Some(s) => Ok(Json(s)),
        | None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Serialize)]
pub struct RunResponse {
    pub report: EvaluationReport,
    pub saved_to: String,
}

#[derive(Serialize)]
pub struct CompareResponse {
    pub result: ComparisonResult,
    pub saved_to: Option<String>,
}

#[derive(Serialize)]
pub struct ListAllResponse {
    pub domains: Vec<DomainSummary>,
    pub agents: Vec<String>,
}

#[derive(Deserialize)]
pub struct RunRequest {
    pub eval_scenario: String,
    pub agent: String,
    #[serde(default)]
    pub output: Option<String>,
}

pub async fn run_eval_scenario(State(st): State<AppState>, Json(req): Json<RunRequest>) -> Result<Json<RunResponse>, (StatusCode, String)> {
    let scen = st.scenarios_dir.clone();
    let reps = st.reports_dir.clone();
    let eval_scenario_label = req.eval_scenario.clone();
    let agent_label = req.agent.clone();
    println!("▶ [web] POST /api/run eval_scenario={} agent={}", eval_scenario_label, agent_label);
    let res = tokio::task::spawn_blocking(move || run_eval_scenario_with_save_impl(&req.eval_scenario, &req.agent, &scen, &reps, req.output.as_deref()))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match &res {
        | Ok((_, saved_to)) => println!(
            "✔ [web] 평가 시나리오 완료: eval_scenario={} agent={} → {}",
            eval_scenario_label, agent_label, saved_to
        ),
        | Err(e) => println!(
            "✘ [web] 평가 시나리오 실패: eval_scenario={} agent={} — {}",
            eval_scenario_label, agent_label, e
        ),
    }
    res.map(|(report, saved_to)| {
        Json(RunResponse {
            report,
            saved_to,
        })
    })
    .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

#[derive(Deserialize)]
pub struct CompareRequest {
    pub baseline: String,
    pub current: String,
    #[serde(default = "default_threshold")]
    pub threshold: f64,
    #[serde(default)]
    pub output: Option<String>,
}
fn default_threshold() -> f64 { 5.0 }

pub async fn compare_reports(State(st): State<AppState>, Json(req): Json<CompareRequest>) -> Result<Json<CompareResponse>, (StatusCode, String)> {
    let reps = st.reports_dir.clone();
    let res = tokio::task::spawn_blocking(move || compare_with_save_impl(&req.baseline, &req.current, req.threshold, &reps, req.output.as_deref()))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    res.map(|(result, saved_to)| {
        Json(CompareResponse {
            result,
            saved_to,
        })
    })
    .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

/// @trace SPEC: SPEC-005
/// @trace TC: SPEC-005/TC-7
/// @trace FR: PRD-005/FR-3
pub async fn list_all(State(st): State<AppState>) -> Json<ListAllResponse> {
    let scen = st.scenarios_dir.clone();
    let res = tokio::task::spawn_blocking(move || list_all_impl(&scen))
        .await
        .unwrap_or_else(|_| ListAllResponse {
            domains: Vec::new(),
            agents: Vec::new(),
        });
    Json(res)
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // =============================================================================
    // @trace SPEC-003
    // @trace PRD: PRD-003
    // @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5, FR-6
    // @trace file-type: test
    // =============================================================================

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn workspace_scenarios() -> PathBuf {
        // SPEC-017 이후 loader 는 내장 시드만 사용하므로 경로 값은 무의미하다.
        // 기존 테스트의 `dir.exists()` 조기 반환을 우회하기 위해 항상 존재하는
        // CARGO_MANIFEST_DIR 을 반환한다.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// @trace TC: SPEC-003/TC-1
    /// @trace FR: PRD-003/FR-1
    /// @trace scenario: agents 목록
    #[test]
    fn test_tc_1_list_agents_includes_passthrough() {
        let reg = build_agent_registry();
        let names = list_agents_impl(&reg);
        assert!(names.contains(&"passthrough".to_string()));
    }

    /// @trace TC: SPEC-003/TC-2
    /// @trace FR: PRD-003/FR-2
    /// @trace scenario: tools 메타데이터
    #[test]
    fn test_tc_2_list_tools_not_empty() {
        let tools = list_tools_impl();
        assert!(!tools.is_empty(), "domain tools must register");
    }

    /// @trace TC: SPEC-003/TC-3
    /// @trace FR: PRD-003/FR-3
    /// @trace scenario: golden-sets 로드 (SPEC-019 이후 내장 시드 기반)
    #[test]
    fn test_tc_3_load_golden_sets() {
        let dir = tempdir().unwrap();
        let out = list_golden_sets_impl(dir.path());
        assert!(out.iter().any(|g| g.domain == "financial"), "embedded financial seed must load");
        assert!(out.iter().any(|g| g.domain == "customer_service"), "embedded customer_service seed must load");
    }

    /// @trace TC: SPEC-003/TC-4
    /// @trace FR: PRD-003/FR-4
    /// @trace scenario: scenario detail 성공
    #[test]
    fn test_tc_4_scenario_detail_found() {
        let dir = workspace_scenarios();
        if !dir.exists() {
            return;
        }
        let s = scenario_detail_impl(&dir, "customer_service", "cs_001");
        assert!(s.is_some());
        assert_eq!(s.unwrap().id, "cs_001");
    }

    /// @trace TC: SPEC-003/TC-5
    /// @trace FR: PRD-003/FR-4
    /// @trace scenario: scenario detail 404
    #[test]
    fn test_tc_5_scenario_detail_missing() {
        let dir = workspace_scenarios();
        if !dir.exists() {
            return;
        }
        assert!(scenario_detail_impl(&dir, "customer_service", "nope_xxx").is_none());
        assert!(scenario_detail_impl(&dir, "../etc", "cs_001").is_none());
    }

    /// @trace TC: SPEC-003/TC-6
    /// @trace FR: PRD-003/FR-5
    /// @trace scenario: run_eval_scenario_impl 정상
    #[test]
    fn test_tc_6_run_eval_scenario_passthrough() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let r = run_eval_scenario_impl("customer_service", "passthrough", &scen, reps.path());
        assert!(r.is_ok(), "run failed: {:?}", r.err());
        let report = r.unwrap();
        assert_eq!(report.eval_scenario, "customer_service");
    }

    /// @trace TC: SPEC-003/TC-7
    /// @trace FR: PRD-003/FR-5
    /// @trace scenario: 알 수 없는 에이전트 거부
    #[test]
    fn test_tc_7_unknown_agent_rejected() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let r = run_eval_scenario_impl("customer_service", "unknown_agent_xyz", &scen, reps.path());
        assert!(r.is_err());
    }

    /// @trace TC: SPEC-003/TC-8
    /// @trace FR: PRD-003/FR-6
    /// @trace scenario: compare 동일파일 pass
    #[test]
    fn test_tc_8_compare_identical_passes() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let report = run_eval_scenario_impl("customer_service", "passthrough", &scen, reps.path()).unwrap();
        let text = serde_json::to_string(&report).unwrap();
        fs::write(reps.path().join("a.json"), &text).unwrap();
        fs::write(reps.path().join("b.json"), &text).unwrap();

        let r = compare_impl("a.json", "b.json", 5.0, reps.path()).unwrap();
        assert_eq!(r.verdict, "pass");
    }

    /// @trace TC: SPEC-003/TC-9
    /// @trace FR: PRD-003/FR-6
    /// @trace scenario: compare 경로 이탈 거부
    #[test]
    fn test_tc_9_compare_rejects_traversal() {
        let reps = tempdir().unwrap();
        assert!(compare_impl("../evil.json", "b.json", 5.0, reps.path()).is_err());
        assert!(compare_impl("a.json", "/etc/passwd", 5.0, reps.path()).is_err());
    }

    // --- SPEC-005 tests --------------------------------------------------

    /// @trace TC: SPEC-005/TC-1
    /// @trace FR: PRD-005/FR-1
    /// @trace scenario: run_eval_scenario_with_save_impl 기본 저장
    #[test]
    fn test_spec005_tc_1_run_with_default_save() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let (_report, path) = run_eval_scenario_with_save_impl("customer_service", "passthrough", &scen, reps.path(), None).unwrap();
        assert!(std::path::Path::new(&path).exists());
        assert!(path.contains("evaluation_report_"));
    }

    /// @trace TC: SPEC-005/TC-2
    /// @trace FR: PRD-005/FR-1
    /// @trace scenario: run_eval_scenario_with_save_impl output 지정
    #[test]
    fn test_spec005_tc_2_run_with_custom_output() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let (_r, path) = run_eval_scenario_with_save_impl("customer_service", "passthrough", &scen, reps.path(), Some("custom_name.json")).unwrap();
        assert!(path.ends_with("custom_name.json"));
        assert!(reps.path().join("custom_name.json").exists());
    }

    /// @trace TC: SPEC-005/TC-3
    /// @trace FR: PRD-005/FR-1
    /// @trace scenario: run output 경로 이탈 거부
    #[test]
    fn test_spec005_tc_3_run_rejects_traversal_output() {
        let reps = tempdir().unwrap();
        let r = run_eval_scenario_with_save_impl("customer_service", "passthrough", Path::new("/nonexistent"), reps.path(), Some("../evil.json"));
        assert!(r.is_err());
    }

    /// @trace TC: SPEC-005/TC-4
    /// @trace FR: PRD-005/FR-2
    /// @trace scenario: compare_with_save_impl output 저장
    #[test]
    fn test_spec005_tc_4_compare_with_save() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let report = run_eval_scenario_impl("customer_service", "passthrough", &scen, reps.path()).unwrap();
        let text = serde_json::to_string(&report).unwrap();
        fs::write(reps.path().join("a.json"), &text).unwrap();
        fs::write(reps.path().join("b.json"), &text).unwrap();
        let (_res, saved) = compare_with_save_impl("a.json", "b.json", 5.0, reps.path(), Some("cmp.json")).unwrap();
        assert!(saved.is_some());
        assert!(reps.path().join("cmp.json").exists());
    }

    /// @trace TC: SPEC-005/TC-5
    /// @trace FR: PRD-005/FR-2
    /// @trace scenario: compare 저장 생략
    #[test]
    fn test_spec005_tc_5_compare_without_save() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let report = run_eval_scenario_impl("customer_service", "passthrough", &scen, reps.path()).unwrap();
        let text = serde_json::to_string(&report).unwrap();
        fs::write(reps.path().join("a.json"), &text).unwrap();
        fs::write(reps.path().join("b.json"), &text).unwrap();
        let (_res, saved) = compare_with_save_impl("a.json", "b.json", 5.0, reps.path(), None).unwrap();
        assert!(saved.is_none());
    }

    /// @trace TC: SPEC-005/TC-6
    /// @trace FR: PRD-005/FR-2
    /// @trace scenario: compare output 경로 이탈 거부
    #[test]
    fn test_spec005_tc_6_compare_rejects_traversal_output() {
        let reps = tempdir().unwrap();
        let r = compare_with_save_impl("a.json", "b.json", 5.0, reps.path(), Some("../evil.json"));
        assert!(r.is_err());
    }

    /// @trace TC: SPEC-005/TC-7
    /// @trace FR: PRD-005/FR-3
    /// @trace scenario: list_all_impl 집계
    #[test]
    fn test_spec005_tc_7_list_all() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let out = list_all_impl(&scen);
        assert!(!out.domains.is_empty());
        assert!(out.agents.contains(&"passthrough".to_string()));
    }
}
