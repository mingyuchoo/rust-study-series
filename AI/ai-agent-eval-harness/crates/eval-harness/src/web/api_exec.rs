// =============================================================================
// @trace SPEC-004
// @trace PRD: PRD-004
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5, FR-6, FR-7, FR-8
// @trace file-type: impl
// =============================================================================

use super::{AppState,
            api::build_agent_registry,
            handlers::is_safe_name};
use agent_models::models::{EvaluationResult,
                           Trajectory};
use axum::{extract::{Path as AxPath,
                     State},
           http::StatusCode,
           response::Json};
use data_scenarios::{loader::ScenarioLoader,
                     models::{GoldenSetEntry,
                              Scenario}};
use execution::runner::HarnessRunner;
use execution_fault_injection::{fault_injector::FaultInjector,
                                models::FaultInjectionConfig};
use execution_tools::registry::ToolRegistry;
use scoring::evaluator::TrajectoryEvaluator;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap,
          fs,
          path::{Path,
                 PathBuf}};

// --------------------------------------------------------------------------
// 공유 유틸
// --------------------------------------------------------------------------

/// 모든 도메인 도구가 등록된 ToolRegistry를 만든다.
///
/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-4
/// @trace FR: PRD-004/FR-3
pub fn build_full_tool_registry() -> ToolRegistry {
    let mut reg = ToolRegistry::new();
    domains::register_customer_service_tools(&mut reg);
    domains::register_financial_tools(&mut reg);
    reg
}

// --------------------------------------------------------------------------
// FR-1: 단일 시나리오 실행
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-1, SPEC-004/TC-2
/// @trace FR: PRD-004/FR-1
pub fn run_scenario_impl(scen_dir: &Path, reps_dir: &Path, domain: &str, id: &str, agent_name: &str) -> Result<EvaluationResult, String> {
    if !is_safe_name(id) || !is_safe_name(agent_name) {
        return Err("invalid identifier".into());
    }
    // 해당 도메인의 전체 DomainConfig를 로드하여 에이전트에 도구를 등록한 뒤
    // 실행한다.
    let dir_str = scen_dir.to_str().ok_or("invalid scenarios_dir")?;
    let configs = ScenarioLoader::new()
        .load_all_domains(dir_str)
        .map_err(|e| format!("failed to load domain configs: {}", e))?;
    let domain_cfg = configs
        .into_iter()
        .find(|c| c.name == domain)
        .ok_or_else(|| format!("domain not found: {}", domain))?;

    let scenario_cfg = domain_cfg
        .scenarios
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| format!("scenario not found: {}", id))?
        .clone();
    let scenario = Scenario {
        id: scenario_cfg.id,
        name: scenario_cfg.name,
        description: scenario_cfg.description,
        task_description: scenario_cfg.task_description,
        initial_environment: scenario_cfg.initial_environment,
        expected_tools: scenario_cfg.expected_tools,
        success_criteria: scenario_cfg.success_criteria,
        difficulty: scenario_cfg.difficulty,
        domain: domain.to_string(),
    };

    let registry = build_agent_registry();
    let agent = registry.get_agent(agent_name).ok_or_else(|| format!("unknown agent: {}", agent_name))?;
    agent.load_domain_tools(&domain_cfg);

    let reps_str = reps_dir.to_str().ok_or("invalid reports_dir")?;
    let runner = HarnessRunner::new(reps_str);
    Ok(runner.run_scenario(&scenario, agent.as_ref()))
}

