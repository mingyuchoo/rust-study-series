// =============================================================================
// @trace SPEC-021
// @trace PRD: PRD-021
// @trace FR: PRD-021/FR-6
// @trace file-type: impl
// =============================================================================
//
// `compare` 명령의 DB 기반 입력 모드. 두 task_id 또는 두 시간 범위(평균)를
// 받아 SqliteStore 에서 평가 결과를 조회하고 ReportComparator 에 넘긴다.

use data_scenarios::sqlite_store::EvaluationWindow;
use execution::{comparator::ReportComparator,
                models::{ComparisonResult,
                         EvaluationReport}};
use std::collections::HashMap;

fn run<F: std::future::Future>(fut: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        | Ok(handle) => tokio::task::block_in_place(|| handle.block_on(fut)),
        | Err(_) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
            .block_on(fut),
    }
}

/// 두 task_id 평가를 비교한다. 각 task_id 는 단일 EvaluationReport 로 변환된다.
///
/// @trace SPEC: SPEC-021
/// @trace FR: PRD-021/FR-6
pub fn compare_two_tasks(baseline_task: &str, current_task: &str, threshold: f64) -> anyhow::Result<ComparisonResult> {
    let store = data_scenarios::loader::try_installed_store().ok_or_else(|| anyhow::anyhow!("SqliteStore not installed"))?;
    let baseline_json = run({
        let store = store.clone();
        let id = baseline_task.to_string();
        async move { store.get_evaluation_json(&id).await }
    })?
    .ok_or_else(|| anyhow::anyhow!("baseline task not found: {}", baseline_task))?;
    let current_json = run({
        let store = store.clone();
        let id = current_task.to_string();
        async move { store.get_evaluation_json(&id).await }
    })?
    .ok_or_else(|| anyhow::anyhow!("current task not found: {}", current_task))?;

    let baseline_report = task_json_to_report(baseline_task, &baseline_json);
    let current_report = task_json_to_report(current_task, &current_json);
    Ok(ReportComparator::new(threshold).compare(&baseline_report, &current_report))
}

/// agent + 시간 범위 두 개를 비교한다. 각 윈도우의 평균 메트릭이 EvaluationReport 로 변환된다.
///
/// @trace SPEC: SPEC-021
/// @trace FR: PRD-021/FR-6
pub fn compare_windows(agent: &str, baseline_since: &str, baseline_until: &str, current_since: &str, current_until: &str, threshold: f64) -> anyhow::Result<ComparisonResult> {
    let store = data_scenarios::loader::try_installed_store().ok_or_else(|| anyhow::anyhow!("SqliteStore not installed"))?;
    let baseline_window = run({
        let store = store.clone();
        let agent = agent.to_string();
        let s = baseline_since.to_string();
        let u = baseline_until.to_string();
        async move { store.evaluation_window_average(&agent, &s, &u).await }
    })?;
    let current_window = run({
        let store = store.clone();
        let agent = agent.to_string();
        let s = current_since.to_string();
        let u = current_until.to_string();
        async move { store.evaluation_window_average(&agent, &s, &u).await }
    })?;

    let baseline = window_to_report(agent, &format!("{}~{}", baseline_since, baseline_until), &baseline_window);
    let current = window_to_report(agent, &format!("{}~{}", current_since, current_until), &current_window);
    Ok(ReportComparator::new(threshold).compare(&baseline, &current))
}

/// 단일 평가 task JSON 을 얇은 EvaluationReport 로 변환. 메트릭 1행짜리.
fn task_json_to_report(task_id: &str, json: &serde_json::Value) -> EvaluationReport {
    let metrics_obj = json.get("metrics").cloned().unwrap_or(serde_json::json!({}));
    let mut average_metrics: HashMap<String, f64> = HashMap::new();
    if let Some(map) = metrics_obj.as_object() {
        for (k, v) in map {
            if let Some(n) = v.as_f64() {
                average_metrics.insert(k.clone(), n);
            }
        }
    }
    // 점수 필드도 메트릭으로 추가
    if let Some(scores) = json.get("scores").and_then(|v| v.as_object()) {
        for (k, v) in scores {
            if let Some(n) = v.as_f64() {
                average_metrics.insert(k.clone(), n);
            }
        }
    }
    let success = json
        .get("trajectory")
        .and_then(|t| t.get("success"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    EvaluationReport {
        version: "1.0".into(),
        timestamp: chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string(),
        agent_name: "ppa".into(),
        eval_scenario: format!("task:{task_id}"),
        total_scenarios: 1,
        success_count: if success { 1 } else { 0 },
        success_rate: if success { 1.0 } else { 0.0 },
        average_metrics,
        scenarios: Vec::new(),
    }
}

/// 시간 범위 평균을 EvaluationReport 로 변환.
fn window_to_report(agent: &str, label: &str, w: &EvaluationWindow) -> EvaluationReport {
    let mut average_metrics: HashMap<String, f64> = HashMap::new();
    if let Some(v) = w.criteria_score {
        average_metrics.insert("criteria_score".into(), v);
    }
    if let Some(v) = w.tool_sequence_score {
        average_metrics.insert("tool_sequence_score".into(), v);
    }
    if let Some(v) = w.domain_routing_score {
        average_metrics.insert("domain_routing_score".into(), v);
    }
    if let Some(v) = w.overall_score {
        average_metrics.insert("overall_score".into(), v);
    }
    let total = w.count.max(0) as usize;
    let success_count = w.successes.max(0) as usize;
    EvaluationReport {
        version: "1.0".into(),
        timestamp: chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string(),
        agent_name: agent.into(),
        eval_scenario: format!("window:{label}"),
        total_scenarios: total,
        success_count,
        success_rate: if total > 0 { success_count as f64 / total as f64 } else { 0.0 },
        average_metrics,
        scenarios: Vec::new(),
    }
}

