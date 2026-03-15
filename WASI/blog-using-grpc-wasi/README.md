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
| `blog-component` | WASI 0.2 유효성 검사 및 콘텐츠 정제 컴포넌트 (이메일, 사용자명, 비밀번호, 콘텐츠, 역할, 공개범위, XSS 방지) | Rust | `wasm32-wasip2` |
| `blog-server` | gRPC 서버 (SurrealDB + wasmtime + RBAC) | Rust (tonic) | native |
| `blog-cli-client` | gRPC CLI 클라이언트 (JSON 파라미터 기반) | Rust (clap + serde_json) | native |
| `blog-web-client` | gRPC 웹 클라이언트 (브라우저 UI) | SvelteKit + Bun | Bun |

## RBAC 권한 모델

### 역할 (Role)

| 역할 | 설명 |
|------|------|
| `admin` | 관리자: 모든 데이터에 대한 CRUD, 사용자 역할 변경, 포스트 공개범위 변경 |
| `user` | 일반 사용자: 자신의 포스트/댓글 CRUD, 공개 포스트 읽기 및 댓글 작성 |

### 포스트 공개범위 (Visibility)

| 공개범위 | 설명 |
|---------|------|
| `public` | 모든 사용자(비인증 포함)가 읽기 가능 |
| `private` | 작성자 본인과 관리자만 읽기 가능 |

### 권한 매트릭스

| 작업 | admin | user (본인) | user (타인) | 비인증 |
|------|-------|------------|------------|--------|
| 포스트 생성 | O | O | - | X |
| 포스트 읽기 (public) | O | O | O | O |
| 포스트 읽기 (private) | O | O | X | X |
| 포스트 수정 | O (모두) | O (본인만) | X | X |
| 포스트 삭제 | O (모두) | O (본인만) | X | X |
| 댓글 작성 | O | O (읽기 가능 포스트) | O (읽기 가능 포스트) | X |
| 댓글 읽기 | O | O (읽기 가능 포스트) | O (읽기 가능 포스트) | O (public 포스트) |
| 댓글 삭제 | O (모두) | O (본인만) | X | X |
| 사용자 목록 | O | X | X | X |
| 역할 변경 | O | X | X | X |
| 공개범위 변경 | O | X | X | X |

## 도메인 모델

```
┌──────────┐       ┌───────────┐       ┌──────────┐
│   User   │──1:N──│   Post    │──1:N──│ Comment  │
│          │       │           │       │          │
│ username │       │ title     │       │ content  │
│ email    │       │ content   │       │ author_id│
│ password │       │ author    │       │ post_id  │
│ role     │       │ visibility│       │          │
│ bio      │       │           │       │          │
│ website  │       │           │       │          │
│ theme    │       │           │       │          │
└──────────┘       └───────────┘       └──────────┘
```

## gRPC API

| 서비스 | 설명 | 권한 |
|--------|------|------|
| `Register` | 회원가입 | 없음 |
| `Login` | 로그인 (JWT 토큰 발급) | 없음 |
| `GetMyProfile` | 내 프로필 조회 (bio, website 포함) | 인증 필요 |
| `UpdateProfile` | 프로필 설정 저장 (자기소개, 웹사이트) | 인증 필요 |
| `ChangePassword` | 비밀번호 변경 (현재 비밀번호 확인) | 인증 필요 |
| `DeleteMyAccount` | 회원 탈퇴 (비밀번호 확인) | 인증 필요 |
| `CreatePost` | 포스트 작성 | 인증 필요 |
| `GetPost` | 포스트 상세 조회 | 공개범위에 따라 |
| `ListPosts` | 포스트 목록 조회 (페이지네이션) | 공개범위에 따라 필터링 |
| `SearchPosts` | 포스트 제목/내용 검색 (페이지네이션) | 공개범위에 따라 필터링 |
| `UpdatePost` | 포스트 수정 | 작성자 또는 admin |
| `DeletePost` | 포스트 삭제 | 작성자 또는 admin |
| `CreateComment` | 댓글 작성 | 인증 + 포스트 읽기 권한 |
| `ListComments` | 댓글 목록 조회 (페이지네이션) | 포스트 읽기 권한 |
| `DeleteComment` | 댓글 삭제 | 작성자 또는 admin |
| `ListUsers` | 사용자 목록 조회 | admin 전용 |
| `UpdateUserRole` | 사용자 역할 변경 | admin 전용 |
| `UpdatePostVisibility` | 포스트 공개범위 변경 | admin 전용 |
| `GetStats` | 관리자 통계 (사용자/포스트/댓글 수) | admin 전용 |
| `GetVersion` | WASI 컴포넌트 버전 조회 | 없음 |

