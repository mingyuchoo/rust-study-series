#![allow(dead_code, clippy::new_without_default)]
use execution_tools::base::{BaseTool,
                            ToolMetadata};
use std::collections::HashMap;

pub struct SimpleInterestCalculatorTool {
    metadata: ToolMetadata,
}

impl SimpleInterestCalculatorTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "calculate_simple_interest".into(),
                description: "단리 이자를 계산합니다 (I = P × r × t)".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "principal": {"type": "number", "description": "원금"},
                        "rate": {"type": "number", "description": "연이율 (예: 0.05 = 5%)"},
                        "time": {"type": "number", "description": "기간 (년)"}
                    },
                    "required": ["principal", "rate", "time"]
                }),
                safety_level: "safe".into(),
                requires_approval: false,
            },
        }
    }
}

impl BaseTool for SimpleInterestCalculatorTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let principal = match params.get("principal").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("principal 파라미터 필요"),
        };
        let rate = match params.get("rate").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("rate 파라미터 필요"),
        };
        let time = match params.get("time").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("time 파라미터 필요"),
        };

        let interest = (principal * rate * time * 100.0).round() / 100.0;
        let total_amount = ((principal + interest) * 100.0).round() / 100.0;

        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("principal".into(), serde_json::json!(principal));
        m.insert("rate".into(), serde_json::json!(rate));
        m.insert("time".into(), serde_json::json!(time));
        m.insert("interest".into(), serde_json::json!(interest));
        m.insert("total_amount".into(), serde_json::json!(total_amount));
        m.insert("calculation_type".into(), serde_json::json!("simple_interest"));
        m
    }
}

pub struct CompoundInterestCalculatorTool {
    metadata: ToolMetadata,
}

impl CompoundInterestCalculatorTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "calculate_compound_interest".into(),
                description: "복리 이자를 계산합니다 (A = P(1 + r/n)^(nt))".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "principal": {"type": "number", "description": "원금"},
                        "rate": {"type": "number", "description": "연이율 (예: 0.05 = 5%)"},
                        "time": {"type": "number", "description": "기간 (년)"},
                        "compounds_per_year": {"type": "integer", "description": "연간 복리 횟수", "default": 12}
                    },
                    "required": ["principal", "rate", "time"]
                }),
                safety_level: "safe".into(),
                requires_approval: false,
            },
        }
    }
}

impl BaseTool for CompoundInterestCalculatorTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let principal = match params.get("principal").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("principal 파라미터 필요"),
        };
        let rate = match params.get("rate").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("rate 파라미터 필요"),
        };
        let time = match params.get("time").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("time 파라미터 필요"),
        };
        let n = params.get("compounds_per_year").and_then(|v| v.as_f64()).unwrap_or(12.0);

        let total_amount = principal * (1.0 + rate / n).powf(n * time);
        let total_amount = (total_amount * 100.0).round() / 100.0;
        let interest = ((total_amount - principal) * 100.0).round() / 100.0;

        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("principal".into(), serde_json::json!(principal));
        m.insert("rate".into(), serde_json::json!(rate));
        m.insert("time".into(), serde_json::json!(time));
        m.insert("compounds_per_year".into(), serde_json::json!(n as i64));
        m.insert("interest".into(), serde_json::json!(interest));
        m.insert("total_amount".into(), serde_json::json!(total_amount));
        m.insert("calculation_type".into(), serde_json::json!("compound_interest"));
        m
    }
}

pub struct TransactionValidatorTool {
    metadata: ToolMetadata,
}

impl TransactionValidatorTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "validate_transaction".into(),
                description: "금융 트랜잭션의 유효성을 검증합니다".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "amount": {"type": "number", "description": "거래 금액"},
                        "account_balance": {"type": "number", "description": "계좌 잔액"},
                        "transaction_type": {"type": "string", "description": "거래 유형 (withdraw, deposit, transfer)"}
                    },
                    "required": ["amount", "account_balance", "transaction_type"]
                }),
                safety_level: "safe".into(),
                requires_approval: false,
            },
        }
    }
}

