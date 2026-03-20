#![allow(dead_code, clippy::new_without_default)]
use execution_tools::base::{BaseTool,
                            ToolMetadata};
use std::collections::HashMap;
use uuid::Uuid;

const HIGH_AMOUNT_THRESHOLD: f64 = 100_000.0;

fn category_keywords() -> HashMap<&'static str, Vec<&'static str>> {
    let mut m = HashMap::new();
    m.insert("refund", vec!["환불", "반품", "취소", "불량"]);
    m.insert("shipping", vec!["배송", "택배", "운송", "도착"]);
    m.insert("complaint", vec!["불만", "엉망", "화가", "최악", "실망", "짜증"]);
    m
}

const HIGH_PRIORITY_KEYWORDS: &[&str] = &["불만", "엉망", "화가", "최악", "실망", "짜증", "도대체", "화"];

pub struct ClassifyInquiryTool {
    metadata: ToolMetadata,
}
impl ClassifyInquiryTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "classify_inquiry".into(),
                description: "고객 문의를 카테고리와 우선순위로 분류합니다".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "inquiry_text": {"type": "string", "description": "고객 문의 내용"},
                        "customer_id": {"type": "string", "description": "고객 ID"}
                    },
                    "required": ["inquiry_text", "customer_id"]
                }),
                safety_level: "safe".into(),
                requires_approval: false,
            },
        }
    }
}

impl BaseTool for ClassifyInquiryTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let text = match params.get("inquiry_text").and_then(|v| v.as_str()) {
            | Some(t) if !t.trim().is_empty() => t,
            | _ => return err("문의 내용이 비어있습니다"),
        };
        let customer_id = params.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");

        let keywords = category_keywords();
        let category = keywords
            .iter()
            .filter(|(cat, _)| *cat != &"general")
            .find(|(_, kws)| kws.iter().any(|kw| text.contains(kw)))
            .map(|(cat, _)| *cat)
            .unwrap_or("general");

        let priority = if category == "complaint" || HIGH_PRIORITY_KEYWORDS.iter().any(|kw| text.contains(kw)) {
            "high"
        } else if category == "refund" {
            "medium"
        } else {
            "low"
        };

        let mut found_kws: Vec<&str> = Vec::new();
        for kws in keywords.values() {
            for kw in kws {
                if text.contains(kw) {
                    found_kws.push(kw);
                }
            }
        }
        if found_kws.is_empty() {
            found_kws = text.split_whitespace().filter(|w| w.chars().count() >= 2).take(3).collect();
        }

        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("category".into(), serde_json::json!(category));
        m.insert("priority".into(), serde_json::json!(priority));
        m.insert("keywords".into(), serde_json::json!(found_kws));
        m.insert("customer_id".into(), serde_json::json!(customer_id));
        m
    }
}

pub struct ProcessRefundTool {
    metadata: ToolMetadata,
}
impl ProcessRefundTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "process_refund".into(),
                description: "고객의 환불 요청을 검증하고 처리합니다".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "order_id": {"type": "string", "description": "주문 ID"},
                        "amount": {"type": "number", "description": "환불 금액"},
                        "reason": {"type": "string", "description": "환불 사유"},
                        "customer_id": {"type": "string", "description": "고객 ID"}
                    },
                    "required": ["order_id", "amount", "reason", "customer_id"]
                }),
                safety_level: "caution".into(),
                requires_approval: true,
            },
        }
    }
}

impl BaseTool for ProcessRefundTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let order_id = params.get("order_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let amount = match params.get("amount").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("amount 파라미터 필요"),
        };
        let reason = params.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let customer_id = params.get("customer_id").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let mut errors: Vec<String> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();

        if amount <= 0.0 {
            errors.push("환불 금액은 0보다 커야 합니다".into());
        }
        if reason.trim().is_empty() {
            warnings.push("환불 사유가 입력되지 않았습니다".into());
        }
        if amount > HIGH_AMOUNT_THRESHOLD {
            warnings.push("고액 환불 요청입니다. 관리자 확인이 필요합니다".into());
        }

        let approved = errors.is_empty();
        let status = if approved { "approved" } else { "rejected" };
        let refund_id = format!("REF-{}", &Uuid::new_v4().to_string().replace("-", "").to_uppercase()[.. 8]);

        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("approved".into(), serde_json::Value::Bool(approved));
        m.insert("status".into(), serde_json::json!(status));
        m.insert("refund_id".into(), serde_json::json!(refund_id));
        m.insert("order_id".into(), serde_json::json!(order_id));
        m.insert("amount".into(), serde_json::json!(amount));
        m.insert("customer_id".into(), serde_json::json!(customer_id));
        m.insert("reason".into(), serde_json::json!(reason));
        m.insert("errors".into(), serde_json::json!(errors));
        m.insert("warnings".into(), serde_json::json!(warnings));
        m
    }
}

pub struct EscalateIssueTool {
    metadata: ToolMetadata,
}
impl EscalateIssueTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "escalate_issue".into(),
                description: "고객 이슈를 심각도에 따라 적절한 담당자에게 에스컬레이션합니다".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "issue_id": {"type": "string", "description": "이슈 ID"},
                        "severity": {"type": "string", "description": "심각도 (low, medium, high)"},
                        "reason": {"type": "string", "description": "에스컬레이션 사유"},
                        "current_agent": {"type": "string", "description": "현재 담당 에이전트"}
                    },
                    "required": ["issue_id", "severity", "reason", "current_agent"]
                }),
                safety_level: "safe".into(),
                requires_approval: false,
            },
        }
    }
}

impl BaseTool for EscalateIssueTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let issue_id = params.get("issue_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let severity = match params.get("severity").and_then(|v| v.as_str()) {
            | Some(s) => s.to_string(),
            | None => return err("severity 파라미터 필요"),
        };
        let reason = params.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let current_agent = params.get("current_agent").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let (assigned_to, priority) = match severity.as_str() {
            | "high" => ("senior_agent", "urgent"),
            | "medium" => ("team_lead", "normal"),
            | "low" => ("peer_agent", "low"),
            | _ => return err(&format!("잘못된 심각도: '{}'. 유효한 값: low, medium, high", severity)),
        };

        let escalation_id = format!("ESC-{}", &Uuid::new_v4().to_string().replace("-", "").to_uppercase()[.. 8]);

        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("escalation_id".into(), serde_json::json!(escalation_id));
        m.insert("issue_id".into(), serde_json::json!(issue_id));
        m.insert("severity".into(), serde_json::json!(severity));
        m.insert("reason".into(), serde_json::json!(reason));
        m.insert("current_agent".into(), serde_json::json!(current_agent));
        m.insert("assigned_to".into(), serde_json::json!(assigned_to));
        m.insert("priority".into(), serde_json::json!(priority));
        m
    }
}

fn err(msg: &str) -> HashMap<String, serde_json::Value> {
    let mut m = HashMap::new();
    m.insert("success".into(), serde_json::Value::Bool(false));
    m.insert("error".into(), serde_json::json!(msg));
    m
}
