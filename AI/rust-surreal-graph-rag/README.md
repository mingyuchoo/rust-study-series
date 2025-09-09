# Rust SurrealDB Graph RAG

**개발 중인 프로젝트입니다.**

Azure OpenAI와 SurrealDB를 활용한 그래프 기반 RAG(Retrieval-Augmented Generation) 시스템입니다. PDF 문서를 인덱싱하고 벡터 검색 및 AI 기반 질의응답을 제공합니다.

## 🚀 주요 기능

- **PDF 문서 처리**: PDF 파일 업로드 및 텍스트 추출 (lopdf 사용)
- **벡터 검색**: TF-IDF 기반 문서 임베딩 및 유사도 검색
- **AI 채팅**: Azure OpenAI를 통한 컨텍스트 기반 질의응답
- **그래프 데이터베이스**: SurrealDB를 활용한 문서 관계 저장
- **웹 인터페이스**: React + TypeScript 기반 사용자 친화적 UI
- **인증 시스템**: JWT 기반 사용자 인증 및 권한 관리
- **API 문서화**: Swagger UI를 통한 자동 API 문서 생성

## 🏗️ 아키텍처

### 백엔드 (Rust)

```text
backend/
├── bin-main/          # 메인 실행 바이너리
├── lib-api/           # REST API 및 웹 서버
├── lib-db/            # SurrealDB 연결 및 데이터베이스 로직
└── lib-index/         # 문서 인덱싱 및 벡터 검색 엔진
```

### 프론트엔드 (React + TypeScript)

```text
frontend/
├── src/
│   ├── components/    # 재사용 가능한 UI 컴포넌트
│   ├── pages/         # 페이지 컴포넌트
│   └── services/      # API 통신 서비스
```

## 🛠️ 기술 스택

### 백엔드

- **언어**: Rust (Edition 2024)
- **웹 프레임워크**: Actix Web 4.9
- **데이터베이스**: SurrealDB 2.1
- **AI 서비스**: Azure OpenAI
- **PDF 처리**: lopdf 0.31
- **인증**: JWT (jsonwebtoken)
- **API 문서화**: utoipa + Swagger UI

### 프론트엔드

- **언어**: TypeScript
- **프레임워크**: React 18.3
- **UI 라이브러리**: Fluent UI (Microsoft)
- **라우팅**: React Router DOM 6.26
- **HTTP 클라이언트**: Axios
- **빌드 도구**: Vite 5.4

### 인프라

- **컨테이너**: Docker Compose
- **데이터베이스**: SurrealDB (Docker)
- **개발 환경**: Node.js 18+, pnpm 8+

## 📋 사전 요구사항

- **Rust**: 1.70+ (Edition 2024 지원)
- **Node.js**: 18.0.0+
- **pnpm**: 8.0.0+
- **Docker**: 최신 버전
- **Azure OpenAI**: API 키 및 엔드포인트

## 🚀 설치 및 실행

### 1. 저장소 클론

```bash
git clone <repository-url>
cd rust-surreal-graph-rag
```

### 2. 환경 변수 설정

```bash
# 백엔드 환경 변수
cp backend/.env.example backend/.env
# Azure OpenAI 설정을 .env 파일에 추가

# 프론트엔드 환경 변수  
cp frontend/.env.example frontend/.env
```

### 3. 데이터베이스 시작

```bash
docker-compose up -d surrealdb
```

### 4. 백엔드 실행

```bash
cd backend
cargo run --bin bin-main
```

서버가 `http://localhost:4000`에서 실행됩니다.

### 5. 프론트엔드 실행

```bash
cd frontend
pnpm install
pnpm dev
```

웹 애플리케이션이 `http://localhost:5173`에서 실행됩니다.

## 📚 API 문서

백엔드 서버 실행 후 다음 URL에서 Swagger UI를 통해 API 문서를 확인할 수 있습니다:

- **Swagger UI**: <http://localhost:4000/swagger-ui/>
- **OpenAPI JSON**: <http://localhost:4000/api-doc/openapi.json>

### 주요 엔드포인트

| 엔드포인트 | 메서드 | 설명 |
|-----------|--------|------|
| `/health` | GET | 헬스체크 |
| `/auth/login` | POST | 사용자 로그인 |
| `/auth/refresh` | POST | 토큰 갱신 |
| `/auth/logout` | POST | 로그아웃 |
| `/auth/me` | GET | 사용자 정보 조회 |
| `/search/vector` | POST | 벡터 검색 |
| `/chat/ask` | POST | AI 질의응답 |
| `/reindex/pdfs` | POST | PDF 재인덱싱 |
| `/reindex/upload` | POST | 파일 업로드 |

## 🔧 개발 가이드

### 백엔드 개발

```bash
cd backend

# 의존성 설치 및 빌드
cargo build

# 테스트 실행
cargo test

# 개발 모드 실행 (자동 재시작)
cargo watch -x "run --bin bin-main"
```

### 프론트엔드 개발

```bash
cd frontend

# 의존성 설치
pnpm install

# 개발 서버 시작
pnpm dev

# 빌드
pnpm build

# 코드 포맷팅
pnpm format
```

## 🧪 테스트

### Postman 컬렉션

`tests/postman/` 디렉토리에 API 테스트용 Postman 컬렉션이 포함되어 있습니다:

- `rust-surreal-graph-rag.postman_collection.json`
- `rust-surreal-graph-rag__dev.postman_environment.json`

### Zaku 테스트

`tests/zaku/` 디렉토리에 추가 테스트 도구가 포함되어 있습니다.

## 🐳 Docker 배포

전체 스택을 Docker로 실행:

```bash
docker-compose up -d
```

이 명령어는 다음을 시작합니다:

- SurrealDB (포트 8000)
- 백엔드 API 서버 (포트 4000)
- 프론트엔드 웹 서버 (포트 5173)

## 🔒 보안 고려사항

- JWT 토큰 기반 인증 시스템
- Azure OpenAI API 키는 환경 변수로 관리
- SurrealDB 접근 권한 설정
- CORS 정책 적용
- 파일 업로드 크기 제한 (100MB)

## 🤝 기여하기

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 `LICENSE` 파일을 참조하세요.

## 📞 지원

문제가 발생하거나 질문이 있으시면 GitHub Issues를 통해 문의해 주세요.

---

**참고**: 이 프로젝트는 Rust 학습 시리즈의 일부로 개발되었습니다.
