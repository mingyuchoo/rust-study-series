// =============================================================================
// @trace SPEC-012
// @trace PRD: PRD-012
// @trace FR: FR-1, FR-2, FR-3
// @trace file-type: impl
// =============================================================================
//
// 이 모듈은 런타임 심볼이 없다. `Makefile.toml` 의 데스크톱 릴리즈 태스크 존재를
// 컴파일/테스트 타임에 보장하기 위한 smoke test 전용 모듈이다.
//
// 구현 대상(= "impl")은 워크스페이스 루트의 `Makefile.toml` 파일 자체이며,
// 이 모듈의 테스트는 해당 파일 내용을 문자열로 검증한다.
//
// 왜 Rust 테스트인가:
//   - `cargo-make` 바이너리가 없는 환경(CI minimal image, 로컬 개발)에서도
//     추적성 체인(SPEC -> TC -> 구현)을 유지하기 위해 Rust 테스트 하네스에 묶는다.
//   - `Makefile.toml` 은 빌드 설정이라 런타임 심볼이 없고, 전통적인
//     함수 단위 단위 테스트를 붙일 수 없다. 문자열 smoke 검증으로 대체한다.
//
// 테스트 범위:
//   - TC-1: desktop-release-windows 태스크 + x86_64-pc-windows-msvc 타겟
//   - TC-2: desktop-release-linux   태스크 + x86_64-unknown-linux-gnu 타겟
//   - TC-3: desktop-release-macos   태스크 + universal-apple-darwin   타겟
//   - TC-4: desktop-release-all     집계 태스크 + 3개 플랫폼 run_task 연결

#[cfg(test)]
mod tests {
    // =========================================================================
    // @trace SPEC-012
    // @trace PRD: PRD-012
    // @trace FR: FR-1, FR-2, FR-3
    // @trace file-type: test
    // =========================================================================

    use std::fs;
    use std::path::PathBuf;

    fn read_root_makefile() -> String {
        // CARGO_MANIFEST_DIR = <repo>/crates/eval-harness
        // workspace root     = <repo>
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path: PathBuf = PathBuf::from(manifest_dir)
            .join("..")
            .join("..")
            .join("Makefile.toml");
        fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
    }

    /// @trace TC: SPEC-012/TC-1
    /// @trace FR: PRD-012/FR-1, PRD-012/FR-3
    /// @trace scenario: Windows 데스크톱 릴리즈 태스크 선언
    #[test]
    fn test_tc_1_makefile_has_desktop_release_windows() {
        let content = read_root_makefile();
        assert!(
            content.contains("[tasks.desktop-release-windows]"),
            "Makefile.toml must declare [tasks.desktop-release-windows]"
        );
        assert!(
            content.contains("x86_64-pc-windows-msvc"),
            "desktop-release-windows must target x86_64-pc-windows-msvc"
        );
        assert!(
            content.contains("\"tauri\""),
            "desktop release tasks must invoke cargo tauri"
        );
    }

    /// @trace TC: SPEC-012/TC-2
    /// @trace FR: PRD-012/FR-1, PRD-012/FR-3
    /// @trace scenario: Linux 데스크톱 릴리즈 태스크 선언
    #[test]
    fn test_tc_2_makefile_has_desktop_release_linux() {
        let content = read_root_makefile();
        assert!(
            content.contains("[tasks.desktop-release-linux]"),
            "Makefile.toml must declare [tasks.desktop-release-linux]"
        );
        assert!(
            content.contains("x86_64-unknown-linux-gnu"),
            "desktop-release-linux must target x86_64-unknown-linux-gnu"
        );
    }

    /// @trace TC: SPEC-012/TC-3
    /// @trace FR: PRD-012/FR-1, PRD-012/FR-3
    /// @trace scenario: macOS 데스크톱 릴리즈 태스크 선언
    #[test]
    fn test_tc_3_makefile_has_desktop_release_macos() {
        let content = read_root_makefile();
        assert!(
            content.contains("[tasks.desktop-release-macos]"),
            "Makefile.toml must declare [tasks.desktop-release-macos]"
        );
        assert!(
            content.contains("universal-apple-darwin"),
            "desktop-release-macos must target universal-apple-darwin"
        );
    }

    /// @trace TC: SPEC-012/TC-4
    /// @trace FR: PRD-012/FR-2
    /// @trace scenario: 3개 플랫폼을 묶는 집계 태스크
    #[test]
    fn test_tc_4_makefile_has_desktop_release_all_aggregate() {
        let content = read_root_makefile();
        assert!(
            content.contains("[tasks.desktop-release-all]"),
            "Makefile.toml must declare [tasks.desktop-release-all]"
        );
        // run_task 로 3개 태스크를 엮어야 한다
        assert!(
            content.contains("desktop-release-windows")
                && content.contains("desktop-release-linux")
                && content.contains("desktop-release-macos"),
            "desktop-release-all must reference all three platform tasks"
        );
        assert!(
            content.contains("run_task"),
            "desktop-release-all must use run_task for sequential execution"
        );
    }
}
