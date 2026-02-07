# demo-rust

Rust + Loco Framework 기반 음악 스트리밍 서비스 웹 애플리케이션입니다.

## 기술 스택

- **언어**: Rust (stable)
- **웹 프레임워크**: [Loco](https://loco.rs) v0.16 (Axum 기반)
- **ORM**: SeaORM 1.1
- **데이터베이스**: SQLite
- **템플릿 엔진**: Tera (SSR) + HTMX
- **인증**: JWT (Bearer Token)
- **비동기 런타임**: Tokio

## 프로젝트 구조

Cargo Workspace 기반 모노레포입니다.

```
crates/
  app/        # Loco 웹 애플리케이션 (demo-app)
  migration/  # SeaORM DB 마이그레이션
  core/       # 공유 도메인 타입, 트레이트 (demo-core)
```

### 주요 레이어

```
Controller → Service (Trait + Impl) → Model (SeaORM Entity)
```

## 주요 기능

| 기능 | 설명 |
|------|------|
| 회원 인증 | 회원가입, 로그인, JWT 토큰 인증 |
| 트랙 관리 | 음악 트랙 CRUD, 공개/비공개 전환, 소유자 권한 검증 |
| 투표 시스템 | 트랙 upvote/downvote, 사용자당 1표 제한, 투표 변경/취소 |
| 댓글 시스템 | 트랙 댓글 작성/삭제, 작성자 권한 검증 |
| 웹 프론트엔드 | Tera SSR + HTMX, 오디오 플레이어 |

## 시작하기

### 사전 요구사항

- Rust stable toolchain (`rustup`)
- SQLite3

### 설치 및 실행

```bash
# 의존성 빌드
cargo build --workspace
cd crates/app/

cargo run -- db migrate

# 시드 데이터 생성 (최초 1회, 100 회원 + 100 트랙 + 100 댓글)
cargo run -- task seed_data

# 개발 서버 실행 (포트 5150)
cargo run -- start --environment development
```

### 테스트

```bash
# 정적 분석 (테스트 전 필수)
cargo clippy --workspace

# 단위/통합 테스트 실행
cargo test --workspace

# 브라우저 E2E 테스트 (개발 서버 실행 상태에서)
# 1. 브라우저에서 접속: http://localhost:5150/static/e2e-test-runner.html
# 2. "Run All Tests" 버튼 클릭
# 17개 시나리오(트랙 7, 투표 6, 댓글 4) 자동 실행
```

## API 엔드포인트

### 인증 (`/api/auth`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| POST | `/api/auth/register` | 회원가입 |
| POST | `/api/auth/login` | 로그인 |
| GET | `/api/auth/current` | 현재 사용자 조회 |
| GET | `/api/auth/verify/{token}` | 이메일 인증 |
| POST | `/api/auth/forgot` | 비밀번호 찾기 |
| POST | `/api/auth/reset` | 비밀번호 재설정 |

### 트랙 (`/api/tracks`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| GET | `/api/tracks/` | 공개 트랙 목록 |
| POST | `/api/tracks/` | 트랙 등록 |
| GET | `/api/tracks/my` | 내 트랙 목록 |
| GET | `/api/tracks/{id}` | 공개 트랙 상세 |
| GET | `/api/tracks/{id}/detail` | 트랙 상세 (소유자) |
| PUT | `/api/tracks/{id}` | 트랙 수정 |
| DELETE | `/api/tracks/{id}` | 트랙 삭제 |
| POST | `/api/tracks/{id}/toggle-public` | 공개/비공개 전환 |

### 투표 (`/api/tracks/{id}/vote`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| POST | `/api/tracks/{id}/vote` | 투표 (upvote/downvote) |
| DELETE | `/api/tracks/{id}/vote` | 투표 취소 |

### 댓글 (`/api/tracks/{id}/comments`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| GET | `/api/tracks/{id}/comments` | 댓글 목록 |
| POST | `/api/tracks/{id}/comments` | 댓글 작성 |
| DELETE | `/api/tracks/{id}/comments/{comment_id}` | 댓글 삭제 |

### 웹 페이지

| 경로 | 설명 |
|------|------|
| `/` | 홈 (공개 트랙 목록) |
| `/auth/login` | 로그인 페이지 |
| `/auth/register` | 회원가입 페이지 |
| `/tracks/new` | 트랙 등록 페이지 |
| `/tracks/{id}` | 트랙 상세 페이지 |
| `/tracks/{id}/edit` | 트랙 수정 페이지 |
| `/my/tracks` | 내 트랙 관리 페이지 |
| `/static/e2e-test-runner.html` | E2E 테스트 러너 (17개 시나리오) |

## 설정

환경별 설정 파일은 `crates/app/config/` 디렉토리에 있습니다.

| 파일 | 설명 |
|------|------|
| `development.yaml` | 개발 환경 (포트 5150, SQLite) |
| `test.yaml` | 테스트 환경 |
| `production.yaml` | 운영 환경 |

## 문서

- [CLAUDE.md](CLAUDE.md) - AI 코드 생성 변경 관리 가이드
- [docs/requirements.md](docs/requirements.md) - 요구사항 관리 문서
- [docs/test-scenarios.md](docs/test-scenarios.md) - 사용자 테스트 시나리오
