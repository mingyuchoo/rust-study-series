# Rust Qdrant Vector RAG

Azure OpenAI 임베딩과 Qdrant 벡터 DB를 기반으로 하는 고성능 RAG(Retrieval-Augmented Generation) 웹 서비스입니다. Actix Web을 사용해 HTTP API를 제공하며, 문서 업로드(마크다운) → 청크 분할 → 임베딩 생성 → Qdrant 저장 → 질의 시 유사 청크 검색 → Azure OpenAI Chat Completion 으로 답변을 생성합니다.

본 README는 코드베이스 구조, 설정, 실행 방법, API 엔드포인트, 테스트 및 모니터링 방법을 종합적으로 설명합니다.

---

## 주요 기능

- 문서 업로드(.md, .markdown) 및 파싱/청크 분할
- Azure OpenAI 임베딩 생성 및 배치 임베딩 지원
- Qdrant 벡터 데이터베이스에 벡터 저장 및 검색
- RAG 파이프라인을 통한 질문-응답
- 상세 로깅과 에러 처리, 재시도(지수 백오프)
- Prometheus 메트릭, 성능 모니터링, 캐시 관리
- OpenAPI 스키마와 Swagger UI 제공

---

## 아키텍처 개요

- `src/main.rs`: HTTP 서버 엔트리포인트. 설정 로드/검증, 의존성 초기화(`AppContainer`), 라우팅, 미들웨어, Swagger UI 설정.
- `src/app.rs`: DI 컨테이너(`AppContainer`) 구성. Azure OpenAI 클라이언트, Qdrant 리포지토리, 임베딩/검색/문서/RAG 서비스 초기화와 헬스 체크, 그레이스풀 셧다운 처리.
- `src/handlers/`: HTTP 핸들러 계층.
  - `health.rs`: 헬스 체크 및 심플 헬스 체크.
  - `upload.rs`: 멀티파트/JSON 업로드, 확장자/사이즈 검증, 문서 처리.
  - `query.rs`: 질의 처리, 옵션형 `QueryConfig`로 RAG 파라미터 제어.
  - `monitoring.rs`: 메트릭/프로메테우스/캐시/벤치마크 엔드포인트.
- `src/services/`: 도메인 서비스 계층.
  - `document.rs`: 문서 처리(청크 분할→임베딩→저장) 로직.
  - `chunker.rs`: 문서 청크 분할 파이프라인과 경계/오버랩 설정.
  - `embedding.rs`: Azure OpenAI 임베딩 생성 서비스.
  - `vector_search.rs`: Qdrant 기반 벡터 검색 서비스.
  - `rag.rs`: RAG 조립(컨텍스트 구성→Chat Completion).
  - `cache.rs`, `resilience.rs`: 캐시와 회복탄력성(재시도/백오프 등).
- `src/clients/`:
  - `azure_openai.rs`: Azure OpenAI API 클라이언트(임베딩/채팅). 지수 백오프 재시도, 에러 매핑, 성능 측정 연동.
  - `connection_pool.rs`: HTTP/Qdrant 연결 풀(Deadpool, 라운드로빈 등).
- `src/repository/`:
  - `qdrant.rs`: 컬렉션 생성/확인, 포인트 업서트/검색 등 Qdrant 연동.
- `src/config/app_config.rs`: `.env` 기반 설정 로드/검증. 서버, Azure OpenAI, Qdrant 설정 구조체 및 유효성 검사 구현.
- `src/middleware/`: 요청 로깅, 에러 핸들링 미들웨어.
- `src/monitoring/`: 메트릭 수집(`metrics`, `metrics-exporter-prometheus`)과 성능 타이머.
- `src/docs.rs`: `utoipa` 기반 OpenAPI 스키마 및 Swagger UI 구성.
- `tests/`: 통합 테스트, E2E/HTTP/성능 테스트 러너 포함.

---

## 디렉터리 구조

