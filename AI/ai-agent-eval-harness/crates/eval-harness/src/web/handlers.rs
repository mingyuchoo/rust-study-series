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
           http::{StatusCode,
                  header},
           response::{Html,
                      IntoResponse,
                      Json}};
use data_scenarios::loader::ScenarioLoader;
use serde::Serialize;
use std::{fs,
          path::{Path,
                 PathBuf}};

const INDEX_HTML: &str = include_str!("index.html");
const HELP_HTML: &str = include_str!("help.html");
const APP_CSS: &str = include_str!("app.css");
const APP_JS: &str = include_str!("app.js");

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

/// SPEC-021 Stage 4: DB 의 evaluations 행을 파일명 형식으로 surface,
/// 디렉토리의 `evaluation_report_*.json` (집계 보고서) 는 파일에서 그대로
/// 합쳐서 반환. PRD-021/FR-5: 집계 보고서는 본 PRD 범위 외(파일 유지).
///
/// @trace SPEC: SPEC-002, SPEC-021
/// @trace TC: SPEC-002/TC-3
/// @trace FR: PRD-002/FR-3, PRD-021/FR-4
pub fn list_reports_impl(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(rows) = super::db_query::list_evaluations() {
        for row in &rows {
            out.push(super::db_query::evaluation_row_to_filename(row));
        }
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            // 집계 보고서(`evaluation_report_*`) 와 임의의 .json 은 항상 surface.
            // 개별 평가 로그(`evaluation_<task_id>_*`)는 DB 가 권위이므로
            // DB 에서 이미 surface 된 경우만 중복을 피한다.
            if !out.iter().any(|x| x == name) {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// SPEC-021 Stage 4: 평가 로그(`evaluation_<task_id>_*.json`) 는 DB 우선,
/// 집계 보고서(`evaluation_report_*.json`) 와 그 외는 파일에서 직접 읽는다.
///
/// @trace SPEC: SPEC-002, SPEC-021
/// @trace TC: SPEC-002/TC-4, SPEC-002/TC-5
/// @trace FR: PRD-002/FR-4, PRD-021/FR-4
pub fn get_report_impl(dir: &Path, name: &str) -> Option<serde_json::Value> {
    if !is_safe_name(name) {
        return None;
    }
    if !name.starts_with("evaluation_report_") {
        if let Some(task_id) = super::db_query::parse_task_id_from_filename(name) {
            if let Some(v) = super::db_query::get_evaluation(&task_id) {
                return Some(v);
            }
        }
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

/// 정적 CSS 에셋 — `/assets/app.css` 로 서빙.
///
/// index.html 에서 분리된 스타일 블록을 `include_str!` 로 임베드하여
/// 기존 "단일 Rust 바이너리 배포" 보장을 유지한다.
pub async fn app_css() -> impl IntoResponse { ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], APP_CSS) }

/// 정적 JS 에셋 — `/assets/app.js` 로 서빙.
///
/// index.html 에서 분리된 스크립트 블록을 `include_str!` 로 임베드.
pub async fn app_js() -> impl IntoResponse { ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], APP_JS) }

pub async fn list_scenarios(State(st): State<AppState>) -> Json<Vec<DomainSummary>> {
    let scen = st.scenarios_dir.clone();
    let out = tokio::task::spawn_blocking(move || list_scenarios_impl(&scen)).await.unwrap_or_default();
    Json(out)
}

pub async fn list_reports(State(st): State<AppState>) -> Json<Vec<String>> { Json(list_reports_impl(&st.reports_dir)) }

pub async fn get_report(State(st): State<AppState>, AxPath(name): AxPath<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    match get_report_impl(&st.reports_dir, &name) {
        | Some(v) => Ok(Json(v)),
        | None => Err(StatusCode::NOT_FOUND),
    }
}

/// 테스트용 헬퍼 — index.html 본문 + 분리된 CSS/JS 를 합쳐
/// 하나의 논리적 "SPA 본문" 으로 반환.
///
/// SPEC-002 이후 CSS/JS 를 외부 파일로 분리했지만, 기존 문자열
/// 매칭 기반 테스트들은 하나의 본문을 검사하도록 작성되어 있으므로
/// 이 헬퍼가 세 파일을 concat 해서 단일 뷰를 제공한다.
#[cfg(test)]
pub fn index_html_body() -> String { format!("{}\n<!--APP_CSS-->\n{}\n<!--APP_JS-->\n{}", INDEX_HTML, APP_CSS, APP_JS) }

#[cfg(test)]
pub fn help_html_body() -> String { HELP_HTML.to_string() }

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
    /// @trace scenario: scenarios 핸들러가 내장 시드 기반 도메인을 로드
    #[test]
    fn test_tc_2_list_scenarios_loads_yaml() {
        let dir = tempdir().unwrap();
        let out = list_scenarios_impl(dir.path());
        let fin = out.iter().find(|d| d.name == "financial").expect("embedded financial domain");
        assert!(fin.scenarios.iter().any(|s| s.id == "fin_001"));
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
    /// @trace scenario: 탭 버튼 존재 (SPEC-019 리워크 후)
    #[test]
    fn test_spec006_tc_1_tabs_present() {
        let html = index_html_body();
        for tab in [
            "data-tab=\"manage\"",
            "data-tab=\"execute\"",
            "data-tab=\"trajectories\"",
            "data-tab=\"reports\"",
            "data-tab=\"tools\"",
            "data-tab=\"agents\"",
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
        assert!(html.contains("editGolden("));
        assert!(html.contains("newGolden("));
        assert!(html.contains("/api/golden-sets"));
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
            "Run",
            "Scenarios",
            "Tools",
            "Agents",
            "Reports",
            "Trajectories",
            "Goldens",
            "/api/run",
            "/api/compare",
            "/api/tools/",
            "/api/agents/",
            "curl",
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

    // --- SPEC-008 i18n --------------------------------------------------

    /// @trace TC: SPEC-008/TC-1
    /// @trace FR: PRD-008/FR-1
    /// @trace scenario: 언어 토글 버튼 존재 (index + help)
    #[test]
    fn test_spec008_tc_1_lang_toggle_buttons() {
        for html in [index_html_body(), help_html_body()] {
            assert!(html.contains("id=\"lang-ko\"") || html.contains("data-lang=\"ko\""));
            assert!(html.contains("data-lang=\"en\""));
            assert!(html.contains("lang-switch"));
        }
    }

    /// @trace TC: SPEC-008/TC-2
    /// @trace FR: PRD-008/FR-1
    /// @trace scenario: setLang + localStorage 영속화
    #[test]
    fn test_spec008_tc_2_setlang_persistence() {
        for html in [index_html_body(), help_html_body()] {
            assert!(html.contains("setLang("));
            assert!(html.contains("localStorage"));
        }
    }

    /// @trace TC: SPEC-008/TC-3
    /// @trace FR: PRD-008/FR-2
    /// @trace scenario: I18N 사전에 ko/en 키
    #[test]
    fn test_spec008_tc_3_i18n_dict_present() {
        let html = index_html_body();
        assert!(html.contains("const I18N"));
        assert!(html.contains("ko:") || html.contains("ko :"));
        assert!(html.contains("en:") || html.contains("en :"));
        // sample keys that must exist in both languages
        assert!(html.contains("\"nav.run\""));
        assert!(html.contains("\"run.title\""));
    }

    /// @trace TC: SPEC-008/TC-4
    /// @trace FR: PRD-008/FR-2
    /// @trace scenario: data-i18n 마킹 충분히 존재
    #[test]
    fn test_spec008_tc_4_data_i18n_markers() {
        let html = index_html_body();
        let count = html.matches("data-i18n=\"").count();
        assert!(count >= 20, "expected at least 20 data-i18n markers, got {}", count);
    }

    /// @trace TC: SPEC-008/TC-5
    /// @trace FR: PRD-008/FR-3
    /// @trace scenario: help.html lang-ko + lang-en 블록 양쪽 존재
    #[test]
    fn test_spec008_tc_5_help_has_both_langs() {
        let html = help_html_body();
        assert!(html.contains("class=\"lang-ko\""));
        assert!(html.contains("class=\"lang-en\""));
        // content sanity: English section must have English headings
        assert!(html.contains("Overview"));
        assert!(html.contains("Quick start"));
        // and Korean section headings
        assert!(html.contains("개요"));
        assert!(html.contains("빠른 시작"));
    }

    // --- SPEC-010 IBM Plex typography -----------------------------------

    /// @trace TC: SPEC-010/TC-1
    /// @trace FR: PRD-010/FR-1
    /// @trace scenario: body font-family 에 IBM Plex Sans KR 포함
    #[test]
    fn test_spec010_tc_1_body_font_family() {
        for html in [index_html_body(), help_html_body()] {
            assert!(html.contains("'IBM Plex Sans KR'"));
            assert!(html.contains("'IBM Plex Sans'"));
        }
    }

    /// @trace TC: SPEC-010/TC-2
    /// @trace FR: PRD-010/FR-2
    /// @trace scenario: mono font-family 에 IBM Plex Mono 포함
    #[test]
    fn test_spec010_tc_2_mono_font_family() {
        for html in [index_html_body(), help_html_body()] {
            assert!(html.contains("'IBM Plex Mono'"));
        }
    }

    /// @trace TC: SPEC-010/TC-3
    /// @trace FR: PRD-010/FR-3
    /// @trace scenario: 두 페이지 모두 Google Fonts 링크 포함
    #[test]
    fn test_spec010_tc_3_google_fonts_link() {
        for html in [index_html_body(), help_html_body()] {
            assert!(html.contains("fonts.googleapis.com/css2"));
            assert!(html.contains("IBM+Plex+Sans+KR"));
            assert!(html.contains("IBM+Plex+Mono"));
            assert!(html.contains("rel=\"preconnect\""));
        }
    }

    // --- SPEC-011 select/option dark styling ----------------------------

    /// @trace TC: SPEC-011/TC-1
    /// @trace FR: PRD-011/FR-1
    /// @trace scenario: 두 페이지 모두 color-scheme: dark
    #[test]
    fn test_spec011_tc_1_color_scheme_dark() {
        for html in [index_html_body(), help_html_body()] {
            assert!(
                html.contains("color-scheme: dark") || html.contains("color-scheme:dark"),
                "color-scheme declaration missing"
            );
        }
    }

    /// @trace TC: SPEC-011/TC-2
    /// @trace FR: PRD-011/FR-2
    /// @trace scenario: option 배경/색상 규칙
    #[test]
    fn test_spec011_tc_2_option_styling() {
        let html = index_html_body();
        assert!(html.contains("select option"));
        assert!(html.contains("#1e1e1e"));
        assert!(html.contains("option:checked"));
    }

    /// @trace TC: SPEC-011/TC-3
    /// @trace FR: PRD-011/FR-3
    /// @trace scenario: select 에 appearance:none + 커스텀 화살표
    #[test]
    fn test_spec011_tc_3_custom_select_appearance() {
        let html = index_html_body();
        assert!(html.contains("appearance: none"));
        assert!(html.contains("-webkit-appearance: none"));
        // 인라인 SVG 화살표 data URL
        assert!(html.contains("data:image/svg+xml"));
        assert!(html.contains("background-position: right"));
    }
}
