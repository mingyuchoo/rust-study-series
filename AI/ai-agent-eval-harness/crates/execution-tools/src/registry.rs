use crate::{base::BaseTool,
            file_tools::{ListDirectoryTool,
                         ReadFileTool,
                         WriteFileTool}};
use std::{collections::HashMap,
          sync::Arc};

/// 도구 레지스트리.
///
/// 도구는 네임스페이스 키(`<domain>__<name>`) 로 저장되어, 여러 도메인이
/// 동일한 도구 이름을 갖더라도 충돌 없이 공존한다. 기본 파일 도구는 도메인
/// 프리픽스 없이 `"general"` 도메인으로 등록된다.
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn BaseTool>>,
    /// 각 레지스트리 키 → 도메인 이름. LLM 시스템 프롬프트에 surface 되어
    /// 에이전트가 task 와 도메인 관계를 이해할 수 있게 한다.
    domains: HashMap<String, String>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
            domains: HashMap::new(),
        };
        registry.register(Arc::new(ReadFileTool::new()));
        registry.register(Arc::new(WriteFileTool::new()));
        registry.register(Arc::new(ListDirectoryTool::new()));
        registry
    }

    /// 기본(general) 도구 등록. 키는 `tool.metadata().name` 그대로 사용.
    pub fn register(&mut self, tool: Arc<dyn BaseTool>) {
        let name = tool.metadata().name.clone();
        self.tools.insert(name.clone(), tool);
        self.domains.insert(name, "general".into());
    }

    /// 도메인 네임스페이스를 붙여 등록한다. 키는 `<domain>__<name>`.
    pub fn register_with_domain(&mut self, tool: Arc<dyn BaseTool>, domain: &str) {
        let orig = tool.metadata().name.clone();
        let key = format!("{domain}__{orig}");
        self.tools.insert(key.clone(), tool);
        self.domains.insert(key, domain.into());
    }

    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn BaseTool>> { self.tools.get(name).cloned() }

    pub fn get_all_tools(&self) -> Vec<Arc<dyn BaseTool>> { self.tools.values().cloned().collect() }

    #[allow(dead_code)]
    pub fn get_tool_names(&self) -> Vec<String> { self.tools.keys().cloned().collect() }

    /// 도구 키에 대응하는 도메인 라벨. 등록되지 않은 키는 None.
    pub fn get_tool_domain(&self, key: &str) -> Option<&str> { self.domains.get(key).map(|s| s.as_str()) }

    /// LLM 에게 전달할 메타데이터. `name` 필드는 레지스트리 키(네임스페이스
    /// 포함) 이며, `domain` 필드가 추가로 포함된다.
    pub fn get_tools_metadata(&self) -> Vec<serde_json::Value> {
        self.tools
            .iter()
            .map(|(key, t)| {
                let m = t.metadata();
                let domain = self.domains.get(key).map(|s| s.as_str()).unwrap_or("general");
                serde_json::json!({
                    "name": key,
                    "description": m.description,
                    "parameters_schema": m.parameters_schema,
                    "safety_level": m.safety_level,
                    "requires_approval": m.requires_approval,
                    "domain": domain,
                })
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn requires_approval(&self, tool_name: &str) -> bool { self.tools.get(tool_name).map(|t| t.metadata().requires_approval).unwrap_or(true) }
}

impl Default for ToolRegistry {
    fn default() -> Self { Self::new() }
}
