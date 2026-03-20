# rust-surreal-graph-rag

Rust와 SurrealDB 기반의 **GraphRAG(Graph-based Retrieval-Augmented Generation)** 풀스택 애플리케이션입니다. PDF 문서를 인덱싱하여 벡터 검색과 지식 그래프 검색을 결합한 통합 질의응답 시스템을 제공합니다.

## 아키텍처 개요

```
┌─────────────────┐     ┌──────────────────────────────────────────┐     ┌───────────────┐
│   Frontend      │     │              Backend (Rust)               │     │   SurrealDB   │
│  React + Fluent │◄───►│  Actix-web HTTP Server (port 4000)       │◄───►│  (port 8000)  │
│  UI (Vite)      │     │                                          │     │               │
└─────────────────┘     │  ┌──────────┐ ┌──────────┐ ┌──────────┐ │     │  - chunk      │
                        │  │ lib-api  │ │ lib-index│ │ lib-db   │ │     │  - entity     │
                        │  │ (API     │ │ (인덱싱  │ │ (DB접속) │ │     │  - relation   │
                        │  │  핸들러) │ │  파이프   │ │          │ │     └───────────────┘
                        │  │          │ │  라인)   │ │          │ │
                        │  └──────────┘ └──────────┘ └──────────┘ │     ┌───────────────┐
                        │                                          │◄───►│ Azure OpenAI  │
                        └──────────────────────────────────────────┘     │ - 임베딩      │
                                                                        │ - 채팅 완성   │
                                                                        └───────────────┘
```

## 주요 기능

### GraphRAG 파이프라인
- **PDF 문서 처리**: 계층적 청킹(제목/섹션/문단)으로 문서를 분할하고 토큰 기반 겹침 윈도우 적용
- **NER(Named Entity Recognition)**: 정규식 기반 엔티티 추출 (인명, 조직, 장소, 날짜)
- **지식 그래프 구축**: 엔티티 간 동시 출현(CO_OCCURS) 관계를 자동 추론
- **다중 관점 임베딩**: 의미적(semantic), 구조적(structural), 기능적(functional) 임베딩 생성

### 검색 및 질의응답
- **벡터 검색** (`POST /api/search/vector`): 코사인 유사도 기반 문서 청크 검색
- **그래프 검색** (`POST /api/search/graph`): 임베딩 기반 시드 엔티티 선택 + BFS 경로 확장
- **통합 채팅** (`POST /api/chat/ask`): 벡터 검색 + 그래프 확장(PageRank, Betweenness 중심성) + LLM 답변 생성

### 인증 및 관리
- **JWT 인증**: Access Token / Refresh Token 기반 인증 시스템
- **재인덱싱** (`POST /api/reindex`): PDF 파일 재처리 및 그래프/임베딩 갱신
- **파일 업로드** (`POST /api/reindex/upload`): PDF 파일 업로드
- **Swagger UI**: `/swagger-ui/` 경로에서 API 문서 확인 가능

## 프로젝트 구조

```
rust-surreal-graph-rag/
├── backend/                    # Rust 백엔드 (Cargo workspace)
│   ├── bin-main/               # 실행 바이너리 (진입점)
│   ├── lib-api/                # API 핸들러
│   │   └── src/
│   │       ├── auth.rs         # JWT 인증 (로그인/로그아웃/갱신)
│   │       ├── chat.rs         # 통합 질의응답 (GraphRAG)
│   │       ├── vector_search.rs # 벡터 검색
│   │       ├── graph_search.rs # 그래프 검색 (BFS 경로 확장)
│   │       ├── reindex.rs      # 재인덱싱 및 파일 업로드
│   │       ├── config.rs       # 환경설정 로더
│   │       ├── azure.rs        # Azure OpenAI 클라이언트
│   │       └── health.rs       # 헬스체크
│   ├── lib-index/              # 인덱싱 파이프라인 라이브러리
│   │   └── src/
│   │       ├── pdf_processor.rs # PDF 처리 및 계층적 청킹
│   │       ├── graph_builder.rs # 엔티티 추출 및 관계 추론
│   │       ├── ner.rs          # NER Trait 및 정규식 기반 구현
│   │       ├── embedding.rs    # 다중 관점 임베딩 생성
│   │       ├── database.rs     # 인덱스 데이터 저장
│   │       └── query_engine.rs # 쿼리 엔진
│   ├── lib-db/                 # SurrealDB 접속 및 초기화
│   ├── Cargo.toml              # 워크스페이스 설정
│   └── Makefile.toml           # cargo-make 태스크 정의
├── frontend/                   # React 프론트엔드
│   └── src/
│       ├── pages/              # 페이지 컴포넌트
│       │   ├── Chat.tsx        # 채팅 페이지
│       │   ├── VectorSearch.tsx # 벡터 검색 페이지
│       │   ├── GraphSearch.tsx # 그래프 검색 페이지
│       │   ├── Reindex.tsx     # 재인덱싱 관리 페이지
│       │   ├── Login.tsx       # 로그인 페이지
│       │   └── Health.tsx      # 헬스체크 페이지
│       └── components/
│           └── NavBar.tsx      # 네비게이션 바
├── tests/                      # API 테스트 (Zaku)
├── docker-compose.yml          # SurrealDB Docker 구성
├── run.sh                      # 통합 실행 스크립트 (Linux/macOS)
├── run.ps1                     # 통합 실행 스크립트 (Windows PowerShell)
└── README.md
```

