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
                         EvaluationReport},
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
        let eval_config = agent_core::config::EvaluationConfig::default();
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
    domains::register_customer_service_tools(&mut reg);
    domains::register_financial_tools(&mut reg);
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
pub fn run_suite_impl(suite: &str, agent_name: &str, scenarios_dir: &Path, reports_dir: &Path) -> Result<EvaluationReport, String> {
    run_suite_with_save_impl(suite, agent_name, scenarios_dir, reports_dir, None).map(|(r, _)| r)
}

/// SPEC-005: 실행 후 aggregate report를 디스크에 저장한다.
/// `output` 지정 시 `reports_dir/<output>`, None이면 기본 파일명.
///
/// @trace SPEC: SPEC-005
/// @trace TC: SPEC-005/TC-1, SPEC-005/TC-2, SPEC-005/TC-3
/// @trace FR: PRD-005/FR-1
pub fn run_suite_with_save_impl(
    suite: &str,
    agent_name: &str,
    scenarios_dir: &Path,
    reports_dir: &Path,
    output: Option<&str>,
) -> Result<(EvaluationReport, String), String> {
    if !is_safe_name(suite) || !is_safe_name(agent_name) {
        return Err("invalid suite/agent name".into());
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
    let report = runner.run_suite(suite, agent.as_ref(), scenarios_str).map_err(|e| e.to_string())?;
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
/// @trace SPEC: SPEC-005
/// @trace TC: SPEC-005/TC-4, SPEC-005/TC-5, SPEC-005/TC-6
/// @trace FR: PRD-005/FR-2
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
    let base_path: PathBuf = reports_dir.join(baseline);
    let cur_path: PathBuf = reports_dir.join(current);
    let comparator = ReportComparator::new(threshold);
    let result = comparator
        .compare_files(base_path.to_str().ok_or("invalid path")?, cur_path.to_str().ok_or("invalid path")?)
        .map_err(|e| e.to_string())?;
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

pub async fn list_golden_sets(State(st): State<AppState>) -> Json<Vec<GoldenSetFile>> { Json(list_golden_sets_impl(&st.golden_sets_dir)) }

pub async fn scenario_detail(State(st): State<AppState>, AxPath((domain, id)): AxPath<(String, String)>) -> Result<Json<ScenarioConfig>, StatusCode> {
    match scenario_detail_impl(&st.scenarios_dir, &domain, &id) {
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
    pub suite: String,
    pub agent: String,
    #[serde(default)]
    pub output: Option<String>,
}

pub async fn run_suite(State(st): State<AppState>, Json(req): Json<RunRequest>) -> Result<Json<RunResponse>, (StatusCode, String)> {
    let scen = st.scenarios_dir.clone();
    let reps = st.reports_dir.clone();
    let res = tokio::task::spawn_blocking(move || run_suite_with_save_impl(&req.suite, &req.agent, &scen, &reps, req.output.as_deref()))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
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
pub async fn list_all(State(st): State<AppState>) -> Json<ListAllResponse> { Json(list_all_impl(&st.scenarios_dir)) }

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
    use std::{fs::{self,
                   File},
              io::Write};
    use tempfile::tempdir;

    fn workspace_scenarios() -> PathBuf {
        // tests 디렉토리는 워크스페이스 루트에서 실행된다
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("eval_data/scenarios")
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
    /// @trace scenario: golden-sets 로드
    #[test]
    fn test_tc_3_load_golden_sets() {
        let dir = tempdir().unwrap();
        let json = r#"{"domain":"demo","golden_sets":[]}"#;
        File::create(dir.path().join("demo.json")).unwrap().write_all(json.as_bytes()).unwrap();
        let out = list_golden_sets_impl(dir.path());
        assert_eq!(out.len(), 1);
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
    /// @trace scenario: run_suite_impl 정상
    #[test]
    fn test_tc_6_run_suite_passthrough() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let r = run_suite_impl("customer_service", "passthrough", &scen, reps.path());
        assert!(r.is_ok(), "run failed: {:?}", r.err());
        let report = r.unwrap();
        assert_eq!(report.suite, "customer_service");
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
        let r = run_suite_impl("customer_service", "unknown_agent_xyz", &scen, reps.path());
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
        let report = run_suite_impl("customer_service", "passthrough", &scen, reps.path()).unwrap();
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
    /// @trace scenario: run_suite_with_save_impl 기본 저장
    #[test]
    fn test_spec005_tc_1_run_with_default_save() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let (_report, path) = run_suite_with_save_impl("customer_service", "passthrough", &scen, reps.path(), None).unwrap();
        assert!(std::path::Path::new(&path).exists());
        assert!(path.contains("evaluation_report_"));
    }

    /// @trace TC: SPEC-005/TC-2
    /// @trace FR: PRD-005/FR-1
    /// @trace scenario: run_suite_with_save_impl output 지정
    #[test]
    fn test_spec005_tc_2_run_with_custom_output() {
        let scen = workspace_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let (_r, path) = run_suite_with_save_impl("customer_service", "passthrough", &scen, reps.path(), Some("custom_name.json")).unwrap();
        assert!(path.ends_with("custom_name.json"));
        assert!(reps.path().join("custom_name.json").exists());
    }

    /// @trace TC: SPEC-005/TC-3
    /// @trace FR: PRD-005/FR-1
    /// @trace scenario: run output 경로 이탈 거부
    #[test]
    fn test_spec005_tc_3_run_rejects_traversal_output() {
        let reps = tempdir().unwrap();
        let r = run_suite_with_save_impl("customer_service", "passthrough", Path::new("/nonexistent"), reps.path(), Some("../evil.json"));
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
        let report = run_suite_impl("customer_service", "passthrough", &scen, reps.path()).unwrap();
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
        let report = run_suite_impl("customer_service", "passthrough", &scen, reps.path()).unwrap();
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
