pub mod customer_service;
pub mod financial;

use execution_tools::registry::ToolRegistry;
use std::sync::Arc;

/// 모든 도메인 도구를 네임스페이스(`<domain>__<name>`) 로 레지스트리에 등록.
/// 새 도메인을 추가하려면 아래 배열에 한 줄을 추가하면 된다.
pub fn register_all(registry: &mut ToolRegistry) {
    register_financial(registry);
    register_customer_service(registry);
}

/// 등록 시 사용된 도메인 이름 목록. 라우터/평가 레이어에서 조회용.
pub fn known_domains() -> &'static [&'static str] { &["financial", "customer_service"] }

pub fn register_financial(registry: &mut ToolRegistry) {
    registry.register_with_domain(Arc::new(financial::tools::SimpleInterestCalculatorTool::new()), "financial");
    registry.register_with_domain(Arc::new(financial::tools::CompoundInterestCalculatorTool::new()), "financial");
    registry.register_with_domain(Arc::new(financial::tools::TransactionValidatorTool::new()), "financial");
}

pub fn register_customer_service(registry: &mut ToolRegistry) {
    registry.register_with_domain(Arc::new(customer_service::tools::ClassifyInquiryTool::new()), "customer_service");
    registry.register_with_domain(Arc::new(customer_service::tools::ProcessRefundTool::new()), "customer_service");
    registry.register_with_domain(Arc::new(customer_service::tools::EscalateIssueTool::new()), "customer_service");
}
