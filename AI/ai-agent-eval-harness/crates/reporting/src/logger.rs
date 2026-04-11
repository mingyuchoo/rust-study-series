// =============================================================================
// @trace SPEC-021
// @trace PRD: PRD-021
// @trace FR: PRD-021/FR-1, PRD-021/FR-2, PRD-021/FR-3
// @trace file-type: impl
// =============================================================================
//
// 궤적/평가 결과를 파일 시스템과 SQLite 양쪽에 동시 기록(dual-write)하는 로거.
// 기존 구현체 `TrajectoryLogger` 는 facade 로 남아 있어 호출부 시그니처는
// 변경되지 않는다.

use agent_models::models::{EvaluationResult,
                           PpaStage,
                           Trajectory};
use anyhow::Result;
use colored::*;
use data_scenarios::sqlite_store::SqliteStore;
use std::{path::{Path,
                 PathBuf},
          sync::Arc};

/// 궤적/평가 결과 영속화 계약. 어떤 백엔드(파일/DB/...) 든 이 트레이트로
/// 노출된다.
pub trait TrajectoryLog: Send + Sync {
    fn save_trajectory(&self, trajectory: &Trajectory) -> Result<()>;
    fn save_evaluation(&self, evaluation: &EvaluationResult) -> Result<()>;
}

/// 파일 시스템 백엔드. SPEC-021 이전 동작을 그대로 유지한다.
pub struct FileLogger {
    log_dir: PathBuf,
    trajectory_dir: PathBuf,
}

impl FileLogger {
    pub fn new(log_dir: impl AsRef<Path>, trajectory_dir: impl AsRef<Path>) -> Self {
        let log_dir = log_dir.as_ref().to_path_buf();
        let trajectory_dir = trajectory_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&log_dir).ok();
        std::fs::create_dir_all(&trajectory_dir).ok();
        Self {
            log_dir,
            trajectory_dir,
        }
    }
}

impl TrajectoryLog for FileLogger {
    fn save_trajectory(&self, trajectory: &Trajectory) -> Result<()> {
        let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("trajectory_{}_{}.json", trajectory.task_id, ts);
        let filepath = self.trajectory_dir.join(&filename);
        let json = serde_json::to_string_pretty(trajectory)?;
        std::fs::write(&filepath, json)?;
        println!("{} 궤적 저장: {}", "✓".green(), filepath.display());
        Ok(())
    }

    fn save_evaluation(&self, evaluation: &EvaluationResult) -> Result<()> {
        let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("evaluation_{}_{}.json", evaluation.trajectory.task_id, ts);
        let filepath = self.log_dir.join(&filename);
        let json = serde_json::to_string_pretty(evaluation)?;
        std::fs::write(&filepath, json)?;
        println!("{} 평가 결과 저장: {}", "✓".green(), filepath.display());
        Ok(())
    }
}

/// SQLite 백엔드. 동기 컨텍스트에서 호출 가능하도록 자체 tokio 런타임을
/// 들고 있다(이미 tokio 런타임 안이면 `Handle::current()` 재사용).
pub struct SqliteLogger {
    store: Arc<SqliteStore>,
}

impl SqliteLogger {
    pub fn new(store: Arc<SqliteStore>) -> Self {
        Self {
            store,
        }
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
}

impl TrajectoryLog for SqliteLogger {
    fn save_trajectory(&self, trajectory: &Trajectory) -> Result<()> {
        let steps_json = serde_json::to_string(&trajectory.steps)?;
        let final_state_json = trajectory.final_state.as_ref().map(serde_json::to_string).transpose()?;
        let started_at = trajectory.start_time.to_rfc3339();
        let ended_at = trajectory.end_time.map(|t| t.to_rfc3339());
        let store = self.store.clone();
        let task_id = trajectory.task_id.clone();
        let task_description = trajectory.task_description.clone();
        // 도메인/시나리오 ID 는 final_state 에서 추출 시도(있으면)
        let (domain, scenario_id) = trajectory
            .final_state
            .as_ref()
            .map(|s| {
                let d = s.perceived_info.get("domain").and_then(|v| v.as_str()).map(String::from);
                let sid = s.perceived_info.get("scenario_id").and_then(|v| v.as_str()).map(String::from);
                (d, sid)
            })
            .unwrap_or((None, None));
        let success = trajectory.success;
        let total_iterations = trajectory.total_iterations as i64;
        let prompt_set_id = trajectory.prompt_set_id;
        Self::run(async move {
            store
                .upsert_trajectory(
                    &task_id,
                    &task_description,
                    "ppa", // FR-1: 기본 agent 라벨. 향후 trajectory 모델에 직접 필드 추가 시 교체.
                    domain.as_deref(),
                    scenario_id.as_deref(),
                    success,
                    total_iterations,
                    &started_at,
                    ended_at.as_deref(),
                    &steps_json,
                    final_state_json.as_deref(),
                    prompt_set_id,
                )
                .await
        })?;
        Ok(())
    }

