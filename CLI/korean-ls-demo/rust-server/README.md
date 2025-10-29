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

## 테스트

```bash
cargo test
```

## 의존성

- tower-lsp: LSP 프로토콜 구현
- tokio: 비동기 런타임
- serde_json: JSON 직렬화/역직렬화
- regex: 정규표현식 패턴 매칭
