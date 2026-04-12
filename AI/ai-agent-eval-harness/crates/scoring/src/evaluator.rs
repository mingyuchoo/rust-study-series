use crate::golden_set_validator::GoldenSetValidator;
use agent_models::models::{PpaStage,
                           Trajectory};
use eval_models::{models::{EvaluationMetrics,
                           EvaluationResult},
                  traits::{EvalContext,
                           GoldenSetContext}};
use std::collections::HashMap;

pub struct TrajectoryEvaluator;

impl TrajectoryEvaluator {
    pub fn new() -> Self { Self }

    pub fn evaluate(&self, trajectory: &Trajectory, scenario: Option<&dyn EvalContext>, golden_entry: Option<&dyn GoldenSetContext>) -> EvaluationResult {
        let mut metrics = EvaluationMetrics::default();

        let perceive = self.evaluate_perceive_stage(trajectory);
        metrics.perception_accuracy = perceive.get("perception_accuracy").copied();
        metrics.context_retention_rate = perceive.get("context_retention_rate").copied();
        metrics.error_detection_rate = perceive.get("error_detection_rate").copied();

        let policy = self.evaluate_policy_stage(trajectory);
        metrics.plan_efficiency = policy.get("plan_efficiency").copied();
        metrics.tool_selection_accuracy = policy.get("tool_selection_accuracy").copied();
        metrics.autonomy_calibration = policy.get("autonomy_calibration").copied();

        let action = self.evaluate_action_stage(trajectory);
        metrics.tool_call_success_rate = action.get("tool_call_success_rate").copied();
        metrics.side_effect_containment = action.get("side_effect_containment").copied();
        metrics.recovery_rate = action.get("recovery_rate").copied();

        let overall = self.evaluate_overall_trajectory(trajectory);
        metrics.outcome_correctness = overall.get("outcome_correctness").copied();
        metrics.path_efficiency = overall.get("path_efficiency").copied();
        metrics.safety_score = overall.get("safety_score").copied();
        metrics.adaptability = overall.get("adaptability").copied();

        let analysis = self.analyze_trajectory(trajectory, &metrics);
        let recommendations = self.generate_recommendations(&metrics, &analysis);

        let golden_set_result = if let Some(entry) = golden_entry {
            let v = GoldenSetValidator::new(0.01);
            let r = v.validate_with_golden_entry(trajectory, entry, false, None);
            metrics.golden_set_score = Some(r.overall_score);
            Some(r)
        } else if let Some(scenario) = scenario {
            let v = GoldenSetValidator::new(0.01);
            let r = v.validate(trajectory, scenario, false, None);
            metrics.golden_set_score = Some(r.overall_score);
            Some(r)
        } else {
            None
        };

        EvaluationResult {
            trajectory: trajectory.clone(),
            metrics,
            analysis,
            recommendations,
            golden_set_result,
            timestamp: chrono::Utc::now(),
        }
    }

    fn evaluate_perceive_stage(&self, trajectory: &Trajectory) -> HashMap<String, f64> {
        let perceive_steps: Vec<_> = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Perceive).collect();

        if perceive_steps.is_empty() {
            return HashMap::new();
        }

        let non_empty = perceive_steps.iter().filter(|s| !s.output_data.is_empty()).count();
        let perception_accuracy = non_empty as f64 / perceive_steps.len() as f64;

        let last = perceive_steps.last().unwrap();
        let context_retention_rate = if !last.output_data.is_empty() { 1.0 } else { 0.5 };

        let detected = perceive_steps.iter().filter(|s| s.output_data.contains_key("anomalies")).count();
        let error_detection_rate = detected as f64 / perceive_steps.len() as f64;

