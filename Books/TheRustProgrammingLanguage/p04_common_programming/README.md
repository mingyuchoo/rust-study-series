# p04_common_programming

> The Rust Programming Language - 일반적인 프로그래밍 개념

## 프로젝트 설명

Rust의 기본 프로그래밍 개념(변수, 데이터 타입, 함수, 흐름 제어)을 학습하는 프로젝트입니다.

## 프로젝트 구조

```
p04_common_programming/
  Cargo.toml
  src/
    main.rs
    variables.rs      # 변수 관련 예제
    data_types.rs     # 데이터 타입 예제
    functions.rs      # 함수 예제
    flow_control.rs   # 흐름 제어 예제
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
cargo doc --no-deps --open  # `target/doc/p04_common_programming/index.html`
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
