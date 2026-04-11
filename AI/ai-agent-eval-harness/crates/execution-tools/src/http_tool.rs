// =============================================================================
// @trace SPEC-023
// @trace PRD: PRD-023
// @trace FR: PRD-023/FR-3
// @trace file-type: impl
// =============================================================================
//
// 외부 HTTP endpoint 를 BaseTool 로 노출하는 일반 도구. SPEC-023 에서 도입.
// PpaAgent 는 이 도구를 다른 컴파일된 도구와 동일하게 LLM function calling 에
// 노출하므로, 사용자는 자기 서버를 띄우고 URL 만 등록해 새 행위를 추가할 수
// 있다.

use crate::base::{BaseTool,
                  ToolMetadata};
use serde_json::Value;
use std::{collections::HashMap,
          time::Duration};

pub struct HttpCallTool {
    metadata: ToolMetadata,
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body_template: String,
    timeout_ms: u64,
}

impl HttpCallTool {
    /// 새 인스턴스. `params_schema_json` 은 LLM 에게 그대로 노출될 JSON Schema
    /// 문자열이며, 파싱 실패 시 빈 객체로 폴백.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        method: impl Into<String>,
        url: impl Into<String>,
        headers: HashMap<String, String>,
        body_template: impl Into<String>,
        params_schema_json: &str,
        timeout_ms: u64,
    ) -> Self {
        let parsed_schema: Value = serde_json::from_str(params_schema_json).unwrap_or(Value::Object(serde_json::Map::new()));
        Self {
            metadata: ToolMetadata {
                name: name.into(),
                description: description.into(),
                parameters_schema: parsed_schema,
                safety_level: "external".into(),
                requires_approval: false,
            },
            method: method.into(),
            url: url.into(),
            headers,
            body_template: body_template.into(),
            timeout_ms,
        }
    }

    /// `{{key}}` 자리표시자를 params 값으로 치환. 객체/배열 값은 JSON 직렬화.
    fn render_body(&self, params: &HashMap<String, Value>) -> String {
        let mut out = self.body_template.clone();
        for (k, v) in params {
            let placeholder = format!("{{{{{k}}}}}");
            let rendered = match v {
                | Value::String(s) => s.clone(),
                | Value::Null => "null".into(),
                | other => other.to_string(),
            };
            out = out.replace(&placeholder, &rendered);
        }
        out
    }

    fn run<F: std::future::Future>(fut: F) -> F::Output {
        match tokio::runtime::Handle::try_current() {
            | Ok(handle) => tokio::task::block_in_place(|| handle.block_on(fut)),
            | Err(_) => tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build tokio runtime")
                .block_on(fut),
        }
    }
}

impl BaseTool for HttpCallTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, Value>) -> HashMap<String, Value> {
        let body = self.render_body(params);
        let method = reqwest::Method::from_bytes(self.method.to_uppercase().as_bytes()).unwrap_or(reqwest::Method::POST);
        let mut header_map = reqwest::header::HeaderMap::new();
        for (k, v) in &self.headers {
            if let (Ok(name), Ok(value)) = (reqwest::header::HeaderName::from_bytes(k.as_bytes()), reqwest::header::HeaderValue::from_str(v)) {
                header_map.insert(name, value);
            }
        }
        if !header_map.contains_key(reqwest::header::CONTENT_TYPE) {
            header_map.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
        }

        let url = self.url.clone();
        let timeout = Duration::from_millis(self.timeout_ms);
        let result: Result<(reqwest::StatusCode, String), String> = Self::run(async move {
            let client = reqwest::Client::builder().timeout(timeout).build().map_err(|e| e.to_string())?;
            let resp = client
                .request(method, &url)
                .headers(header_map)
                .body(body)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = resp.status();
            let text = resp.text().await.map_err(|e| e.to_string())?;
            Ok((status, text))
        });

        match result {
            | Ok((status, text)) if status.is_success() => match serde_json::from_str::<Value>(&text) {
                | Ok(Value::Object(obj)) => {
                    let mut out: HashMap<String, Value> = obj.into_iter().collect();
                    out.insert("success".into(), Value::Bool(true));
                    out
                },
                | Ok(other) => {
                    let mut out = HashMap::new();
                    out.insert("success".into(), Value::Bool(true));
                    out.insert("response".into(), other);
                    out
                },
                | Err(_) => {
                    let mut out = HashMap::new();
                    out.insert("success".into(), Value::Bool(true));
                    out.insert("response_text".into(), Value::String(text));
                    out
                },
            },
            | Ok((status, text)) => {
                let mut out = HashMap::new();
                out.insert("success".into(), Value::Bool(false));
                out.insert(
                    "error".into(),
                    Value::String(format!("HTTP {}: {}", status, text.chars().take(500).collect::<String>())),
                );
                out
            },
            | Err(e) => {
                let mut out = HashMap::new();
                out.insert("success".into(), Value::Bool(false));
                out.insert("error".into(), Value::String(format!("request failed: {e}")));
                out
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// @trace TC: SPEC-023/TC-4
    #[test]
    fn render_body_substitutes_string_param() {
        let tool = HttpCallTool::new(
            "search",
            "검색",
            "POST",
            "http://example.com/api",
            HashMap::new(),
            r#"{"q":"{{topic}}","limit":{{n}}}"#,
            "{}",
            5000,
        );
        let mut params = HashMap::new();
        params.insert("topic".to_string(), json!("AI"));
        params.insert("n".to_string(), json!(10));
        let body = tool.render_body(&params);
        assert!(body.contains("\"q\":\"AI\""));
        assert!(body.contains("\"limit\":10"));
    }

    /// @trace TC: SPEC-023/TC-4
    #[test]
    fn render_body_handles_missing_param() {
        let tool = HttpCallTool::new("x", "x", "POST", "http://e.com", HashMap::new(), r#"{"a":"{{a}}","b":"{{b}}"}"#, "{}", 5000);
        let mut params = HashMap::new();
        params.insert("a".into(), json!("hello"));
        // b 는 미제공 → placeholder 그대로 남음
        let body = tool.render_body(&params);
        assert!(body.contains("\"a\":\"hello\""));
        assert!(body.contains("{{b}}"));
    }

    #[test]
    fn metadata_uses_provided_name_and_schema() {
        let schema = r#"{"type":"object","required":["topic"],"properties":{"topic":{"type":"string"}}}"#;
        let tool = HttpCallTool::new(
            "healthcare__search_patient",
            "환자 검색",
            "POST",
            "http://localhost:9000/q",
            HashMap::new(),
            "{}",
            schema,
            5000,
        );
        let m = tool.metadata();
        assert_eq!(m.name, "healthcare__search_patient");
        assert_eq!(m.safety_level, "external");
        assert_eq!(m.parameters_schema["properties"]["topic"]["type"], "string");
    }
}
