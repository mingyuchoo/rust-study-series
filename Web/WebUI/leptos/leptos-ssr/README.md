# Leptos SSR

Leptos 프레임워크 SSR(서버 사이드 렌더링) 예제 프로젝트입니다.
[Actix-web](https://actix.rs/) 웹 서버와 함께 동작하며, 하이드레이션을 지원합니다.

## 프로젝트 구조

```
leptos-ssr/
├── src/
│   ├── main.rs              # 서버 진입점 (Actix-web)
│   ├── lib.rs               # hydrate 함수
│   ├── app.rs               # App 컴포넌트 및 라우팅 정의
│   ├── client/
│   │   └── pages/
│   │       ├── home.rs      # 홈 페이지
│   │       ├── todos.rs     # Todos 페이지
│   │       └── not_found.rs # 404 페이지
│   ├── models/
│   │   └── todo.rs          # Todo 모델
│   └── server/
│       └── todo.rs          # 서버 함수 (fetch_todos)
├── assets/                  # 정적 자원 (todos.json 등)
├── end2end/                 # E2E 테스트 (Playwright)
├── style/                   # SCSS 스타일
├── Cargo.toml
└── LICENSE
```

## 사전 준비

```shell
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
npm install -g sass
```

## 실행 방법

### 개발 모드

```shell
cargo leptos watch
```

접속 주소: `http://localhost:3000`

### 릴리즈 빌드

```shell
cargo leptos build --release
```

빌드 결과물:
1. 서버 바이너리: `target/server/release`
2. 사이트 디렉터리: `target/site`

### 원격 서버 실행

```shell
export LEPTOS_OUTPUT_NAME="leptos-ssr"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```

## 주요 의존성

| 크레이트 | 버전 | 용도 |
|---------|------|------|
| `leptos` | 0.6 | 반응형 웹 프레임워크 (SSR/CSR/Hydrate) |
| `leptos_meta` | 0.6 | 메타 태그 관리 |
| `leptos_router` | 0.6 | 라우팅 |
| `leptos_actix` | 0.6 | Actix-web 통합 |
| `actix-web` | 4 | HTTP 웹 서버 |
| `actix-files` | 0.6 | 정적 파일 서빙 |
| `serde` / `serde_json` | 1.0.215 / 1.0.133 | 직렬화/역직렬화 |
| `thiserror` | 2.0.3 | 에러 타입 정의 |
| `wasm-bindgen` | 0.2.95 | WASM 바인딩 |

## 주요 기능 / 라우트

- `/` - 홈 페이지
- `/todos` - Todos 페이지 (서버에서 `assets/todos.json` 파일 로드)
- `/*` - 404 Not Found

### 서버 함수

- `fetch_todos()` - `assets/todos.json` 파일에서 Todo 목록을 읽어 반환하는 서버 함수

## E2E 테스트

```shell
npx playwright test
```
