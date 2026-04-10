// =============================================================================
// @trace SPEC-002
// @trace PRD: PRD-002
// @trace FR: FR-2, FR-3, FR-4, FR-5
// @trace file-type: impl
// =============================================================================

use super::AppState;
use agent_models::domain_config::DomainConfig;
use axum::{extract::{Path as AxPath,
                     State},
           http::StatusCode,
           response::{Html,
                      Json}};
use data_scenarios::loader::ScenarioLoader;
use serde::Serialize;
use std::{fs,
          path::{Path,
                 PathBuf}};

const INDEX_HTML: &str = include_str!("index.html");
const HELP_HTML: &str = include_str!("help.html");

#[derive(Serialize)]
pub struct ScenarioSummary {
    pub id: String,
    pub name: String,
    pub difficulty: String,
}

#[derive(Serialize)]
pub struct DomainSummary {
    pub name: String,
    pub description: String,
    pub scenarios: Vec<ScenarioSummary>,
}

/// 리포트 파일명이 경로 순회 공격에 안전한지 검사.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-5
/// @trace FR: PRD-002/FR-4
pub fn is_safe_name(name: &str) -> bool { !name.is_empty() && !name.contains('/') && !name.contains('\\') && !name.contains("..") && name != "." }

/// YAML 시나리오 디렉토리를 읽어 도메인 요약을 만든다.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-2
/// @trace FR: PRD-002/FR-2
pub fn list_scenarios_impl(dir: &Path) -> Vec<DomainSummary> {
    let loader = ScenarioLoader::new();
    let Some(dir_str) = dir.to_str() else {
        return Vec::new();
    };
    let configs = loader.load_all_domains(dir_str).unwrap_or_default();
    configs.into_iter().map(to_summary).collect()
}

fn to_summary(c: DomainConfig) -> DomainSummary {
    DomainSummary {
        name: c.name,
        description: c.description,
        scenarios: c
            .scenarios
            .into_iter()
            .map(|s| ScenarioSummary {
                id: s.id,
                name: s.name,
                difficulty: s.difficulty,
            })
            .collect(),
    }
}

/// 리포트 디렉토리에서 .json 파일명 리스트.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-3
/// @trace FR: PRD-002/FR-3
pub fn list_reports_impl(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|x| x.to_str()) == Some("json") {
            if let Some(n) = p.file_name().and_then(|n| n.to_str()) {
                out.push(n.to_string());
            }
        }
    }
    out.sort();
    out
}

/// 리포트 파일 내용을 JSON 값으로 반환. 존재/안전하지 않으면 None.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-4, SPEC-002/TC-5
/// @trace FR: PRD-002/FR-4
pub fn get_report_impl(dir: &Path, name: &str) -> Option<serde_json::Value> {
    if !is_safe_name(name) {
        return None;
    }
    let path: PathBuf = dir.join(name);
    let text = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&text).ok()
}

// -------- axum handler wrappers --------

pub async fn index() -> Html<&'static str> { Html(INDEX_HTML) }

/// @trace SPEC: SPEC-007
/// @trace TC: SPEC-007/TC-1
/// @trace FR: PRD-007/FR-1
pub async fn help() -> Html<&'static str> { Html(HELP_HTML) }

pub async fn list_scenarios(State(st): State<AppState>) -> Json<Vec<DomainSummary>> { Json(list_scenarios_impl(&st.scenarios_dir)) }

pub async fn list_reports(State(st): State<AppState>) -> Json<Vec<String>> { Json(list_reports_impl(&st.reports_dir)) }

pub async fn get_report(State(st): State<AppState>, AxPath(name): AxPath<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    match get_report_impl(&st.reports_dir, &name) {
        | Some(v) => Ok(Json(v)),
        | None => Err(StatusCode::NOT_FOUND),
    }
}

#[cfg(test)]
pub fn index_html_body() -> &'static str { INDEX_HTML }

#[cfg(test)]
pub fn help_html_body() -> &'static str { HELP_HTML }

