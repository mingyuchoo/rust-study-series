# Axum로 구현한 클린 아키텍처 예제

이 저장소는 Rust 웹 프레임워크인 `axum`과 `sqlx`를 사용해 클린 아키텍처(로버트 C. 마틴)를 실용적으로 적용한 예제입니다. 계층 간 의존성을 명확히 분리하고, `도메인 → 애플리케이션 → 어댑터/인프라` 흐름으로 유지보수성과 테스트 용이성을 높입니다.

- 원본 소개 영상: <https://youtu.be/TrNpyFMtnzI>

## 주요 기능

- 사용자 등록 API 제공
  - `POST /api/user/register`
  - 본문: `username`, `email`, `password`
  - 응답: `{ "success": true }` (201 Created)

## 프로젝트 구조

```text
src/
  domain/                # 핵심 도메인 모델 (`entities/`)
  application/           # 유스케이스 계층 (`use_cases/`, 에러 타입)
  adapters/
    http/                # HTTP 라우터/핸들러, 상태 (`routes/`, `app_state.rs`)
    persistence/         # 영속성 어댑터 (PostgreSQL 구현)
    crypto/              # 암호화 어댑터 (Argon2 패스워드 해시)
  infra/                 # 앱 조립, 설정, DB, 로깅 (`app.rs`, `setup.rs`, `config.rs`, `db.rs`)
  main.rs                # 엔트리포인트 (`create_app`, `init_app_state` 호출)

migrations/
  20250819195936_create_users.sql  # `users` 테이블 생성
```

각 계층 간 의존 규칙:

- `domain`은 어떤 외부에도 의존하지 않습니다.
- `application`은 `domain`에만 의존합니다.
- `adapters`는 `application`에 의존하며, 외부 IO(HTTP, DB, Crypto)를 캡슐화합니다.
- `infra`는 실제 런타임 조립(의존성 주입, 라우팅, 로깅, CORS 등)을 담당합니다.

## 기술 스택

- 런타임/웹: `axum`, `tokio`, `tower`, `tower-http`
- DB: `sqlx` (PostgreSQL, `runtime-tokio-rustls`, `macros`, `uuid`, `chrono`)
- 설정/보안: `dotenvy`, `secrecy`
- 암호화: `argon2`
- 로깅/관찰성: `tracing`, `tracing-subscriber`
- 기타: `anyhow`, `thiserror`, `uuid`, `chrono`, `time`

## 환경 변수 (.env)

다음 키를 `.env`에 설정하세요. 샘플은 `/.env.example` 참고.

```env
DATABASE_URL=postgres://USER:PASSWORD@HOST:PORT/DB
JWT_SECRET=your-very-secret-value
ACCESS_TOKEN_TTL_SECS=30        # 액세스 토큰 TTL(초)
REFRESH_TOKEN_TTL_DAYS=30       # 리프레시 토큰 TTL(일)
```

`infra::config::AppConfig`는 위 값을 읽어 `access_token_ttl`, `refresh_token_ttl`을 `time::Duration`으로 보관합니다.

## 데이터베이스 마이그레이션

`migrations/20250819195936_create_users.sql`의 스키마는 다음과 같습니다.

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

로컬에서 `sqlx` 클라이언트나 선호하는 마이그레이션 툴로 적용하세요. 앱은 `DATABASE_URL`로 연결합니다.

### sqlx 설치 및 실행

- `sqlx-cli` 설치 (PowerShell):

  ```powershell
  cargo install sqlx-cli --no-default-features --features rustls,postgres
  ```

- 데이터베이스 연결 문자열 설정 (세션 한정):

  ```powershell
  $env:DATABASE_URL = "postgres://USER:PASSWORD@HOST:PORT/DB"
  ```

- 데이터베이스 생성:

  ```powershell
  sqlx database create
  ```

- 마이그레이션 적용:

  ```powershell
  sqlx migrate run
  ```

- 마이그레이션 상태 확인:

  ```powershell
  sqlx migrate info
  ```

## 빌드 및 실행

사전 요구사항: Rust(edition 2024), PostgreSQL, `.env` 준비

```bash
cargo run
```

서버는 기본적으로 `127.0.0.1:8000`에서 기동됩니다. (`src/main.rs`)

## HTTP 엔드포인트

- 베이스 경로: `/api` (`infra::app::create_app()`에서 `adapters::http::routes::router()`를 `/api`로 네스트)

### 사용자 등록

- 경로: `POST /api/user/register`
- 요청 본문(JSON):

  ```json
  {
    "username": "jane",
    "email": "jane@example.com",
    "password": "plain-text-password"
  }
  ```

- 응답: `201 Created`

  ```json
  { "success": true }
  ```

핸들러: `adapters::http::routes::user::register`

- 입력 모델: `RegisterPayload { username, email, password }`
- 유스케이스 호출: `UserUseCases::add(...)`
- 비밀번호는 어댑터 계층의 Argon2 해시기로 해시되어 `users.password_hash`에 저장됩니다.

## CORS 및 로깅

- CORS: `http://localhost:5173` 오리진 허용. 메서드 `GET`, `POST`, 헤더 `Content-Type`, `Authorization`, 크리덴셜 허용. (`infra::app::create_app`)
- 로깅: 콘솔(프리티) + 파일(JSON, `app.log`). 환경 변수 필터(`RUST_LOG` 등)가 없으면 `axum_trainer=debug,tower_http=debug` 기본 필터 사용. (`infra::setup::init_tracing`)

## 아키텍처 연결 고리

- `src/main.rs`: `init_app_state()`로 의존성 조립 → `create_app()`으로 라우터 구성 → `axum::serve()`로 기동
- `infra::setup::init_app_state`: `AppConfig`, DB 풀, Argon2 해시기, `UserUseCases`를 생성해 `AppState`로 주입
- `adapters::http::app_state::AppState`: 라우트 핸들러에서 `State<Arc<UserUseCases>>`로 주입받아 호출

## 참고

- 본 예제는 교육용이며, 토큰 발급/검증, 세션 관리, 에러 응답 표준화, 트랜잭션/리포지토리 패턴 확장 등은 필요에 따라 보완하세요.

