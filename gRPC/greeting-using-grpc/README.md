# gRPC Greeting Service — Cargo Workspace

Rust로 구현된 gRPC Greeting 서비스 학습 프로젝트입니다. Cargo workspace(모노레포) 구조로 관리됩니다.

## 프로젝트 구조

```
greeting-using-grpc/
├── Cargo.toml               # Workspace 루트 설정
├── proto/
│   └── greeter.proto        # Protobuf 서비스 정의
├── crates/
│   ├── proto/               # Proto 코드 생성 크레이트
│   │   ├── build.rs
│   │   └── src/lib.rs
│   ├── common/              # 공통 에러 타입
│   │   └── src/
│   │       ├── lib.rs
│   │       └── error.rs
│   ├── server/              # gRPC 서버
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── lib.rs
│   │   │   └── service.rs
│   │   ├── examples/
│   │   │   ├── simple_server.rs
│   │   │   ├── custom_port_server.rs
│   │   │   └── greeting_logic.rs
│   │   ├── benches/
│   │   │   └── server_benchmark.rs
│   │   └── tests/
│   │       └── server_service_test.rs
│   └── client/              # gRPC 클라이언트
│       ├── src/
│       │   ├── main.rs
│       │   ├── lib.rs
│       │   └── service.rs
│       ├── examples/
│       │   ├── simple_client.rs
│       │   ├── basic_client.rs
│       │   ├── advanced_client.rs
│       │   ├── multiple_requests.rs
│       │   └── concurrent_requests.rs
│       ├── benches/
│       │   ├── client_benchmark.rs
│       │   └── concurrent_benchmark.rs
│       └── tests/
│           └── integration_test.rs
└── docs/
    ├── ARCHITECTURE.md
    ├── WORKSPACE.md
    └── BENCHES_EXAMPLES.md
```

## 크레이트 구성

| 크레이트 | 역할 |
|---|---|
| `greeting-proto` | `.proto` 파일 컴파일 및 gRPC 스텁 생성 |
| `greeting-common` | 공통 에러 타입(`AppError`, `AppResult`) |
| `greeting-server` | gRPC 서버 구현 및 바이너리 |
| `greeting-client` | gRPC 클라이언트 구현 및 바이너리 |

## 주요 의존성

| 패키지 | 버전 | 역할 |
|---|---|---|
| `tonic` | 0.14.5 | gRPC 프레임워크 |
| `prost` | 0.14.3 | Protocol Buffers 구현 |
| `tokio` | 1.50.0 | 비동기 런타임 |
| `tracing` | 0.1.44 | 구조화 로깅 |
| `thiserror` | 2.0.18 | 에러 타입 파생 |

## 사전 요구사항

### macOS

```bash
brew install protobuf
```

### Ubuntu / Debian

```bash
sudo apt-get install protobuf-compiler
```

## 빌드

```bash
# 전체 워크스페이스 빌드
cargo build --workspace

# 개별 크레이트 빌드
cargo build -p greeting-server
cargo build -p greeting-client

# 릴리스 빌드
cargo build --workspace --release
```

## 실행

터미널 두 개를 열어 서버와 클라이언트를 각각 실행합니다.

### 서버 실행

```bash
cargo run -p greeting-server --bin server
```

기본 주소: `[::1]:50051`

### 클라이언트 실행

```bash
cargo run -p greeting-client --bin client
```

### 로그 레벨 설정

`tracing`을 사용하므로 `RUST_LOG` 환경 변수로 로그 레벨을 조절할 수 있습니다.

```bash
RUST_LOG=info cargo run -p greeting-server --bin server
RUST_LOG=debug cargo run -p greeting-server --bin server
```

## 서버 예제

```bash
# 기본 서버 (포트 50051)
cargo run -p greeting-server --example simple_server

# 커스텀 포트 서버 (포트 50052)
cargo run -p greeting-server --example custom_port_server

# 서버 없이 비즈니스 로직만 단독 실행
cargo run -p greeting-server --example greeting_logic
```

## 클라이언트 예제

> 클라이언트 예제는 서버가 실행 중이어야 합니다.

```bash
# 기본 클라이언트
cargo run -p greeting-client --example simple_client

# 단일 요청 예제
cargo run -p greeting-client --example basic_client

# 에러 처리 포함 고급 예제
cargo run -p greeting-client --example advanced_client

# 순차 다중 요청
cargo run -p greeting-client --example multiple_requests

# 동시 다중 요청 (JoinSet 활용)
cargo run -p greeting-client --example concurrent_requests
```

## 테스트

```bash
# 전체 테스트
cargo test --workspace

# 크레이트별 테스트
cargo test -p greeting-server
cargo test -p greeting-client
```

통합 테스트(`integration_test.rs`)는 서버 프로세스 없이 독립 실행됩니다. 랜덤 포트로 인-프로세스 서버를 띄워 테스트합니다.

## 벤치마크

> 벤치마크 실행 전 서버가 실행 중이어야 합니다.

```bash
# 전체 벤치마크
cargo bench --workspace

# 서버 벤치마크 (greeting 처리 로직)
cargo bench -p greeting-server

# 클라이언트 벤치마크 (단일 요청)
cargo bench -p greeting-client --bench client_benchmark

# 클라이언트 동시성 벤치마크
cargo bench -p greeting-client --bench concurrent_benchmark
```

## 공개 API

### `greeting-server`

```rust
// 소켓 주소 파싱 (동기)
pub fn parse_socket_address(addr_str: &str) -> AppResult<SocketAddr>

// greeting 요청 처리 (동기)
pub fn process_greeting_request(name: String) -> AppResult<HelloResponse>

// 서버 시작 (비동기)
pub async fn start_server(addr: SocketAddr) -> AppResult<()>
```

### `greeting-client`

```rust
// 기본 주소(http://[::1]:50051)로 연결 (비동기)
pub async fn connect_client() -> AppResult<GreeterClient<Channel>>

// 임의 주소로 연결 (비동기)
pub async fn connect_client_at(url: &str) -> AppResult<GreeterClient<Channel>>

// 요청 전송 (비동기)
pub async fn create_and_send_request(client, name) -> AppResult<Response<HelloResponse>>

// 응답에서 메시지 추출 (동기)
pub fn process_response(response: Response<HelloResponse>) -> AppResult<String>
```

## 문서

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — 크레이트 의존 그래프 및 데이터 흐름
- [docs/WORKSPACE.md](docs/WORKSPACE.md) — 워크스페이스 상세 구조
- [docs/BENCHES_EXAMPLES.md](docs/BENCHES_EXAMPLES.md) — 벤치마크 및 예제 상세 가이드