    fn save_evaluation(&self, evaluation: &EvaluationResult) -> Result<()> {
        // trajectory 가 먼저 DB 에 있어야 FK 충족. dual-write 컨텍스트에서는
        // 상위 MultiLogger 가 trajectory 를 먼저 호출해 보장한다.
        let metrics_map = evaluation.metrics.to_map();
        let metrics_json = serde_json::to_string(&metrics_map)?;
        let golden_set_result_json = evaluation.golden_set_result.as_ref().map(serde_json::to_string).transpose()?;
        let task_id = evaluation.trajectory.task_id.clone();
        let success = evaluation.trajectory.success;
        let (domain, scenario_id) = evaluation
            .trajectory
            .final_state
            .as_ref()
            .map(|s| {
                let d = s.perceived_info.get("domain").and_then(|v| v.as_str()).map(String::from);
                let sid = s.perceived_info.get("scenario_id").and_then(|v| v.as_str()).map(String::from);
                (d, sid)
            })
            .unwrap_or((None, None));
        let (criteria_score, tool_sequence_score, domain_routing_score, overall_score) = match &evaluation.golden_set_result {
            | Some(g) => (
                Some(g.criteria_score),
                Some(g.tool_sequence_score),
                g.domain_routing_score,
                Some(g.overall_score),
            ),
            | None => (None, None, None, None),
        };
        let store = self.store.clone();
        Self::run(async move {
            store
                .upsert_evaluation(
                    &task_id,
                    "ppa",
                    domain.as_deref(),
                    scenario_id.as_deref(),
                    success,
                    criteria_score,
                    tool_sequence_score,
                    domain_routing_score,
                    overall_score,
                    &metrics_json,
                    golden_set_result_json.as_deref(),
                )
                .await
        })?;
        Ok(())
    }
}

/// fan-out 로거. 모든 child 에 호출을 전달하고, 개별 실패는 stderr 경고로
/// 변환하여 다른 child 의 진행을 막지 않는다.
pub struct MultiLogger {
    children: Vec<Box<dyn TrajectoryLog>>,
}

impl MultiLogger {
    pub fn new(children: Vec<Box<dyn TrajectoryLog>>) -> Self {
        Self {
            children,
        }
    }
}

impl TrajectoryLog for MultiLogger {
    fn save_trajectory(&self, trajectory: &Trajectory) -> Result<()> {
        for (idx, child) in self.children.iter().enumerate() {
            if let Err(e) = child.save_trajectory(trajectory) {
                eprintln!("[warn] logger #{idx} save_trajectory 실패: {e}");
            }
        }
        Ok(())
    }

    fn save_evaluation(&self, evaluation: &EvaluationResult) -> Result<()> {
        for (idx, child) in self.children.iter().enumerate() {
            if let Err(e) = child.save_evaluation(evaluation) {
                eprintln!("[warn] logger #{idx} save_evaluation 실패: {e}");
            }
        }
        Ok(())
    }
}

// =============================================================================
// 기존 facade — 호출부 호환을 위해 동일한 시그니처 유지
// =============================================================================

/// 기존 코드 호환용 facade. 내부적으로 `MultiLogger { FileLogger, SqliteLogger?
/// }` 를 들고 있어, `HarnessRunner::new` 같은 호출부 변경 없이 dual-write 가
/// 활성화된다. SqliteStore 가 install 되지 않은 환경(테스트 등)에서는
/// 자동으로 FileLogger 단독 모드로 떨어진다.
pub struct TrajectoryLogger {
    inner: Box<dyn TrajectoryLog>,
}

impl TrajectoryLogger {
    pub fn new(log_dir: &str, trajectory_dir: &str) -> Self {
        let file = Box::new(FileLogger::new(log_dir, trajectory_dir));
        let inner: Box<dyn TrajectoryLog> = match try_global_store() {
            | Some(store) => {
                let sqlite = Box::new(SqliteLogger::new(store));
                Box::new(MultiLogger::new(vec![file, sqlite]))
            },
            | None => file,
        };
        Self {
            inner,
        }
    }

    pub fn save_trajectory(&self, trajectory: &Trajectory) -> Result<PathBuf> {
        self.inner.save_trajectory(trajectory)?;
        // 하위호환: 호출자가 PathBuf 를 기대하는 경우가 있어 빈 PathBuf 반환.
        // 새 코드는 trait 직접 사용 권장.
        Ok(PathBuf::new())
    }

    pub fn save_evaluation(&self, evaluation: &EvaluationResult) -> Result<PathBuf> {
        self.inner.save_evaluation(evaluation)?;
        Ok(PathBuf::new())
    }

    #[allow(dead_code)]
    pub fn print_trajectory_summary(&self, trajectory: &Trajectory) {
        let status = if trajectory.success { "✅" } else { "❌" };
        println!("\n{} 궤적 요약: {}", status, trajectory.task_id);
        println!("{:<20} {}", "작업 설명:", trajectory.task_description);
        println!("{:<20} {}", "시작 시간:", trajectory.start_time);
        if let Some(end) = trajectory.end_time {
            let duration = (end - trajectory.start_time).num_milliseconds() as f64 / 1000.0;
            println!("{:<20} {}", "종료 시간:", end);
            println!("{:<20} {:.2}초", "소요 시간:", duration);
        }
        println!("{:<20} {}", "총 반복 횟수:", trajectory.total_iterations);
        println!("{:<20} {}", "총 단계 수:", trajectory.steps.len());

        let perceive = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Perceive).count();
        let policy = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Policy).count();
        let action = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Action).count();
        println!("\nPPA 단계별 통계:");
        println!("  Perceive: {}", perceive);
        println!("  Policy:   {}", policy);
        println!("  Action:   {}", action);
    }
}

