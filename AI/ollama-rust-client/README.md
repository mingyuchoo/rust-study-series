# ollama-rust-client

Ollama API Rust Client
=====================

이 프로젝트는 Rust로 작성된 간단한 Ollama API 클라이언트 예제입니다. Ollama 서버에 프롬프트를 전송하고, 스트리밍 방식으로 응답을 받아 출력합니다.

## 주요 기능
- Ollama API(`/api/generate`)에 프롬프트 전송
- 스트리밍 응답 처리 및 실시간 출력
- Reqwest, Tokio, Serde 등 Rust 비동기 생태계 활용

## 빌드 방법

Rust와 Cargo가 설치되어 있어야 합니다.

```bash
# 의존성 설치 및 빌드
cargo build --release
```

또는 Makefile.toml을 사용하는 경우, [cargo-make](https://sagiegurari.github.io/cargo-make/)를 설치한 뒤 아래와 같이 빌드할 수 있습니다.

```bash
cargo make build
```

## 실행 방법

Ollama 서버가 로컬에서 실행 중이어야 하며, 기본 엔드포인트는 `http://localhost:11434/api/generate`입니다.

```bash
cargo run --release
```
또는
```bash
cargo make run
```

실행 후 프롬프트가 출력되면, 질문을 입력하고 Enter를 누르세요.

```
Enter your prompt (press Enter to submit):
> (여기에 질문 입력)
```

응답이 스트리밍 방식으로 출력됩니다.

## 의존성
- reqwest
- serde
- serde_json
- tokio
- futures-util
- tokio-stream

## 참고
- Ollama 서버가 사전에 설치 및 실행되어 있어야 합니다. [Ollama 공식 문서](https://ollama.com/) 참고
- 기본 모델명은 `phi4`로 설정되어 있습니다. 필요에 따라 main.rs에서 변경하세요.

---

문의: [프로젝트 소유자 이메일 또는 깃허브 링크]
