# ollama-rust-client

Ollama API를 활용한 Rust 기반 QA 파이프라인 웹 애플리케이션입니다. 사용자의 질문을 4단계(의도 파악 -> 분석 -> 답변 생성 -> 요약)로 처리하여 구조화된 응답을 제공합니다.

## 주요 기능

- **4단계 QA 파이프라인**: 의도 파악(Intent) -> 분석(Analysis) -> 답변(Answer) -> 요약(Summary)
- **웹 UI**: Axum 기반 웹 서버에서 HTML 폼을 통해 질문 입력 및 결과 확인
- **Ollama API 연동**: 로컬 Ollama 서버(`/api/generate`)에 프롬프트 전송
- **비동기 처리**: Tokio + Reqwest 기반 비동기 HTTP 통신

## 프로젝트 구조

```
ollama-rust-client/
├── src/
│   └── main.rs              # 웹 서버, QA 파이프라인, Ollama 클라이언트, HTML UI
├── Cargo.toml               # 의존성 정의
├── Makefile.toml            # cargo-make 태스크
├── rust-toolchain.toml      # Rust 툴체인 설정
└── rustfmt.toml             # 코드 포매팅 설정
```

## 요구사항

- Rust (Edition 2024)
- 로컬 Ollama 서버 실행 중 (기본 `http://localhost:11434`)
- 기본 모델: `phi4` (코드 내 `DEFAULT_MODEL` 상수로 변경 가능)

## 빌드 방법

```bash
# 의존성 설치 및 빌드
cargo build --release
```

또는 [cargo-make](https://sagiegurari.github.io/cargo-make/) 사용 시:

```bash
cargo make build
```

## 실행 방법

Ollama 서버가 로컬에서 실행 중이어야 합니다.

```bash
cargo run --release
```

또는:

```bash
cargo make run
```

실행 후 브라우저에서 `http://localhost:3000`으로 접속하여 질문을 입력하세요.

### API 엔드포인트

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `GET` | `/` | 웹 UI (HTML 폼) |
| `POST` | `/ask` | 질문 처리 (JSON 응답) |

## 주요 의존성

- **axum**: 웹 프레임워크
- **reqwest**: HTTP 클라이언트 (Ollama API 호출)
- **tokio**: 비동기 런타임
- **serde / serde_json**: 직렬화/역직렬화
- **tower / tower-http**: 미들웨어 (파일 서빙, 트레이싱)
- **tracing / tracing-subscriber**: 구조화된 로깅

## 참고

- Ollama 서버가 사전에 설치 및 실행되어 있어야 합니다. [Ollama 공식 문서](https://ollama.com/) 참고
- 기본 모델명은 `phi4`로 설정되어 있습니다. `src/main.rs`의 `DEFAULT_MODEL` 상수를 변경하세요.

---

문의: [프로젝트 소유자 이메일 또는 깃허브 링크]
