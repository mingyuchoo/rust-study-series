use agent_models::models::{EvaluationResult,
                           PpaStage,
                           Trajectory};
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

pub struct TrajectoryLogger {
    log_dir: PathBuf,
    trajectory_dir: PathBuf,
}

impl TrajectoryLogger {
    pub fn new(log_dir: &str, trajectory_dir: &str) -> Self {
        let log_dir = PathBuf::from(log_dir);
        let trajectory_dir = PathBuf::from(trajectory_dir);
        std::fs::create_dir_all(&log_dir).ok();
        std::fs::create_dir_all(&trajectory_dir).ok();
        Self {
            log_dir,
            trajectory_dir,
        }
    }

    pub fn save_trajectory(&self, trajectory: &Trajectory) -> Result<PathBuf> {
        let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("trajectory_{}_{}.json", trajectory.task_id, ts);
        let filepath = self.trajectory_dir.join(&filename);
        let json = serde_json::to_string_pretty(trajectory)?;
        std::fs::write(&filepath, json)?;
        println!("{} 궤적 저장: {}", "✓".green(), filepath.display());
        Ok(filepath)
    }

    pub fn save_evaluation(&self, evaluation: &EvaluationResult) -> Result<PathBuf> {
        let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("evaluation_{}_{}.json", evaluation.trajectory.task_id, ts);
        let filepath = self.log_dir.join(&filename);
        let json = serde_json::to_string_pretty(evaluation)?;
        std::fs::write(&filepath, json)?;
        println!("{} 평가 결과 저장: {}", "✓".green(), filepath.display());
        Ok(filepath)
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
