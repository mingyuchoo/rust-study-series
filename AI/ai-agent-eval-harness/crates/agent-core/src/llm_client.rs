use crate::config::AzureOpenAiConfig;
use anyhow::Result;
use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

// =============================================================================
// SPEC-025: 도메인별 PromptSet bootstrap 상수 + 슬롯 렌더러 + 검증
//
// 기존 `create_perceive_prompt` / `create_policy_prompt` 의 하드코딩 문구를
// 상수로 분리하여 `data-scenarios::seed_bootstrap_prompt_sets` 가 v1 시드로
// 그대로 주입할 수 있게 한다. user 프롬프트는 고정 슬롯으로 이관되어 DB 의
// PromptSet 행이 런타임에 `str::replace` 로 치환된다.
//
// @trace SPEC: SPEC-025
// @trace FR: PRD-025/FR-2, PRD-025/FR-4, PRD-025/FR-5
// =============================================================================

/// Perceive 스테이지 system 프롬프트 (v1 bootstrap).
/// 기존 하드코딩 문구와 1:1 동등.
pub const BOOTSTRAP_PERCEIVE_SYSTEM: &str = "당신은 AI Agent의 Perceive(인지) 단계를 담당합니다.\n\
     주어진 환경 상태를 분석하고, 작업 수행에 필요한 핵심 정보를 추출하세요.\n\n\
     출력 형식:\n\
     - perceived_facts: 환경에서 인지한 사실들\n\
     - missing_info: 부족한 정보\n\
     - anomalies: 발견된 이상 징후\n\
     - context_summary: 맥락 요약";

/// Perceive 스테이지 user 프롬프트 템플릿.
/// 필수 슬롯: `{task_description}`, `{environment_state}`. 선택 슬롯: `{context}`.
pub const BOOTSTRAP_PERCEIVE_USER: &str =
    "작업: {task_description}\n\n환경 상태:\n{environment_state}{context}\n\n위 정보를 분석하여 JSON 형식으로 출력하세요.";

/// Policy 스테이지 system 프롬프트 (v1 bootstrap).
/// 기존 하드코딩 문구와 1:1 동등.
pub const BOOTSTRAP_POLICY_SYSTEM: &str = "당신은 AI Agent의 Policy(판단) 단계를 담당합니다.\n\
     인지된 정보를 바탕으로 다음 행동을 계획하세요.\n\n\
     **중요 규칙**:\n\
     1. 반드시 사용 가능한 도구 중에서 선택하세요. 도구 이름은 반드시 `<domain>__<tool>` 네임스페이스 형식 그대로 사용하세요 (예: `financial__calculate_simple_interest`). `general` 도메인의 기본 파일 도구(read_file, write_file, list_directory)는 네임스페이스 없이 이름 그대로 사용합니다.\n\
     2. 각 도구에는 `[domain=...]` 라벨이 붙어 있습니다. task 의 성격과 일치하는 도메인의 도구를 우선 고려하세요. 여러 도메인이 필요한 task 도 가능합니다.\n\
     3. 주어진 환경 정보를 도구의 파라미터로 직접 활용하세요.\n\
     4. 도구를 호출할 때는 required 파라미터를 모두 포함하세요.\n\
     5. 한 번에 하나의 도구만 선택하세요.\n\
     6. 모든 필요한 도구를 순차적으로 실행한 후 작업이 완료되면 \"task_completed\": true를 설정하세요.\n\n\
     출력 형식 (JSON):\n\
     - reasoning: 판단 근거 (어떤 도메인의 도구를 왜 선택했는지 포함)\n\
     - selected_tool: 선택한 도구의 네임스페이스 포함 이름 (작업 완료 시 null)\n\
     - tool_parameters: 도구에 전달할 파라미터\n\
     - confidence: 확신도 (0.0-1.0)\n\
     - requires_human_approval: 인간 승인 필요 여부\n\
     - task_completed: 작업 완료 여부 (true/false)\n\
     - next_step: 다음 단계 설명";

/// Policy 스테이지 user 프롬프트 템플릿.
/// 필수 슬롯: `{task_description}`, `{perceived_info}`, `{tools}`. 선택: `{context}`.
pub const BOOTSTRAP_POLICY_USER: &str =
    "작업: {task_description}\n\n인지된 정보:\n{perceived_info}\n\n사용 가능한 도구:\n{tools}{context}\n\n\
     위 도구 중 적절한 것을 선택하고, 환경 정보에서 파라미터 값을 추출하여 JSON 형식으로 출력하세요.\n\
     모든 필요한 도구 호출이 완료되면 \"task_completed\": true로 설정하세요.";

/// 기동 시점 bootstrap 주입용 번들.
/// 호출자(`eval-harness` 기동)가 `store.seed_bootstrap_prompt_sets` 에 복사/참조해 넘긴다.
#[derive(Debug, Clone, Copy)]
pub struct BootstrapBundle {
    pub perceive_system: &'static str,
    pub perceive_user:   &'static str,
    pub policy_system:   &'static str,
    pub policy_user:     &'static str,
}

pub const BOOTSTRAP_BUNDLE: BootstrapBundle = BootstrapBundle {
    perceive_system: BOOTSTRAP_PERCEIVE_SYSTEM,
    perceive_user:   BOOTSTRAP_PERCEIVE_USER,
    policy_system:   BOOTSTRAP_POLICY_SYSTEM,
    policy_user:     BOOTSTRAP_POLICY_USER,
};