/// 전역 ScenarioLoader 에 install 된 SqliteStore 를 빌려온다. install 안
/// 되었으면(예: 단위 테스트) None.
fn try_global_store() -> Option<Arc<SqliteStore>> { data_scenarios::loader::try_installed_store() }

#[cfg(test)]
mod spec025_tests {
    use super::*;
    use agent_models::models::{AgentState, PpaStage, PpaStep, Trajectory};
    use std::collections::HashMap;

    fn v1_bundle() -> data_scenarios::sqlite_store::BootstrapBundleRef<'static> {
        data_scenarios::sqlite_store::BootstrapBundleRef {
            perceive_system: "PSYS",
            perceive_user:   "{task_description} {environment_state}",
            policy_system:   "OSYS",
            policy_user:     "{task_description} {perceived_info} {tools}",
        }
    }

    fn make_trajectory(task_id: &str, domain: &str, prompt_set_id: Option<i64>) -> Trajectory {
        let now = chrono::Utc::now();
        let step = PpaStep {
            stage: PpaStage::Perceive,
            iteration: 1,
            timestamp: now,
            input_data: HashMap::new(),
            output_data: HashMap::new(),
            tool_calls: Vec::new(),
            duration_ms: Some(1.0),
        };
        let mut final_state = AgentState::new("e2e-test".into());
        final_state
            .perceived_info
            .insert("domain".into(), serde_json::Value::String(domain.into()));
        final_state.is_complete = true;
        Trajectory {
            task_id: task_id.into(),
            task_description: "e2e-test".into(),
            start_time: now,
            end_time: Some(now),
            steps: vec![step],
            final_state: Some(final_state),
            success: true,
            total_iterations: 1,
            prompt_set_id,
        }
    }

    /// @trace TC: SPEC-025/TC-14
    /// @trace FR: PRD-025/FR-8
    /// dual-write 경로의 통합 계약:
    ///   SqliteLogger::save_trajectory 가 prompt_set_id 를 실제로 DB 에
    ///   기록하고, `get_trajectory_json` 응답에 동일한 id 가 포함된다.
    /// LLM 을 호출하지 않고 Trajectory 구조체를 직접 주입해 PpaAgent 가
    /// 할 일 (=resolve → Trajectory.prompt_set_id 설정) 을 미리 수행한
    /// 상태에서 저장/복원 round-trip 을 검증한다.
    ///
    /// 주의: `SqliteLogger::save_trajectory` 는 동기 API 로 `block_in_place +
    /// Handle::block_on` 을 사용하므로 반드시 `multi_thread` 런타임에서 실행.
    /// in-memory sqlite (`:memory:`) 는 max_connections=1 이라 동일 커넥션
    /// 재진입 시 풀 timeout 발생 → 파일 기반 tempfile 로 전환.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn spec025_logger_round_trips_prompt_set_id() {
        let tmp = tempfile::TempDir::new().unwrap();
        let db_path = tmp.path().join("e2e.db");
        let store = Arc::new(SqliteStore::open(&db_path).await.unwrap());
        store.insert_domain("customer_service", "").await.unwrap();
        store.seed_bootstrap_prompt_sets(&v1_bundle()).await.unwrap();
        let active = store.get_active_prompt_set("customer_service").await.unwrap().expect("bootstrap");
        let task_id = "e2e-abc";
        let traj = make_trajectory(task_id, "customer_service", Some(active.id));

        let logger = SqliteLogger::new(store.clone());
        // 현재 런타임(multi_thread) 안에서 동기 save_trajectory 를 호출해도
        // block_in_place 가 워커 스레드를 양보하므로 안전.
        logger.save_trajectory(&traj).expect("save ok");

        let json = store.get_trajectory_json(task_id).await.unwrap().expect("row");
        assert_eq!(json["prompt_set_id"], serde_json::json!(active.id));
        assert_eq!(json["task_id"], serde_json::json!(task_id));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn spec025_logger_round_trips_none_prompt_set_id() {
        let tmp = tempfile::TempDir::new().unwrap();
        let db_path = tmp.path().join("e2e-none.db");
        let store = Arc::new(SqliteStore::open(&db_path).await.unwrap());
        store.insert_domain("general", "").await.unwrap();
        let task_id = "e2e-none";
        let traj = make_trajectory(task_id, "general", None);
        let logger = SqliteLogger::new(store.clone());
        logger.save_trajectory(&traj).expect("save ok");
        let json = store.get_trajectory_json(task_id).await.unwrap().expect("row");
        assert!(json["prompt_set_id"].is_null());
    }
}