// --------------------------------------------------------------------------
// FR-2: 에이전트 직접 호출
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-3, SPEC-004/TC-12, SPEC-004/TC-13
/// @trace FR: PRD-004/FR-2, PRD-004/FR-8
pub fn agent_execute_impl(
    scen_dir: &Path,
    agent_name: &str,
    task: &str,
    env: Option<HashMap<String, Value>>,
    domain: Option<&str>,
) -> Result<Trajectory, String> {
    if !is_safe_name(agent_name) {
        return Err("invalid agent name".into());
    }
    let registry = build_agent_registry();
    let agent = registry.get_agent(agent_name).ok_or_else(|| format!("unknown agent: {}", agent_name))?;

    if let Some(d) = domain {
        if !is_safe_name(d) {
            return Err("invalid domain name".into());
        }
        let dir_str = scen_dir.to_str().ok_or("invalid scenarios_dir")?;
        let configs = ScenarioLoader::new()
            .load_all_domains(dir_str)
            .map_err(|e| format!("failed to load domain configs: {}", e))?;
        let cfg = configs.into_iter().find(|c| c.name == d).ok_or_else(|| format!("domain not found: {}", d))?;
        agent.load_domain_tools(&cfg);
    }

    Ok(agent.execute_task(task, env))
}

// --------------------------------------------------------------------------
// FR-3: 도구 단일 호출
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-4, SPEC-004/TC-5
/// @trace FR: PRD-004/FR-3
pub fn tool_invoke_impl(name: &str, params: &HashMap<String, Value>) -> Result<HashMap<String, Value>, String> {
    if !is_safe_name(name) {
        return Err("invalid tool name".into());
    }
    let reg = build_full_tool_registry();
    let tool = reg.get_tool(name).ok_or_else(|| format!("unknown tool: {}", name))?;
    Ok(tool.execute(params))
}

// --------------------------------------------------------------------------
// FR-4: 골든셋 엔트리 조회
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-6, SPEC-004/TC-7
/// @trace FR: PRD-004/FR-4
pub fn golden_entry_impl(dir: &Path, domain: &str, sid: &str) -> Option<GoldenSetEntry> {
    if !is_safe_name(domain) || !is_safe_name(sid) {
        return None;
    }
    let dir_str = dir.to_str()?;
    let files = ScenarioLoader::new().load_all_golden_sets(dir_str).ok()?;
    for f in files {
        if f.domain == domain {
            if let Some(entry) = f.get_by_scenario_id(sid) {
                return Some(entry.clone());
            }
        }
    }
    None
}

// --------------------------------------------------------------------------
// FR-5: 궤적 채점
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-8
/// @trace FR: PRD-004/FR-5
pub fn score_impl(trajectory: Trajectory) -> EvaluationResult { TrajectoryEvaluator::new().evaluate(&trajectory, None, None) }

// --------------------------------------------------------------------------
// FR-6: 폴트 주입 단일 도구 실행
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-9
/// @trace FR: PRD-004/FR-6
pub fn fault_sim_impl(name: &str, params: &HashMap<String, Value>, config: FaultInjectionConfig) -> Result<HashMap<String, Value>, String> {
    if !is_safe_name(name) {
        return Err("invalid tool name".into());
    }
    let base = build_full_tool_registry();
    // FaultInjector::wrap_registry는 enabled=false일 때 빈 레지스트리를 반환하므로
    // 폴백으로 원본 레지스트리를 사용한다.
    let wrapped = FaultInjector::new(config.clone()).wrap_registry(&base);
    let effective = if config.enabled { &wrapped } else { &base };
    let tool = effective.get_tool(name).ok_or_else(|| format!("unknown tool: {}", name))?;
    Ok(tool.execute(params))
}

// --------------------------------------------------------------------------
// FR-7: 궤적 파일 조회
// --------------------------------------------------------------------------

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-10
/// @trace FR: PRD-004/FR-7
pub fn list_trajectories_impl(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|x| x.to_str()) == Some("json") {
            if let Some(n) = p.file_name().and_then(|n| n.to_str()) {
                out.push(n.to_string());
            }
        }
    }
    out.sort();
    out
}

/// @trace SPEC: SPEC-004
/// @trace TC: SPEC-004/TC-11
/// @trace FR: PRD-004/FR-7
pub fn get_trajectory_impl(dir: &Path, name: &str) -> Option<Value> {
    if !is_safe_name(name) {
        return None;
    }
    let path: PathBuf = dir.join(name);
    let text = fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

// --------------------------------------------------------------------------
// Axum handler wrappers
// --------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct RunScenarioReq {
    pub agent: String,
}