```
AI/rust-qdrant-vector-rag/
├─ src/
│  ├─ main.rs              # 서버 엔트리포인트 및 라우팅
│  ├─ app.rs               # AppContainer, 헬스체크, 셧다운
│  ├─ docs.rs              # OpenAPI/Swagger 설정
│  ├─ handlers/            # 요청 핸들러(health, upload, query, monitoring)
│  ├─ services/            # document, chunker, embedding, rag, vector_search, cache, resilience
│  ├─ clients/             # azure_openai, connection_pool
│  ├─ repository/          # qdrant 리포지토리
│  ├─ config/              # AppConfig 및 하위 설정 구조체
│  ├─ middleware/          # 로깅/에러 미들웨어
│  ├─ monitoring/          # metrics, performance
│  └─ models/              # 에러/응답/도메인 모델
├─ tests/                  # 통합/성능 테스트
├─ docs/azure_openai_integration.md
├─ docker-compose.yml      # Qdrant 구동 구성
├─ .env.example            # 환경 변수 예시
├─ Cargo.toml              # 의존성
└─ Makefile.toml           # cargo-make 태스크(선택)
```

---

## 의존성 요약

- 웹: `actix-web`, `actix-cors`, `actix-multipart`
- 비동기: `tokio`
- 직렬화: `serde`, `serde_json`
- HTTP 클라이언트: `reqwest`
- 벡터 DB: `qdrant-client`
- 설정: `dotenvy`, `config`
- 로깅: `tracing`, `tracing-subscriber`, `tracing-actix-web`
- 메트릭: `metrics`, `metrics-exporter-prometheus`
- OpenAPI: `utoipa`, `utoipa-swagger-ui`

---

## 실행 전 요구사항

- Rust 1.78+ (Rust 2021 에디션 권장)
- 로컬 또는 원격 Qdrant 인스턴스
  - 로컬 실행은 `docker-compose.yml` 제공
- Azure OpenAI 리소스 및 배포(임베딩/챗)

---

## 환경 변수 설정

`.env.example`를 참고하여 `.env`를 생성하세요.

필수/주요 변수:

```
# 서버
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
SERVER_MAX_REQUEST_SIZE=10485760
SERVER_TIMEOUT_SECONDS=30

# Azure OpenAI
AZURE_OPENAI_ENDPOINT=https://<your>.openai.azure.com/
AZURE_OPENAI_API_KEY=<your-key>
AZURE_OPENAI_API_VERSION=2024-02-01   # 기본값 존재
AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4    # 배포명
AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-3-large
AZURE_OPENAI_MAX_RETRIES=3
AZURE_OPENAI_TIMEOUT_SECONDS=60

# Qdrant
QDRANT_URL=http://localhost:6334
QDRANT_COLLECTION_NAME=document_chunks
QDRANT_VECTOR_SIZE=3072               # text-embedding-3-large 기준
QDRANT_TIMEOUT_SECONDS=30
QDRANT_MAX_RETRIES=3
```

설정 검증은 애플리케이션 시작 시 자동 수행됩니다(`AppConfig::validate()`). 값 형식이 유효하지 않으면 부팅 실패 또는 경고 로그가 발생합니다.

---

## 로컬 실행 방법

1) Qdrant 실행(도커 권장):

```
docker compose up -d
```

- 기본 포트: HTTP 6333, gRPC 6334

2) 애플리케이션 실행:

```
cargo run
```

- 서버가 `SERVER_HOST:SERVER_PORT`(기본 127.0.0.1:8080)에서 기동됩니다.
- OpenAPI/Swagger UI: <http://127.0.0.1:8080/swagger-ui/>
- OpenAPI 스펙 JSON: <http://127.0.0.1:8080/api-doc/openapi.json>

(선택) `cargo-make` 사용 시:

```
cargo make run
```

---

## API 엔드포인트

모든 최신 엔드포인트는 `src/main.rs`의 `/api/v1` 스코프로 제공됩니다. 하위 호환을 위해 루트에도 동일 라우트가 등록되어 있습니다.

