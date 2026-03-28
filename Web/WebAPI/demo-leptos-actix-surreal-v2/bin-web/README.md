<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# bin-web (demo-leptos-actix-surreal-v2)

Leptos + Actix-Web 기반 풀스택 웹 애플리케이션의 프론트엔드/서버 바이너리 크레이트입니다.
워크스페이스 내 `lib-adder`, `lib-repo` 라이브러리를 사용합니다.

## 주요 의존성

- `leptos` 0.6.5 - 반응형 웹 프레임워크 (nightly 기능)
- `leptos_actix` 0.6.5 - Actix-Web 서버 통합
- `leptos_meta` 0.6.5 - 메타 태그 관리
- `leptos_router` 0.6.5 - 클라이언트 사이드 라우팅
- `actix-web` 4.9.0 - 웹 서버 (SSR 모드)
- `lib-adder` - 워크스페이스 내 유틸리티 라이브러리
- `lib-repo` - 워크스페이스 내 저장소 라이브러리

## 기능 (Feature Flags)

- `csr` - 클라이언트 사이드 렌더링
- `hydrate` - 하이드레이션
- `ssr` - 서버 사이드 렌더링 (Actix-Web 기반)

## 사전 준비

1. Rust nightly 툴체인 설치

   ```bash
   rustup toolchain install nightly --allow-downgrade
   ```

2. WebAssembly 타겟 추가

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. `cargo-leptos` 설치

   ```bash
   cargo install cargo-leptos --locked
   ```

## 실행 방법

```bash
cargo leptos watch
```

기본 접속 주소: `http://localhost:3000`

## 릴리즈 빌드

```bash
cargo leptos build --release
```

## 원격 서버 배포

빌드 후 필요한 파일:

1. `target/server/release` 의 서버 바이너리
2. `target/site` 디렉토리 전체

환경 변수 설정:

```bash
export LEPTOS_OUTPUT_NAME="bin_web"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```

## Licensing

This template itself is released under the Unlicense.
