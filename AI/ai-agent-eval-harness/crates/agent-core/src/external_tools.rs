// =============================================================================
// @trace SPEC-023
// @trace PRD: PRD-023
// @trace FR: PRD-023/FR-2, PRD-023/FR-6
// @trace file-type: impl
// =============================================================================
//
// SPEC-023 Layer 2: 전역 SqliteStore 에서 external_tools 행을 읽어 HttpCallTool
// 인스턴스를 ToolRegistry 에 등록한다. PpaAgent::load_all_tools 가 매번
// 호출하므로 CRUD 후 다음 task 부터 자동 반영된다.

use data_scenarios::sqlite_store::ExternalToolRow;
use execution_tools::{http_tool::HttpCallTool,
                      registry::ToolRegistry};
use std::{collections::HashMap,
          sync::Arc};

/// 전역 store 가 install 되어 있으면 모든 external tool 을 registry 에 등록.
/// 없으면 조용히 반환.
pub fn register_external_tools_from_db(registry: &mut ToolRegistry) {
    let Some(store) = data_scenarios::loader::try_installed_store() else {
        return;
    };
    let result = match tokio::runtime::Handle::try_current() {
        | Ok(handle) => {
            let store = store.clone();
            tokio::task::block_in_place(|| handle.block_on(async move { store.list_external_tools().await }))
        },
        | Err(_) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .ok()
            .map(|rt| rt.block_on(async move { store.list_external_tools().await }))
            .unwrap_or_else(|| Ok(Vec::new())),
    };
    let rows = match result {
        | Ok(r) => r,
        | Err(e) => {
            eprintln!("[warn] external_tools 로드 실패: {e}");
            return;
        },
    };
    for row in rows {
        match build_tool_from_row(&row) {
            | Ok(tool) => registry.register_with_domain(Arc::new(tool), &row.domain),
            | Err(e) => eprintln!("[warn] HttpCallTool '{}/{}' 등록 실패: {}", row.domain, row.name, e),
        }
    }
}

fn build_tool_from_row(row: &ExternalToolRow) -> Result<HttpCallTool, String> {
    if row.timeout_ms < 0 {
        return Err("timeout_ms must be >= 0".into());
    }
    let headers: HashMap<String, String> = match row.headers_json.as_deref() {
        | Some(s) if !s.trim().is_empty() => serde_json::from_str(s).map_err(|e| format!("headers_json parse: {e}"))?,
        | _ => HashMap::new(),
    };
    Ok(HttpCallTool::new(
        row.name.clone(),
        row.description.clone(),
        row.method.clone(),
        row.url.clone(),
        headers,
        row.body_template.clone(),
        &row.params_schema,
        row.timeout_ms as u64,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use execution_tools::base::BaseTool;

    fn sample_row(name: &str) -> ExternalToolRow {
        ExternalToolRow {
            name: name.to_string(),
            domain: "healthcare".into(),
            description: "환자 검색".into(),
            method: "POST".into(),
            url: "http://localhost:9999/q".into(),
            headers_json: Some(r#"{"X-Auth":"abc"}"#.to_string()),
            body_template: r#"{"q":"{{topic}}"}"#.to_string(),
            params_schema: r#"{"type":"object","properties":{"topic":{"type":"string"}}}"#.to_string(),
            timeout_ms: 5000,
        }
    }

    /// @trace TC: SPEC-023/TC-9
    #[test]
    fn build_tool_from_row_ok() {
        let row = sample_row("search_patient");
        let tool = build_tool_from_row(&row).unwrap();
        let m = tool.metadata();
        assert_eq!(m.name, "search_patient");
        assert_eq!(m.parameters_schema["properties"]["topic"]["type"], "string");
    }

    #[test]
    fn build_tool_from_row_invalid_headers_returns_err() {
        let mut row = sample_row("x");
        row.headers_json = Some("not-json".into());
        assert!(build_tool_from_row(&row).is_err());
    }

    #[test]
    fn register_skips_when_no_store() {
        // 전역 store 가 install 되지 않은 단위 테스트 컨텍스트에서는 no-op.
        let mut reg = ToolRegistry::new();
        let before = reg.get_tool_names().len();
        register_external_tools_from_db(&mut reg);
        let after = reg.get_tool_names().len();
        assert_eq!(before, after, "store 없으면 변화 없음");
    }
}
