use crate::config::AzureOpenAiConfig;
use anyhow::Result;
use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

/// Azure OpenAI LLM 클라이언트
pub struct LlmClient {
    config: AzureOpenAiConfig,
    http: reqwest::Client,
}

impl LlmClient {
    pub fn new(config: AzureOpenAiConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    pub async fn invoke(&self, messages: Vec<Message>) -> Result<String> {
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.config.azure_openai_endpoint.trim_end_matches('/'),
            self.config.azure_openai_deployment,
            self.config.azure_openai_api_version,
        );

        // gpt-5.x 계열 배포는 temperature가 기본값(1.0)일 때만 허용되므로, 기본값이면
        // 필드 자체를 생략한다.
        let temperature = if (self.config.temperature - 1.0).abs() < f64::EPSILON {
            None
        } else {
            Some(self.config.temperature)
        };

        let req = ChatRequest {
            messages,
            temperature,
            max_completion_tokens: self.config.max_tokens,
        };

        let resp = self
            .http
            .post(&url)
            .header("api-key", &self.config.azure_openai_api_key)
            .json(&req)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Azure OpenAI {} at {}: {}", status, url, body);
        }

        let parsed: ChatResponse = resp.json().await?;
        Ok(parsed.choices.into_iter().next().map(|c| c.message.content).unwrap_or_default())
    }

    /// LLM 응답에서 JSON 파싱
    pub fn parse_json_response(text: &str) -> HashMap<String, serde_json::Value> {
        // ```json ... ``` 블록 추출 시도
        let json_text = if let Some(start) = text.find("```json") {
            let s = &text[start + 7 ..];
            if let Some(end) = s.find("```") { s[.. end].trim() } else { text.trim() }
        } else if let Some(start) = text.find("```") {
            let s = &text[start + 3 ..];
            if let Some(end) = s.find("```") { s[.. end].trim() } else { text.trim() }
        } else {
            text.trim()
        };

        serde_json::from_str(json_text).unwrap_or_else(|_| {
            let mut m = HashMap::new();
            m.insert("raw_response".into(), serde_json::Value::String(text.to_string()));
            m
        })
    }

    pub fn create_perceive_prompt(
        task: &str,
        environment_state: &HashMap<String, serde_json::Value>,
        context: Option<&HashMap<String, serde_json::Value>>,
    ) -> Vec<Message> {
        let system = Message::system(
            "당신은 AI Agent의 Perceive(인지) 단계를 담당합니다.\n\
             주어진 환경 상태를 분석하고, 작업 수행에 필요한 핵심 정보를 추출하세요.\n\n\
             출력 형식:\n\
             - perceived_facts: 환경에서 인지한 사실들\n\
             - missing_info: 부족한 정보\n\
             - anomalies: 발견된 이상 징후\n\
             - context_summary: 맥락 요약",
        );

        let ctx_str = context.map(|c| format!("\n이전 맥락: {:?}", c)).unwrap_or_default();
        let human = Message::user(format!(
            "작업: {}\n\n환경 상태:\n{:?}{}\n\n위 정보를 분석하여 JSON 형식으로 출력하세요.",
            task, environment_state, ctx_str
        ));

        vec![system, human]
    }

    pub fn create_policy_prompt(
        task: &str,
        perceived_info: &HashMap<String, serde_json::Value>,
        tools_metadata: &[serde_json::Value],
        context: Option<&HashMap<String, serde_json::Value>>,
    ) -> Vec<Message> {
        let system = Message::system(
            "당신은 AI Agent의 Policy(판단) 단계를 담당합니다.\n\
             인지된 정보를 바탕으로 다음 행동을 계획하세요.\n\n\
             **중요 규칙**:\n\
             1. 반드시 사용 가능한 도구 중에서 선택하세요.\n\
             2. 주어진 환경 정보를 도구의 파라미터로 직접 활용하세요.\n\
             3. 도구를 호출할 때는 required 파라미터를 모두 포함하세요.\n\
             4. 한 번에 하나의 도구만 선택하세요.\n\
             5. 모든 필요한 도구를 순차적으로 실행한 후 작업이 완료되면 \"task_completed\": true를 설정하세요.\n\n\
             출력 형식 (JSON):\n\
             - reasoning: 판단 근거\n\
             - selected_tool: 선택한 도구 이름 (작업 완료 시 null)\n\
             - tool_parameters: 도구에 전달할 파라미터\n\
             - confidence: 확신도 (0.0-1.0)\n\
             - requires_human_approval: 인간 승인 필요 여부\n\
             - task_completed: 작업 완료 여부 (true/false)\n\
             - next_step: 다음 단계 설명",
        );

        let mut tools_info_parts = Vec::new();
        for meta in tools_metadata {
            let name = meta.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let desc = meta.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let schema = meta.get("parameters_schema").cloned().unwrap_or_default();
            let empty_map = serde_json::Map::new();
            let props = schema.get("properties").and_then(|v| v.as_object()).unwrap_or(&empty_map);
            let required: Vec<&str> = schema
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();

            let mut params_desc = Vec::new();
            for (pname, pinfo) in props {
                let req = if required.contains(&pname.as_str()) { "(필수)" } else { "(선택)" };
                let ptype = pinfo.get("type").and_then(|v| v.as_str()).unwrap_or("any");
                let pdesc = pinfo.get("description").and_then(|v| v.as_str()).unwrap_or("");
                params_desc.push(format!("    - {} ({}) {}: {}", pname, ptype, req, pdesc));
            }
            tools_info_parts.push(format!("  * {}: {}\n{}", name, desc, params_desc.join("\n")));
        }

        let ctx_str = context.map(|c| format!("\n이전 맥락: {:?}", c)).unwrap_or_default();
        let human = Message::user(format!(
            "작업: {}\n\n인지된 정보:\n{:?}\n\n사용 가능한 도구:\n{}{}\n\n\
             위 도구 중 적절한 것을 선택하고, 환경 정보에서 파라미터 값을 추출하여 JSON 형식으로 출력하세요.\n\
             모든 필요한 도구 호출이 완료되면 \"task_completed\": true로 설정하세요.",
            task,
            perceived_info,
            tools_info_parts.join("\n"),
            ctx_str
        ));

        vec![system, human]
    }
}