pub async fn run_scenario(
    State(st): State<AppState>,
    AxPath((domain, id)): AxPath<(String, String)>,
    Json(req): Json<RunScenarioReq>,
) -> Result<Json<EvaluationResult>, (StatusCode, String)> {
    let scen = st.scenarios_dir.clone();
    let reps = st.reports_dir.clone();
    println!("▶ [web] POST /api/scenarios/{}/{}/run agent={}", domain, id, req.agent);
    let label = format!("{}/{}", domain, id);
    let agent_label = req.agent.clone();
    let res = tokio::task::spawn_blocking(move || run_scenario_impl(&scen, &reps, &domain, &id, &req.agent))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match &res {
        | Ok(_) => println!("✔ [web] 시나리오 완료: {} (agent={})", label, agent_label),
        | Err(e) => println!("✘ [web] 시나리오 실패: {} (agent={}) — {}", label, agent_label, e),
    }
    res.map(Json).map_err(|e| (StatusCode::NOT_FOUND, e))
}

#[derive(Deserialize)]
pub struct AgentExecReq {
    pub task: String,
    #[serde(default)]
    pub environment: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub domain: Option<String>,
}

pub async fn agent_execute(
    State(st): State<AppState>,
    AxPath(name): AxPath<String>,
    Json(req): Json<AgentExecReq>,
) -> Result<Json<Trajectory>, (StatusCode, String)> {
    let scen = st.scenarios_dir.clone();
    let agent_label = name.clone();
    let domain_label = req.domain.clone().unwrap_or_else(|| "(none)".into());
    println!("▶ [web] POST /api/agents/{}/execute domain={}", agent_label, domain_label);
    let res = tokio::task::spawn_blocking(move || agent_execute_impl(&scen, &name, &req.task, req.environment, req.domain.as_deref()))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match &res {
        | Ok(_) => println!("✔ [web] 에이전트 실행 완료: agent={} domain={}", agent_label, domain_label),
        | Err(e) => println!("✘ [web] 에이전트 실행 실패: agent={} domain={} — {}", agent_label, domain_label, e),
    }
    res.map(Json).map_err(|e| (StatusCode::NOT_FOUND, e))
}

#[derive(Deserialize)]
pub struct ToolInvokeReq {
    #[serde(default)]
    pub params: HashMap<String, Value>,
}

pub async fn tool_invoke(AxPath(name): AxPath<String>, Json(req): Json<ToolInvokeReq>) -> Result<Json<HashMap<String, Value>>, (StatusCode, String)> {
    let res = tokio::task::spawn_blocking(move || tool_invoke_impl(&name, &req.params))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    res.map(Json).map_err(|e| (StatusCode::NOT_FOUND, e))
}

pub async fn golden_entry(State(st): State<AppState>, AxPath((domain, sid)): AxPath<(String, String)>) -> Result<Json<GoldenSetEntry>, StatusCode> {
    match golden_entry_impl(&st.golden_sets_dir, &domain, &sid) {
        | Some(e) => Ok(Json(e)),
        | None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Deserialize)]
pub struct ScoreReq {
    pub trajectory: Trajectory,
}

pub async fn score(Json(req): Json<ScoreReq>) -> Result<Json<EvaluationResult>, (StatusCode, String)> {
    let res = tokio::task::spawn_blocking(move || score_impl(req.trajectory))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(res))
}

#[derive(Deserialize)]
pub struct FaultSimReq {
    #[serde(default)]
    pub params: HashMap<String, Value>,
    #[serde(default)]
    pub config: FaultInjectionConfig,
}

pub async fn fault_sim(AxPath(name): AxPath<String>, Json(req): Json<FaultSimReq>) -> Result<Json<HashMap<String, Value>>, (StatusCode, String)> {
    let res = tokio::task::spawn_blocking(move || fault_sim_impl(&name, &req.params, req.config))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    res.map(Json).map_err(|e| (StatusCode::NOT_FOUND, e))
}