- 헬스체크
  - `GET /api/v1/health` → 종합 상태(`handlers/health.rs::health_handler()`)
  - `GET /api/v1/health/performance` → 성능 지표(핸들러는 `handlers/monitoring.rs` 내)
  - `GET /health`, `GET /health/simple` → 레거시/심플 응답

- 업로드
  - `POST /api/v1/upload` → 멀티파트 파일 업로드(필드명 `file`)
  - `POST /api/v1/upload/json` → JSON 업로드(`{"filename","content"}`)

- 질의
  - `POST /api/v1/query` → 본문 `{"question": String, "config"?: QueryConfig}`
  - `GET /api/v1/query/{question}` → 경로 파라미터 질의(간단)

- 모니터링/관리
  - `GET /api/v1/metrics` → 내부 메트릭(요약)
  - `GET /api/v1/metrics/prometheus` → Prometheus 포맷
  - `GET /api/v1/cache/stats` → 캐시 통계
  - `POST /api/v1/cache/clear` → 캐시 비우기
  - `POST /api/v1/benchmark` → 벤치마크 트리거

OpenAPI 스키마는 `src/docs.rs`에서 `utoipa` 매크로로 정의됩니다.

---

## 요청 예시

- 업로드(multipart):

```
curl -X POST "http://127.0.0.1:8080/api/v1/upload" \
     -H "Content-Type: multipart/form-data" \
     -F "file=@README.md"
```

- 업로드(JSON):

```
curl -X POST "http://127.0.0.1:8080/api/v1/upload/json" \
     -H "Content-Type: application/json" \
     -d '{"filename":"sample.md","content":"# 제목\n내용..."}'
```

- 질의(POST):

```
curl -X POST "http://127.0.0.1:8080/api/v1/query" \
     -H "Content-Type: application/json" \
     -d '{
           "question": "이 프로젝트의 아키텍처를 설명해줘",
           "config": {"max_chunks": 5, "similarity_threshold": 0.75, "max_response_tokens": 256}
         }'
```

- 질의(GET):

```
curl "http://127.0.0.1:8080/api/v1/query/이%20프로젝트는%20무엇인가?"
```

---

## 테스트

```
cargo test
```

- 통합/E2E 테스트는 `tests/`에 위치합니다.
- Azure OpenAI 연동 통합 테스트는 실제 자격 증명이 필요할 수 있습니다.

---

## 모니터링 및 로깅

- 로깅 레벨 설정:

```
RUST_LOG=backend=debug,actix_web=info cargo run
```

- Prometheus 스크레이프 엔드포인트: `GET /api/v1/metrics/prometheus`
- 성능 타이머와 메트릭은 `src/monitoring/` 모듈을 참고하세요.

---

## 운영 팁

- Qdrant 컬렉션은 부팅 시 존재 여부 확인 후 자동 생성됩니다(`AppContainer::init_vector_repository`)。
- 기본 벡터 차원은 `QDRANT_VECTOR_SIZE` 환경 변수(기본 3072)로 제어합니다.
- Azure OpenAI 재시도는 지수 백오프(최대 30초 대기)로 구현되어 있습니다.
- 업로드 용량은 `SERVER_MAX_REQUEST_SIZE`로 제한됩니다(기본 10MB).

---

## 자주 묻는 질문

- Swagger UI가 보이지 않아요
  - 서버가 기동 중인지, `http://127.0.0.1:8080/swagger-ui/` 경로로 접근하는지 확인하세요.
- 429(레이트 리밋)가 자주 발생해요
  - 요청 간격을 늘리거나 배치 임베딩을 활용하세요. Azure 포털에서 쿼터 확인도 필요합니다.
- 임베딩 차원이 맞지 않아요
  - 임베딩 배포가 `text-embedding-3-large(3072차원)`인지, `QDRANT_VECTOR_SIZE`와 일치하는지 확인하세요.

---

## 라이선스

이 저장소의 루트에 별도 라이선스 파일이 없으므로, 조직/프로젝트 정책에 따릅니다.
