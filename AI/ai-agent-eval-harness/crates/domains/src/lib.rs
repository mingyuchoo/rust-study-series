pub mod customer_service;
pub mod financial;

use execution_tools::registry::ToolRegistry;
use std::sync::Arc;

#[allow(dead_code)]
pub fn register_financial_tools(registry: &mut ToolRegistry) {
    registry.register(Arc::new(financial::tools::SimpleInterestCalculatorTool::new()));
    registry.register(Arc::new(financial::tools::CompoundInterestCalculatorTool::new()));
    registry.register(Arc::new(financial::tools::TransactionValidatorTool::new()));
}

#[allow(dead_code)]
pub fn register_customer_service_tools(registry: &mut ToolRegistry) {
    registry.register(Arc::new(customer_service::tools::ClassifyInquiryTool::new()));
    registry.register(Arc::new(customer_service::tools::ProcessRefundTool::new()));
    registry.register(Arc::new(customer_service::tools::EscalateIssueTool::new()));
}
