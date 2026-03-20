use crate::{agent_core::models::{CriteriaCheckResult,
                                 GoldenSetResult,
                                 PpaStage,
                                 Trajectory},
            data_scenarios::models::{GoldenSetEntry,
                                     Scenario}};
use std::collections::HashMap;

pub struct GoldenSetValidator {
    tolerance: f64,
}

impl GoldenSetValidator {
    pub fn new(tolerance: f64) -> Self {
        Self {
            tolerance,
        }
    }

    pub fn validate(&self, trajectory: &Trajectory, scenario: &Scenario, _enable_llm_judge: bool, _llm_client: Option<()>) -> GoldenSetResult {
        let criteria_results = self.validate_tool_results(trajectory, &scenario.success_criteria);
        let criteria_score = if criteria_results.is_empty() {
            1.0
        } else {
            criteria_results.values().filter(|r| r.passed).count() as f64 / criteria_results.len() as f64
        };

        let (tool_seq_score, actual_tools, _, _) = self.validate_tool_sequence(trajectory, &scenario.expected_tools);

        let overall = criteria_score * 0.7 + tool_seq_score * 0.3;

        GoldenSetResult {
            criteria_results,
            criteria_score,
            expected_tools: scenario.expected_tools.clone(),
            actual_tools,
            tool_sequence_score: tool_seq_score,
            llm_judge_score: None,
            llm_judge_reasoning: None,
            overall_score: overall,
        }
    }

    pub fn validate_with_golden_entry(
        &self,
        trajectory: &Trajectory,
        entry: &GoldenSetEntry,
        _enable_llm_judge: bool,
        _llm_client: Option<()>,
    ) -> GoldenSetResult {
        let tolerance = entry.expected_output.tolerance;
        let validator = GoldenSetValidator::new(tolerance);

        let actual_results = self.collect_tool_results(trajectory);
        let mut criteria_results = HashMap::new();

        for (key, expected) in &entry.expected_output.tool_results {
            if let Some(actual) = actual_results.get(key) {
                let (passed, match_type) = validator.compare_values(expected, actual);
                criteria_results.insert(
                    key.clone(),
                    CriteriaCheckResult {
                        key: key.clone(),
                        expected: expected.clone(),
                        actual: Some(actual.clone()),
                        passed,
                        match_type,
                    },
                );
            } else {
                criteria_results.insert(
                    key.clone(),
                    CriteriaCheckResult {
                        key: key.clone(),
                        expected: expected.clone(),
                        actual: None,
                        passed: false,
                        match_type: "missing".into(),
                    },
                );
            }
        }

        let criteria_score = if criteria_results.is_empty() {
            1.0
        } else {
            criteria_results.values().filter(|r| r.passed).count() as f64 / criteria_results.len() as f64
        };

        let (tool_seq_score, actual_tools, _, _) = self.validate_tool_sequence(trajectory, &entry.expected_output.tool_sequence);
        let overall = criteria_score * 0.7 + tool_seq_score * 0.3;

        GoldenSetResult {
            criteria_results,
            criteria_score,
            expected_tools: entry.expected_output.tool_sequence.clone(),
            actual_tools,
            tool_sequence_score: tool_seq_score,
            llm_judge_score: None,
            llm_judge_reasoning: None,
            overall_score: overall,
        }
    }

    fn validate_tool_results(&self, trajectory: &Trajectory, criteria: &HashMap<String, serde_json::Value>) -> HashMap<String, CriteriaCheckResult> {
        let actual = self.collect_tool_results(trajectory);
        let mut results = HashMap::new();

        for (key, expected) in criteria {
            if let Some(actual_val) = actual.get(key) {
                let (passed, match_type) = self.compare_values(expected, actual_val);
                results.insert(
                    key.clone(),
                    CriteriaCheckResult {
                        key: key.clone(),
                        expected: expected.clone(),
                        actual: Some(actual_val.clone()),
                        passed,
                        match_type,
                    },
                );
            } else {
                results.insert(
                    key.clone(),
                    CriteriaCheckResult {
                        key: key.clone(),
                        expected: expected.clone(),
                        actual: None,
                        passed: false,
                        match_type: "missing".into(),
                    },
                );
            }
        }
        results
    }

    pub fn validate_tool_sequence(&self, trajectory: &Trajectory, expected_tools: &[String]) -> (f64, Vec<String>, Vec<String>, Vec<String>) {
        let actual_tools = self.collect_tool_names(trajectory);

        if expected_tools.is_empty() {
            return (1.0, actual_tools.clone(), Vec::new(), actual_tools);
        }

        let mut matched = 0usize;
        let mut actual_idx = 0usize;
        for expected in expected_tools {
            while actual_idx < actual_tools.len() {
                if &actual_tools[actual_idx] == expected {
                    matched += 1;
                    actual_idx += 1;
                    break;
                }
                actual_idx += 1;
            }
        }

        let score = matched as f64 / expected_tools.len() as f64;
        let missing: Vec<String> = expected_tools.iter().filter(|t| !actual_tools.contains(t)).cloned().collect();
        let extra: Vec<String> = actual_tools.iter().filter(|t| !expected_tools.contains(t)).cloned().collect();

        (score, actual_tools, missing, extra)
    }

