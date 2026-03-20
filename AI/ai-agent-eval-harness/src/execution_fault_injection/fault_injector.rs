#![allow(dead_code)]
use super::{models::FaultInjectionConfig,
            tool_wrapper::FaultInjectedTool};
use crate::execution_tools::registry::ToolRegistry;
use std::sync::Arc;

pub struct FaultInjector {
    config: FaultInjectionConfig,
}

impl FaultInjector {
    pub fn new(config: FaultInjectionConfig) -> Self {
        Self {
            config,
        }
    }

    pub fn wrap_registry(&self, registry: &ToolRegistry) -> ToolRegistry {
        if !self.config.enabled {
            return ToolRegistry::new();
        }

        let mut wrapped = ToolRegistry::new();
        for tool in registry.get_all_tools() {
            let tool_name = tool.metadata().name.clone();
            let failure_rate = self
                .config
                .tool_specific_rates
                .get(&tool_name)
                .copied()
                .unwrap_or(self.config.global_failure_rate);

            let wrapped_tool = Arc::new(FaultInjectedTool::new(
                tool,
                failure_rate,
                &self.config.failure_mode_distribution,
                self.config.seed,
            ));
            wrapped.register(wrapped_tool);
        }
        wrapped
    }
}
