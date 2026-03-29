<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# leptos-csr-extern-api

Leptos CSR (클라이언트 사이드 렌더링) + 외부 API 연동 예제 프로젝트입니다.
[Trunk](https://github.com/trunk-rs/trunk) 빌드 도구를 사용합니다.

## 프로젝트 구조

```
leptos-csr-extern-api/
├── src/
│   ├── main.rs              # 진입점
│   ├── lib.rs               # App 컴포넌트 및 라우팅 정의
│   ├── components/
│   │   └── counter_btn.rs   # 카운터 버튼 컴포넌트
│   └── pages/
│       ├── home.rs          # 홈 페이지
│       ├── api.rs           # 외부 API 호출 페이지
│       └── not_found.rs     # 404 페이지
├── index.html
├── Cargo.toml
└── Trunk.toml
```

## 주요 의존성

| 크레이트 | 버전 | 용도 |
|---------|------|------|
| `leptos` | 0.6 | 반응형 웹 프레임워크 (CSR, nightly) |
| `leptos_meta` | 0.6 | 메타 태그 관리 |
| `leptos_router` | 0.6 | 클라이언트 사이드 라우팅 |
| `gloo-net` | 0.6.0 | HTTP 클라이언트 (WASM용) |
| `gloo-timers` | 0.2 | 타이머 유틸리티 |
| `serde` / `serde_json` | 1.0.215 / 1.0.132 | 직렬화/역직렬화 |

## 사전 준비

1. Rust nightly 툴체인 설치

   ```bash
   rustup toolchain install nightly --allow-downgrade
   ```

2. WebAssembly 타겟 추가

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Trunk 설치

   ```bash
   cargo install trunk
   ```

4. (선택) `cargo-generate` 설치

   ```bash
   cargo install cargo-generate
   ```

## 개발 모드 실행

```bash
trunk serve --port 3000 --open
```

기본 접속 주소: `http://localhost:3000`

## 릴리즈 빌드

```bash
trunk build --release
```

빌드 결과물은 `dist` 폴더에 생성되며, 정적 파일 호스팅 서비스에 배포할 수 있습니다.

## 주요 기능 / 라우트

- `/` - 홈 페이지
- `/api` - 외부 API 호출 페이지 (JSONPlaceholder Todos API)
- `/*` - 404 Not Found

### API 페이지 (`/api`)

"Fetch Todos" 버튼을 클릭하면 `https://jsonplaceholder.typicode.com/todos`에서 Todo 목록을 가져와 화면에 표시합니다.

## 참고 자료

- [Leptos 공식 문서](https://book.leptos.dev/)
- [Trunk 공식 문서](https://trunkrs.dev/)
- [CSR 배포 가이드](https://book.leptos.dev/deployment/csr.html)

## Licensing

This template itself is released under the Unlicense.
