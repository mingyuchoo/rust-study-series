# blog-using-grpc-wasi

WASI 0.2 Component Model + gRPC + SurrealDB 블로그 서비스 (Rust Mono-repo)

## 아키텍처

```
┌──────────────────────────────────────────────────────────────────────┐
│                       Mono Repo Workspace                            │
│                                                                      │
│  ┌─────────────────┐                                                 │
│  │   blog-cli-client   │──────────────────┐                             │
│  │  (Native Rust)  │                  │ gRPC                        │
│  └─────────────────┘                  │                             │
│                                       ▼                             │
│  ┌──────────────────────┐   ┌──────────────────────┐                │
│  │   blog-web-client    │──▶│     blog-server      │                │
│  │  (SvelteKit + Bun)   │◀──│ (Native Rust + tonic)│                │
│  └──────────────────────┘   └────────┬─────┬───────┘                │
│           gRPC                       │     │                        │
│                            wasmtime  │     │ WebSocket              │
│                       ┌──────────────▼┐  ┌─▼──────────────────┐     │
│                       │ blog-component│  │     SurrealDB      │     │
│                       │(WASI 0.2/WASM)│  │   (User/Post/      │     │
│                       │ wasm32-wasip2 │  │    Comment)         │     │
│                       └───────────────┘  └────────────────────┘     │
└──────────────────────────────────────────────────────────────────────┘
```

## 구성 요소

| 구성 요소 | 설명 | 언어/런타임 | 빌드 타겟 |
|----------|------|------------|----------|
| `blog-component` | WASI 0.2 콘텐츠 유효성 검사 컴포넌트 | Rust | `wasm32-wasip2` |
| `blog-server` | gRPC 서버 (SurrealDB + wasmtime) | Rust (tonic) | native |
| `blog-cli-client` | gRPC CLI 클라이언트 (JSON 파라미터 기반) | Rust (clap + serde_json) | native |
| `blog-web-client` | gRPC 웹 클라이언트 (브라우저 UI) | SvelteKit + Bun | Bun |

## 도메인 모델

```
┌─────────┐       ┌─────────┐       ┌──────────┐
│  User   │──1:N──│  Post   │──1:N──│ Comment  │
│         │       │         │       │          │
│ username│       │ title   │       │ content  │
│ email   │       │ content │       │ author_id│
│ password│       │ author  │       │ post_id  │
└─────────┘       └─────────┘       └──────────┘
```

## gRPC API

| 서비스 | 설명 |
|--------|------|
| `Register` | 회원가입 |
| `Login` | 로그인 (JWT 토큰 발급) |
| `CreatePost` | 포스트 작성 (인증 필요) |
| `GetPost` | 포스트 상세 조회 |
| `ListPosts` | 포스트 목록 조회 (페이지네이션) |
| `UpdatePost` | 포스트 수정 (작성자만) |
| `DeletePost` | 포스트 삭제 (작성자만) |
| `CreateComment` | 댓글 작성 (인증 필요) |
| `ListComments` | 댓글 목록 조회 |
| `DeleteComment` | 댓글 삭제 (작성자만) |
| `GetVersion` | WASI 컴포넌트 버전 조회 |

## WIT 인터페이스

```wit
package component:blog@0.1.0;

interface blogger {
  validate-title: func(title: string) -> string;
  validate-content: func(content: string) -> string;
  validate-comment: func(content: string) -> string;
  get-version: func() -> string;
}

world blog-world {
  export blogger;
}
```

## SurrealDB 스키마

```sql
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string;
DEFINE FIELD password_hash ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE string;
DEFINE INDEX idx_user_username ON TABLE user COLUMNS username UNIQUE;
DEFINE INDEX idx_user_email ON TABLE user COLUMNS email UNIQUE;

DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD title ON TABLE post TYPE string;
DEFINE FIELD content ON TABLE post TYPE string;
DEFINE FIELD author_id ON TABLE post TYPE string;
DEFINE FIELD author_username ON TABLE post TYPE string;
DEFINE FIELD created_at ON TABLE post TYPE string;
DEFINE FIELD updated_at ON TABLE post TYPE string;

DEFINE TABLE comment SCHEMAFULL;
DEFINE FIELD content ON TABLE comment TYPE string;
DEFINE FIELD post_id ON TABLE comment TYPE string;
DEFINE FIELD author_id ON TABLE comment TYPE string;
DEFINE FIELD author_username ON TABLE comment TYPE string;
DEFINE FIELD created_at ON TABLE comment TYPE string;
```

## 시작하기

### 사전 요구사항

```bash
# Rust 설치 (https://rustup.rs)
cargo install cargo-make

# Docker 설치 (https://docs.docker.com/get-docker/)
# SurrealDB는 Docker 컨테이너로 실행됩니다.

# Bun 설치 (https://bun.sh)

# WASI 타겟 추가
cargo make setup
```

### 실행

**터미널 1** - SurrealDB 시작:
```bash
cargo make start-db
# 또는 파일 기반 영구 저장:
# cargo make start-db-file
```

**터미널 2** - gRPC 서버 실행:
```bash
cargo make run-server
```

**터미널 3** - CLI 클라이언트 실행:
```bash
cargo make run-cli-client -- version
```

**터미널 4** - 웹 클라이언트 실행:
```bash
cargo make run-web-client
# http://localhost:5173 에서 접속
```

### 빌드

```bash
# 전체 빌드
cargo make build

# 개별 빌드
cargo make build-component      # WASI 컴포넌트
cargo make build-server         # gRPC 서버
cargo make build-cli-client     # CLI 클라이언트
cargo make build-web-client     # 웹 클라이언트
```

