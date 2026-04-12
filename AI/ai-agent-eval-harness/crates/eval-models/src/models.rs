use agent_models::models::Trajectory;
use chrono::{DateTime,
             Utc};
use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

/// 개별 기준 검증 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriteriaCheckResult {
    pub key: String,
    pub expected: serde_json::Value,
    pub actual: Option<serde_json::Value>,
    pub passed: bool,
    pub match_type: String,
}

/// 골든셋 검증 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetResult {
    pub criteria_results: HashMap<String, CriteriaCheckResult>,
    pub criteria_score: f64,
    pub expected_tools: Vec<String>,
    pub actual_tools: Vec<String>,
    pub tool_sequence_score: f64,
    pub llm_judge_score: Option<f64>,
    pub llm_judge_reasoning: Option<String>,
    pub overall_score: f64,
    /// 에이전트의 첫 tool call 이 `expected_domain` 과 일치하는지(SPEC-020).
    /// `expected_domain` 이 None 이면 이 필드도 None.
    #[serde(default)]
    pub domain_routing_score: Option<f64>,
    /// 실제로 처음 호출된 도구의 도메인(디버깅용). None 이면 tool call 없음.
    #[serde(default)]
    pub actual_first_domain: Option<String>,
}

/// 평가 메트릭
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    // Perceive 단계
    pub perception_accuracy: Option<f64>,
    pub context_retention_rate: Option<f64>,
    pub error_detection_rate: Option<f64>,
    // Policy 단계
    pub plan_efficiency: Option<f64>,
    pub tool_selection_accuracy: Option<f64>,
    pub autonomy_calibration: Option<f64>,
    // Action 단계
    pub tool_call_success_rate: Option<f64>,
    pub side_effect_containment: Option<f64>,
    pub recovery_rate: Option<f64>,
    // 전체 궤적
    pub outcome_correctness: Option<f64>,
    pub path_efficiency: Option<f64>,
    pub safety_score: Option<f64>,
    pub adaptability: Option<f64>,
    // 골든셋 검증
    pub golden_set_score: Option<f64>,
}

impl EvaluationMetrics {
    pub fn to_map(&self) -> HashMap<String, Option<f64>> {
        let mut m = HashMap::new();
        m.insert("perception_accuracy".into(), self.perception_accuracy);
        m.insert("context_retention_rate".into(), self.context_retention_rate);
        m.insert("error_detection_rate".into(), self.error_detection_rate);
        m.insert("plan_efficiency".into(), self.plan_efficiency);
        m.insert("tool_selection_accuracy".into(), self.tool_selection_accuracy);
        m.insert("autonomy_calibration".into(), self.autonomy_calibration);
        m.insert("tool_call_success_rate".into(), self.tool_call_success_rate);
        m.insert("side_effect_containment".into(), self.side_effect_containment);
        m.insert("recovery_rate".into(), self.recovery_rate);
        m.insert("outcome_correctness".into(), self.outcome_correctness);
        m.insert("path_efficiency".into(), self.path_efficiency);
        m.insert("safety_score".into(), self.safety_score);
        m.insert("adaptability".into(), self.adaptability);
        m.insert("golden_set_score".into(), self.golden_set_score);
        m
    }
}

/// 평가 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub trajectory: Trajectory,
    pub metrics: EvaluationMetrics,
    pub analysis: HashMap<String, serde_json::Value>,
    pub recommendations: Vec<String>,
    pub golden_set_result: Option<GoldenSetResult>,
    pub timestamp: DateTime<Utc>,
}
