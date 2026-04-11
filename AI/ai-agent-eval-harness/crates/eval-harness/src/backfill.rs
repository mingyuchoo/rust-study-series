// =============================================================================
// @trace SPEC-021
// @trace PRD: PRD-021
// @trace FR: PRD-021/FR-7
// @trace file-type: impl
// =============================================================================
//
// 디렉토리에 쌓여 있는 trajectory_*.json / evaluation_*.json 을 SQLite 로
// 일회성 import. 부분 실패는 stderr 로 경고만 하고 계속 진행한다.

use agent_models::models::{EvaluationResult,
                           Trajectory};
use std::path::Path;

pub struct BackfillReport {
    pub trajectories_imported: usize,
    pub trajectories_failed: usize,
    pub evaluations_imported: usize,
    pub evaluations_failed: usize,
}

/// trajectories_dir 의 `trajectory_*.json` 과 logs_dir 의 `evaluation_*.json`
/// 을 모두 읽어 DB 에 INSERT OR REPLACE. 전역 store 가 install 안 된 경우
/// 에러 메세지와 함께 종료(아무 것도 import 안 함).
///
/// @trace SPEC: SPEC-021
/// @trace FR: PRD-021/FR-7
pub fn backfill_results(trajectories_dir: &Path, logs_dir: &Path) -> BackfillReport {
    let mut report = BackfillReport {
        trajectories_imported: 0,
        trajectories_failed: 0,
        evaluations_imported: 0,
        evaluations_failed: 0,
    };
    let Some(store) = data_scenarios::loader::try_installed_store() else {
        eprintln!("[backfill] SqliteStore 가 install 되지 않았습니다. ScenarioLoader::install 호출 후 재시도하세요.");
        return report;
    };

    // ---- trajectories ----
    if let Ok(entries) = std::fs::read_dir(trajectories_dir) {
        for e in entries.flatten() {
            let p = e.path();
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else { continue };
            if !name.starts_with("trajectory_") || !name.ends_with(".json") {
                continue;
            }
            match parse_and_upsert_trajectory(&p, &store) {
                | Ok(_) => report.trajectories_imported += 1,
                | Err(e) => {
                    eprintln!("[backfill] trajectory 실패 {}: {}", name, e);
                    report.trajectories_failed += 1;
                },
            }
        }
    }

    // ---- evaluations ----
    if let Ok(entries) = std::fs::read_dir(logs_dir) {
        for e in entries.flatten() {
            let p = e.path();
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else { continue };
            if !name.starts_with("evaluation_") || !name.ends_with(".json") || name.starts_with("evaluation_report_") {
                continue;
            }
            match parse_and_upsert_evaluation(&p, &store) {
                | Ok(_) => report.evaluations_imported += 1,
                | Err(e) => {
                    eprintln!("[backfill] evaluation 실패 {}: {}", name, e);
                    report.evaluations_failed += 1;
                },
            }
        }
    }

    report
}

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

fn parse_and_upsert_trajectory(path: &Path, store: &std::sync::Arc<data_scenarios::sqlite_store::SqliteStore>) -> anyhow::Result<()> {
    let text = std::fs::read_to_string(path)?;
    let traj: Trajectory = serde_json::from_str(&text)?;
    let steps_json = serde_json::to_string(&traj.steps)?;
    let final_state_json = traj.final_state.as_ref().map(serde_json::to_string).transpose()?;
    let started_at = traj.start_time.to_rfc3339();
    let ended_at = traj.end_time.map(|t| t.to_rfc3339());
    let store = store.clone();
    let task_id = traj.task_id.clone();
    let task_description = traj.task_description.clone();
    let success = traj.success;
    let total = traj.total_iterations as i64;
    run(async move {
        store
            .upsert_trajectory(
                &task_id,
                &task_description,
                "ppa",
                None,
                None,
                success,
                total,
                &started_at,
                ended_at.as_deref(),
                &steps_json,
                final_state_json.as_deref(),
            )
            .await
    })?;
    Ok(())
}

fn parse_and_upsert_evaluation(path: &Path, store: &std::sync::Arc<data_scenarios::sqlite_store::SqliteStore>) -> anyhow::Result<()> {
    let text = std::fs::read_to_string(path)?;
    let eval: EvaluationResult = serde_json::from_str(&text)?;
    // 평가는 trajectory FK 가 필요하므로, trajectory 를 먼저 upsert.
    let traj = &eval.trajectory;
    let steps_json = serde_json::to_string(&traj.steps)?;
    let final_state_json = traj.final_state.as_ref().map(serde_json::to_string).transpose()?;
    let started_at = traj.start_time.to_rfc3339();
    let ended_at = traj.end_time.map(|t| t.to_rfc3339());
    let metrics_map = eval.metrics.to_map();
    let metrics_json = serde_json::to_string(&metrics_map)?;
    let golden_json = eval.golden_set_result.as_ref().map(serde_json::to_string).transpose()?;
    let (criteria, tool_seq, routing, overall) = match &eval.golden_set_result {
        | Some(g) => (Some(g.criteria_score), Some(g.tool_sequence_score), g.domain_routing_score, Some(g.overall_score)),
        | None => (None, None, None, None),
    };
    let task_id = traj.task_id.clone();
    let task_description = traj.task_description.clone();
    let success = traj.success;
    let total = traj.total_iterations as i64;
    let store = store.clone();
    run(async move {
        store
            .upsert_trajectory(
                &task_id,
                &task_description,
                "ppa",
                None,
                None,
                success,
                total,
                &started_at,
                ended_at.as_deref(),
                &steps_json,
                final_state_json.as_deref(),
            )
            .await?;
        store
            .upsert_evaluation(
                &task_id,
                "ppa",
                None,
                None,
                success,
                criteria,
                tool_seq,
                routing,
                overall,
                &metrics_json,
                golden_json.as_deref(),
            )
            .await?;
        Ok::<(), data_scenarios::sqlite_store::StoreError>(())
    })?;
    Ok(())
}
