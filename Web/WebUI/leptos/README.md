# Leptos 학습 시리즈

Leptos 프레임워크를 사용한 Rust 웹 프로젝트 모음입니다. CSR(클라이언트 사이드 렌더링)과 SSR(서버 사이드 렌더링) 예제를 포함합니다.

## 프로젝트 구조

```
leptos/
├── leptos-csr/              # Leptos CSR 튜토리얼 (Trunk 기반)
├── leptos-csr-extern-api/   # Leptos CSR + 외부 API 연동 예제
└── leptos-ssr/              # Leptos SSR 예제 (Actix-web 기반)
```

## 사전 준비

### 툴체인 설정

```shell
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
```

### Leptos CSR용 도구 설치

```shell
cargo install cargo-generate
cargo install trunk
```

### Leptos SSR용 도구 설치

```shell
cargo install cargo-leptos
```

### 코드 포맷터 설치

```shell
cargo install leptosfmt
```

## 실행 방법

### CSR 프로젝트 (leptos-csr, leptos-csr-extern-api)

```shell
cd leptos-csr  # 또는 leptos-csr-extern-api
trunk serve --port 3000 --open
```

### SSR 프로젝트 (leptos-ssr)

```shell
cd leptos-ssr
cargo leptos watch
```

접속 주소: `http://localhost:3000`

## 새 프로젝트 시작하기

### CSR 프로젝트 생성

```shell
cargo init <project-name>
cd <project-name>
cargo add leptos --features=csr,nightly
cargo add leptos_meta --features=csr,nightly
cargo add leptos_router --features=csr,nightly
cargo add console_error_panic_hook
```

### SSR 프로젝트 생성

```shell
cargo leptos new --git https://github.com/leptos-rs/start
cd <project-name>
cargo leptos watch
```

### 설정 파일

`rust-toolchain.toml` 내용 추가

```toml
[toolchain]
  channel    = "nightly"
  components = ["clippy", "rust-analyzer", "rust-src", "rustfmt"]
  profile    = "default"
```

`rustfmt.toml` 내용 추가

```toml
# Stable options
editon       = "2021"
max_width    = 80
# Unstable options
format_code_in_doc_comments = true
format_strings              = true
group_imports               = "One"
imports_granularity         = "Crate"
```

`Makefile.toml` 내용 추가

```toml
[tasks.build]
  args       = ["build"]
  command    = "cargo"
[tasks.run]
  args       = ["run"]
  command    = "cargo"
[tasks.test]
  args       = ["test"]
  command    = "cargo"
[tasks.start]
  args       = ["serve", "--port", "3000"]
  command    = "trunk"
[tasks.default]
  dependencies = [
    "build",
    "run",
    "test",
    "start",
  ]
```

```shell
cargo make
# or
cargo make build
# or
cargo make run
# or
cargo make test
```

### CSR용 `index.html` 파일

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <emta charset="utf-8"/>
    <link data-trunk rel="rust" data-wasm-opt="z"/>
    <link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
    <link data-trunk rel="tailwind-css" href="/style/tailwind.css"/>
    <title>Leptos CSR</title>
  </head>
  <body>
  </body>
</html>
```

### CSR용 `src/main.rs` 예시

```rust
use leptos::*;
fn main() {
  mount_to_body(|| view! {
    <p>
      "Hello, Leptos!"
    </p>
  })
}
```

### Tailwind CSS 설정

```shell
mkdir public
mkdir style
touch tailwind.config.js
```

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*/rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
}
```

## 참고 자료

- [Leptos 공식 문서](https://book.leptos.dev/)
- [brookjeynes.dev](https://brookjeynes.dev/posts/learning-leptos-part1/)
