// =============================================================================
// @trace SPEC-009
// @trace PRD: PRD-009
// @trace FR: FR-2
// @trace file-type: impl
// =============================================================================

use std::{io,
          net::{SocketAddr,
                TcpListener,
                TcpStream},
          time::{Duration,
                 Instant}};

/// 사용 가능한 로컬호스트 TCP 포트를 하나 선택해서 반환한다.
///
/// OS 에 `127.0.0.1:0` 으로 바인드 요청한 뒤 즉시 해제해 포트 번호만 얻는다.
/// 데스크톱 앱처럼 "빈 포트 확보 후 곧바로 서버를 띄우는" 패턴에서 사용한다.
///
/// @trace SPEC: SPEC-009
/// @trace TC: SPEC-009/TC-2
/// @trace FR: PRD-009/FR-2
pub fn pick_free_port() -> io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

/// 지정 호스트/포트에 TCP 연결 가능해질 때까지 대기한다. 타임아웃 시 false.
///
/// 데스크톱 부팅 순서에서 서버 `bind` 완료 전에 WebView 가 연결을 시도하는
/// 레이스를 방지하기 위해 사용한다.
///
/// @trace SPEC: SPEC-009
/// @trace TC: SPEC-009/TC-3, SPEC-009/TC-4
/// @trace FR: PRD-009/FR-2
pub fn wait_for_port(host: &str, port: u16, timeout_ms: u64) -> bool {
    let addr: SocketAddr = match format!("{host}:{port}").parse() {
        | Ok(a) => a,
        | Err(_) => return false,
    };
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(timeout_ms) {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}

#[cfg(test)]
mod tests {
    // =============================================================================
    // @trace SPEC-009
    // @trace PRD: PRD-009
    // @trace FR: FR-1, FR-2
    // @trace file-type: test
    // =============================================================================

    use super::*;

    /// @trace TC: SPEC-009/TC-1
    /// @trace FR: PRD-009/FR-1
    /// @trace scenario: lib 경로로 web::build_router 접근
    #[test]
    fn test_tc_1_lib_reexports_web_module() {
        // 라이브러리 모드에서 `web::build_router` 심볼이 공개되어 있는지 컴파일 타임
        // 검증.
        let state = crate::web::AppState {
            scenarios_dir: std::path::PathBuf::from("."),
            reports_dir: std::path::PathBuf::from("."),
            golden_sets_dir: std::path::PathBuf::from("."),
            trajectories_dir: std::path::PathBuf::from("."),
            store: None,
        };
        let _router: axum::Router = crate::web::build_router(state);
    }

    /// @trace TC: SPEC-009/TC-2
    /// @trace FR: PRD-009/FR-2
    /// @trace scenario: pick_free_port 유효 포트 반환
    #[test]
    fn test_tc_2_pick_free_port_returns_usable_port() {
        let port = pick_free_port().expect("OS must allocate a port");
        assert!(port > 0);
        // 동일 포트를 바로 다시 요청하면 일반적으로 다른 값이 나오거나 재사용될 수 있다
        // — 핵심은 > 0 이고, 뒤이어 그 포트에 바인드할 수 있다는 것.
        let bound = TcpListener::bind(("127.0.0.1", port));
        assert!(bound.is_ok(), "port {port} should be bindable");
    }

    /// @trace TC: SPEC-009/TC-3
    /// @trace FR: PRD-009/FR-2
    /// @trace scenario: wait_for_port 타임아웃 false
    #[test]
    fn test_tc_3_wait_for_port_times_out() {
        // 사용하지 않을 높은 포트 + 짧은 타임아웃
        let port = pick_free_port().unwrap();
        // 포트 확보 후 아무도 바인드하지 않은 상태에서 대기 → false
        let ok = wait_for_port("127.0.0.1", port, 200);
        assert!(!ok, "no listener on {port}, expected timeout");
    }

    /// @trace TC: SPEC-009/TC-4
    /// @trace FR: PRD-009/FR-2
    /// @trace scenario: wait_for_port 실제 리스너 감지
    #[test]
    fn test_tc_4_wait_for_port_detects_listener() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        // listener 가 살아있는 상태에서 wait_for_port 는 즉시 true
        assert!(wait_for_port("127.0.0.1", port, 1000));
        drop(listener);
    }
}