impl BaseTool for TransactionValidatorTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let amount = match params.get("amount").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("amount 파라미터 필요"),
        };
        let balance = match params.get("account_balance").and_then(|v| v.as_f64()) {
            | Some(v) => v,
            | None => return err("account_balance 파라미터 필요"),
        };
        let tx_type = match params.get("transaction_type").and_then(|v| v.as_str()) {
            | Some(v) => v.to_string(),
            | None => return err("transaction_type 파라미터 필요"),
        };

        let mut errors: Vec<String> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();

        if amount <= 0.0 {
            errors.push("거래 금액은 0보다 커야 합니다".into());
        }

        if tx_type == "withdraw" || tx_type == "transfer" {
            if amount > balance {
                errors.push(format!("잔액 부족: 현재 {}, 요청 {}", balance, amount));
            } else if amount > balance * 0.9 {
                warnings.push("거래 금액이 잔액의 90%를 초과합니다".into());
            }
        }

        if amount > 1_000_000.0 {
            warnings.push("대액 거래입니다. 추가 확인이 필요할 수 있습니다".into());
        }

        let is_valid = errors.is_empty();
        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("is_valid".into(), serde_json::Value::Bool(is_valid));
        m.insert("amount".into(), serde_json::json!(amount));
        m.insert("account_balance".into(), serde_json::json!(balance));
        m.insert("transaction_type".into(), serde_json::json!(tx_type));
        m.insert("errors".into(), serde_json::json!(errors));
        m.insert("warnings".into(), serde_json::json!(warnings));
        m
    }
}

fn err(msg: &str) -> HashMap<String, serde_json::Value> {
    let mut m = HashMap::new();
    m.insert("success".into(), serde_json::Value::Bool(false));
    m.insert("error".into(), serde_json::json!(msg));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(pairs: &[(&str, serde_json::Value)]) -> HashMap<String, serde_json::Value> { pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect() }

    #[test]
    fn simple_interest_basic() {
        let tool = SimpleInterestCalculatorTool::new();
        let p = params(&[
            ("principal", serde_json::json!(1000.0)),
            ("rate", serde_json::json!(0.05)),
            ("time", serde_json::json!(2.0)),
        ]);
        let result = tool.execute(&p);
        assert_eq!(result["success"], serde_json::Value::Bool(true));
        assert_eq!(result["interest"], serde_json::json!(100.0));
        assert_eq!(result["total_amount"], serde_json::json!(1100.0));
    }

    #[test]
    fn simple_interest_missing_param() {
        let tool = SimpleInterestCalculatorTool::new();
        let p = params(&[("principal", serde_json::json!(1000.0))]);
        let result = tool.execute(&p);
        assert_eq!(result["success"], serde_json::Value::Bool(false));
    }

    #[test]
    fn compound_interest_basic() {
        let tool = CompoundInterestCalculatorTool::new();
        let p = params(&[
            ("principal", serde_json::json!(1000.0)),
            ("rate", serde_json::json!(0.12)),
            ("time", serde_json::json!(1.0)),
            ("compounds_per_year", serde_json::json!(12)),
        ]);
        let result = tool.execute(&p);
        assert_eq!(result["success"], serde_json::Value::Bool(true));
        let total = result["total_amount"].as_f64().unwrap();
        assert!(total > 1000.0);
    }

    #[test]
    fn transaction_validator_valid_withdraw() {
        let tool = TransactionValidatorTool::new();
        let p = params(&[
            ("amount", serde_json::json!(100.0)),
            ("account_balance", serde_json::json!(500.0)),
            ("transaction_type", serde_json::json!("withdraw")),
        ]);
        let result = tool.execute(&p);
        assert_eq!(result["success"], serde_json::Value::Bool(true));
        assert_eq!(result["is_valid"], serde_json::Value::Bool(true));
        assert!(result["errors"].as_array().unwrap().is_empty());
    }

    #[test]
    fn transaction_validator_insufficient_funds() {
        let tool = TransactionValidatorTool::new();
        let p = params(&[
            ("amount", serde_json::json!(600.0)),
            ("account_balance", serde_json::json!(500.0)),
            ("transaction_type", serde_json::json!("withdraw")),
        ]);
        let result = tool.execute(&p);
        assert_eq!(result["is_valid"], serde_json::Value::Bool(false));
        assert!(!result["errors"].as_array().unwrap().is_empty());
    }
}