## 기술 스택

| 계층 | 기술 |
|------|------|
| **백엔드** | Rust (Edition 2024), Actix-web 4, Tokio |
| **데이터베이스** | SurrealDB (벡터 검색 + 그래프 관계 + 문서 저장) |
| **AI/ML** | Azure OpenAI (text-embedding-3-large, GPT-4.1) |
| **인증** | JWT (HS256, Access/Refresh Token) |
| **API 문서** | utoipa + Swagger UI |
| **프론트엔드** | React 18, TypeScript, Fluent UI, Vite |
| **패키지 관리** | Bun (프론트엔드), Cargo (백엔드) |

## 사전 요구 사항

- Rust stable 툴체인 (`rustup`)
- [cargo-make](https://github.com/sagiegurari/cargo-make) (`cargo install cargo-make`)
- Bun >= 1.0
- Docker & Docker Compose
- Azure OpenAI API 키

## 시작하기

### 1. SurrealDB 실행

```bash
# Docker 네트워크 생성 (최초 1회)
docker network create docker-link

# SurrealDB 컨테이너 시작
docker compose up -d
```

### 2. 환경 변수 설정

프로젝트 루트에 `.env` 파일을 생성합니다:

```env
# SurrealDB
SURREALDB_URL=localhost:8000
SURREALDB_USERNAME=root
SURREALDB_PASSWORD=root
SURREALDB_NS=namespace
SURREALDB_DB=database

# Azure OpenAI
AZURE_OPENAI_ENDPOINT=https://<your-resource>.openai.azure.com
AZURE_OPENAI_API_KEY=<your-api-key>
AZURE_OPENAI_CHAT_API_VERSION=2024-06-01
AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4.1
AZURE_OPENAI_EMBED_API_VERSION=2024-02-01
AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-3-large

# JWT (필수 - 미설정 시 서버 시작 실패)
JWT_SECRET=<your-jwt-secret>
ACCESS_TOKEN_TTL_SECS=3600
REFRESH_TOKEN_TTL_SECS=604800

# 서버 설정 (선택)
SERVER_HOST=localhost
SERVER_PORT=4000
UPLOAD_DIR=uploads
```

### 3. 백엔드 빌드 및 실행

```bash
cd backend

# 빌드 (check → clippy → format → build)
cargo make build

# 실행 (RUST_LOG=lib_api=debug,actix_web=info)
cargo make run
```

백엔드 서버가 `http://localhost:4000`에서 시작됩니다.

### 4. 프론트엔드 실행

```bash
cd frontend

# .env 파일 생성
cp .env.example .env
# VITE_API_BASE_URL=http://localhost:4000 설정

# 의존성 설치
bun install

# 개발 서버 시작
bun dev
```

### 5. 통합 실행 (Backend + Frontend 동시)

```bash
# Linux / macOS
./run.sh

# Windows PowerShell
.\run.ps1
```

### 6. API 문서 확인

브라우저에서 `http://localhost:4000/swagger-ui/` 접속

## API 엔드포인트

| 메서드 | 경로 | 설명 | 인증 |
|--------|------|------|------|
| `GET` | `/health` | 헬스체크 | - |
| `POST` | `/api/auth/login` | 로그인 | - |
| `POST` | `/api/auth/refresh` | 토큰 재발급 | Bearer (Refresh) |
| `POST` | `/api/auth/logout` | 로그아웃 | Bearer |
| `GET` | `/api/auth/me` | 내 정보 조회 | Bearer |
| `POST` | `/api/search/vector` | 벡터 검색 | Bearer |
| `POST` | `/api/search/graph` | 그래프 검색 | Bearer |
| `POST` | `/api/chat/ask` | 통합 질의응답 | Bearer |
| `POST` | `/api/reindex` | PDF 재인덱싱 | Bearer |
| `POST` | `/api/reindex/upload` | 파일 업로드 | Bearer |

## 개발 명령어

### 백엔드 (cargo-make)

```bash
cargo make clean    # 빌드 산출물 정리
cargo make check    # 컴파일 검사
cargo make clippy   # 린트 검사
cargo make format   # 코드 포맷팅
cargo make build    # 개발 빌드
cargo make release  # 릴리즈 빌드
cargo make test     # 테스트 실행
cargo make run      # 서버 실행
```

### 프론트엔드

```bash
bun dev             # 개발 서버 (Vite)
bun run build       # 프로덕션 빌드
bun run preview     # 빌드 미리보기
bun run format      # Prettier 포맷팅
bun run format:check # 포맷 검사
```

## 라이선스

이 프로젝트는 학습 및 연구 목적으로 작성되었습니다.