    fn collect_tool_results(&self, trajectory: &Trajectory) -> HashMap<String, serde_json::Value> {
        let mut merged = HashMap::new();
        for step in &trajectory.steps {
            if step.stage == PpaStage::Action {
                for tc in &step.tool_calls {
                    if tc.success {
                        if let Some(result) = &tc.result {
                            if let Some(obj) = result.as_object() {
                                for (k, v) in obj {
                                    merged.insert(k.clone(), v.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        merged
    }

    fn collect_tool_names(&self, trajectory: &Trajectory) -> Vec<String> {
        trajectory
            .steps
            .iter()
            .filter(|s| s.stage == PpaStage::Action)
            .flat_map(|s| s.tool_calls.iter().map(|tc| tc.tool_name.clone()))
            .collect()
    }

    fn compare_values(&self, expected: &serde_json::Value, actual: &serde_json::Value) -> (bool, String) {
        if expected.is_boolean() {
            return (expected == actual, "exact".into());
        }
        if let (Some(e), Some(a)) = (expected.as_f64(), actual.as_f64()) {
            if (e - a).abs() < f64::EPSILON {
                return (true, "exact".into());
            }
            if e != 0.0 && ((a - e) / e).abs() <= self.tolerance {
                return (true, "numeric_tolerance".into());
            }
            return (false, "numeric_tolerance".into());
        }
        if let (Some(e), Some(a)) = (expected.as_str(), actual.as_str()) {
            if a.to_lowercase().contains(&e.to_lowercase()) {
                return (true, "contains".into());
            }
            return (e == a, "exact".into());
        }
        (expected == actual, "exact".into())
    }
}

impl Default for GoldenSetValidator {
    fn default() -> Self { Self::new(0.01) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_core::models::{PpaStep,
                                    ToolCall,
                                    Trajectory};
    use chrono::Utc;

    fn make_trajectory(tool_calls: Vec<(&str, bool, Option<serde_json::Value>)>) -> Trajectory {
        let calls: Vec<ToolCall> = tool_calls
            .into_iter()
            .map(|(name, success, result)| ToolCall {
                tool_name: name.to_string(),
                parameters: HashMap::new(),
                timestamp: Utc::now(),
                success,
                result,
                error: None,
            })
            .collect();

        let step = PpaStep {
            stage: PpaStage::Action,
            iteration: 1,
            timestamp: Utc::now(),
            input_data: HashMap::new(),
            output_data: HashMap::new(),
            tool_calls: calls,
            duration_ms: None,
        };

        Trajectory {
            task_id: "test".into(),
            task_description: "test task".into(),
            start_time: Utc::now(),
            end_time: None,
            steps: vec![step],
            final_state: None,
            success: true,
            total_iterations: 1,
        }
    }

    #[test]
    fn compare_values_exact_bool() {
        let v = GoldenSetValidator::new(0.01);
        let (passed, match_type) = v.compare_values(&serde_json::json!(true), &serde_json::json!(true));
        assert!(passed);
        assert_eq!(match_type, "exact");
        let (passed2, _) = v.compare_values(&serde_json::json!(true), &serde_json::json!(false));
        assert!(!passed2);
    }

    #[test]
    fn compare_values_numeric_tolerance() {
        let v = GoldenSetValidator::new(0.01);
        let (passed, match_type) = v.compare_values(&serde_json::json!(100.0), &serde_json::json!(100.5));
        assert!(passed);
        assert_eq!(match_type, "numeric_tolerance");
        let (failed, _) = v.compare_values(&serde_json::json!(100.0), &serde_json::json!(110.0));
        assert!(!failed);
    }

    #[test]
    fn compare_values_string_contains() {
        let v = GoldenSetValidator::new(0.01);
        let (passed, match_type) = v.compare_values(&serde_json::json!("hello"), &serde_json::json!("say hello world"));
        assert!(passed);
        assert_eq!(match_type, "contains");
    }

    #[test]
    fn validate_tool_sequence_all_present() {
        let trajectory = make_trajectory(vec![("tool_a", true, None), ("tool_b", true, None)]);
        let v = GoldenSetValidator::new(0.01);
        let (score, actual, missing, _) = v.validate_tool_sequence(&trajectory, &["tool_a".to_string(), "tool_b".to_string()]);
        assert_eq!(score, 1.0);
        assert_eq!(actual, vec!["tool_a", "tool_b"]);
        assert!(missing.is_empty());
    }

    #[test]
    fn validate_tool_sequence_partial() {
        let trajectory = make_trajectory(vec![("tool_a", true, None)]);
        let v = GoldenSetValidator::new(0.01);
        let (score, _, missing, _) = v.validate_tool_sequence(&trajectory, &["tool_a".to_string(), "tool_b".to_string()]);
        assert_eq!(score, 0.5);
        assert_eq!(missing, vec!["tool_b"]);
    }

    #[test]
    fn validate_tool_sequence_empty_expected() {
        let trajectory = make_trajectory(vec![("tool_a", true, None)]);
        let v = GoldenSetValidator::new(0.01);
        let (score, _, _, _) = v.validate_tool_sequence(&trajectory, &[]);
        assert_eq!(score, 1.0);
    }
}
