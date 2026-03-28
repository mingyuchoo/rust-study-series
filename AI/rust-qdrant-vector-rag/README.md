# rust-qdrant-vector-rag

Azure OpenAI와 Qdrant 벡터 데이터베이스를 활용한 고성능 Rust 백엔드와 Svelte/TypeScript 프론트엔드로 구성된 풀스택 RAG(Retrieval-Augmented Generation) 애플리케이션입니다.

## 아키텍처 개요

이 프로젝트는 세 가지 주요 컴포넌트로 구성됩니다:

- **백엔드**: Actix Web 기반 Rust RAG 서비스
- **프론트엔드**: Vite 빌드 시스템을 사용하는 Svelte/TypeScript 애플리케이션
- **Docker**: 컨테이너화된 배포 구성

```
project-root/
├── backend/           # Rust RAG service
├── frontend/          # Svelte/TypeScript UI
├── docker/            # Docker configuration
└── README.md          # This file
```

## 빠른 시작

### 사전 요구사항

- **Rust** 1.78+ (백엔드)
- **Node.js** 18+ 및 **pnpm** (프론트엔드)
- **Docker** 및 **Docker Compose** (인프라 서비스)
- **Azure OpenAI** 리소스 (임베딩 및 채팅 배포)

### 1. 인프라 서비스 시작

```bash
cd docker
docker-compose up -d
```

Qdrant 벡터 데이터베이스 및 기타 필수 서비스를 시작합니다.

### 2. 환경 설정

환경 파일을 복사하고 설정합니다:

```bash
# 백엔드 설정
cp backend/.env.example backend/.env
# backend/.env에 Azure OpenAI 자격 증명 입력

# 프론트엔드 설정
cp frontend/.env.example frontend/.env
# frontend/.env에 API 엔드포인트 설정
```

### 3. 백엔드 시작

```bash
cd backend
cargo run
```

백엔드 API는 `http://localhost:8080`에서 사용 가능합니다.

### 4. 프론트엔드 시작

```bash
cd frontend
pnpm install
pnpm dev
```

프론트엔드는 `http://localhost:5173`에서 사용 가능합니다.

## 프로젝트 구조

### 백엔드 (`/backend`)

Rust 백엔드는 다음 아키텍처를 갖춘 고성능 RAG 서비스를 제공합니다:

```
backend/src/
├── main.rs              # Server entry point and routing
├── app.rs               # DI container and application setup
├── docs.rs              # OpenAPI/Swagger configuration
├── handlers/            # HTTP request handlers
│   ├── health.rs        # Health check endpoints
│   ├── upload.rs        # Document upload handlers
│   ├── query.rs         # Query processing handlers
│   └── monitoring.rs    # Metrics and monitoring
├── services/            # Business logic layer
│   ├── document.rs      # Document processing service
│   ├── chunker.rs       # Text chunking pipeline
│   ├── embedding.rs     # Azure OpenAI embedding service
│   ├── vector_search.rs # Qdrant vector search
│   ├── rag.rs           # RAG orchestration
│   ├── cache.rs         # Caching layer
│   └── resilience.rs    # Retry and circuit breaker
├── clients/             # External service clients
├── repository/          # Data access layer (Qdrant)
├── config/              # Configuration management
├── middleware/          # HTTP middleware
├── monitoring/          # Metrics and performance
└── models/              # Data models and types
```

**주요 기능:**
- 문서 업로드 및 처리 (마크다운 지원)
- Azure OpenAI 배치 임베딩 생성
- Qdrant 벡터 저장 및 유사도 검색
- 컨텍스트 검색 및 답변 생성 RAG 파이프라인
- Prometheus 메트릭을 활용한 종합 모니터링
- OpenAPI 문서 및 Swagger UI
- 재시도 로직 및 캐싱을 포함한 복원력 설계

### 프론트엔드 (`/frontend`)

TypeScript를 사용하는 최신 Svelte 애플리케이션:

```
frontend/src/
├── app.html             # HTML template
├── app.css              # Global styles
├── main.ts              # Application entry point
├── lib/                 # Reusable components and utilities
├── routes/              # SvelteKit routes (if using SvelteKit)
└── static/              # Static assets
```

**주요 기능:**
- Svelte 및 TypeScript로 구축된 반응형 UI
- Tailwind CSS 스타일링
- Vite 기반 빠른 개발 및 빌드
- ESLint 및 Prettier 코드 품질 관리
- Vitest 테스팅

### Docker 구성 (`/docker`)

컨테이너화된 배포 설정:

- `docker-compose.yml`: Qdrant 및 기타 서비스 오케스트레이션
- `.dockerignore`: Docker 빌드 컨텍스트 최적화

## 설정

### 백엔드 설정

`backend/.env`의 주요 환경 변수:

