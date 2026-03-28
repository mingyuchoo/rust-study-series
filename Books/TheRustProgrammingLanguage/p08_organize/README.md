# p08_organize

> The Rust Programming Language - 패키지, 크레이트, 모듈 관리

## 프로젝트 설명

Rust의 모듈 시스템을 학습하는 프로젝트입니다.
`mod`, `pub`, `use`, `as`, `super`, `pub use` 키워드의 사용법을 다룹니다.

## 프로젝트 구조

```
p08_organize/
  Cargo.toml
  src/
    main.rs
    mod_keyword.rs       # mod 키워드 예제
    pub_keyword.rs       # pub 키워드 예제
    use_keyword.rs       # use 키워드 예제
    as_keyword.rs        # as 키워드 예제
    super_keyword.rs     # super 키워드 예제
    pub_use_keyword.rs   # pub use 키워드 예제
```

## 주요 의존성

- Rust Edition: 2024
- `rand` = "0.8.5"

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
cargo doc --no-deps --open  # `target/doc/p08_organize/index.html`
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
