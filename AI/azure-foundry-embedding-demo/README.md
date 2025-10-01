# Azure Foundry Embedding Demo

Clean Architecture 원칙을 따르는 Azure OpenAI 임베딩 서비스 데모 애플리케이션입니다.

## 🏗️ 아키텍처

이 프로젝트는 Clean Architecture 원칙을 따라 구현되었습니다:

```text
src/
├── domain/              # 도메인 계층 (비즈니스 로직)
│   ├── entities.rs      # 엔티티
│   └── value_objects.rs # 값 객체
├── application/         # 애플리케이션 계층 (유스케이스)
│   ├── ports.rs         # 포트 (인터페이스)
│   └── usecases.rs      # 유스케이스
├── adapters/            # 어댑터 계층 (외부 인터페이스)
│   └── http/            # HTTP API
│       ├── handlers.rs  # 핸들러
│       ├── models.rs    # DTO
│       └── routes.rs    # 라우팅
├── infra/               # 인프라 계층 (구현체)
│   ├── azure_embedding_service.rs  # Azure OpenAI 클라이언트
│   ├── database.rs                 # 데이터베이스 설정
│   └── sqlite_repository.rs        # SQLite 저장소
├── lib.rs               # 라이브러리 진입점
└── main.rs              # 애플리케이션 진입점
```

### Clean Architecture 원칙

- **도메인 계층**: 비즈니스 로직과 엔티티를 포함하며, 외부 의존성이 없습니다.
- **애플리케이션 계층**: 유스케이스와 포트를 정의하며, 도메인 계층에만 의존합니다.
- **어댑터 계층**: 외부 인터페이스(HTTP API)를 제공하며, 애플리케이션 계층에 의존합니다.
- **인프라 계층**: 외부 서비스와 데이터베이스 구현체를 포함하며, 애플리케이션 계층의 포트를 구현합니다.

## 🚀 시작하기

### 환경 변수 설정

`.env` 파일을 생성하고 다음 환경 변수를 설정합니다:

```env
AZURE_OPENAI_ENDPOINT=https://your-endpoint.openai.azure.com
AZURE_OPENAI_API_KEY=your-api-key
AZURE_OPENAI_DEPLOYMENT_NAME=text-embedding-3-large
DATABASE_URL=sqlite:./data/embeddings.db
SERVER_HOST=0.0.0.0
SERVER_PORT=8000
```

### 로컬 실행

```powershell
# 의존성 설치 및 빌드
cargo build --release

# 실행
cargo run --release
```

### Docker 실행

```powershell
# Docker 이미지 빌드
docker build -f docker/Dockerfile -t azure-foundry-embedding-demo .

# Docker 컨테이너 실행
docker run -p 8000:8000 `
  -e AZURE_OPENAI_ENDPOINT=your-endpoint `
  -e AZURE_OPENAI_API_KEY=your-api-key `
  -v ${PWD}/data:/app/data `
  azure-foundry-embedding-demo
```

### Docker Compose 실행

```powershell
# .env 파일 설정 후
cd docker
docker-compose up -d
```

## 📚 API 엔드포인트

### 헬스 체크

```http
GET /health
```

### 임베딩 생성

```http
POST /embeddings
Content-Type: application/json

{
  "text": "안녕하세요, 오늘 날씨가 참 좋네요."
}
```

### 배치 임베딩 생성

```http
POST /embeddings/batch
Content-Type: application/json

{
  "texts": [
    "첫 번째 텍스트",
    "두 번째 텍스트"
  ]
}
```

### 유사도 검색

```http
POST /embeddings/search
Content-Type: application/json

{
  "query": "날씨",
  "limit": 10
}
```

### 모든 임베딩 조회

```http
GET /embeddings
```

### 특정 임베딩 조회

```http
GET /embeddings/:id
```

### 임베딩 삭제

```http
DELETE /embeddings/:id
```

## 🧪 테스트

```powershell
cargo test
```

## 📦 빌드 최적화

Docker 이미지는 멀티 스테이지 빌드를 사용하여 빌드 속도와 이미지 크기를 최적화합니다:

1. **Chef 단계**: cargo-chef를 사용하여 의존성 캐싱 준비
2. **Planner 단계**: 의존성 레시피 생성
3. **Builder 단계**: 의존성 및 애플리케이션 빌드
4. **Runtime 단계**: 최소한의 런타임 이미지 생성

## 🛠️ 기술 스택

- **언어**: Rust
- **웹 프레임워크**: Axum
- **데이터베이스**: SQLite (sqlx)
- **HTTP 클라이언트**: reqwest
- **비동기 런타임**: Tokio
- **직렬화**: serde
- **에러 처리**: anyhow

## 📄 라이선스

MIT License
