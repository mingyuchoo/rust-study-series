# p05_ownership

> The Rust Programming Language - 소유권 (Ownership)

## 프로젝트 설명

Rust의 핵심 개념인 소유권(Ownership) 시스템을 학습하는 프로젝트입니다.

## 소유권 규칙

- Rust의 각 값에는 소유자(owner)가 있다.
- 한 번에 하나의 소유자만 존재할 수 있다.
- 소유자가 스코프를 벗어나면 값은 삭제(drop)된다.

## 프로젝트 구조

```
p05_ownership/
  Cargo.toml
  src/
    main.rs
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

## 선택 명령어

### 프로젝트 의존성 업데이트

```bash
cargo update
```

### HTML 문서 생성 및 웹 브라우저로 열기

```bash
cargo doc --no-deps --open  # `target/doc/p05_ownership/index.html`
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
