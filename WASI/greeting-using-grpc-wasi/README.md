# greeting-using-grpc-wasi

WASI 0.2 Component Model + gRPC 통신 예제 (Rust Mono-repo)

## 아키텍처

```
┌──────────────────────────────────────────────────────────────────┐
│                       Mono Repo Workspace                        │
│                                                                  │
│  ┌─────────────────┐                                             │
│  │ greeting-client │──────────────────┐                         │
│  │  (Native Rust)  │                  │ gRPC                    │
│  └─────────────────┘                  │                         │
│                                       ▼                         │
│  ┌──────────────────────┐   ┌──────────────────────┐            │
│  │ greeting-web-client  │──▶│   greeting-server    │            │
│  │ (SvelteKit + Node.js)│◀──│ (Native Rust + tonic)│            │
│  └──────────────────────┘   └──────────┬───────────┘            │
│           gRPC                         │ wasmtime                │
│                             ┌──────────▼───────────┐            │
│                             │  greeting-component  │            │
│                             │   (WASI 0.2 / WASM)  │            │
│                             │    wasm32-wasip2      │            │
│                             └──────────────────────┘            │
└──────────────────────────────────────────────────────────────────┘
```

## 구성 요소

| 구성 요소 | 설명 | 언어/런타임 | 빌드 타겟 |
|----------|------|------------|----------|
| `greeting-component` | WASI 0.2 비즈니스 로직 컴포넌트 | Rust | `wasm32-wasip2` |
| `greeting-server` | gRPC 서버 (wasmtime으로 WASM 실행) | Rust (tonic) | native |
| `greeting-client` | gRPC CLI 클라이언트 | Rust | native |
| `greeting-web-client` | gRPC 웹 클라이언트 (브라우저 UI) | SvelteKit + Node.js | Node.js |

## WIT 인터페이스

```wit
package component:greeting@0.1.0;

interface greeter {
  greet: func(name: string) -> string;
  get-version: func() -> string;
}

world greeting-world {
  export greeter;
}
```

## gRPC 프로토콜

```protobuf
service GreetingService {
  rpc Greet (GreetRequest) returns (GreetResponse);
  rpc GetVersion (VersionRequest) returns (VersionResponse);
}
```

## 시작하기

### 사전 요구사항

```bash
# Rust 설치 (https://rustup.rs)
# cargo-make 설치
cargo install cargo-make

# Node.js / npm 설치 (https://nodejs.org)

# WASI 타겟 추가
cargo make setup
```

### 빌드

```bash
# 전체 빌드 (WASM 컴포넌트 + 서버 + 클라이언트 + 웹 클라이언트)
cargo make build

# 개별 빌드
cargo make build-component      # WASI 컴포넌트만
cargo make build-server         # gRPC 서버만
cargo make build-client         # gRPC CLI 클라이언트만
cargo make build-web-client     # 웹 클라이언트만 (npm install 포함)
```

### 실행

터미널 1 - gRPC 서버 실행:
```bash
cargo make run-server
```

터미널 2 - CLI 클라이언트 실행:
```bash
cargo make run-client
```

터미널 3 - 웹 클라이언트 실행 (개발 서버):
```bash
cargo make run-web-client
# http://localhost:5173 에서 접속
```

또는 웹 클라이언트 프로덕션 서버:
```bash
cargo make run-web-client-prod
# http://localhost:3000 에서 접속
```

### 예상 출력

서버:
```
2024-03-13T00:00:00Z  INFO greeting_server: Loading WASM component from: "target/wasm32-wasip2/release/greeting_component.wasm"
2024-03-13T00:00:00Z  INFO greeting_server: WASM component loaded successfully
2024-03-13T00:00:00Z  INFO greeting_server: gRPC server listening on 0.0.0.0:50051
```

CLI 클라이언트:
```
Version: greeting-component v0.1.0 (WASI 0.2)
Response: [WASI 0.2 Component] Hello, World! Greetings from WebAssembly System Interface!
Response: [WASI 0.2 Component] Hello, WASI! Greetings from WebAssembly System Interface!
Response: [WASI 0.2 Component] Hello, Rust! Greetings from WebAssembly System Interface!
Response: [WASI 0.2 Component] Hello, gRPC! Greetings from WebAssembly System Interface!
```

## cargo-make 태스크 목록

| 태스크 | 설명 |
|--------|------|
| `cargo make setup` | WASI 빌드 타겟 및 도구 설치 |
| `cargo make build` | 전체 빌드 |
| `cargo make build-component` | WASI 컴포넌트 빌드 |
| `cargo make build-server` | gRPC 서버 빌드 |
| `cargo make build-client` | gRPC CLI 클라이언트 빌드 |
| `cargo make build-web-client` | 웹 클라이언트 빌드 (npm) |
| `cargo make install-web-client` | 웹 클라이언트 npm 의존성 설치 |
| `cargo make run-server` | gRPC 서버 실행 |
| `cargo make run-client` | gRPC CLI 클라이언트 실행 |
| `cargo make run-client-dev` | gRPC CLI 클라이언트 실행 (debug) |
| `cargo make run-web-client` | 웹 클라이언트 개발 서버 실행 (Vite) |
| `cargo make run-web-client-prod` | 웹 클라이언트 프로덕션 서버 실행 |
| `cargo make check` | 전체 코드 오류 검사 |
| `cargo make fmt` | 코드 포맷팅 |
| `cargo make clippy` | Clippy 린트 실행 |
| `cargo make clean` | 빌드 산출물 삭제 |

## 기술 스택

- **Rust** - 시스템 프로그래밍 언어
- **WASI 0.2** - WebAssembly System Interface (Component Model)
- **wasmtime** - WebAssembly 런타임
- **wit-bindgen** - WIT 인터페이스 바인딩 생성
- **tonic** - Rust gRPC 프레임워크
- **prost** - Protocol Buffers 구현
- **tokio** - 비동기 런타임
- **SvelteKit** - 웹 프론트엔드 프레임워크
- **@grpc/grpc-js** - Node.js gRPC 클라이언트 라이브러리

## 동작 원리

1. `greeting-component`가 WIT 인터페이스에 따라 비즈니스 로직을 구현
2. `wasm32-wasip2` 타겟으로 컴파일되어 `.wasm` 파일 생성
3. `greeting-server`가 시작 시 wasmtime으로 `.wasm` 파일 로드
4. gRPC 요청이 들어오면 wasmtime을 통해 WASM 컴포넌트 함수 호출
5. 결과를 gRPC 응답으로 클라이언트에 반환
6. `greeting-web-client`는 브라우저 UI에서 gRPC 서버로 직접 요청 전송
