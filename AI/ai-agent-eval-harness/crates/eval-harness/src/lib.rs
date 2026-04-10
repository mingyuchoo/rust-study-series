// =============================================================================
// @trace SPEC-009
// @trace PRD: PRD-009
// @trace FR: FR-1, FR-2
// @trace file-type: impl
// =============================================================================
//
// eval-harness 라이브러리 루트. main.rs(바이너리)와 외부 크레이트(desktop)에서
// 모두 재사용할 수 있도록 TUI/웹/데스크톱 헬퍼 모듈을 공개한다.

pub mod build_release;
pub mod desktop_helpers;
pub mod tui;
pub mod web;
pub mod web_theme;
