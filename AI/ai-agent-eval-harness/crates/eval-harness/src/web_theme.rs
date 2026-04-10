// =============================================================================
// @trace SPEC-014
// @trace PRD: PRD-014
// @trace FR: FR-1, FR-2, FR-3
// @trace file-type: test
// =============================================================================
//
// 웹 UI(`index.html`, `help.html`)에 라이트/다크 테마 토글이 올바르게
// 구성되어 있는지 검증하는 단위 테스트 모듈. 렌더링은 다루지 않고,
// 정적 HTML 콘텐츠에 필수 토큰이 존재하는지를 문자열 매칭으로 확인한다.

pub const WEB_INDEX_HTML: &str = include_str!("web/index.html");
pub const WEB_HELP_HTML: &str = include_str!("web/help.html");

#[cfg(test)]
mod tests {
    use super::*;

    // @trace TC: TC-1
    // @trace FR: FR-1
    // @trace scenario: index.html 에 테마 토글 버튼 컨테이너와 라이트/다크 버튼이 존재
    #[test]
    fn tc1_index_has_theme_switch_buttons() {
        assert!(
            WEB_INDEX_HTML.contains(r#"id="theme-switch""#),
            "index.html 은 #theme-switch 컨테이너를 포함해야 함"
        );
        assert!(
            WEB_INDEX_HTML.contains(r#"data-theme="light""#),
            "index.html 은 data-theme=\"light\" 버튼을 포함해야 함"
        );
        assert!(
            WEB_INDEX_HTML.contains(r#"data-theme="dark""#),
            "index.html 은 data-theme=\"dark\" 버튼을 포함해야 함"
        );
    }

    // @trace TC: TC-2
    // @trace FR: FR-2
    // @trace scenario: index.html 이 CSS 변수 기반 라이트/다크 테마를 정의
    #[test]
    fn tc2_index_defines_css_variable_themes() {
        assert!(
            WEB_INDEX_HTML.contains(r#"[data-theme="light"]"#),
            "index.html 은 [data-theme=\"light\"] CSS 셀렉터 블록을 포함해야 함"
        );
        assert!(
            WEB_INDEX_HTML.contains(r#"[data-theme="dark"]"#),
            "index.html 은 [data-theme=\"dark\"] CSS 셀렉터 블록을 포함해야 함"
        );
        for var in ["--bg", "--fg", "--accent"] {
            assert!(
                WEB_INDEX_HTML.contains(var),
                "index.html 은 CSS 변수 `{var}` 를 정의해야 함"
            );
        }
    }

    // @trace TC: TC-3
    // @trace FR: FR-3
    // @trace scenario: index.html 이 setTheme 에서 선택된 테마를 localStorage 에 저장
    #[test]
    fn tc3_index_persists_theme_to_localstorage() {
        assert!(
            WEB_INDEX_HTML.contains("setTheme"),
            "index.html 은 setTheme 함수를 포함해야 함"
        );
        assert!(
            WEB_INDEX_HTML.contains("localStorage.setItem('theme'"),
            "index.html 은 localStorage.setItem('theme', ...) 호출을 포함해야 함"
        );
    }

    // @trace TC: TC-4
    // @trace FR: FR-3
    // @trace scenario: index.html 이 초기화 시 저장된 테마를 localStorage 에서 로드
    #[test]
    fn tc4_index_loads_theme_from_localstorage() {
        assert!(
            WEB_INDEX_HTML.contains("localStorage.getItem('theme')"),
            "index.html 은 localStorage.getItem('theme') 호출을 포함해야 함"
        );
    }

    // @trace TC: TC-5
    // @trace FR: FR-2
    // @trace scenario: help.html 에도 동일한 테마 토글 시스템이 적용
    #[test]
    fn tc5_help_has_same_theme_system() {
        assert!(
            WEB_HELP_HTML.contains(r#"id="theme-switch""#),
            "help.html 은 #theme-switch 컨테이너를 포함해야 함"
        );
        assert!(
            WEB_HELP_HTML.contains(r#"[data-theme="light"]"#),
            "help.html 은 [data-theme=\"light\"] CSS 셀렉터 블록을 포함해야 함"
        );
        assert!(
            WEB_HELP_HTML.contains("setTheme"),
            "help.html 은 setTheme 함수를 포함해야 함"
        );
        assert!(
            WEB_HELP_HTML.contains("localStorage.setItem('theme'"),
            "help.html 도 localStorage 에 테마를 저장해야 함"
        );
    }
}
