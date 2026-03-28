# demo-actix-web

`actix-web` 프레임워크를 사용한 간단한 웹 API 예제 프로젝트입니다.

## 프로젝트 구조

```text
src/
└── main.rs    # 메인 진입점
```

## 주요 의존성

- `actix-web` 4.9.0 - 웹 프레임워크
- `clap` 4.5.20 (derive 기능) - 명령줄 인자 파싱

## 빌드 및 실행

```bash
cargo check
cargo test
cargo run -- --port 8080
```

## 릴리즈 빌드

```bash
cargo build --release
./target/release/demo-actix-web --port 8080
```