### CLI 클라이언트 사용법

모든 명령의 파라미터는 JSON 형식으로 전달합니다.

#### 인증

```bash
# 회원가입
blog-cli-client register '{"username":"alice","email":"alice@example.com","password":"secret123"}'

# 로그인 (토큰이 ~/.blog-cli-token에 저장됨)
blog-cli-client login '{"email":"alice@example.com","password":"secret123"}'
```

#### 포스트

```bash
# 포스트 작성 (인증 필요)
blog-cli-client post create '{"title":"첫 번째 포스트","content":"WASI 블로그입니다."}'

# 포스트 목록 조회 (파라미터 생략 시 page=1, per_page=10)
blog-cli-client post list
blog-cli-client post list '{"page":2,"per_page":5}'

# 포스트 상세 조회
blog-cli-client post get '{"id":"post:xxx"}'

# 포스트 수정 (인증 필요, 본인 포스트만)
blog-cli-client post update '{"id":"post:xxx","title":"수정된 제목","content":"수정된 내용"}'

# 포스트 삭제 (인증 필요, 본인 포스트만)
blog-cli-client post delete '{"id":"post:xxx"}'
```

#### 댓글

```bash
# 댓글 작성 (인증 필요)
blog-cli-client comment create '{"post_id":"post:xxx","content":"좋은 글이네요!"}'

# 특정 포스트의 댓글 목록 조회
blog-cli-client comment list '{"post_id":"post:xxx"}'

# 댓글 삭제 (인증 필요, 본인 댓글만)
blog-cli-client comment delete '{"id":"comment:xxx"}'
```

#### 시스템

```bash
# 서버 버전 확인
blog-cli-client version

# gRPC 서버 주소 지정
blog-cli-client --server http://192.168.1.100:50051 post list
# 또는 환경변수 사용
SERVER_ADDR=http://192.168.1.100:50051 blog-cli-client post list
```

### 환경 변수

| 변수 | 기본값 | 설명 |
|------|--------|------|
| `BLOG_WASM_PATH` | `../target/wasm32-wasip2/release/blog_component.wasm` | WASM 컴포넌트 경로 |
| `SURREALDB_ADDR` | `127.0.0.1:8000` | SurrealDB 서버 주소 |
| `SURREALDB_USER` | `root` | SurrealDB 사용자 |
| `SURREALDB_PASS` | `root` | SurrealDB 비밀번호 |
| `SERVER_ADDR` | `http://127.0.0.1:50051` | gRPC 서버 주소 (클라이언트용) |

## cargo-make 태스크 목록

| 태스크 | 설명 |
|--------|------|
| `cargo make setup` | WASI 빌드 타겟 설치 |
| `cargo make start-db` | SurrealDB 시작 (인메모리, Docker) |
| `cargo make start-db-file` | SurrealDB 시작 (파일 기반, Docker) |
| `cargo make stop-db` | SurrealDB 중지 |
| `cargo make build` | 전체 빌드 |
| `cargo make build-component` | WASI 컴포넌트 빌드 |
| `cargo make build-server` | gRPC 서버 빌드 |
| `cargo make build-cli-client` | CLI 클라이언트 빌드 |
| `cargo make build-web-client` | 웹 클라이언트 빌드 |
| `cargo make run-server` | gRPC 서버 실행 |
| `cargo make run-cli-client` | CLI 클라이언트 실행 (인자를 `--` 뒤에 전달) |
| `cargo make run-web-client` | 웹 클라이언트 개발 서버 실행 |
| `cargo make run-web-client-prod` | 웹 클라이언트 프로덕션 서버 실행 |
| `cargo make install-web-client` | 웹 클라이언트 의존성 설치 (bun) |
| `cargo make check` | 전체 코드 오류 검사 |
| `cargo make fmt` | 코드 포맷팅 |
| `cargo make clippy` | Clippy 린트 |
| `cargo make clean` | 빌드 산출물 삭제 |

## 기술 스택

- **Rust** - 시스템 프로그래밍 언어
- **WASI 0.2** - WebAssembly System Interface (Component Model)
- **wasmtime** - WebAssembly 런타임
- **wit-bindgen** - WIT 인터페이스 바인딩 생성
- **tonic** - Rust gRPC 프레임워크
- **prost** - Protocol Buffers 구현
- **clap** - CLI 인자 파싱
- **serde_json** - JSON 파라미터 역직렬화
- **SurrealDB** - 멀티모델 데이터베이스
- **jsonwebtoken** - JWT 인증
- **argon2** - 비밀번호 해싱
- **tokio** - 비동기 런타임
- **Bun** - JavaScript/TypeScript 런타임 및 패키지 매니저
- **SvelteKit** - 웹 프론트엔드 프레임워크
- **@grpc/grpc-js** - gRPC 클라이언트

## 동작 원리

1. `blog-component`가 WIT 인터페이스에 따라 콘텐츠 유효성 검사 로직 구현
2. `wasm32-wasip2` 타겟으로 컴파일되어 `.wasm` 파일 생성
3. `blog-server`가 시작 시 wasmtime으로 `.wasm` 파일 로드 + SurrealDB 연결
4. 회원가입/로그인 시 argon2 해싱 + JWT 토큰 발급
5. 포스트/댓글 작성 시 WASI 컴포넌트로 유효성 검사 → SurrealDB에 저장
6. `blog-web-client`는 SvelteKit SSR로 gRPC 서버와 통신, 쿠키 기반 인증