pub async fn list_trajectories(State(st): State<AppState>) -> Json<Vec<String>> { Json(list_trajectories_impl(&st.trajectories_dir)) }

pub async fn get_trajectory(State(st): State<AppState>, AxPath(name): AxPath<String>) -> Result<Json<Value>, StatusCode> {
    match get_trajectory_impl(&st.trajectories_dir, &name) {
        | Some(v) => Ok(Json(v)),
        | None => Err(StatusCode::NOT_FOUND),
    }
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // =============================================================================
    // @trace SPEC-004
    // @trace PRD: PRD-004
    // @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5, FR-6, FR-7, FR-8
    // @trace file-type: test
    // =============================================================================

    use super::*;
    use agent_models::models::Trajectory as TrajModel;
    use chrono::Utc;
    use std::{fs::File,
              io::Write};
    use tempfile::tempdir;

    fn ws_scenarios() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("eval_data/eval_scenarios")
    }

    /// @trace TC: SPEC-004/TC-1
    /// @trace FR: PRD-004/FR-1
    /// @trace scenario: run_scenario_impl 성공
    #[test]
    fn test_tc_1_run_scenario_ok() {
        let scen = ws_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        let res = run_scenario_impl(&scen, reps.path(), "customer_service", "cs_001", "passthrough");
        assert!(res.is_ok(), "{:?}", res.err());
    }

    /// @trace TC: SPEC-004/TC-2
    /// @trace FR: PRD-004/FR-1
    /// @trace scenario: 없는 시나리오
    #[test]
    fn test_tc_2_run_scenario_missing() {
        let scen = ws_scenarios();
        if !scen.exists() {
            return;
        }
        let reps = tempdir().unwrap();
        assert!(run_scenario_impl(&scen, reps.path(), "customer_service", "nope_xxx", "passthrough").is_err());
        assert!(run_scenario_impl(&scen, reps.path(), "customer_service", "cs_001", "unknown").is_err());
    }

    /// @trace TC: SPEC-004/TC-3
    /// @trace FR: PRD-004/FR-2
    /// @trace scenario: agent_execute_impl passthrough
    #[test]
    fn test_tc_3_agent_execute_passthrough() {
        let scen = ws_scenarios();
        let res = agent_execute_impl(&scen, "passthrough", "hello world", None, None);
        assert!(res.is_ok());
        let traj = res.unwrap();
        assert_eq!(traj.task_description, "hello world");
    }

    /// @trace TC: SPEC-004/TC-12
    /// @trace FR: PRD-004/FR-8
    /// @trace scenario: agent_execute_impl + customer_service 도메인으로 도구
    /// 로드
    #[test]
    fn test_tc_12_agent_execute_with_domain_loads_tools() {
        let scen = ws_scenarios();
        if !scen.exists() {
            return;
        }
        let res = agent_execute_impl(&scen, "passthrough", "refund please", None, Some("customer_service"));
        assert!(res.is_ok(), "{:?}", res.err());
        let traj = res.unwrap();
        assert_eq!(traj.task_description, "refund please");
    }

    /// @trace TC: SPEC-004/TC-13
    /// @trace FR: PRD-004/FR-8
    /// @trace scenario: agent_execute_impl + 알 수 없는 도메인 Err
    #[test]
    fn test_tc_13_agent_execute_unknown_domain() {
        let scen = ws_scenarios();
        if !scen.exists() {
            return;
        }
        let res = agent_execute_impl(&scen, "passthrough", "hi", None, Some("bogus_xxx"));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert!(err.contains("domain not found"), "unexpected error: {}", err);
    }

    /// @trace TC: SPEC-004/TC-4
    /// @trace FR: PRD-004/FR-3
    /// @trace scenario: tool_invoke_impl 성공
    #[test]
    fn test_tc_4_tool_invoke_ok() {
        let mut params = HashMap::new();
        params.insert("inquiry_text".into(), serde_json::json!("환불하고 싶어요"));
        params.insert("customer_id".into(), serde_json::json!("C001"));
        let res = tool_invoke_impl("classify_inquiry", &params);
        assert!(res.is_ok(), "{:?}", res.err());
    }

    /// @trace TC: SPEC-004/TC-5
    /// @trace FR: PRD-004/FR-3
    /// @trace scenario: 알 수 없는 도구 거부
    #[test]
    fn test_tc_5_unknown_tool() {
        let res = tool_invoke_impl("nonexistent_tool_xyz", &HashMap::new());
        assert!(res.is_err());
    }

    /// @trace TC: SPEC-004/TC-6
    /// @trace FR: PRD-004/FR-4
    /// @trace scenario: golden entry 조회 성공
    #[test]
    fn test_tc_6_golden_entry_found() {
        let dir = tempdir().unwrap();
        let json = r#"{
            "domain":"demo",
            "golden_sets":[{
                "scenario_id":"s1",
                "input":{"task":"t","environment":{}},
                "expected_output":{"tool_sequence":[],"tool_results":{},"success_criteria":{},"tolerance":0.0},
                "metadata":{}
            }]
        }"#;
        File::create(dir.path().join("demo.json")).unwrap().write_all(json.as_bytes()).unwrap();
        let e = golden_entry_impl(dir.path(), "demo", "s1");
        assert!(e.is_some());
    }

    /// @trace TC: SPEC-004/TC-7
    /// @trace FR: PRD-004/FR-4
    /// @trace scenario: golden entry 404
    #[test]
    fn test_tc_7_golden_entry_missing() {
        let dir = tempdir().unwrap();
        assert!(golden_entry_impl(dir.path(), "nope", "s1").is_none());
        assert!(golden_entry_impl(dir.path(), "../etc", "s1").is_none());
    }

    /// @trace TC: SPEC-004/TC-8
    /// @trace FR: PRD-004/FR-5
    /// @trace scenario: score_impl 빈 궤적
    #[test]
    fn test_tc_8_score_empty_trajectory() {
        let traj = TrajModel {
            task_id: "t1".into(),
            task_description: "test".into(),
            start_time: Utc::now(),
            end_time: Some(Utc::now()),
            steps: vec![],
            final_state: None,
            success: false,
            total_iterations: 0,
        };
        let res = score_impl(traj);
        assert_eq!(res.trajectory.task_id, "t1");
    }

    /// @trace TC: SPEC-004/TC-9
    /// @trace FR: PRD-004/FR-6
    /// @trace scenario: fault sim 결과 반환
    #[test]
    fn test_tc_9_fault_sim_returns() {
        let mut params = HashMap::new();
        params.insert("inquiry_text".into(), serde_json::json!("환불"));
        params.insert("customer_id".into(), serde_json::json!("C1"));
        let mut config = FaultInjectionConfig::default();
        config.enabled = false; // 정상 경로 유지
        let res = fault_sim_impl("classify_inquiry", &params, config);
        assert!(res.is_ok(), "{:?}", res.err());
    }

    /// @trace TC: SPEC-004/TC-10
    /// @trace FR: PRD-004/FR-7
    /// @trace scenario: list_trajectories
    #[test]
    fn test_tc_10_list_trajectories() {
        let dir = tempdir().unwrap();
        for n in ["t1.json", "t2.json", "skip.txt"] {
            File::create(dir.path().join(n)).unwrap().write_all(b"{}").unwrap();
        }
        let out = list_trajectories_impl(dir.path());
        assert_eq!(out, vec!["t1.json", "t2.json"]);
    }

    /// @trace TC: SPEC-004/TC-11
    /// @trace FR: PRD-004/FR-7
    /// @trace scenario: get_trajectory 경로 이탈 거부
    #[test]
    fn test_tc_11_get_trajectory_traversal_rejected() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("ok.json")).unwrap().write_all(b"{\"x\":1}").unwrap();
        assert!(get_trajectory_impl(dir.path(), "../evil.json").is_none());
        assert!(get_trajectory_impl(dir.path(), "ok.json").is_some());
    }
}
