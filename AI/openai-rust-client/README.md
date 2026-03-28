# openai-rust-client

Rust와 Axum으로 구축된 OpenAI 기반 웹 챗봇 애플리케이션입니다. Onion Architecture(양파 아키텍처)를 따라 설계되었습니다.

## 주요 기능

- Azure OpenAI API를 통한 멀티턴 대화
- 현대적이고 반응형 웹 UI
- 한국어 지원
- 마크다운 형식의 채팅 메시지

## 아키텍처 (Onion Architecture)

```
src/
├── domain/              # 도메인 계층 (핵심 비즈니스 로직)
│   ├── entities/        # 엔티티 (Message 등)
│   └── repositories/    # 리포지토리 인터페이스
├── application/         # 애플리케이션 계층 (유스케이스)
│   ├── ports/           # 포트 (입출력 인터페이스)
│   └── services/        # 서비스 (ChatService)
├── infrastructure/      # 인프라 계층 (외부 서비스 구현체)
│   ├── adapters/        # 어댑터 (OpenAIAdapter)
│   └── config/          # 설정 (AppConfig)
├── presentation/        # 프레젠테이션 계층 (UI/API)
│   ├── api/             # API 컨트롤러 (ChatController, 요청/응답 모델)
│   └── web/             # 웹 핸들러 (정적 파일 서빙)
├── lib.rs               # 라이브러리 진입점
└── main.rs              # 애플리케이션 진입점
```

### 의존성 방향

도메인 -> 애플리케이션 -> 인프라 -> 프레젠테이션 (안쪽에서 바깥쪽으로만 의존)

## 요구사항

- Rust (Edition 2024)
- Azure OpenAI 서비스 계정

## 설치 및 설정

1. 저장소를 클론합니다.
2. `.env.example`을 참고하여 루트 디렉터리에 `.env` 파일을 생성합니다.
3. Azure OpenAI API 키를 `.env` 파일에 설정합니다.

```env
AZURE_API_KEY=your_api_key_here
OPENAI_ENDPOINT=https://your-resource.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2025-01-01-preview
OPENAI_MODEL=gpt-4o
```

## 실행 방법

```bash
cargo run
```

애플리케이션이 `http://localhost:8080`에서 시작됩니다.

## 프로젝트 구조

```
openai-rust-client/
├── src/                 # Rust 소스 코드 (Onion Architecture)
├── static/              # 프론트엔드 정적 파일
│   ├── index.html       # HTML 구조
│   ├── styles.css       # CSS 스타일링
│   └── script.js        # 채팅 기능 JavaScript
├── Cargo.toml           # 의존성 정의
├── Makefile.toml        # cargo-make 태스크
└── docs/                # 문서
```

## 주요 의존성

- **axum**: 웹 프레임워크
- **tokio**: 비동기 런타임
- **reqwest**: HTTP 클라이언트 (OpenAI API 호출)
- **serde / serde_json**: 직렬화/역직렬화
- **dotenv**: 환경 변수 로드
- **tower-http**: CORS 및 정적 파일 서빙
- **thiserror**: 에러 타입 정의

## License

MIT