## WIT 인터페이스

```wit
package component:blog@0.1.0;

interface blogger {
  validate-title: func(title: string) -> string;
  validate-content: func(content: string) -> string;
  validate-comment: func(content: string) -> string;
  validate-role: func(role: string) -> string;
  validate-visibility: func(visibility: string) -> string;
  validate-email: func(email: string) -> string;
  validate-username: func(username: string) -> string;
  validate-password-strength: func(password: string) -> string;
  sanitize-content: func(content: string) -> string;
  validate-bio: func(bio: string) -> string;
  validate-website: func(website: string) -> string;
  validate-theme: func(theme: string) -> string;
  get-version: func() -> string;
}

world blog-world {
  export blogger;
}
```

### 입력 검증 규칙

| 함수 | 검증 규칙 |
|------|----------|
| `validate-email` | RFC 기본 형식 검증 (로컬@도메인.TLD, 254자 제한, TLD 2자 이상) |
| `validate-username` | 2~30자, 영문/숫자/밑줄/하이픈만, 영문자로 시작, 예약어 차단 |
| `validate-password-strength` | 8~128자, 대문자/소문자/숫자/특수문자 중 2가지 이상 조합 |
| `sanitize-content` | XSS 방지 (script 태그, javascript:, 이벤트 핸들러 제거, 대소문자 무시) |
| `validate-bio` | 500자 제한 (빈값 허용) |
| `validate-website` | 200자 제한, http:// 또는 https:// 필수, 도메인에 `.` 포함 (빈값 허용) |
| `validate-theme` | `dark` 또는 `light`만 허용 |

## SurrealDB 스키마

```sql
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string;
DEFINE FIELD password_hash ON TABLE user TYPE string;
DEFINE FIELD role ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE string;
DEFINE FIELD bio ON TABLE user TYPE string DEFAULT '';
DEFINE FIELD website ON TABLE user TYPE string DEFAULT '';
DEFINE FIELD theme ON TABLE user TYPE string DEFAULT 'dark';
DEFINE INDEX idx_user_username ON TABLE user COLUMNS username UNIQUE;
DEFINE INDEX idx_user_email ON TABLE user COLUMNS email UNIQUE;

DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD title ON TABLE post TYPE string;
DEFINE FIELD content ON TABLE post TYPE string;
DEFINE FIELD author_id ON TABLE post TYPE string;
DEFINE FIELD author_username ON TABLE post TYPE string;
DEFINE FIELD visibility ON TABLE post TYPE string;
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
# 서버 시작 시 admin 사용자가 없으면 기본 관리자가 자동 생성됩니다.
# 기본 관리자: admin / admin@email.com / Pa55w0rd!
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
# 회원가입 (기본 역할: user)
blog-cli-client register '{"username":"alice","email":"alice@example.com","password":"secret123"}'

# 로그인 (토큰이 ~/.blog-cli-token에 저장됨)
blog-cli-client login '{"email":"alice@example.com","password":"secret123"}'

# 관리자 로그인
blog-cli-client login '{"email":"admin@email.com","password":"Pa55w0rd!"}'
```

#### 프로필

```bash
# 내 프로필 조회
blog-cli-client profile me

# 비밀번호 변경
blog-cli-client profile change-password '{"current_password":"secret123","new_password":"newPw456!"}'

# 회원 탈퇴 (비밀번호 확인 필요, 관련 포스트/댓글 모두 삭제)
blog-cli-client profile delete-account '{"password":"secret123"}'
```

#### 포스트

