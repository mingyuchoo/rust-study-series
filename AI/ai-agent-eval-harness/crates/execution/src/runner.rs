use crate::{models::{EvaluationReport,
                     ScenarioResult},
            report_renderer::ReportRenderer};
use agent_models::{base_agent::BaseAgent,
                   models::EvaluationResult};
use anyhow::Result;
use chrono::Utc;
use colored::*;
use data_scenarios::{loader::ScenarioLoader,
                     models::Scenario};
use reporting::logger::TrajectoryLogger;
use scoring::evaluator::TrajectoryEvaluator;
use std::{collections::HashMap,
          path::Path};

pub struct HarnessRunner {
    output_dir: std::path::PathBuf,
    logger: TrajectoryLogger,
    evaluator: TrajectoryEvaluator,
    results: Vec<EvaluationResult>,
}

impl HarnessRunner {
    pub fn new(output_dir: &str) -> Self {
        let path = std::path::PathBuf::from(output_dir);
        std::fs::create_dir_all(&path).ok();
        Self {
            output_dir: path,
            logger: TrajectoryLogger::new("reporting_logs", "reporting_trajectories"),
            evaluator: TrajectoryEvaluator::new(),
            results: Vec::new(),
        }
    }

    pub fn run_scenario(&self, scenario: &Scenario, agent: &dyn BaseAgent) -> EvaluationResult {
        let _meta = agent.metadata();
        println!("\n  {}", scenario.name.cyan());

        let start = std::time::Instant::now();
        let trajectory = agent.execute_task(&scenario.task_description, Some(scenario.initial_environment.clone()));
        let elapsed = start.elapsed().as_secs_f64();
        println!("  실행 시간: {:.2}초", elapsed);

        let evaluation = self.evaluator.evaluate(&trajectory, Some(scenario), None);
        self.logger.save_trajectory(&trajectory).ok();
        self.logger.save_evaluation(&evaluation).ok();

        let status = if evaluation.trajectory.success { "성공" } else { "실패" };
        println!("  결과: {}", status);

        evaluation
    }

    pub fn run_suite(&mut self, suite_name: &str, agent: &dyn BaseAgent, scenarios_dir: &str) -> Result<EvaluationReport> {
        let loader = ScenarioLoader::new();
        let scenarios_path = Path::new(scenarios_dir);

        let domain_configs = if suite_name == "all" {
            loader.load_all_domains(scenarios_dir)?
        } else {
            let config_path = scenarios_path.join(format!("{}.yaml", suite_name));
            vec![loader.load_domain_config(&config_path.to_string_lossy())?]
        };

        let mut all_scenarios: Vec<Scenario> = Vec::new();
        for config in &domain_configs {
            for s in &config.scenarios {
                all_scenarios.push(Scenario {
                    id: s.id.clone(),
                    name: s.name.clone(),
                    description: s.description.clone(),
                    task_description: s.task_description.clone(),
                    initial_environment: s.initial_environment.clone(),
                    expected_tools: s.expected_tools.clone(),
                    success_criteria: s.success_criteria.clone(),
                    difficulty: s.difficulty.clone(),
                    domain: config.name.clone(),
                });
            }
        }

        println!("\n{}", "평가 시작".yellow().bold());
        println!("에이전트: {}", agent.metadata().name);
        println!("스위트: {}", suite_name);
        println!("총 {}개 시나리오\n", all_scenarios.len());

        self.results.clear();
        for (i, scenario) in all_scenarios.iter().enumerate() {
            println!("[{}/{}] {}", i + 1, all_scenarios.len(), scenario.name);
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.run_scenario(scenario, agent))) {
                | Ok(eval) => self.results.push(eval),
                | Err(e) => eprintln!("  {}: {:?}", "오류".red(), e),
            }
        }

        let report = self.build_report(suite_name, &agent.metadata().name);
        Ok(report)
    }

    fn build_report(&self, suite_name: &str, agent_name: &str) -> EvaluationReport {
        let avg_metrics = self.calculate_average_metrics();
        let success_count = self.results.iter().filter(|r| r.trajectory.success).count();
        let total = self.results.len();

        let scenarios: Vec<ScenarioResult> = self
            .results
            .iter()
            .map(|r| ScenarioResult {
                task_id: r.trajectory.task_id.clone(),
                task_description: r.trajectory.task_description.clone(),
                success: r.trajectory.success,
                total_iterations: r.trajectory.total_iterations,
                metrics: r.metrics.to_map(),
                domain: "general".into(),
                scenario_id: String::new(),
            })
            .collect();

        EvaluationReport {
            version: "1.0".into(),
            timestamp: Utc::now().format("%Y%m%d_%H%M%S").to_string(),
            agent_name: agent_name.to_string(),
            suite: suite_name.to_string(),
            total_scenarios: total,
            success_count,
            success_rate: if total > 0 { success_count as f64 / total as f64 } else { 0.0 },
            average_metrics: avg_metrics,
            scenarios,
        }
    }

    fn calculate_average_metrics(&self) -> HashMap<String, f64> {
        let mut sum: HashMap<String, f64> = HashMap::new();
        let mut count: HashMap<String, usize> = HashMap::new();

        for result in &self.results {
            for (key, val) in result.metrics.to_map() {
                if let Some(v) = val {
                    *sum.entry(key.clone()).or_default() += v;
                    *count.entry(key).or_default() += 1;
                }
            }
        }

        sum.into_iter().filter_map(|(k, v)| count.get(&k).map(|&c| (k, v / c as f64))).collect()
    }

    pub fn save_report(&self, report: &EvaluationReport, output_path: Option<&str>) -> Result<std::path::PathBuf> {
        let filepath = if let Some(p) = output_path {
            std::path::PathBuf::from(p)
        } else {
            self.output_dir.join(format!("evaluation_report_{}.json", report.timestamp))
        };

        if let Some(parent) = filepath.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(report)?;
        std::fs::write(&filepath, json)?;

        let renderer = ReportRenderer::new();
        println!("{}", format!("리포트 저장: {}", filepath.display()).green());
        let _ = renderer;
        Ok(filepath)
    }
}
