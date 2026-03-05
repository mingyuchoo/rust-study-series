# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 프로젝트 개요

Azure OpenAI와 SurrealDB를 활용한 그래프 기반 RAG(Retrieval-Augmented Generation) 시스템입니다. PDF 문서를 인덱싱하고 벡터 검색 및 AI 기반 질의응답을 제공합니다.

## 아키텍처

### 백엔드 워크스페이스 구조

- **bin-main**: 메인 실행 바이너리, `lib_api::run_server()`를 호출하여 서버 시작
- **lib-api**: REST API 레이어 (Actix Web + Swagger UI)
  - `auth.rs`: JWT 기반 인증 (로그인, 토큰 갱신, 로그아웃, 사용자 정보 조회)
  - `azure.rs`: Azure OpenAI 클라이언트 래퍼
  - `chat.rs`: 통합 질의응답 엔드포인트 (벡터 검색 + 그래프 검색 + AI 답변 생성)
  - `vector_search.rs`: TF-IDF 기반 벡터 검색
  - `graph_search.rs`: SurrealDB 그래프 기반 엔티티/관계 검색
  - `reindex.rs`: PDF 업로드 및 재인덱싱 관리자 도구
  - `config.rs`: 환경변수 기반 설정 로드 (AppConfig, AzureConfig)
  - `models.rs`: API 요청/응답 모델 (Swagger 스키마 포함)
  - `types.rs`: 애플리케이션 상태 (AppState: cfg + azure)
- **lib-db**: SurrealDB 연결 관리
  - `LazyLock<Surreal<Client>>` 패턴으로 전역 DB 인스턴스 관리
  - `setup_database()`: WebSocket 연결, Root 인증, 네임스페이스/DB 선택 (재시도 로직 포함)
- **lib-index**: GraphRAG 인덱싱 파이프라인
  - `pdf_processor.rs`: PDF → 텍스트 추출 (lopdf)
  - `embedding.rs`: TF-IDF 기반 벡터 임베딩
  - `ner.rs`: 정규식 기반 NER (Named Entity Recognition)
  - `graph_builder.rs`: 엔티티/관계 추출 및 그래프 구축
  - `database.rs`: 청크/엔티티/관계를 SurrealDB에 저장
  - `query_engine.rs`: 벡터 검색 및 그래프 쿼리 실행

### 프론트엔드 구조

- `src/pages/`: 각 기능별 페이지 컴포넌트 (Login, Chat, VectorSearch, GraphSearch, Reindex)
- `src/services/`: API 통신 레이어 (axios 기반)
- `src/components/`: 공통 UI 컴포넌트 (NavBar)
- Fluent UI (Microsoft) 사용하여 UI 구성

### 데이터 흐름

1. **인덱싱**: PDF 업로드 → 텍스트 추출 → 청크 분할 → NER → 엔티티/관계 추출 → 그래프 저장 → TF-IDF 임베딩
2. **검색**: 사용자 질의 → 벡터 검색 (TF-IDF 유사도) + 그래프 검색 (엔티티/관계 탐색) → 컨텍스트 조합
3. **AI 답변**: 검색된 컨텍스트 → Azure OpenAI 프롬프트 → 자연어 답변 생성

## 개발 명령어

### 백엔드

```bash
cd backend

# cargo-make 명령어 (Makefile.toml 참조)
cargo make format   # 포맷팅 (check + clippy + fmt)
cargo make build    # 개발 빌드
cargo make test     # 테스트 실행
cargo make run      # 서버 실행 (bin-main, RUST_LOG=lib_api=debug,actix_web=info)
cargo make release  # 릴리스 빌드

# 직접 cargo 명령어
cargo check                    # 타입 체크
cargo clippy                   # 린트
cargo test                     # 모든 테스트 실행
cargo test --lib               # 라이브러리 테스트만
cargo test --package lib-api   # 특정 패키지 테스트
cargo run --bin bin-main       # 메인 바이너리 실행

# 자동 재시작 (cargo-watch 필요)
cargo watch -x "run --bin bin-main"
```