```bash
# 포스트 작성 (인증 필요, Markdown 지원, visibility: "public"(기본) 또는 "private")
blog-cli-client post create '{"title":"첫 번째 포스트","content":"# 안녕하세요\n\nWASI 블로그입니다. **Markdown**을 지원합니다."}'
blog-cli-client post create '{"title":"비공개 포스트","content":"나만 보는 글","visibility":"private"}'

# 포스트 목록 조회 (로그인 상태에 따라 보이는 범위가 다름)
blog-cli-client post list
blog-cli-client post list '{"page":2,"per_page":5}'

# 포스트 상세 조회
blog-cli-client post get '{"id":"post:xxx"}'

# 포스트 수정 (본인 포스트 또는 관리자)
blog-cli-client post update '{"id":"post:xxx","title":"수정된 제목","content":"수정된 내용"}'

# 포스트 삭제 (본인 포스트 또는 관리자)
blog-cli-client post delete '{"id":"post:xxx"}'

# 포스트 검색 (제목/내용 키워드 검색)
blog-cli-client post search '{"query":"WASI"}'
blog-cli-client post search '{"query":"gRPC","page":1,"per_page":5}'
```

#### 댓글

```bash
# 댓글 작성 (인증 필요, 읽기 권한이 있는 포스트에만)
blog-cli-client comment create '{"post_id":"post:xxx","content":"좋은 글이네요!"}'

# 특정 포스트의 댓글 목록 조회 (읽기 권한 필요)
blog-cli-client comment list '{"post_id":"post:xxx"}'

# 댓글 삭제 (본인 댓글 또는 관리자)
blog-cli-client comment delete '{"id":"comment:xxx"}'
```

#### 관리자 전용

```bash
# 사용자 목록 조회 (admin 역할 필요)
blog-cli-client admin list-users
blog-cli-client admin list-users '{"page":1,"per_page":20}'

# 사용자 역할 변경 (admin 역할 필요)
blog-cli-client admin update-role '{"user_id":"xxx","role":"admin"}'
blog-cli-client admin update-role '{"user_id":"xxx","role":"user"}'

# 포스트 공개범위 변경 (admin 역할 필요)
blog-cli-client admin update-visibility '{"post_id":"xxx","visibility":"private"}'
blog-cli-client admin update-visibility '{"post_id":"xxx","visibility":"public"}'
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
| `JWT_SECRET` | `blog-secret-key-change-in-production` | JWT 토큰 서명 시크릿 (프로덕션에서 반드시 변경) |
| `BLOG_WASM_PATH` | `../target/wasm32-wasip2/release/blog_component.wasm` | WASM 컴포넌트 경로 |
| `SURREALDB_ADDR` | `127.0.0.1:8000` | SurrealDB 서버 주소 |
| `SURREALDB_USER` | `root` | SurrealDB 사용자 |
| `SURREALDB_PASS` | `root` | SurrealDB 비밀번호 |
| `SERVER_ADDR` | `http://127.0.0.1:50051` | gRPC 서버 주소 (클라이언트용) |
| `SEED_PATH` | `blog-server/data/seed.json` | 시드 데이터 JSON 파일 경로 |

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
- **jsonwebtoken** - JWT 인증 (RBAC 역할 포함)
- **argon2** - 비밀번호 해싱
- **tokio** - 비동기 런타임
- **marked** - Markdown → HTML 렌더링
- **Bun** - JavaScript/TypeScript 런타임 및 패키지 매니저
- **SvelteKit** - 웹 프론트엔드 프레임워크
- **@grpc/grpc-js** - gRPC 클라이언트

## 동작 원리

1. `blog-component`가 WIT 인터페이스에 따라 이메일/사용자명/비밀번호/콘텐츠/역할/공개범위 유효성 검사 및 XSS 방지 정제 로직 구현
2. `wasm32-wasip2` 타겟으로 컴파일되어 `.wasm` 파일 생성
3. `blog-server`가 시작 시 wasmtime으로 `.wasm` 파일 로드 + SurrealDB 연결 + 기본 admin 시드
4. 회원가입 시 WASI 컴포넌트로 이메일/사용자명/비밀번호 강도 검증 → argon2 해싱 + JWT 토큰 발급 (역할 정보 포함)
5. 포스트/댓글 작성/수정 시 WASI 컴포넌트로 유효성 검사 + XSS 콘텐츠 정제 → RBAC 권한 확인 → SurrealDB에 저장
6. 포스트 조회 시 visibility + 사용자 역할에 따라 접근 제어
7. `blog-web-client`는 SvelteKit SSR로 gRPC 서버와 통신, 쿠키 기반 인증
