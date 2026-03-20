use crate::{base::BaseTool,
            file_tools::{ListDirectoryTool,
                         ReadFileTool,
                         WriteFileTool}};
use std::{collections::HashMap,
          sync::Arc};

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn BaseTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.register(Arc::new(ReadFileTool::new()));
        registry.register(Arc::new(WriteFileTool::new()));
        registry.register(Arc::new(ListDirectoryTool::new()));
        registry
    }

    pub fn register(&mut self, tool: Arc<dyn BaseTool>) { self.tools.insert(tool.metadata().name.clone(), tool); }

    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn BaseTool>> { self.tools.get(name).cloned() }

    pub fn get_all_tools(&self) -> Vec<Arc<dyn BaseTool>> { self.tools.values().cloned().collect() }

    #[allow(dead_code)]
    pub fn get_tool_names(&self) -> Vec<String> { self.tools.keys().cloned().collect() }

    pub fn get_tools_metadata(&self) -> Vec<serde_json::Value> {
        self.tools
            .values()
            .map(|t| {
                let m = t.metadata();
                serde_json::json!({
                    "name": m.name,
                    "description": m.description,
                    "parameters_schema": m.parameters_schema,
                    "safety_level": m.safety_level,
                    "requires_approval": m.requires_approval,
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
