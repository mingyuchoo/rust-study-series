# rust-llm-actor-system

Rust로 구현된 멀티 LLM 에이전트 시스템입니다. Tokio 채널 기반 액터 패턴과 Actix-web 웹 서버를 결합하여 여러 LLM 에이전트를 동적으로 관리하고 라우팅합니다.

## 아키텍처

```
[Actix-web HTTP Server (port 8080)]
    └── [AppState]
            ├── [AgentRouter] ── 키워드 기반 라우팅 + 우선순위/신뢰도
            │       ├── [LLMActor "agent-1"] (OpenAI API 스트리밍)
            │       ├── [LLMActor "agent-2"]
            │       └── [Manager Agent] (기본 라우팅 대상)
            ├── [Metrics] ── 요청 수, 에러 수, 처리 시간 추적
            └── [Chat History] ── 대화 이력 관리
```

## 주요 컴포넌트

### LLMActor

- OpenAI API(Azure) 스트리밍 호출로 실제 LLM 응답 생성
- 에이전트별 시스템 프롬프트 설정
- 헬스 체크 상태 관리 (Healthy / Degraded / Unhealthy)
- 비동기 프롬프트 처리 (`tokio::sync::oneshot` 채널)

### AgentRouter

- 키워드 기반 에이전트 라우팅 (우선순위 및 신뢰도 임계값)
- 런타임에 에이전트 동적 추가/제거
- 매니저 에이전트 설정 (기본 라우팅 대상)
- 전체 에이전트 헬스 체크

### Metrics

- 프롬프트 처리 수, 에러 수, 총 처리 시간 추적
- 평균 처리 시간 계산
- 통계 리셋 기능

### 웹 인터페이스

- Actix-web 기반 HTTP 서버
- 정적 파일 서빙 (HTML/CSS/JS)
- 대화 이력 관리

## 프로젝트 구조

```
rust-llm-actor-system/
├── src/
│   ├── main.rs          # Actix-web 서버 진입점
│   └── lib.rs           # LLMActor, AgentRouter, Metrics, 웹 핸들러
├── static/
│   ├── index.html       # 웹 UI
│   ├── css/             # 스타일시트
│   └── js/              # JavaScript
├── Cargo.toml           # 의존성 정의
└── Makefile.toml        # cargo-make 태스크
```

## 요구사항

- Rust (Edition 2024)
- OpenAI API 키 (Azure OpenAI 또는 호환 API)

## 환경 변수 설정

`.env` 파일을 생성하고 다음 변수를 설정합니다:

```env
OPENAI_API_KEY=your-api-key
OPENAI_API_URL=https://your-resource.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2025-01-01-preview
OPENAI_API_MODEL=gpt-4o
OPENAI_API_MAX_TOKENS=1024
OPENAI_API_TEMPERATURE=1.0
OPENAI_API_TOP_P=1.0
```

## 설치 및 실행

```bash
# 빌드
cargo build

# 실행
cargo run
```

서버가 `http://127.0.0.1:8080`에서 시작됩니다.

## 주요 의존성

- **actix-web / actix-files**: 웹 프레임워크 및 정적 파일 서빙
- **tokio**: 비동기 런타임 및 채널 기반 메시징
- **reqwest**: HTTP 클라이언트 (OpenAI API 스트리밍 호출)
- **serde / serde_json**: 직렬화/역직렬화
- **dotenv**: 환경 변수 로드
- **tracing / tracing-subscriber**: 구조화된 로깅
- **uuid**: 고유 ID 생성
- **chrono**: 날짜/시간 처리

## 라이센스

MIT
