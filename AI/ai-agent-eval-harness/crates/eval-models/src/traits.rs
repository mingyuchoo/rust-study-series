use std::collections::HashMap;

/// 평가기가 시나리오로부터 필요로 하는 최소 계약.
/// `data-scenarios::Scenario` 의 구체 구조체 대신 이 트레이트를 받으면
/// `scoring` 크레이트가 `data-scenarios` 에 직접 의존하지 않아도 된다.
pub trait EvalContext {
    fn expected_tools(&self) -> &[String];
    fn success_criteria(&self) -> &HashMap<String, serde_json::Value>;
}

/// 골든셋 엔트리로부터 평가기가 필요로 하는 최소 계약.
pub trait GoldenSetContext {
    fn tool_sequence(&self) -> &[String];
    fn tool_results(&self) -> &HashMap<String, serde_json::Value>;
    fn tolerance(&self) -> f64;
    fn expected_domain(&self) -> Option<&str>;
}
