# Diesel PostgreSQL 클린 아키텍처

Diesel ORM과 PostgreSQL을 사용하여 클린 아키텍처 원칙을 적용한 Rust 프로젝트입니다.

## 빠른 시작

```bash
# Docker로 전체 실행
docker-compose -f docker/docker-compose.yaml up --build

# 브라우저에서 접속
# 웹 UI: http://localhost:8000
# API: http://localhost:8000/api/todos
```

앱이 자동으로 데이터베이스 설정, 마이그레이션, 샘플 데이터 삽입을 수행합니다.

## 아키텍처

클린 아키텍처 원칙에 따라 여러 크레이트로 분리된 워크스페이스 구조입니다:

- **domain**: 핵심 비즈니스 엔티티 및 리포지토리 트레이트
- **application**: 유스케이스 및 비즈니스 로직
- **adapters**: 리포지토리 구현체 및 외부 어댑터
- **infrastructure**: 데이터베이스 인프라, 마이그레이션, 모델
- **main**: 애플리케이션 진입점 및 의존성 주입

## 주요 기능

- Rust 기반 클린 아키텍처 구현
- Diesel ORM을 통한 PostgreSQL 연동
- Axum 웹 프레임워크 기반 RESTful API
- 전체 CRUD 기능 (생성, 조회, 수정, 삭제)
- Vanilla JavaScript 기반 웹 UI
- Docker 컨테이너화 (멀티 스테이지 빌드)
- 데이터베이스 마이그레이션 및 시딩
- 스레드 안전 데이터베이스 연결 관리

## 사전 준비

- Rust 1.86+ (`rust-toolchain.toml` 참조)
- Docker 및 Docker Compose
- PostgreSQL (로컬 실행 시)

## 시작하기

### Docker 사용 (권장)

```bash
# PostgreSQL과 앱 시작
docker-compose -f docker/docker-compose.yaml up --build

# 또는 make 사용
make docker-up
```

### 로컬 개발

```bash
# Diesel CLI 설치
cargo install diesel_cli --no-default-features --features postgres

# 환경 설정
cp .env.test .env

# PostgreSQL 시작
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:17.6

# 마이그레이션 실행
diesel migration run --config-file infrastructure/diesel.toml

# 애플리케이션 실행
cargo run -p main
```

## 프로젝트 구조

```text
├── adapters/           # 리포지토리 구현체
├── application/        # 유스케이스 (CreateTodo, ListTodos)
├── domain/            # 엔티티 및 리포지토리 트레이트
├── infrastructure/    # 데이터베이스 인프라
│   ├── migrations/    # Diesel 마이그레이션
│   └── src/          # DB 모델 및 스키마
├── main/              # 애플리케이션 진입점
└── docker/            # Docker 설정
```

## 데이터베이스 스키마

```sql
CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    title VARCHAR DEFAULT 'NEW TODO' NOT NULL
);
```

## API 엔드포인트

| 메서드 | 엔드포인트 | 설명 |
|--------|----------|------|
| GET | `/api/todos` | 전체 할일 목록 |
| GET | `/api/todos/:id` | 특정 할일 조회 |
| POST | `/api/todos` | 새 할일 생성 |
| PUT | `/api/todos/:id` | 할일 수정 |
| DELETE | `/api/todos/:id` | 할일 삭제 |

### API 예시

```bash
# 전체 조회
curl http://localhost:8000/api/todos

# 생성
curl -X POST http://localhost:8000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"Learn Rust"}'

# 수정
curl -X PUT http://localhost:8000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Master Rust"}'

# 삭제
curl -X DELETE http://localhost:8000/api/todos/1
```

## 웹 UI

모던하고 반응형인 웹 인터페이스가 포함되어 있습니다:

- 할일 추가: 입력 필드에 텍스트 입력 후 "추가" 클릭 또는 Enter
- 할일 수정: "수정" 버튼 클릭
- 할일 삭제: "삭제" 버튼 클릭 (확인 후 삭제)
- 실시간 갱신: 작업 후 자동 갱신

접속 주소: `http://localhost:8000`

## 주요 의존성

- **웹 프레임워크**: Axum 0.7
- **ORM**: Diesel 2.3
- **데이터베이스**: PostgreSQL 17.6
- **비동기 런타임**: Tokio
- **프론트엔드**: Vanilla JavaScript, HTML5, CSS3

## 환경 변수

- `DATABASE_URL`: PostgreSQL 연결 문자열 (예: `postgres://user:password@localhost:5432/dbname`)
- `RUST_LOG`: 로깅 레벨 (기본: info)

## 개발

### 빌드

```bash
# 전체 크레이트 빌드
cargo build

# 특정 크레이트 빌드
cargo build -p main
```

### 코드 포맷팅

```bash
cargo fmt
```

### API 테스트

```bash
./test_api.sh
```

## Docker

멀티 스테이지 Dockerfile로 빌드 시간을 최적화합니다:
1. 의존성과 소스 코드를 분리하여 캐싱
2. 슬림 런타임 이미지로 최종 크기 축소
3. 런타임에 필요한 의존성만 포함

## 아키텍처 다이어그램

```text
┌──────────────────────────────────────┐
│         프레젠테이션 계층              │
│  (웹 UI + REST API 핸들러)            │
│         main/src/web/                │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│         애플리케이션 계층              │
│  (유스케이스: 생성, 조회, 수정 등)      │
│         application/src/             │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│           도메인 계층                  │
│  (엔티티 + 리포지토리 트레이트)         │
│         domain/src/                  │
└──────────────▲───────────────────────┘
               │
┌──────────────┴───────────────────────┐
│      인프라스트럭처 계층               │
│  어댑터: 리포지토리 구현체             │
│  인프라: 데이터베이스, 마이그레이션, 모델│
│  adapters/src/ + infrastructure/src/ │
└──────────────────────────────────────┘
```

## License

This project serves as a Clean Architecture demonstration in Rust.
