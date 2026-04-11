use chrono::{DateTime,
             Utc};
use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

/// PPA 루프의 단계
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PpaStage {
    Perceive,
    Policy,
    Action,
}

/// Agent의 상태 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub current_stage: PpaStage,
    pub iteration: u32,
    pub task_description: String,
    pub perceived_info: HashMap<String, serde_json::Value>,
    pub planned_actions: Vec<serde_json::Value>,
    pub executed_actions: Vec<HashMap<String, serde_json::Value>>,
    pub is_complete: bool,
    pub error_message: Option<String>,
}

impl AgentState {
    pub fn new(task_description: String) -> Self {
        Self {
            current_stage: PpaStage::Perceive,
            iteration: 0,
            task_description,
            perceived_info: HashMap::new(),
            planned_actions: Vec::new(),
            executed_actions: Vec::new(),
            is_complete: false,
            error_message: None,
        }
    }

    pub fn with_environment(mut self, env: HashMap<String, serde_json::Value>) -> Self {
        self.perceived_info = env;
        self
    }
}

/// 도구 호출 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl ToolCall {
    pub fn new(tool_name: String, parameters: HashMap<String, serde_json::Value>) -> Self {
        Self {
            tool_name,
            parameters,
            timestamp: Utc::now(),
            success: true,
            result: None,
            error: None,
        }
    }
}

/// PPA 루프의 단일 단계
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpaStep {
    pub stage: PpaStage,
    pub iteration: u32,
    pub timestamp: DateTime<Utc>,
    pub input_data: HashMap<String, serde_json::Value>,
    pub output_data: HashMap<String, serde_json::Value>,
    pub tool_calls: Vec<ToolCall>,
    pub duration_ms: Option<f64>,
}

/// Agent의 전체 궤적
///
/// SPEC-025: `prompt_set_id` 는 이 실행에 사용된 PromptSet 의 DB id.
/// 구버전 JSON (`prompt_set_id` 키 없음) 역직렬화 호환을 위해 `serde(default)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub task_id: String,
    pub task_description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub steps: Vec<PpaStep>,
    pub final_state: Option<AgentState>,
    pub success: bool,
    pub total_iterations: u32,
    #[serde(default)]
    pub prompt_set_id: Option<i64>,
}

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

#[cfg(test)]
mod spec025_tests {
    use super::*;

    /// @trace TC: SPEC-025/TC-15
    /// @trace FR: PRD-025/FR-8
    #[test]
    fn spec025_tc_15_legacy_trajectory_json_parses_with_none() {
        // prompt_set_id 가 없는 구 JSON (SPEC-025 이전 포맷)
        let legacy = r#"{
            "task_id": "t-legacy-1",
            "task_description": "test",
            "start_time": "2026-04-11T00:00:00Z",
            "end_time": null,
            "steps": [],
            "final_state": null,
            "success": true,
            "total_iterations": 0
        }"#;
        let t: Trajectory = serde_json::from_str(legacy).expect("legacy JSON must parse");
        assert_eq!(t.task_id, "t-legacy-1");
        assert!(t.prompt_set_id.is_none(), "#[serde(default)] 으로 None");
    }

    /// @trace TC: SPEC-025/TC-14
    /// @trace FR: PRD-025/FR-8
    #[test]
    fn spec025_tc_14_trajectory_records_prompt_set_id() {
        // 실행 경로의 핵심 계약: 첫 스텝에서 resolve 된 prompt_set_id 를
        // Trajectory 에 한 번 기록하고, 이후 스텝에서는 덮어쓰지 않는다.
        let mut t = Trajectory {
            task_id: "x".into(),
            task_description: "".into(),
            start_time: Utc::now(),
            end_time: None,
            steps: vec![],
            final_state: None,
            success: false,
            total_iterations: 0,
            prompt_set_id: None,
        };
        // 첫 스텝에서 id=42 주입
        if t.prompt_set_id.is_none() {
            t.prompt_set_id = Some(42);
        }
        // 두 번째 스텝에서 id=99 (무시되어야 함)
        if t.prompt_set_id.is_none() {
            t.prompt_set_id = Some(99);
        }
        assert_eq!(t.prompt_set_id, Some(42));
        // Serde round-trip 시에도 필드가 살아 있는지
        let j = serde_json::to_string(&t).unwrap();
        assert!(j.contains("\"prompt_set_id\":42"));
        let back: Trajectory = serde_json::from_str(&j).unwrap();
        assert_eq!(back.prompt_set_id, Some(42));
    }
}
