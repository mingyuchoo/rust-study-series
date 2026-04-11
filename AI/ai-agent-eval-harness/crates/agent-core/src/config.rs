use serde::{Deserialize,
            Serialize};

/// Azure OpenAI 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureOpenAiConfig {
    pub azure_openai_endpoint: String,
    pub azure_openai_api_key: String,
    pub azure_openai_deployment: String,
    pub azure_openai_api_version: String,
    pub azure_openai_region: String,
    pub temperature: f64,
    pub max_tokens: Option<u32>,
}

impl AzureOpenAiConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            azure_openai_endpoint: std::env::var("AZURE_OPENAI_ENDPOINT")?,
            azure_openai_api_key: std::env::var("AZURE_OPENAI_API_KEY")?,
            azure_openai_deployment: std::env::var("AZURE_OPENAI_DEPLOYMENT").unwrap_or_else(|_| "gpt-4o-mini".into()),
            azure_openai_api_version: std::env::var("AZURE_OPENAI_API_VERSION").unwrap_or_else(|_| "2024-12-01-preview".into()),
            azure_openai_region: std::env::var("AZURE_OPENAI_REGION").unwrap_or_else(|_| "koreacentral".into()),
            temperature: std::env::var("AZURE_OPENAI_TEMPERATURE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
            max_tokens: std::env::var("AZURE_OPENAI_MAX_TOKENS").ok().and_then(|v| v.parse().ok()).or(Some(4096)),
        })
    }
}

/// 평가 시스템 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationConfig {
    pub log_dir: String,
    pub trajectory_dir: String,
    pub max_iterations: u32,
    pub early_stop_threshold: u32,
    pub enable_human_in_loop: bool,
    pub safety_check_enabled: bool,
    pub enable_llm_judge: bool,
    /// SPEC-020: 도메인 키워드 pre-filter 의 top-K. 0 이면 라우터 비활성 (모든
    /// 도메인 도구 사용). 1 이상이면 task_description 에서 키워드 매칭이 가장
    /// 많은 상위 K 개 도메인의 도구만 LLM 에 노출한다. `general` 도메인(기본
    /// 파일 도구)은 항상 포함.
    #[serde(default)]
    pub domain_router_top_k: usize,
}

impl Default for EvaluationConfig {
    fn default() -> Self {
        Self {
            log_dir: "reporting_logs".into(),
            trajectory_dir: "reporting_trajectories".into(),
            max_iterations: 3,
            early_stop_threshold: 3,
            enable_human_in_loop: false,
            safety_check_enabled: true,
            enable_llm_judge: false,
            domain_router_top_k: 0,
        }
    }
}
