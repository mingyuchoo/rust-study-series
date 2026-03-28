# p16_crates_io

> The Rust Programming Language - Crates.io와 워크스페이스

## 프로젝트 설명

Rust 워크스페이스 구성 방법과 여러 크레이트를 함께 관리하는 방법을 학습하는 프로젝트입니다.

## 프로젝트 구조

```
p16_crates_io/
  Cargo.toml          # 워크스페이스 설정
  adder/
    Cargo.toml
    src/main.rs        # 바이너리 크레이트
  add_one/
    Cargo.toml
    src/lib.rs         # 라이브러리 크레이트
  art/
    Cargo.toml
    src/lib.rs         # 라이브러리 크레이트
```

## 워크스페이스 멤버

- `adder` - 바이너리 크레이트
- `add_one` - 라이브러리 크레이트
- `art` - 라이브러리 크레이트

## Rust 워크스페이스 생성 방법

```bash
rustup default stable
rustup update stable

mkdir <workspace_name>
cd <workspace_name>
touch Cargo.toml
```

`<workspace_name>/Cargo.toml` 편집

```toml
[workspace]

members = [
  "<binary_crate_name>",
]
```

바이너리 크레이트 추가

```bash
cargo new <binary_crate_name>
```

`<workspace_name>/Cargo.toml` 편집

```toml
[workspace]

members = [
  "<binary_crate_name>",
  "<library_crate_name>",
]
```

라이브러리 크레이트 추가

```bash
cargo new <library_crate_name> --lib
```

## 빌드 방법

```bash
cargo clean
cargo fmt
cargo test
cargo doc --no-deps --open  # `target/doc/<package_name>/index.html`
cargo build --profile dev     # 개발용 빌드
cargo build --profile release # 릴리즈용 빌드
cargo run -p adder # 또는 cargo run
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

## 참고 사항

**Rust 크레이트 이름에는 하이픈을 사용할 수 없지만, Cargo 패키지 이름에는 허용됩니다.**

- <https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md>
