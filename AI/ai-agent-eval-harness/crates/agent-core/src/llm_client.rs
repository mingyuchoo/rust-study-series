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
/// 필수 슬롯: `{task_description}`, `{environment_state}`. 선택 슬롯:
/// `{context}`.
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
/// 필수 슬롯: `{task_description}`, `{perceived_info}`, `{tools}`. 선택:
/// `{context}`.
pub const BOOTSTRAP_POLICY_USER: &str = "작업: {task_description}\n\n인지된 정보:\n{perceived_info}\n\n사용 가능한 도구:\n{tools}{context}\n\n\
     위 도구 중 적절한 것을 선택하고, 환경 정보에서 파라미터 값을 추출하여 JSON 형식으로 출력하세요.\n\
     모든 필요한 도구 호출이 완료되면 \"task_completed\": true로 설정하세요.";

/// 기동 시점 bootstrap 주입용 번들.
/// 호출자(`eval-harness` 기동)가 `store.seed_bootstrap_prompt_sets` 에
/// 복사/참조해 넘긴다.
#[derive(Debug, Clone, Copy)]
pub struct BootstrapBundle {
    pub perceive_system: &'static str,
    pub perceive_user: &'static str,
    pub policy_system: &'static str,
    pub policy_user: &'static str,
}

pub const BOOTSTRAP_BUNDLE: BootstrapBundle = BootstrapBundle {
    perceive_system: BOOTSTRAP_PERCEIVE_SYSTEM,
    perceive_user: BOOTSTRAP_PERCEIVE_USER,
    policy_system: BOOTSTRAP_POLICY_SYSTEM,
    policy_user: BOOTSTRAP_POLICY_USER,
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

/// 런타임 해석 결과. 활성 PromptSet 을 DB 에서 조회해 담거나, 조회 실패 시
/// bootstrap 으로 채워진다. `id` 는 조회된 DB 행의 식별자이며 bootstrap
/// 폴백의 경우 `None`. 호출자는 첫 Perceive 시점에 1회 해석해 `Trajectory`
/// 에 기록한다.
///
/// @trace SPEC: SPEC-025
/// @trace FR: PRD-025/FR-3, PRD-025/FR-8
#[derive(Debug, Clone)]
pub struct ResolvedPromptSet {
    pub id: Option<i64>,
    pub perceive_system: String,
    pub perceive_user: String,
    pub policy_system: String,
    pub policy_user: String,
}

impl ResolvedPromptSet {
    pub fn bootstrap() -> Self {
        Self {
            id: None,
            perceive_system: BOOTSTRAP_PERCEIVE_SYSTEM.to_string(),
            perceive_user: BOOTSTRAP_PERCEIVE_USER.to_string(),
            policy_system: BOOTSTRAP_POLICY_SYSTEM.to_string(),
            policy_user: BOOTSTRAP_POLICY_USER.to_string(),
        }
    }
}

impl From<data_scenarios::sqlite_store::PromptSetRow> for ResolvedPromptSet {
    fn from(row: data_scenarios::sqlite_store::PromptSetRow) -> Self {
        Self {
            id: Some(row.id),
            perceive_system: row.perceive_system,
            perceive_user: row.perceive_user,
            policy_system: row.policy_system,
            policy_user: row.policy_user,
        }
    }
}

/// 도메인 → `general` → bootstrap 순서로 PromptSet 을 해석.
/// 호출자가 `block_on_future` 같은 동기 컨텍스트에서 await 할 수 있도록 async.
///
/// @trace SPEC: SPEC-025
/// @trace FR: PRD-025/FR-3
pub async fn resolve_prompt_set(store: Option<&data_scenarios::sqlite_store::SqliteStore>, domain: &str) -> ResolvedPromptSet {
    let Some(s) = store else {
        return ResolvedPromptSet::bootstrap();
    };
    if let Ok(Some(row)) = s.get_active_prompt_set(domain).await {
        return row.into();
    }
    if domain != "general" {
        if let Ok(Some(row)) = s.get_active_prompt_set("general").await {
            return row.into();
        }
    }
    ResolvedPromptSet::bootstrap()
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

    /// Perceive 프롬프트 생성. 호출자가 사전에 `resolve_prompt_set(...).await`
    /// 으로 해석한 `bundle` 을 전달한다. `domain` 은 `{domain_name}` 슬롯
    /// 치환에 쓰인다. 반환되는 `Option<i64>` 는 `bundle.id` 와 동일하며
    /// 호출자가 Trajectory 에 기록한다.
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-3, PRD-025/FR-4
    pub fn create_perceive_prompt(
        bundle: &ResolvedPromptSet,
        domain: &str,
        task: &str,
        environment_state: &HashMap<String, serde_json::Value>,
        context: Option<&HashMap<String, serde_json::Value>>,
    ) -> (Vec<Message>, Option<i64>) {
        let ctx_str = context.map(|c| format!("\n이전 맥락: {:?}", c)).unwrap_or_default();
        let env_str = format!("{:?}", environment_state);

        let mut vars: HashMap<&str, String> = HashMap::new();
        vars.insert("domain_name", domain.to_string());
        vars.insert("task_description", task.to_string());
        vars.insert("environment_state", env_str);
        vars.insert("context", ctx_str);

        let system = Message::system(render_template(&bundle.perceive_system, &vars));
        let user = Message::user(render_template(&bundle.perceive_user, &vars));
        (vec![system, user], bundle.id)
    }

    /// Policy 프롬프트 생성. 도구 메타데이터는 이전과 동일한 규칙으로
    /// 사람 가독 문자열로 직렬화된 뒤 `{tools}` 슬롯에 주입된다.
    ///
    /// @trace SPEC: SPEC-025
    /// @trace FR: PRD-025/FR-3, PRD-025/FR-4
    pub fn create_policy_prompt(
        bundle: &ResolvedPromptSet,
        domain: &str,
        task: &str,
        perceived_info: &HashMap<String, serde_json::Value>,
        tools_metadata: &[serde_json::Value],
        context: Option<&HashMap<String, serde_json::Value>>,
    ) -> (Vec<Message>, Option<i64>) {
        let tools_str = format_tools_block(tools_metadata);
        let ctx_str = context.map(|c| format!("\n이전 맥락: {:?}", c)).unwrap_or_default();
        let perc_str = format!("{:?}", perceived_info);

        let mut vars: HashMap<&str, String> = HashMap::new();
        vars.insert("domain_name", domain.to_string());
        vars.insert("task_description", task.to_string());
        vars.insert("perceived_info", perc_str);
        vars.insert("tools", tools_str);
        vars.insert("context", ctx_str);

        let system = Message::system(render_template(&bundle.policy_system, &vars));
        let user = Message::user(render_template(&bundle.policy_user, &vars));
        (vec![system, user], bundle.id)
    }
}

/// 도구 메타데이터 배열을 Policy 프롬프트의 `{tools}` 슬롯에 주입할
/// 사람 가독 형식으로 조립. 기존 하드코딩 로직과 1:1 동등.
fn format_tools_block(tools_metadata: &[serde_json::Value]) -> String {
    let mut parts = Vec::new();
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
        parts.push(format!("  * [domain={}] {}: {}\n{}", domain, name, desc, params_desc.join("\n")));
    }
    parts.join("\n")
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

    // -------- L3: resolution + render (TC-11 / TC-12 / TC-13) --------

    use data_scenarios::sqlite_store::{PromptSetInsert,
                                       SqliteStore};

    fn v1_bundle_ref() -> data_scenarios::sqlite_store::BootstrapBundleRef<'static> {
        data_scenarios::sqlite_store::BootstrapBundleRef {
            perceive_system: BOOTSTRAP_PERCEIVE_SYSTEM,
            perceive_user: BOOTSTRAP_PERCEIVE_USER,
            policy_system: BOOTSTRAP_POLICY_SYSTEM,
            policy_user: BOOTSTRAP_POLICY_USER,
        }
    }

    /// @trace TC: SPEC-025/TC-11
    /// @trace FR: PRD-025/FR-3
    #[tokio::test]
    async fn spec025_tc_11_create_perceive_prompt_uses_active_version() {
        let store = SqliteStore::open_in_memory_for_loader().await.unwrap();
        store.insert_domain("customer_service", "").await.unwrap();
        let b = v1_bundle_ref();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        // v2 삽입(비활성), 그 다음 활성화
        let v2 = store
            .insert_prompt_set(PromptSetInsert {
                domain_name: "customer_service",
                perceive_system: "XSYS {domain_name}",
                perceive_user: "작업: {task_description} / env: {environment_state}{context}",
                policy_system: "POLX",
                policy_user: "작업: {task_description} / perc: {perceived_info} / tools: {tools}{context}",
                notes: None,
                is_bootstrap: false,
            })
            .await
            .unwrap();
        store.activate_prompt_set("customer_service", v2.version).await.unwrap();

        let bundle = resolve_prompt_set(Some(&store), "customer_service").await;
        assert_eq!(bundle.id, Some(v2.id));
        let env = HashMap::from([("k".to_string(), serde_json::json!(1))]);
        let (msgs, id) = LlmClient::create_perceive_prompt(&bundle, "customer_service", "환불 처리", &env, None);
        assert_eq!(id, Some(v2.id));
        assert_eq!(msgs.len(), 2);
        // system 에 {domain_name} 치환
        assert_eq!(msgs[0].content, "XSYS customer_service");
        // user 에 작업/환경 치환
        assert!(msgs[1].content.contains("작업: 환불 처리"));
        assert!(msgs[1].content.contains("env: "));
    }

    /// @trace TC: SPEC-025/TC-12
    /// @trace FR: PRD-025/FR-3, PRD-025/FR-4
    #[tokio::test]
    async fn spec025_tc_12_create_policy_prompt_renders_tools_slot() {
        let store = SqliteStore::open_in_memory_for_loader().await.unwrap();
        store.insert_domain("financial", "").await.unwrap();
        let b = v1_bundle_ref();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();

        let bundle = resolve_prompt_set(Some(&store), "financial").await;
        assert!(bundle.id.is_some(), "bootstrap 활성 버전 조회 성공");

        let tools = vec![serde_json::json!({
            "name": "financial__calc",
            "description": "단리 계산",
            "domain": "financial",
            "parameters_schema": {
                "type": "object",
                "properties": {"amount": {"type": "number", "description": "금액"}},
                "required": ["amount"]
            }
        })];
        let perc = HashMap::from([("x".to_string(), serde_json::json!(1))]);
        let (msgs, _id) = LlmClient::create_policy_prompt(&bundle, "financial", "이자 계산", &perc, &tools, None);
        // user 메시지에 tools 블록이 조립돼 삽입되었는지
        let user = &msgs[1].content;
        assert!(user.contains("[domain=financial] financial__calc"));
        assert!(user.contains("amount"));
        assert!(user.contains("작업: 이자 계산"));
    }

    /// @trace TC: SPEC-025/TC-13
    /// @trace FR: PRD-025/FR-3
    #[tokio::test]
    async fn spec025_tc_13_fallback_chain_general_then_bootstrap() {
        let store = SqliteStore::open_in_memory_for_loader().await.unwrap();
        // 어떤 도메인도 없음 → bootstrap 폴백
        let bundle1 = resolve_prompt_set(Some(&store), "xxx").await;
        assert_eq!(bundle1.id, None);
        assert_eq!(bundle1.perceive_system, BOOTSTRAP_PERCEIVE_SYSTEM);

        // general 도메인에만 활성 PromptSet 을 두고 요청 도메인은 xxx
        store.insert_domain("general", "").await.unwrap();
        let b = v1_bundle_ref();
        store.seed_bootstrap_prompt_sets(&b).await.unwrap();
        let bundle2 = resolve_prompt_set(Some(&store), "xxx").await;
        assert!(bundle2.id.is_some(), "general 로 폴백된 PromptSet 사용");

        // 요청 도메인에도 PromptSet 을 두면 그게 우선
        store.insert_domain("xxx", "").await.unwrap();
        let b2 = data_scenarios::sqlite_store::BootstrapBundleRef {
            perceive_system: "X-ONLY SYS",
            perceive_user: "{task_description} {environment_state}",
            policy_system: "X-ONLY POL",
            policy_user: "{task_description} {perceived_info} {tools}",
        };
        store.seed_bootstrap_prompt_sets(&b2).await.unwrap();
        let bundle3 = resolve_prompt_set(Some(&store), "xxx").await;
        assert_eq!(bundle3.perceive_system, "X-ONLY SYS");
    }

    /// store=None 이면 항상 bootstrap.
    #[tokio::test]
    async fn spec025_resolve_without_store_is_bootstrap() {
        let bundle = resolve_prompt_set(None, "customer_service").await;
        assert!(bundle.id.is_none());
        assert_eq!(bundle.perceive_system, BOOTSTRAP_PERCEIVE_SYSTEM);
    }
}