```bash
# 서버 설정
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
SERVER_MAX_REQUEST_SIZE=10485760
SERVER_TIMEOUT_SECONDS=30

# Azure OpenAI
AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com/
AZURE_OPENAI_API_KEY=your-api-key
AZURE_OPENAI_EMBEDDING_DEPLOYMENT=text-embedding-3-large
AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4

# Qdrant 벡터 데이터베이스
QDRANT_URL=http://localhost:6334
QDRANT_COLLECTION_NAME=documents
QDRANT_VECTOR_SIZE=3072

# 성능 및 캐싱
CACHE_TTL_SECONDS=3600
MAX_CONCURRENT_REQUESTS=100
```

### 프론트엔드 설정

`frontend/.env`에서 API 엔드포인트를 설정합니다:

```bash
VITE_API_BASE_URL=http://localhost:8080/api/v1
VITE_APP_TITLE=RAG Application
```

## API 엔드포인트

백엔드는 종합적인 REST API를 제공합니다:

### 문서 관리
- `POST /api/v1/upload` - 문서 업로드 (멀티파트)
- `POST /api/v1/upload/json` - JSON으로 업로드

### 질의 처리
- `POST /api/v1/query` - RAG 질의 처리
- `GET /api/v1/query/{question}` - URL 경로 파라미터를 통한 간단 질의

### 모니터링 및 관리
- `GET /api/v1/health` - 헬스 체크
- `GET /api/v1/metrics` - 애플리케이션 메트릭
- `GET /api/v1/metrics/prometheus` - Prometheus 포맷 메트릭
- `GET /api/v1/cache/stats` - 캐시 통계
- `POST /api/v1/cache/clear` - 캐시 초기화

### 문서화
- `GET /swagger-ui/` - 대화형 API 문서

## 테스트

### 백엔드 테스트

```bash
cd backend

# 단위 테스트 실행
cargo test

# 커버리지 측정
cargo tarpaulin --ignore-tests

# 통합 테스트 실행
cargo test --test integration_tests

# 성능 벤치마크
cargo test --release benchmark
```

### 프론트엔드 테스트

```bash
cd frontend

# 단위 테스트 실행
pnpm test

# 감시 모드 테스트
pnpm test:watch

# 커버리지 포함 테스트
pnpm test:coverage

# 타입 검사
pnpm check
```

## 모니터링

애플리케이션은 종합적인 모니터링을 포함합니다:

- **Prometheus 메트릭**: 요청 비율, 지연 시간, 에러율
- **헬스 체크**: 서비스 가용성 및 의존성 상태
- **성능 모니터링**: 응답 시간 및 리소스 사용량
- **캐시 통계**: 적중률 및 메모리 사용량
- **로깅**: tracing을 활용한 구조화된 로깅

메트릭 접근:
- 애플리케이션 메트릭: `GET /api/v1/metrics`
- Prometheus 포맷: `GET /api/v1/metrics/prometheus`

## 배포

### 개발 환경

위의 빠른 시작 가이드를 참고하세요.

### 프로덕션

1. **애플리케이션 빌드:**

```bash
# 백엔드 빌드
cd backend
cargo build --release

# 프론트엔드 빌드
cd frontend
pnpm build
```

2. **Docker로 배포:**

```bash
cd docker
docker-compose -f docker-compose.prod.yml up -d
```

## 개발 도구

### 백엔드 도구
- **cargo-make**: 태스크 러너 (`Makefile.toml` 참고)
- **rustfmt**: 코드 포매팅
- **clippy**: 린팅
- **tarpaulin**: 커버리지 리포트

### 프론트엔드 도구
- **ESLint**: 코드 린팅
- **Prettier**: 코드 포매팅
- **Vitest**: 테스팅 프레임워크
- **TypeScript**: 타입 검사

## 문제 해결

### 일반적인 문제

**백엔드 문제:**
- **레이트 리미팅**: 요청 빈도를 줄이거나 배치 임베딩 사용
- **벡터 차원 불일치**: 임베딩 모델이 `QDRANT_VECTOR_SIZE`와 일치하는지 확인
- **연결 에러**: Qdrant가 실행 중이고 접근 가능한지 확인

**프론트엔드 문제:**
- **API 연결**: `VITE_API_BASE_URL` 설정 확인
- **빌드 에러**: Node.js 버전 호환성 확인
- **타입 에러**: `pnpm check`로 TypeScript 검증

### 로그 및 디버깅

- 백엔드 로그: 설정 가능한 레벨의 구조화된 로깅
- 프론트엔드 로그: 브라우저 콘솔 및 네트워크 탭
- 서비스 로그: `docker-compose logs <service-name>`

## 라이선스

이 프로젝트는 조직의 라이선스 정책을 따릅니다. 구체적인 라이선스 정보는 개별 컴포넌트 디렉터리를 참고하세요.

## 기여 방법

1. 저장소를 포크합니다.
2. 기능 브랜치를 생성합니다.
3. 변경 사항을 적용합니다.
4. 새 기능에 대한 테스트를 추가합니다.
5. 모든 테스트가 통과하는지 확인합니다.
6. Pull Request를 제출합니다.

상세한 기여 가이드라인은 `/backend` 및 `/frontend` 디렉터리의 개별 README를 참고하세요.