#[cfg(test)]
mod tests {
    // =============================================================================
    // @trace SPEC-002
    // @trace PRD: PRD-002
    // @trace FR: FR-2, FR-3, FR-4, FR-5
    // @trace file-type: test
    // =============================================================================

    use super::*;
    use std::{fs::File,
              io::Write};
    use tempfile::tempdir;

    /// @trace TC: SPEC-002/TC-2
    /// @trace FR: PRD-002/FR-2
    /// @trace scenario: scenarios 핸들러가 YAML 로드
    #[test]
    fn test_tc_2_list_scenarios_loads_yaml() {
        let dir = tempdir().unwrap();
        let yaml = r#"
name: demo
description: demo domain
tools: []
scenarios:
  - id: d_001
    name: 데모 시나리오
    description: desc
    task_description: t
    initial_environment: {}
    expected_tools: []
    success_criteria: {}
    difficulty: easy
"#;
        let p = dir.path().join("demo.yaml");
        File::create(&p).unwrap().write_all(yaml.as_bytes()).unwrap();
        let out = list_scenarios_impl(dir.path());
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "demo");
        assert_eq!(out[0].scenarios.len(), 1);
        assert_eq!(out[0].scenarios[0].id, "d_001");
    }

    /// @trace TC: SPEC-002/TC-3
    /// @trace FR: PRD-002/FR-3
    /// @trace scenario: reports 핸들러가 .json 파일 나열
    #[test]
    fn test_tc_3_list_reports_filters_json() {
        let dir = tempdir().unwrap();
        for n in ["a.json", "b.json", "c.json", "notes.txt"] {
            File::create(dir.path().join(n)).unwrap().write_all(b"{}").unwrap();
        }
        let out = list_reports_impl(dir.path());
        assert_eq!(out, vec!["a.json", "b.json", "c.json"]);
    }

    /// @trace TC: SPEC-002/TC-4
    /// @trace FR: PRD-002/FR-4
    /// @trace scenario: report content 반환
    #[test]
    fn test_tc_4_get_report_returns_content() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("r1.json")).unwrap().write_all(br#"{"foo":42}"#).unwrap();
        let v = get_report_impl(dir.path(), "r1.json").unwrap();
        assert_eq!(v["foo"], 42);

        assert!(get_report_impl(dir.path(), "missing.json").is_none());
    }

    /// @trace TC: SPEC-002/TC-5
    /// @trace FR: PRD-002/FR-4
    /// @trace scenario: 경로 순회 거부
    #[test]
    fn test_tc_5_path_traversal_rejected() {
        assert!(!is_safe_name("../etc/passwd"));
        assert!(!is_safe_name("a/b.json"));
        assert!(!is_safe_name("a\\b.json"));
        assert!(!is_safe_name(""));
        assert!(!is_safe_name("."));
        assert!(is_safe_name("report-2026.json"));

        let dir = tempdir().unwrap();
        assert!(get_report_impl(dir.path(), "../x.json").is_none());
    }

    /// @trace TC: SPEC-002/TC-6
    /// @trace FR: PRD-002/FR-5
    /// @trace scenario: index.html 임베드 반환
    #[test]
    fn test_tc_6_index_html_embedded() {
        let body = index_html_body();
        assert!(!body.is_empty());
        assert!(body.to_lowercase().contains("<html"));
    }

    // --- SPEC-006 SPA smoke tests ---------------------------------------

    /// @trace TC: SPEC-006/TC-1
    /// @trace FR: PRD-006/FR-1
    /// @trace scenario: 탭 버튼 7개 존재
    #[test]
    fn test_spec006_tc_1_tabs_present() {
        let html = index_html_body();
        for tab in [
            "data-tab=\"run\"",
            "data-tab=\"scenarios\"",
            "data-tab=\"tools\"",
            "data-tab=\"agents\"",
            "data-tab=\"reports\"",
            "data-tab=\"trajectories\"",
            "data-tab=\"goldens\"",
        ] {
            assert!(html.contains(tab), "missing tab marker: {}", tab);
        }
    }

    /// @trace TC: SPEC-006/TC-2
    /// @trace FR: PRD-006/FR-2
    /// @trace scenario: run 폼 존재
    #[test]
    fn test_spec006_tc_2_run_form() {
        let html = index_html_body();
        assert!(html.contains("id=\"run-form\""));
        assert!(html.contains("'/api/run'"));
    }

    /// @trace TC: SPEC-006/TC-3
    /// @trace FR: PRD-006/FR-3
    /// @trace scenario: scenario run 함수
    #[test]
    fn test_spec006_tc_3_scenario_run() {
        let html = index_html_body();
        assert!(html.contains("runScenario("));
        assert!(html.contains("/api/scenarios/") && html.contains("/run"));
    }

    /// @trace TC: SPEC-006/TC-4
    /// @trace FR: PRD-006/FR-4
    /// @trace scenario: tool invoke + fault
    #[test]
    fn test_spec006_tc_4_tool_invoke() {
        let html = index_html_body();
        assert!(html.contains("invokeTool("));
        assert!(html.contains("/invoke"));
        assert!(html.contains("/simulate-fault"));
    }

    /// @trace TC: SPEC-006/TC-5
    /// @trace FR: PRD-006/FR-5
    /// @trace scenario: agent execute
    #[test]
    fn test_spec006_tc_5_agent_execute() {
        let html = index_html_body();
        assert!(html.contains("executeAgent("));
        assert!(html.contains("/api/agents/"));
        assert!(html.contains("/execute"));
    }

    /// @trace TC: SPEC-006/TC-6
    /// @trace FR: PRD-006/FR-6
    /// @trace scenario: compare 폼
    #[test]
    fn test_spec006_tc_6_compare() {
        let html = index_html_body();
        assert!(html.contains("compareReports("));
        assert!(html.contains("'/api/compare'"));
    }

    /// @trace TC: SPEC-006/TC-7
    /// @trace FR: PRD-006/FR-7
    /// @trace scenario: trajectories + score
    #[test]
    fn test_spec006_tc_7_trajectories_score() {
        let html = index_html_body();
        assert!(html.contains("/api/trajectories"));
        assert!(html.contains("scoreTrajectory("));
        assert!(html.contains("'/api/score'"));
    }

    /// @trace TC: SPEC-006/TC-8
    /// @trace FR: PRD-006/FR-8
    /// @trace scenario: goldens fetch
    #[test]
    fn test_spec006_tc_8_goldens() {
        let html = index_html_body();
        assert!(html.contains("fetchGolden("));
        assert!(html.contains("/api/golden-sets/"));
    }

    // --- SPEC-007 help page -------------------------------------------

    /// @trace TC: SPEC-007/TC-1
    /// @trace FR: PRD-007/FR-1
    /// @trace scenario: help 본문 비어있지 않음 + HTML 형식
    #[test]
    fn test_spec007_tc_1_help_embedded() {
        let body = help_html_body();
        assert!(!body.is_empty());
        assert!(body.to_lowercase().contains("<html"));
        assert!(body.contains("사용안내"));
    }

    /// @trace TC: SPEC-007/TC-2
    /// @trace FR: PRD-007/FR-1
    /// @trace scenario: help 본문에 탭/엔드포인트 키워드 포함
    #[test]
    fn test_spec007_tc_2_help_has_guides() {
        let body = help_html_body();
        for keyword in [
            "Run", "Scenarios", "Tools", "Agents", "Reports", "Trajectories", "Goldens",
            "/api/run", "/api/compare", "/api/tools/", "/api/agents/", "curl",
        ] {
            assert!(body.contains(keyword), "help page missing keyword: {}", keyword);
        }
    }

    /// @trace TC: SPEC-007/TC-3
    /// @trace FR: PRD-007/FR-2
    /// @trace scenario: index.html 헤더에 /help 링크
    #[test]
    fn test_spec007_tc_3_index_has_help_link() {
        let html = index_html_body();
        assert!(html.contains("href=\"/help\""));
        assert!(html.contains("사용안내"));
    }
}
