# wasm-game-of-life

Rust 라이브러리를 WebAssembly로 컴파일하는 프로젝트입니다.

## 소개

이 프로젝트는 `wasm-pack` 템플릿을 기반으로 Rust 코드를 WebAssembly로 컴파일하고 NPM 패키지로 배포할 수 있도록 구성되어 있습니다.

- **패키지명**: `wasm-game-of-life`
- **버전**: `0.1.0`
- **Rust 에디션**: 2024

## 프로젝트 구조

```text
src/
├── lib.rs      # 메인 라이브러리 (greet 함수 제공)
└── utils.rs    # 유틸리티 함수 (panic hook 설정)
tests/
└── web.rs      # 웹 브라우저용 테스트
```

## 주요 기능

- `greet()`: JavaScript의 `alert()`를 통해 "안녕하세요?" 메시지를 표시합니다.

## 사용 방법

### 사전 요구 사항

```bash
rustup default stable
rustup update stable
cargo install cargo-make
cargo install wasm-pack
```

### cargo-make를 사용한 빌드

이 프로젝트는 `cargo-make`를 사용하여 빌드 작업을 관리합니다.

```bash
# 코드 검사 및 포맷팅
cargo make check
cargo make clippy
cargo make format

# 개발용 빌드
cargo make build

# 릴리스 빌드
cargo make release

# WASM 패키지 빌드
cargo make wasm-build

# WASM 릴리스 빌드
cargo make wasm-build-release

# 테스트 실행
cargo make test

# 정리
cargo make clean
```

### wasm-pack 직접 사용

```bash
# WASM 빌드
wasm-pack build

# 헤드리스 브라우저에서 테스트
wasm-pack test --headless --firefox

# NPM에 배포
wasm-pack publish
```

## 포함된 의존성

| 패키지 | 버전 | 설명 |
| ------ | ---- | ---- |
| [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) | 0.2.95 | WebAssembly와 JavaScript 간의 통신을 담당합니다. |
| [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) | 0.1.7 | 패닉 메시지를 개발자 콘솔에 로깅합니다. (선택적 기능) |
| [`wee_alloc`](https://github.com/rustwasm/wee_alloc) | 0.4.5 | 작은 코드 크기에 최적화된 할당자입니다. (선택적 기능) |

## 참고 자료

- [wasm-pack 튜토리얼](https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html)
- [NPM 브라우저 패키지 튜토리얼](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html)