### 프론트엔드

```bash
cd frontend

pnpm install         # 의존성 설치
pnpm dev             # 개발 서버 (http://localhost:5173)
pnpm build           # 프로덕션 빌드 (tsc + vite build)
pnpm preview         # 빌드 결과 미리보기
pnpm format          # Prettier 포맷팅
pnpm format:check    # 포맷 검사
```

### 인프라

```bash
# SurrealDB 시작 (Docker Compose)
docker-compose up -d surrealdb

# 전체 스택 시작 (SurrealDB + 백엔드 + 프론트엔드)
docker-compose up -d

# 로그 확인
docker-compose logs -f surrealdb

# 중지
docker-compose down
```

## 환경 설정

### 백엔드 (.env)

```bash
# Azure OpenAI
AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com/
AZURE_OPENAI_API_KEY=your-api-key
AZURE_OPENAI_DEPLOYMENT=gpt-4

# SurrealDB
SURREALDB_URL=localhost:8000
SURREALDB_USERNAME=root
SURREALDB_PASSWORD=root
SURREALDB_NS=namespace
SURREALDB_DB=database

# JWT 시크릿 (HS256)
JWT_SECRET=your-secret-key
```

### 프론트엔드 (.env)

```bash
VITE_API_BASE_URL=http://localhost:4000
```

## API 문서

서버 실행 후 Swagger UI에서 전체 API 스펙 확인 가능:
- Swagger UI: http://localhost:4000/swagger-ui/
- OpenAPI JSON: http://localhost:4000/api-doc/openapi.json

## 테스트 도구

- **Postman**: `tests/postman/` 디렉토리에 컬렉션 및 환경 변수 파일 포함
- **Zaku**: `tests/zaku/` 디렉토리에 추가 테스트 도구

## 주요 기술 결정

1. **Rust Edition 2024**: `rust-toolchain.toml`에서 nightly 채널 사용
2. **TF-IDF 임베딩**: Azure OpenAI embeddings 대신 로컬 TF-IDF 사용 (비용 절감)
3. **LazyLock DB**: SurrealDB 전역 인스턴스를 `LazyLock`으로 관리하여 초기화 지연 및 스레드 안전성 확보
4. **Actix Web**: 고성능 비동기 웹 프레임워크
5. **Swagger 자동 생성**: `utoipa` 매크로로 API 문서 자동 생성
6. **Fluent UI**: Microsoft 디자인 시스템 사용하여 일관된 UX 제공

## 코딩 컨벤션

- **Rust 포맷팅**: `rustfmt.toml` 설정 준수 (에디션 2024, 임포트 정렬, 최대 너비 120)
- **TOML 포맷팅**: `.taplo.toml` 설정 준수
- **프론트엔드**: `.prettierrc.json` 설정 준수 (세미콜론, 싱글쿼트 등)
- **BDD 명세**: 모든 기능 명세는 BDD 형식으로 작성
- **TDD 개발**: 코드 작성 시 테스트 우선 작성

## 디버깅

- **백엔드 로그**: `RUST_LOG` 환경변수 조정 (`lib_api=debug,actix_web=info`)
- **DB 연결 문제**: `lib_db::setup_database()` 재시도 로직이 5회 시도, 각 800ms 대기
- **파일 업로드 제한**: 100MB (lib-api/src/lib.rs의 PayloadConfig)
- **CORS 설정**: 현재 CORS 미들웨어 미적용, 필요시 `actix-cors` 추가

## 알려진 제약사항

- TF-IDF 임베딩은 의미론적 검색보다 키워드 기반 검색에 최적화됨
- NER은 정규식 기반으로 단순하며, 실제 엔티티 추출 정확도는 제한적
- 그래프 쿼리는 SurrealDB의 `RELATE` 문법을 사용하여 관계 탐색