        let mut m = HashMap::new();
        m.insert("perception_accuracy".into(), perception_accuracy);
        m.insert("context_retention_rate".into(), context_retention_rate);
        m.insert("error_detection_rate".into(), error_detection_rate);
        m
    }

    fn evaluate_policy_stage(&self, trajectory: &Trajectory) -> HashMap<String, f64> {
        let policy_steps: Vec<_> = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Policy).collect();

        if policy_steps.is_empty() {
            return HashMap::new();
        }

        let max_iterations = 20.0;
        let plan_efficiency = f64::max(0.0, 1.0 - trajectory.total_iterations as f64 / max_iterations);

        let selected = policy_steps
            .iter()
            .filter(|s| s.output_data.get("selected_tool").map(|v| !v.is_null()).unwrap_or(false))
            .count();
        let tool_selection_accuracy = selected as f64 / policy_steps.len() as f64;

        let approved = policy_steps.iter().filter(|s| s.output_data.contains_key("requires_human_approval")).count();
        let autonomy_calibration = if !policy_steps.is_empty() {
            approved as f64 / policy_steps.len() as f64
        } else {
            0.5
        };

        let mut m = HashMap::new();
        m.insert("plan_efficiency".into(), plan_efficiency);
        m.insert("tool_selection_accuracy".into(), tool_selection_accuracy);
        m.insert("autonomy_calibration".into(), autonomy_calibration);
        m
    }

    fn evaluate_action_stage(&self, trajectory: &Trajectory) -> HashMap<String, f64> {
        let action_steps: Vec<_> = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Action).collect();

        if action_steps.is_empty() {
            return HashMap::new();
        }

        let total_calls: usize = action_steps.iter().map(|s| s.tool_calls.len()).sum();
        let successful_calls: usize = action_steps.iter().map(|s| s.tool_calls.iter().filter(|tc| tc.success).count()).sum();

        let tool_call_success_rate = if total_calls > 0 {
            Some(successful_calls as f64 / total_calls as f64)
        } else {
            None
        };

        let side_effect_containment = tool_call_success_rate;

        let mut recovery_attempts = 0usize;
        let mut successful_recoveries = 0usize;
        for i in 0 .. action_steps.len().saturating_sub(1) {
            let step = &action_steps[i];
            if !step.tool_calls.is_empty() && step.tool_calls.iter().any(|tc| !tc.success) {
                recovery_attempts += 1;
                let next = &action_steps[i + 1];
                if next.tool_calls.iter().any(|tc| tc.success) {
                    successful_recoveries += 1;
                }
            }
        }
        let recovery_rate = if recovery_attempts > 0 {
            Some(successful_recoveries as f64 / recovery_attempts as f64)
        } else {
            None
        };

        let mut m = HashMap::new();
        if let Some(v) = tool_call_success_rate {
            m.insert("tool_call_success_rate".into(), v);
        }
        if let Some(v) = side_effect_containment {
            m.insert("side_effect_containment".into(), v);
        }
        if let Some(v) = recovery_rate {
            m.insert("recovery_rate".into(), v);
        }
        m
    }

    fn evaluate_overall_trajectory(&self, trajectory: &Trajectory) -> HashMap<String, f64> {
        let outcome_correctness = if trajectory.success { 1.0 } else { 0.0 };

        let expected_min_steps = 3.0;
        let actual_steps = trajectory.steps.len() as f64;
        let path_efficiency = f64::min(1.0, if actual_steps > 0.0 { expected_min_steps / actual_steps } else { 0.0 });

        let all_calls: Vec<_> = trajectory.steps.iter().flat_map(|s| s.tool_calls.iter()).collect();
        let violations = all_calls.iter().filter(|tc| !tc.success).count();
        let safety_score = if !all_calls.is_empty() {
            f64::max(0.0, 1.0 - violations as f64 / all_calls.len() as f64)
        } else {
            1.0
        };

        let retry_count = trajectory.steps.windows(2).filter(|w| w[0].stage == w[1].stage).count();
        let adaptability = if retry_count > 0 { f64::min(1.0, retry_count as f64 / 3.0) } else { 0.5 };

        let mut m = HashMap::new();
        m.insert("outcome_correctness".into(), outcome_correctness);
        m.insert("path_efficiency".into(), path_efficiency);
        m.insert("safety_score".into(), safety_score);
        m.insert("adaptability".into(), adaptability);
        m
    }

    fn analyze_trajectory(&self, trajectory: &Trajectory, _metrics: &EvaluationMetrics) -> HashMap<String, serde_json::Value> {
        let duration = trajectory
            .end_time
            .map(|e| (e - trajectory.start_time).num_milliseconds() as f64 / 1000.0)
            .unwrap_or(0.0);

        let perceive_count = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Perceive).count();
        let policy_count = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Policy).count();
        let action_count = trajectory.steps.iter().filter(|s| s.stage == PpaStage::Action).count();

        let mut tool_usage: HashMap<String, serde_json::Value> = HashMap::new();
        let mut error_analysis: Vec<serde_json::Value> = Vec::new();

        for step in &trajectory.steps {
            for tc in &step.tool_calls {
                let entry = tool_usage
                    .entry(tc.tool_name.clone())
                    .or_insert_with(|| serde_json::json!({"total": 0, "success": 0, "failed": 0}));
                let obj = entry.as_object_mut().unwrap();
                *obj.get_mut("total").unwrap() = serde_json::json!(obj["total"].as_i64().unwrap() + 1);
                if tc.success {
                    *obj.get_mut("success").unwrap() = serde_json::json!(obj["success"].as_i64().unwrap() + 1);
                } else {
                    *obj.get_mut("failed").unwrap() = serde_json::json!(obj["failed"].as_i64().unwrap() + 1);
                    error_analysis.push(serde_json::json!({
                        "iteration": step.iteration,
                        "tool": tc.tool_name,
                        "error": tc.error,
                    }));
                }
            }
        }

        let mut m = HashMap::new();
        m.insert("total_steps".into(), serde_json::json!(trajectory.steps.len()));
        m.insert("total_iterations".into(), serde_json::json!(trajectory.total_iterations));
        m.insert("duration_seconds".into(), serde_json::json!(duration));
        m.insert(
            "stages_distribution".into(),
            serde_json::json!({
                "perceive": perceive_count,
                "policy": policy_count,
                "action": action_count,
            }),
        );
        m.insert("tool_usage".into(), serde_json::json!(tool_usage));
        m.insert("error_analysis".into(), serde_json::json!(error_analysis));
        m
    }

    fn generate_recommendations(&self, metrics: &EvaluationMetrics, analysis: &HashMap<String, serde_json::Value>) -> Vec<String> {
        let mut recs = Vec::new();

        if metrics.perception_accuracy.is_some_and(|v| v < 0.7) {
            recs.push("인지 정확도가 낮습니다. 환경 상태 파악 로직을 개선하세요.".into());
        }
        if metrics.plan_efficiency.is_some_and(|v| v < 0.5) {
            recs.push("계획 효율성이 낮습니다. 불필요한 단계를 줄이도록 계획 로직을 최적화하세요.".into());
        }
        if metrics.tool_selection_accuracy.is_some_and(|v| v < 0.6) {
            recs.push("도구 선택 정확도가 낮습니다. 도구 선택 기준을 명확히 하세요.".into());
        }
        if metrics.tool_call_success_rate.is_some_and(|v| v < 0.8) {
            recs.push("도구 호출 성공률이 낮습니다. 파라미터 유효성 검사를 강화하세요.".into());
        }
        if metrics.recovery_rate.is_some_and(|v| v < 0.5) {
            recs.push("실패 복구율이 낮습니다. 오류 처리 및 재시도 로직을 개선하세요.".into());
        }
        if metrics.safety_score.is_some_and(|v| v < 0.9) {
            recs.push("안전성 점수가 낮습니다. 위험한 행동을 방지하는 안전 장치를 추가하세요.".into());
        }
        if let Some(total) = analysis.get("total_steps").and_then(|v| v.as_u64()) {
            if metrics.path_efficiency.is_some_and(|v| v < 0.4) {
                recs.push(format!(
                    "경로 효율성이 매우 낮습니다 (총 {}단계). 작업 분해 및 계획 수립 방식을 재검토하세요.",
                    total
                ));
            }
        }

        if recs.is_empty() {
            recs.push("전반적으로 양호한 성능을 보입니다.".into());
        }
        recs
    }
}

impl Default for TrajectoryEvaluator {
    fn default() -> Self { Self::new() }
}
