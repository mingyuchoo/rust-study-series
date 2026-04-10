use crate::models::EvaluationReport;
use anyhow::Result;
use colored::*;

pub struct ReportRenderer;

impl ReportRenderer {
    pub fn new() -> Self { Self }

    pub fn render(&self, report: &EvaluationReport) {
        println!(
            "\n{}",
            format!("=== 평가 리포트 - {} (에이전트: {}) ===", report.eval_scenario, report.agent_name)
                .cyan()
                .bold()
        );

        self.render_summary(report);
        self.render_average_metrics(report);
        self.render_scenario_results(report);
    }

    fn render_summary(&self, report: &EvaluationReport) {
        println!("\n{:<20} {}", "타임스탬프:", report.timestamp);
        println!("{:<20} {}", "평가 시나리오:", report.eval_scenario);
        println!("{:<20} {}", "에이전트:", report.agent_name);
        println!("{:<20} {}", "총 시나리오:", report.total_scenarios);
        println!("{:<20} {}", "성공:", report.success_count);
        println!("{:<20} {}", "실패:", report.total_scenarios - report.success_count);
        println!("{:<20} {:.1}%", "성공률:", report.success_rate * 100.0);
    }

    fn render_average_metrics(&self, report: &EvaluationReport) {
        if report.average_metrics.is_empty() {
            return;
        }

        println!("\n{}", "── 평균 메트릭 ──".cyan());
        let mut keys: Vec<&String> = report.average_metrics.keys().collect();
        keys.sort();
        for name in keys {
            let val = report.average_metrics[name];
            let formatted = self.format_score(val);
            println!("  {:<35} {}", name, formatted);
        }
    }

    fn render_scenario_results(&self, report: &EvaluationReport) {
        if report.scenarios.is_empty() {
            return;
        }

        println!("\n{}", "── 시나리오별 결과 ──".cyan());
        println!("{:<55} {:^6} {:>6}", "작업", "성공", "반복");
        println!("{}", "-".repeat(70));
        for s in &report.scenarios {
            let desc = truncate_str(&s.task_description, 52);
            let success_icon = if s.success { "O".green().to_string() } else { "X".red().to_string() };
            println!("{:<55} {:^6} {:>6}", desc, success_icon, s.total_iterations);
        }
    }

    fn format_score(&self, score: f64) -> String {
        let pct = format!("{:.1}%", score * 100.0);
        if score >= 0.8 {
            pct.green().to_string()
        } else if score >= 0.6 {
            pct.yellow().to_string()
        } else {
            pct.red().to_string()
        }
    }

    pub fn load_report(&self, filepath: &str) -> Result<EvaluationReport> {
        let content = std::fs::read_to_string(filepath)?;
        Ok(serde_json::from_str(&content)?)
    }
}

impl Default for ReportRenderer {
    fn default() -> Self { Self::new() }
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        format!("{}...", chars[.. max_chars].iter().collect::<String>())
    }
}