/// 단순 슬롯 렌더러. 동일 키는 한 번만 치환한다. 외부 엔진 없음.
///
/// @trace SPEC: SPEC-025
/// @trace FR: PRD-025/FR-4
pub fn render_template(tmpl: &str, vars: &HashMap<&str, String>) -> String {
    let mut out = tmpl.to_string();
    for (k, v) in vars {
        out = out.replace(&format!("{{{}}}", k), v);
    }
    out
}

/// 저장 시 검증용. 지정 필드가 필수 슬롯을 모두 포함하는지 확인하고,
/// 누락된 슬롯을 반환한다. system 필드는 필수 슬롯이 없어 항상 빈 벡터.
///
/// @trace SPEC: SPEC-025
/// @trace FR: PRD-025/FR-5
pub fn validate_required_slots(field: &str, tmpl: &str) -> Vec<String> {
    let required: &[&str] = match field {
        | "perceive_user" => &["{task_description}", "{environment_state}"],
        | "policy_user" => &["{task_description}", "{perceived_info}", "{tools}"],
        | _ => return Vec::new(),
    };
    required.iter().filter(|slot| !tmpl.contains(*slot)).map(|s| (*s).to_string()).collect()
}

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
             1. 반드시 사용 가능한 도구 중에서 선택하세요. 도구 이름은 반드시 `<domain>__<tool>` 네임스페이스 형식 그대로 사용하세요 (예: `financial__calculate_simple_interest`). `general` 도메인의 기본 파일 도구(read_file, write_file, list_directory)는 네임스페이스 없이 이름 그대로 사용합니다.\n\
             2. 각 도구에는 `[domain=...]` 라벨이 붙어 있습니다. task 의 성격과 일치하는 도메인의 도구를 우선 고려하세요. 여러 도메인이 필요한 task 도 가능합니다.\n\
             3. 주어진 환경 정보를 도구의 파라미터로 직접 활용하세요.\n\
             4. 도구를 호출할 때는 required 파라미터를 모두 포함하세요.\n\
             5. 한 번에 하나의 도구만 선택하세요.\n\
             6. 모든 필요한 도구를 순차적으로 실행한 후 작업이 완료되면 \"task_completed\": true를 설정하세요.\n\n\
             출력 형식 (JSON):\n\
             - reasoning: 판단 근거 (어떤 도메인의 도구를 왜 선택했는지 포함)\n\
             - selected_tool: 선택한 도구의 네임스페이스 포함 이름 (작업 완료 시 null)\n\
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
            let domain = meta.get("domain").and_then(|v| v.as_str()).unwrap_or("general");
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
            tools_info_parts.push(format!("  * [domain={}] {}: {}\n{}", domain, name, desc, params_desc.join("\n")));
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

#[cfg(test)]
mod spec025_tests {
    use super::*;

    /// @trace TC: SPEC-025/TC-4
    /// @trace FR: PRD-025/FR-4
    #[test]
    fn spec025_tc_4_render_template_replaces_all_slots() {
        let mut vars: HashMap<&str, String> = HashMap::new();
        vars.insert("task_description", "환불".into());
        vars.insert("environment_state", "{k:1}".into());
        vars.insert("context", "".into());
        let out = render_template("작업: {task_description}\n환경: {environment_state}{context}", &vars);
        assert_eq!(out, "작업: 환불\n환경: {k:1}");
    }

    #[test]
    fn spec025_render_unknown_slot_preserved() {
        let vars: HashMap<&str, String> = HashMap::new();
        let out = render_template("literal {unknown} braces", &vars);
        assert_eq!(out, "literal {unknown} braces");
    }

    /// @trace TC: SPEC-025/TC-5
    /// @trace FR: PRD-025/FR-5
    #[test]
    fn spec025_tc_5_validate_required_slots_missing_tools() {
        let bad = "작업: {task_description}\n인지: {perceived_info}\n도구 없음!";
        let missing = validate_required_slots("policy_user", bad);
        assert_eq!(missing, vec!["{tools}".to_string()]);
    }

    #[test]
    fn spec025_validate_perceive_user_all_present_ok() {
        let missing = validate_required_slots("perceive_user", BOOTSTRAP_PERCEIVE_USER);
        assert!(missing.is_empty());
    }

    #[test]
    fn spec025_validate_system_has_no_required_slots() {
        let missing = validate_required_slots("perceive_system", "anything");
        assert!(missing.is_empty());
        let missing2 = validate_required_slots("policy_system", "");
        assert!(missing2.is_empty());
    }

    /// @trace FR: PRD-025/FR-2
    #[test]
    fn spec025_bootstrap_bundle_is_non_empty() {
        assert!(!BOOTSTRAP_BUNDLE.perceive_system.is_empty());
        assert!(!BOOTSTRAP_BUNDLE.perceive_user.is_empty());
        assert!(!BOOTSTRAP_BUNDLE.policy_system.is_empty());
        assert!(!BOOTSTRAP_BUNDLE.policy_user.is_empty());
        // 필수 슬롯이 bootstrap 에 존재해야 함 (자기 검증)
        assert!(validate_required_slots("perceive_user", BOOTSTRAP_PERCEIVE_USER).is_empty());
        assert!(validate_required_slots("policy_user", BOOTSTRAP_POLICY_USER).is_empty());
    }
}
