# SPEC-009: Tauri + 내장 Axum 통합

## 메타데이터
- SPEC ID: SPEC-009
- PRD: PRD-009
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향
| PRD | FR | 요구사항 |
|-----|----|---------|
| PRD-009 | FR-1 | lib+bin 전환 |
| PRD-009 | FR-2 | pick_free_port / wait_for_port |
| PRD-009 | FR-3 | desktop/ 스캐폴드 |
| PRD-009 | FR-4 | Tauri 메인이 Axum 서버 + WebView 로드 |
| PRD-009 | FR-5 | README 업데이트 |

### 역방향
| TC | 시나리오 | FR | 테스트 파일 |
|----|---------|----|-----------|
| TC-1 | lib 경로로 web::build_router 접근 가능 | FR-1 | crates/eval-harness/src/desktop_helpers.rs |
| TC-2 | pick_free_port 는 > 0 포트 반환 | FR-2 | crates/eval-harness/src/desktop_helpers.rs |
| TC-3 | wait_for_port 가 사용 불가 포트에 대해 타임아웃 false | FR-2 | crates/eval-harness/src/desktop_helpers.rs |
| TC-4 | wait_for_port 가 실제 리스너 감지 시 true | FR-2 | crates/eval-harness/src/desktop_helpers.rs |
| TC-5 | desktop/Cargo.toml + tauri.conf.json 파일 존재 | FR-3 | (파일 존재 검증, 수동/verify_trace) |
| TC-6 | desktop/src/main.rs 에 build_router/run_server 사용 키워드 | FR-4 | (수동 grep + 빌드 성공 시) |
| TC-7 | README 에 `cargo tauri dev` 포함 | FR-5 | (grep) |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/lib.rs (신규) | `pub mod tui; pub mod web; pub mod desktop_helpers` | FR-1 |
| eval-harness | src/desktop_helpers.rs (신규) | `pick_free_port`, `wait_for_port` + tests | FR-2 |
| eval-harness | src/main.rs | `use eval_harness::{tui, web};` 전환 | FR-1 |
| (external) | desktop/Cargo.toml | tauri + eval-harness path dep | FR-3 |
| (external) | desktop/tauri.conf.json | windows:[], dynamic webview | FR-3 |
| (external) | desktop/build.rs | tauri_build::build() | FR-3 |
| (external) | desktop/src/main.rs | setup hook → pick_free_port → spawn server → WebviewWindowBuilder | FR-4 |
| (external) | desktop/dist/index.html | 플레이스홀더 | FR-3 |
| (root) | Cargo.toml | `[workspace] exclude = ["desktop"]` | FR-3 |
| (root) | README.md | 데스크톱 섹션 | FR-5 |

## 기술 설계

### lib+bin 전환 (FR-1)
```rust
// crates/eval-harness/src/lib.rs (신규)
pub mod tui;
pub mod web;
pub mod desktop_helpers;
```
`main.rs` 는 `mod tui; mod web;` 제거 후 `use eval_harness::{tui, web};`. Cargo 는 `src/lib.rs` + `src/main.rs` 공존을 자동 감지하므로 Cargo.toml 변경 불필요.

### desktop_helpers (FR-2)
```rust
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::time::{Duration, Instant};

pub fn pick_free_port() -> io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

pub fn wait_for_port(host: &str, port: u16, timeout_ms: u64) -> bool {
    let addr: SocketAddr = format!("{host}:{port}").parse().unwrap_or_else(|_| "127.0.0.1:0".parse().unwrap());
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(timeout_ms) {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}
```

### desktop/src/main.rs 핵심
```rust
use eval_harness::{desktop_helpers, web};
use std::path::PathBuf;
use tauri::{WebviewUrl, WebviewWindowBuilder};

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn main() {
    let root = project_root();
    let port = desktop_helpers::pick_free_port().expect("no free port");
    let addr = format!("127.0.0.1:{port}").parse().unwrap();

    let scen = root.join("eval_data/eval_scenarios");
    let reps = root.join("reporting_logs");
    let gold = root.join("eval_data/golden_sets");
    let traj = root.join("reporting_trajectories");

    std::thread::spawn(move || {
        let _ = web::run_server(addr, scen, reps, gold, traj);
    });

    assert!(desktop_helpers::wait_for_port("127.0.0.1", port, 5000), "server didn't start");

    tauri::Builder::default()
        .setup(move |app| {
            let url: tauri::Url = format!("http://127.0.0.1:{}", port).parse().unwrap();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("eval-harness")
                .inner_size(1280.0, 820.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("tauri run failed");
}
```

### tauri.conf.json 핵심
```json
{
  "productName": "eval-harness",
  "version": "0.1.0",
  "identifier": "com.mingyuchoo.eval-harness",
  "build": { "frontendDist": "../dist" },
  "app": { "windows": [], "security": { "csp": null } },
  "bundle": { "active": false, "targets": "all", "icon": ["icons/icon.png"] }
}
```

### 시스템 의존성
| OS | 필수 패키지 |
|----|------------|
| Ubuntu 22.04+ | `libwebkit2gtk-4.1-dev build-essential libssl-dev libayatana-appindicator3-dev librsvg2-dev` |
| macOS | Xcode Command Line Tools (`xcode-select --install`) |
| Windows | WebView2 (Win11 기본 포함, Win10 설치 필요) + MSVC 빌드 도구 |

## 완료 정의
- `cargo test -p eval-harness` → 기존 + SPEC-009 TC 통과
- `cargo build --workspace` 성공 (desktop 제외됨)
- `desktop/` 스캐폴드 파일 존재 (TC-5, TC-6 grep 확인)
- README 데스크톱 섹션에 `cargo tauri dev` 포함 (TC-7)
- 실제 Tauri 빌드는 시스템 의존성 요구로 문서로만 안내
