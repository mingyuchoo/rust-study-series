# Korean Language Server (Rust)

한국어 프로그래밍 언어를 위한 Language Server 구현입니다.

## 빌드

```bash
cargo build --release
```

## 실행

```bash
cargo run --release
```

빌드된 바이너리는 `target/release/korean-language-server`에 생성됩니다.

## 테스트

```bash
cargo test
```

## 주요 의존성

- **tower-lsp 0.20.0**: LSP 프로토콜 구현
- **tokio 1.48.0**: 비동기 런타임 (full 피처)
- **serde_json 1.0.145**: JSON 직렬화/역직렬화
- **regex 1.12.2**: 정규표현식 패턴 매칭

**Rust Edition**: 2024
