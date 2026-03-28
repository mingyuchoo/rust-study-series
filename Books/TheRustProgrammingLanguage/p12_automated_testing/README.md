# p12_automated_testing

> The Rust Programming Language - 자동화 테스트

## 프로젝트 설명

Rust의 테스트 작성 방법을 학습하는 프로젝트입니다.
단위 테스트, 통합 테스트, 커스텀 메시지, 패닉 테스트 등 다양한 테스트 기법을 다룹니다.

## 프로젝트 구조

```
p12_automated_testing/
  Cargo.toml
  src/
    lib.rs
    basic.rs              # 기본 테스트
    rectangle.rs          # 사각형 관련 테스트
    custom_message.rs     # 커스텀 메시지 테스트
    panic_situation.rs    # 패닉 상황 테스트
    equiv_negativ.rs      # 동등/부정 테스트
    show_test_result.rs   # 테스트 결과 출력
    internal_fn.rs        # 내부 함수 테스트
    exec_based_name.rs    # 이름 기반 실행 테스트
  tests/
    common.rs             # 공통 테스트 유틸리티
    integration.rs        # 통합 테스트
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
```

## 선택 명령어

### 프로젝트 의존성 업데이트

```bash
cargo update
```

### HTML 문서 생성 및 웹 브라우저로 열기

```bash
cargo doc --no-deps --open  # `target/doc/p12_automated_testing/index.html`
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
