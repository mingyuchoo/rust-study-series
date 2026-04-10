# SPEC-012: Makefile.toml Desktop 릴리즈 태스크

## 메타데이터
- SPEC ID: SPEC-012
- PRD: PRD-012
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향
| PRD | FR | 요구사항 |
|-----|----|---------|
| PRD-012 | FR-1 | 플랫폼별 `desktop-release-*` 태스크 3개 존재 |
| PRD-012 | FR-2 | `desktop-release-all` 집계 태스크 존재 |
| PRD-012 | FR-3 | 각 태스크가 `cargo tauri build` 와 명시적 `--target` triple 포함 |

### 역방향
| TC | 시나리오 | FR | 테스트 파일 |
|----|---------|----|-----------|
| TC-1 | Makefile.toml 에 `desktop-release-windows` 태스크와 `x86_64-pc-windows-msvc` 타겟 포함 | FR-1, FR-3 | crates/eval-harness/src/build_release.rs |
| TC-2 | Makefile.toml 에 `desktop-release-linux` 태스크와 `x86_64-unknown-linux-gnu` 타겟 포함 | FR-1, FR-3 | crates/eval-harness/src/build_release.rs |
| TC-3 | Makefile.toml 에 `desktop-release-macos` 태스크와 `universal-apple-darwin` 타겟 포함 | FR-1, FR-3 | crates/eval-harness/src/build_release.rs |
| TC-4 | Makefile.toml 에 `desktop-release-all` 집계 태스크가 3개 플랫폼 태스크를 run_task 로 연결 | FR-2 | crates/eval-harness/src/build_release.rs |

### 구현 추적
| 파일 | 변경 |
|------|------|
| Makefile.toml | `desktop-release-windows`, `desktop-release-linux`, `desktop-release-macos`, `desktop-release-all` 태스크 추가 |
| crates/eval-harness/src/build_release.rs | 신규 파일. Makefile.toml 콘텐츠 검증 단위 테스트 4개 |
| crates/eval-harness/src/lib.rs | `pub mod build_release;` 선언 추가 (테스트 컴파일 대상) |

## 기술 설계

### Makefile.toml 태스크 구조

각 플랫폼 태스크는 `cargo-tauri` CLI 에 의존한다. `cargo-make` 의 `install_crate` 메커니즘으로 자동 설치하며, `desktop/` 디렉토리로 `cwd` 진입한 뒤 `cargo tauri build` 를 실행한다.

```toml
[tasks.desktop-release-windows]
  description = "Build Windows MSI/NSIS desktop bundle via Tauri"
  cwd         = "desktop"
  workspace   = false
  command     = "cargo"
  args        = [
    "tauri", "build",
    "--target", "x86_64-pc-windows-msvc",
    "--bundles", "msi", "nsis",
  ]
  install_crate = { crate_name = "tauri-cli", binary = "cargo", test_arg = ["tauri", "--version"] }

[tasks.desktop-release-linux]
  description = "Build Linux AppImage/Deb desktop bundle via Tauri"
  cwd         = "desktop"
  workspace   = false
  command     = "cargo"
  args        = [
    "tauri", "build",
    "--target", "x86_64-unknown-linux-gnu",
    "--bundles", "deb", "appimage",
  ]
  install_crate = { crate_name = "tauri-cli", binary = "cargo", test_arg = ["tauri", "--version"] }

[tasks.desktop-release-macos]
  description = "Build macOS .app/.dmg desktop bundle via Tauri (universal)"
  cwd         = "desktop"
  workspace   = false
  command     = "cargo"
  args        = [
    "tauri", "build",
    "--target", "universal-apple-darwin",
    "--bundles", "app", "dmg",
  ]
  install_crate = { crate_name = "tauri-cli", binary = "cargo", test_arg = ["tauri", "--version"] }

[tasks.desktop-release-all]
  description = "Build Windows, Linux, and macOS desktop bundles sequentially"
  workspace   = false
  run_task    = { name = ["desktop-release-windows", "desktop-release-linux", "desktop-release-macos"] }
```

### 왜 `workspace = false` 인가

루트 `Cargo.toml` 에는 `exclude = ["desktop"]` 가 선언되어 있어 `desktop/` 는 가상 워크스페이스 밖의 독립 크레이트다. `cargo-make` 는 기본적으로 각 워크스페이스 멤버에 태스크를 개별 적용하므로, `workspace = false` 로 루트에서 단 한 번만 실행되도록 강제해야 한다.

### 테스트 전략

Rust 유닛 테스트로 `Makefile.toml` 파일을 `include_str!` 또는 `std::fs::read_to_string` 으로 읽어 고정된 토큰 존재를 검증한다. `cargo-make` 자체를 실행할 수는 없으므로(테스트 환경에 없음) **문자열 매칭** 수준의 smoke 검증만 수행한다. 이는 FR-1/FR-2/FR-3 의 "선언적 존재" 를 보장한다.

`build_release.rs` 는 런타임 심볼이 없는 테스트 전용 모듈이다. 경로는 `env!("CARGO_MANIFEST_DIR")` 기준 상대 경로 `../../Makefile.toml` 로 워크스페이스 루트의 파일을 읽는다.

## 완료 정의
- TC-1 ~ TC-4 통과
- `cargo build -p eval-harness` 성공 (신규 모듈 컴파일)
- 기존 `cargo make build`, `cargo make test`, `cargo make format` 태스크 영향 없음
- `traceability-matrix.md` 에 SPEC-012 행 추가
