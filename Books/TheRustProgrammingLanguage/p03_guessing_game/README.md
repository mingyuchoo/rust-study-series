# p03_guessing_game

> The Rust Programming Language - 숫자 맞히기 게임

## 프로젝트 설명

사용자 입력을 받아 임의의 숫자를 맞히는 게임입니다.
`rand` 크레이트를 사용한 난수 생성, 사용자 입력 처리, 반복문, 비교 연산 등을 학습합니다.

## 프로젝트 구조

```
p03_guessing_game/
  Cargo.toml
  src/
    main.rs
```

## 주요 의존성

- Rust Edition: 2024
- `rand` = "0.6.5"

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
cargo doc --no-deps --open  # `target/doc/p03_guessing_game/index.html`
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
