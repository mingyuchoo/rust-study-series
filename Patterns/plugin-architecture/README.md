# Rust 플러그인 아키텍처 시스템

런타임에 기능을 확장할 수 있는 Rust 동적 플러그인 시스템입니다. Cargo 워크스페이스 구조를 사용하며, 핵심 애플리케이션이 재컴파일 없이 플러그인을 로드하고 실행할 수 있습니다.

## 아키텍처

```bash
plugin-architecture/
├── Cargo.toml           # 워크스페이스 루트 (virtual manifest)
├── Cargo.lock
├── crates/
│   ├── plugin-interface/     # 플러그인용 공유 트레이트 정의
│   ├── plugin-manager/       # 플러그인 관리 라이브러리
│   └── plugins/              # 플러그인 구현체
│       ├── hello-plugin/     # 예제: 간단한 인사 플러그인
│       └── math-plugin/      # 예제: 수학 연산 플러그인
├── apps/
│   └── cli/                  # 플러그인을 로드하고 관리하는 메인 애플리케이션
└── target/debug/
    └── plugins/              # 런타임 플러그인 라이브러리 디렉토리
```

## 빌드 방법

### 사전 요구사항

- Rust 툴체인 (1.70 이상 권장)
- Cargo

### 프로젝트 빌드

1. **전체 워크스페이스 빌드:**

   ```bash
   cargo build
   ```

2. **플러그인을 런타임 디렉토리에 배포:**

   ```bash
   ./deploy-plugins.sh
   ```

   Windows:

   ```cmd
   deploy-plugins.bat
   ```

3. **핵심 애플리케이션 실행:**
   ```bash
   cargo run --bin cli
   ```

### 빌드 순서

워크스페이스가 자동으로 빌드 의존성을 관리합니다:

1. `plugin-interface` 먼저 빌드 (모든 크레이트의 의존성)
2. `plugin-manager` 라이브러리 빌드
3. 플러그인 구현체 빌드
4. `cli` 애플리케이션 마지막으로 빌드

## 애플리케이션 실행

빌드 및 플러그인 배포 후:

```bash
cargo run --bin cli
```

애플리케이션은 다음을 수행합니다:

1. `target/debug/plugins/` 에서 플러그인 탐색
2. 각 플러그인을 동적으로 로드
3. 플러그인 기능 실행
4. 종료 시 플러그인 정리 및 언로드

## 새 플러그인 만들기

1. **plugins 디렉토리에 새 크레이트 생성:**

   ```bash
   cd crates/plugins
   cargo new --lib my-plugin
   ```

2. **Cargo.toml 설정:**

   ```toml
   [package]
   name = "my-plugin"
   version = "0.1.0"
   edition = "2024"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   plugin-interface = { path = "../../plugin-interface" }
   ```

3. **Plugin 트레이트 구현:**

   ```rust
   use plugin_interface::{Plugin, PluginContext};
   use std::error::Error;

   pub struct MyPlugin;

   impl Plugin for MyPlugin {
       fn name(&self) -> &str { "My Plugin" }
       fn version(&self) -> &str { "0.1.0" }
       fn description(&self) -> &str { "플러그인 설명" }

       fn on_load(&mut self) -> Result<(), Box<dyn Error>> {
           println!("MyPlugin 로드됨");
           Ok(())
       }

       fn execute(&self, context: &PluginContext) -> Result<String, Box<dyn Error>> {
           Ok("플러그인 실행 성공".to_string())
       }

       fn on_unload(&mut self) -> Result<(), Box<dyn Error>> {
           println!("MyPlugin 언로드됨");
           Ok(())
       }
   }

   #[no_mangle]
   pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
       Box::into_raw(Box::new(MyPlugin))
   }
   ```

4. **루트 Cargo.toml의 workspace members에 추가:**

   ```toml
   [workspace]
   members = [
       "crates/plugin-interface",
       "crates/plugin-manager",
       "crates/plugins/hello-plugin",
       "crates/plugins/math-plugin",
       "crates/plugins/my-plugin",  # 새 플러그인 추가
       "apps/cli",
   ]
   ```

5. **빌드 및 배포:**
   ```bash
   cargo build
   ./deploy-plugins.sh
   ```

## 플랫폼별 참고사항

- **Linux**: `.so` 확장자 (예: `libhello_plugin.so`)
- **Windows**: `.dll` 확장자 (예: `hello_plugin.dll`)
- **macOS**: `.dylib` 확장자 (예: `libhello_plugin.dylib`)

## 문제 해결

### 플러그인이 로드되지 않는 경우

- `crate-type = ["cdylib"]` 설정 확인
- 플러그인 라이브러리가 `target/debug/plugins/`에 있는지 확인
- `_plugin_create` 함수에 `#[no_mangle]` 및 `extern "C"` 적용 확인

### 심볼을 찾을 수 없는 경우

- plugin-interface 버전이 일치하는지 확인
- 전체 재빌드: `cargo clean && cargo build`

## 테스트

```bash
# 전체 워크스페이스 테스트
cargo test

# 특정 크레이트 테스트
cargo test -p plugin-interface
cargo test -p plugin-manager
cargo test -p hello-plugin
cargo test -p cli
```

## License

[Your License Here]
