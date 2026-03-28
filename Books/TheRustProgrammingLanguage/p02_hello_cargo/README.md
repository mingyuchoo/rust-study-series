# p02_hello_cargo

> The Rust Programming Language - Hello Cargo 예제

## 프로젝트 설명

Cargo를 사용하여 Rust 프로젝트를 생성하고 빌드하는 방법을 학습하는 프로젝트입니다.
변수와 함수의 기본 사용법을 다룹니다.

## 프로젝트 구조

```
p02_hello_cargo/
  Cargo.toml
  src/
    main.rs
    basic/
      mod.rs
      vars.rs    # 변수 관련 예제
      funs.rs    # 함수 관련 예제
```

## 주요 의존성

- Rust Edition: 2024
- 외부 의존성 없음

## 빌드 및 실행 방법

```bash
rustup default stable
rustup update stable

cargo check
cargo build --profile dev     # 개발용 빌드
cargo build --profile release # 릴리즈용 빌드
cargo test
cargo run
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
cargo watch -x 'run --bin app'

# 인수를 전달하여 실행
cargo watch -x 'run -- --some-arg'

# 임의의 명령어 실행
cargo watch -- echo Hello world

# feature를 전달하여 실행
cargo watch --features "foo,bar"
```

## 참고 자료

- <https://learning-rust.github.io/>
