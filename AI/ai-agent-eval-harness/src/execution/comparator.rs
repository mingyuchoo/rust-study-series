use super::models::{ComparisonResult,
                    EvaluationReport,
                    MetricDelta};
use anyhow::Result;
use chrono::Utc;
use colored::*;
use std::collections::HashSet;

pub struct ReportComparator {
    pub threshold_percent: f64,
}

impl ReportComparator {
    pub fn new(threshold_percent: f64) -> Self {
        Self {
            threshold_percent,
        }
    }

    pub fn compare(&self, baseline: &EvaluationReport, current: &EvaluationReport) -> ComparisonResult {
        let all_keys: HashSet<&String> = baseline.average_metrics.keys().chain(current.average_metrics.keys()).collect();
        let mut sorted_keys: Vec<&&String> = all_keys.iter().collect();
        sorted_keys.sort();

        let metric_deltas: Vec<MetricDelta> = sorted_keys
            .iter()
            .map(|key| {
                let baseline_val = baseline.average_metrics.get(**key).copied();
                let current_val = current.average_metrics.get(**key).copied();
                self.compute_delta(key, baseline_val, current_val)
            })
            .collect();

        let regression_count = metric_deltas.iter().filter(|d| d.is_regression).count();
        let improvement_count = metric_deltas.iter().filter(|d| d.direction == "improved").count();
        let verdict = if regression_count > 0 { "fail" } else { "pass" }.to_string();
        let summary = self.build_summary(regression_count, improvement_count, &verdict);

        ComparisonResult {
            version: "1.0".into(),
            timestamp: Utc::now().format("%Y%m%d_%H%M%S").to_string(),
            baseline_timestamp: baseline.timestamp.clone(),
            current_timestamp: current.timestamp.clone(),
            metric_deltas,
            regression_count,
            improvement_count,
            verdict,
            threshold_percent: self.threshold_percent,
            summary,
        }
    }

    fn compute_delta(&self, name: &str, baseline: Option<f64>, current: Option<f64>) -> MetricDelta {
        match (baseline, current) {
            | (Some(b), Some(c)) => {
                let delta = c - b;
                let delta_percent = if b != 0.0 { delta / b * 100.0 } else { 0.0 };
                let is_regression = delta_percent < -self.threshold_percent;
                let direction = if delta > 0.0 {
                    "improved"
                } else if delta < 0.0 {
                    "degraded"
                } else {
                    "unchanged"
                };
                MetricDelta {
                    metric_name: name.to_string(),
                    baseline_value: Some(b),
                    current_value: Some(c),
                    delta: Some(delta),
                    delta_percent: Some(delta_percent),
                    is_regression,
                    direction: direction.to_string(),
                }
            },
            | _ => MetricDelta {
                metric_name: name.to_string(),
                baseline_value: baseline,
                current_value: current,
                delta: None,
                delta_percent: None,
                is_regression: false,
                direction: "unchanged".to_string(),
            },
        }
    }

    fn build_summary(&self, regressions: usize, improvements: usize, verdict: &str) -> String {
        if verdict == "fail" {
            format!("회귀 감지: {}개 메트릭이 {:.1}% 이상 하락했습니다.", regressions, self.threshold_percent)
        } else {
            format!("통과: {}개 개선, 회귀 없음 (임계값: {:.1}%)", improvements, self.threshold_percent)
        }
    }

    pub fn compare_files(&self, baseline_path: &str, current_path: &str) -> Result<ComparisonResult> {
        let baseline: EvaluationReport = serde_json::from_str(&std::fs::read_to_string(baseline_path)?)?;
        let current: EvaluationReport = serde_json::from_str(&std::fs::read_to_string(current_path)?)?;
        Ok(self.compare(&baseline, &current))
    }

    pub fn print_comparison(&self, result: &ComparisonResult) {
        if result.verdict == "pass" {
            println!("\n{}", "판정: PASS".green().bold());
        } else {
            println!("\n{}", "판정: FAIL".red().bold());
        }
        println!("{}", result.summary);

        println!("\n{:<30} {:>12} {:>12} {:>10} 상태", "메트릭", "베이스라인", "현재", "변화");
        println!("{}", "-".repeat(80));

        for d in &result.metric_deltas {
            let baseline_str = d.baseline_value.map(|v| format!("{:.3}", v)).unwrap_or_else(|| "-".into());
            let current_str = d.current_value.map(|v| format!("{:.3}", v)).unwrap_or_else(|| "-".into());
            let delta_str = d.delta_percent.map(|v| format!("{:+.1}%", v)).unwrap_or_else(|| "-".into());

            let status = if d.is_regression {
                "REGRESSION".red().bold().to_string()
            } else if d.direction == "improved" {
                "IMPROVED".green().to_string()
            } else if d.direction == "degraded" {
                "DEGRADED".yellow().to_string()
            } else {
                "OK".dimmed().to_string()
            };

            println!("{:<30} {:>12} {:>12} {:>10} {}", d.metric_name, baseline_str, current_str, delta_str, status);
        }
    }

    pub fn save_comparison(&self, result: &ComparisonResult, output_path: &str) -> Result<()> {
        let path = std::path::Path::new(output_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(result)?;
        std::fs::write(path, json)?;
        println!("{}", format!("비교 결과 저장: {}", output_path).green());
        Ok(())
    }
}

impl Default for ReportComparator {
    fn default() -> Self { Self::new(5.0) }
}
