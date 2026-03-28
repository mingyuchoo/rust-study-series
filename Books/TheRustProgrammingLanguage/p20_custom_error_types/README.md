# p20_custom_error_types

> The Rust Programming Language - 커스텀 에러 타입

## 프로젝트 설명

`thiserror` 크레이트를 활용하여 커스텀 에러 타입을 정의하고 사용하는 방법을 학습하는 프로젝트입니다.

## 프로젝트 구조

```
p20_custom_error_types/
  Cargo.toml
  src/
    main.rs    # 바이너리 진입점
```

## 주요 의존성

- Rust Edition: 2024
- `thiserror` = "2.0.0"

## 빌드 및 실행 방법

```bash
rustup default stable
rustup update stable

cargo check
cargo build --profile dev     # 개발용 빌드
cargo build --profile release # 릴리즈용 빌드
cargo test
cargo run --bin main
```

## 선택 명령어

### 프로젝트 의존성 업데이트

```bash
cargo update
```

### HTML 문서 생성 및 웹 브라우저로 열기

```bash
cargo doc --no-deps --open  # `target/doc/p20_custom_error_types/index.html`
```

## Watch 모드 사용법

### `cargo-watch` 설치

```bash
cargo install cargo-watch
```

### `cargo-watch`로 Watch 모드 실행

```bash
# 테스트만 실행
cargo watch -x test

# check 후 테스트 실행
cargo watch -x check -x test

# 현재 애플리케이션 실행
cargo watch -x 'run --bin main'

# 인수를 전달하여 실행
cargo watch -x 'run -- --some-arg'

# 임의의 명령어 실행
cargo watch -- echo Hello world

# feature를 전달하여 실행
cargo watch --features "foo,bar"
```
