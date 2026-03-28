# p01_hello_world

> The Rust Programming Language - Hello World 예제

## 프로젝트 설명

Rust의 기본 출력 매크로(`println!`, `print!`, `format!`)와 디버그 출력(`{:?}`, `{:#?}`) 사용법을 학습하는 프로젝트입니다.
Cargo를 사용하지 않고 `rustc`로 직접 컴파일하는 방식입니다.

## 프로젝트 구조

```
p01_hello_world/
  main.rs       # 메인 소스 파일
  README.md
```

## 빌드 및 실행 방법

```bash
rustup default stable
rustup update stable
rustc main.rs
./main
```

## 주요 기능

- `println!` 매크로를 활용한 다양한 출력 형식
- `format!` 매크로를 사용한 문자열 포맷팅
- `{:?}`, `{:#?}`를 사용한 디버그 출력

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
