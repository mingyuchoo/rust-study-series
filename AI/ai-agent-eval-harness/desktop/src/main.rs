// =============================================================================
// @trace SPEC-009
// @trace SPEC-015
// @trace PRD: PRD-009, PRD-015
// @trace FR: PRD-009/FR-3, PRD-009/FR-4, PRD-015/FR-6
// @trace file-type: impl
// =============================================================================
//
// Tauri 2.x 데스크톱 래퍼.
//
// 흐름:
//   1) 로컬호스트 빈 포트 선택
//   2) 프로젝트 루트 상대 경로로 내장 Axum 서버(detached thread)를 기동
//   3) wait_for_port 로 readiness 대기
//   4) Tauri setup 훅에서 WebviewWindow 를 서버 URL 로 생성
//
// 웹 SPA(index.html, help.html, i18n, 7-탭) 가 그대로 재사용되므로 Tauri 측에서
// 별도 IPC 명령을 정의할 필요가 없다.

use eval_harness::{data_paths::DataPaths, desktop_helpers, web};
use std::net::SocketAddr;
use std::path::PathBuf;
use tauri::{WebviewUrl, WebviewWindowBuilder};

/// 리포 루트 경로(워크스페이스 Cargo.toml 이 있는 위치).
///
/// desktop/ 크레이트 기준에서 한 단계 상위로 올라간다. 컴파일 타임 상수이므로
/// 배포 바이너리에서도 동일한 절대 경로가 박힌다. 배포용으로는 `std::env::current_dir`
/// 또는 인스톨 디렉토리 기반 경로 해석으로 교체해야 한다 (범위 외).
fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("desktop/ has no parent")
        .to_path_buf()
}

fn main() {
    let root = project_root();
    let port = desktop_helpers::pick_free_port().expect("no free TCP port");
    let addr: SocketAddr = format!("127.0.0.1:{port}")
        .parse()
        .expect("valid loopback addr");

    let data_paths = DataPaths::resolve_for_root(&root).unwrap_or_else(|e| {
        eprintln!("[eval-harness-desktop] data path 설정 오류: {e}");
        std::process::exit(1);
    });
    let scenarios_dir = data_paths.scenarios_dir;
    let reports_dir = root.join("reporting_logs");
    let golden_sets_dir = data_paths.golden_sets_dir;
    let trajectories_dir = root.join("reporting_trajectories");

    // 내장 Axum 서버를 별도 OS 스레드에서 기동. 프로세스 종료 시 OS 가 정리한다.
    std::thread::spawn(move || {
        if let Err(e) = web::run_server(
            addr,
            scenarios_dir,
            reports_dir,
            golden_sets_dir,
            trajectories_dir,
        ) {
            eprintln!("[eval-harness-desktop] embedded server error: {e}");
        }
    });

    if !desktop_helpers::wait_for_port("127.0.0.1", port, 5_000) {
        eprintln!("[eval-harness-desktop] embedded server did not become ready in time");
        std::process::exit(1);
    }

    tauri::Builder::default()
        .setup(move |app| {
            let url: tauri::Url = format!("http://127.0.0.1:{port}")
                .parse()
                .expect("valid url");
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("eval-harness")
                .inner_size(1280.0, 820.0)
                .min_inner_size(960.0, 640.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run tauri app");
}
