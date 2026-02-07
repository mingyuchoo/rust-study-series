# CLAUDE.md — AI 코드 생성 변경 관리 가이드

## 프로젝트 개요

- **프로젝트명**: demo-rust
- **기술 스택**: Rust + Loco Framework + Tera + SeaORM + SQLite
- **프로젝트 유형**: SaaS 웹 애플리케이션 (REST API + Tera 서버사이드 렌더링)
- **Loco 앱 타입**: SaaS App (인증, DB, 백그라운드 작업 포함)
- **프로젝트 구조**: Cargo Workspace 기반 모노레포 (Multi-crate Monorepo)
- **워크스페이스 멤버**:
  - `crates/app` — Loco 웹 애플리케이션 (메인 크레이트)
  - `crates/migration` — SeaORM 데이터베이스 마이그레이션
  - `crates/core` — 공유 도메인 타입, 트레이트, 유틸리티

이 프로젝트는 AI 코드 생성 도구의 비결정성 문제를 방지하고, 스펙↔코드 간 형상 일관성을 유지하기 위한 규칙을 따릅니다.

---

## 핵심 원칙

### 1. 절대 재생성 금지 (No Regeneration)

- 이미 존재하는 코드를 **전체 재생성하지 마세요.**
- 스펙이 변경되면 기존 코드 위에서 **해당 부분만 증분 수정(incremental edit)**하세요.
- 파일을 새로 작성하는 것이 아니라, 기존 파일을 읽고 변경이 필요한 부분만 수정합니다.

### 2. 컨벤션 준수 (Convention First)

- 새 코드를 생성할 때는 반드시 기존 코드의 패턴을 따르세요.
- 동일 디렉토리 내 기존 파일을 먼저 확인하고, 그 구조·네이밍·패턴을 그대로 적용하세요.
- 아래 "코드 컨벤션" 섹션의 규칙을 우선 적용합니다.

### 3. 변경 추적성 (Traceability)

- **모든 작업은 `docs/requirements.md`에 등록된 요구사항 ID 기반으로 수행합니다.**
- 사용자의 지시가 요구사항 ID 없이 들어오면, AI가 먼저 `docs/requirements.md`에 요구사항을 등록한 뒤 작업을 시작합니다.
- 커밋 메시지 형식: `[REQ-XXXX] 변경 내용 요약`
- 코드 내 모든 변경 지점에 주석을 남기세요:

  ```
  // [REQ-XXXX] 요구사항 변경에 따른 수정 (2026-XX-XX)
  ```

#### 요구사항 등록 → 작업 수행 흐름

```
사용자 지시 입력
    │
    ▼
┌─────────────────────────────────┐
│ 1. 요구사항 ID가 명시되어 있는가?    │
└─────────┬───────────┬───────────┘
          │ Yes       │ No
          ▼           ▼
    ID 확인 후    docs/requirements.md에
    작업 진행     신규 등록 (ID 자동 부여)
          │           │
          ▼           ▼
┌─────────────────────────────────┐
│ 2. docs/requirements.md 상태 갱신  │
│    (등록 → 분석완료 → 진행중)       │
└─────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ 3. 영향 범위 파악                  │
│    - 변경 대상 파일 목록 확인        │
│    - docs/requirements.md 영향 파일 갱신│
└─────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ 4. 증분 수정 수행                  │
│    - 코드 주석에 REQ-XXXX 명시      │
│    - 한 번에 1개 파일/모듈 단위    │
└─────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ 5. 작업 완료 후                   │
│    - docs/requirements.md 상태 → 완료│
│    - 커밋 메시지에 REQ-XXXX 포함   │
└─────────────────────────────────┘
```

### 4. 생성 범위 최소화 (Small Scope)

- 한 번에 하나의 함수 또는 하나의 구조체(struct)/모듈만 생성/수정하세요.
- 전체 모듈을 한 번에 만들지 마세요.
- 생성 후 반드시 기존 코드와의 일관성을 확인하세요.

### 5. 테스트 코드 필수 (Test Required)

- **모든 신규 코드 작성 시 대응하는 테스트 코드를 반드시 함께 작성하세요.**
- 기존 코드를 수정할 때는 **영향받는 기존 테스트를 함께 수정**하고, 필요시 새 테스트 케이스를 추가하세요.
- 테스트 없이 프로덕션 코드만 작성·커밋하는 것은 금지합니다.
- 테스트는 프로덕션 코드와 동일한 요구사항 ID(`REQ-XXXX`)로 추적합니다.

---

## 코드 컨벤션

### 언어 및 프레임워크

- 언어: Rust (latest stable)
- 프레임워크: Loco Framework (v0.x)
- 웹 화면 템플릿: Tera
- 데이터베이스: SeaORM + SQLite
- 빌드 도구: Cargo
- HTTP 프레임워크: Axum (Loco 내부)
- 인증: JWT 토큰 인증 (Bearer)
- 비동기 런타임: Tokio

### 네이밍 규칙

- 구조체(Struct)/열거형(Enum)/트레이트(Trait): UpperCamelCase (예: `OrderService`, `AppError`)
- 함수/메서드: snake_case (예: `find_order_by_id`)
- 상수/정적 변수: SCREAMING_SNAKE_CASE (예: `MAX_RETRY_COUNT`)
- 모듈/크레이트: snake_case (예: `post_service`)
- 파일명: snake_case (예: `order_service.rs`)
- 테이블명: 복수형 snake_case (예: `users`, `posts`)
- 컬럼명: snake_case (예: `created_at`, `user_id`)

### 아키텍처 패턴

#### 워크스페이스 크레이트 구조

- **`crates/core`** (라이브러리 크레이트): 공유 도메인 타입, 트레이트, 유틸리티, 에러 타입 정의
  - 다른 내부 크레이트에 의존하지 않음 (최하위 레이어)
  - 크레이트명: `demo-core`
- **`crates/migration`** (라이브러리 크레이트): SeaORM 마이그레이션 전용
  - `demo-core`에 의존 가능, `demo-app`에는 의존 불가
  - 크레이트명: `migration`
- **`crates/app`** (바이너리 + 라이브러리 크레이트): Loco 웹 애플리케이션 메인
  - `demo-core`, `migration`에 의존
  - 크레이트명: `demo-app`
- **의존성 방향**: `app` → `core` ← `migration` (단방향, 순환 참조 금지)
- **공통 의존성**: 루트 `Cargo.toml`의 `[workspace.dependencies]`에서 버전을 통합 관리하고, 각 크레이트에서 `{ workspace = true }`로 참조

#### 앱 내부 레이어 구조 (crates/app)

- 레이어 구조: Controller → Service (Trait + Impl) → Model (SeaORM Entity)
- `models/_entities/` (SeaORM 자동 생성, 직접 수정 금지)와 `models/` (커스텀 ActiveRecord 확장) 분리
- View 구조체(`views/`)와 Entity(`models/`)는 반드시 분리
- 예외 처리: `loco_rs::Error` enum + `Result<T>` 패턴 매칭 사용
- 응답 형식: `format::json()`, `format::html()`, `format::empty()` 사용
- 요청 유효성 검증: `validator` 크레이트의 `#[derive(Validate)]` + `JsonValidate<T>` 사용

#### 신규 크레이트 추가 규칙

- 신규 크레이트는 `crates/` 하위에 생성하고 루트 `Cargo.toml`의 `[workspace] members`에 등록
- 크레이트명은 `demo-{기능}` 패턴 (예: `demo-core`, `demo-auth`)
- 기존 크레이트의 의존성 방향 원칙을 위반하지 않도록 주의

### 테스트 컨벤션

- 테스트 프레임워크: `cargo test` + `loco-rs` testing feature + `insta` (스냅샷 테스트) + `serial_test`
- 테스트 파일 네이밍: `crates/app/tests/requests/{리소스}.rs`, `crates/app/tests/models/{리소스}.rs`, `crates/app/tests/services/{서비스}.rs`
- `crates/core` 단위 테스트: `crates/core/src/` 내 `#[cfg(test)] mod tests` 인라인 테스트
- 테스트 함수 네이밍: `#[tokio::test] async fn can_{동작}()` (예: `can_list_posts`, `can_create_post`)
- 레이어별 테스트 전략:
  - **Controller 계층**: `loco_rs::testing::request::<App, _, _>()` 요청 테스트, HTTP 엔드포인트 전체 검증
  - **Service 계층**: `crates/app/tests/services/{서비스}.rs` 단위 테스트
  - **Model 계층**: `boot_test::<App, Migrator>()` 모델 테스트, 실제 SQLite DB 사용
  - **통합 테스트**: `request_with_create_db::<App, _, _>()` 필요 시에만 사용 (격리된 DB 생성)
  - **Core 계층**: `crates/core/` 내 인라인 단위 테스트 (`#[cfg(test)]`)
- 테스트 실행:
  - 전체 워크스페이스: `cargo test --workspace`
  - 특정 크레이트: `cargo test -p demo-app`, `cargo test -p demo-core`
  - 특정 테스트: `cargo test -p demo-app can_list_posts`
- 테스트 데이터: `crates/app/src/fixtures/` YAML 파일 또는 각 테스트 함수 내에서 직접 생성, `#[serial]` 어트리뷰트로 테스트 간 독립성 보장
- Assertion: `assert!`, `assert_eq!`, `assert_debug_snapshot!` (insta) 사용 권장
- 스냅샷 테스트: `configure_insta!()` 매크로로 초기화, 응답 구조 검증에 활용
- 인증 테스트: `prepare_data::init_user_login()` + `prepare_data::auth_header()` 활용

### 디렉토리 구조

```
├── Cargo.toml                        # 워크스페이스 루트 매니페스트 ([workspace] 정의)
├── Cargo.lock                        # 워크스페이스 전역 락 파일
├── CLAUDE.md                         # AI 코드 생성 변경 관리 가이드
├── README.md                         # 워크스페이스 정보
├── docs/
│   ├── requirements.md               # 요구사항 관리 문서
│   └── test-scenarios.md             # 사용자 테스트 시나리오
├── crates/
│   ├── app/                          # 메인 Loco 웹 애플리케이션 (demo-app)
│   │   ├── Cargo.toml                # demo-app 크레이트 의존성
│   │   ├── config/
│   │   │   ├── development.yaml      # 개발 환경 설정
│   │   │   ├── test.yaml             # 테스트 환경 설정
│   │   │   └── production.yaml       # 운영 환경 설정
│   │   ├── src/
│   │   │   ├── app.rs                # 앱 등록 및 Hooks 트레이트 구현 (라우트, 초기화)
│   │   │   ├── lib.rs                # 라이브러리 엔트리포인트
│   │   │   ├── bin/
│   │   │   │   └── main.rs           # 바이너리 엔트리포인트
│   │   │   ├── controllers/          # REST API + 웹 컨트롤러
│   │   │   │   ├── mod.rs
│   │   │   │   └── post.rs
│   │   │   ├── services/             # 비즈니스 로직 (트레이트 + 구현체)
│   │   │   │   ├── mod.rs
│   │   │   │   └── post_service.rs
│   │   │   ├── models/               # 데이터 모델 (엔티티 + 커스텀 확장)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── _entities/        # SeaORM 자동 생성 (직접 수정 금지)
│   │   │   │   └── posts.rs          # ActiveRecord 확장 (Validator 포함)
│   │   │   ├── views/                # 응답 직렬화 구조체
│   │   │   │   ├── mod.rs
│   │   │   │   └── post.rs
│   │   │   ├── initializers/         # 뷰 엔진 등 초기화
│   │   │   ├── workers/              # 백그라운드 작업 핸들러
│   │   │   ├── fixtures/             # 시드 데이터 (YAML)
│   │   │   └── tasks/                # CLI 태스크
│   │   ├── tests/
│   │   │   ├── mod.rs
│   │   │   ├── requests/             # 컨트롤러(HTTP) 테스트
│   │   │   ├── models/               # 모델 테스트
│   │   │   ├── services/             # 서비스 테스트
│   │   │   └── tasks/                # 태스크 테스트
│   │   └── assets/
│   │       ├── views/                # Tera HTML 템플릿
│   │       ├── static/               # CSS, JS, 이미지 등 정적 파일
│   │       └── i18n/                 # 국제화 파일
│   ├── migration/                    # SeaORM DB 마이그레이션 (migration)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── m20220101_000001_users.rs  # 마이그레이션 파일 예시
│   └── core/                         # 공유 도메인 로직 (demo-core)
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs                # 공유 타입, 트레이트, 유틸리티
```

#### 루트 Cargo.toml 구조 예시

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.dependencies]
# 공통 의존성 버전 통합 관리
loco-rs = { version = "0.x" }
sea-orm = { version = "1.x", features = ["sqlx-sqlite", "runtime-tokio-rustls"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
# 내부 크레이트 참조
demo-core = { path = "crates/core" }
migration = { path = "crates/migration" }
```

---

## 변경 작업 시 워크플로우

### 사용자 지시 수신 시 (최우선 수행)

1. **요구사항 등록 확인**: 사용자의 지시에 REQ-ID가 있는지 확인
2. **ID 없으면 자동 등록**: `docs/requirements.md`에 적절한 구분(F/N/B/R)으로 신규 등록
3. **등록 내용 사용자에게 고지**: "REQ-F002로 등록했습니다. 작업을 진행합니다." 형태로 안내
4. **이후 아래 워크플로우 진행**

### 스펙 변경이 발생했을 때

1. **영향 범위 파악**: 변경되는 스펙이 어떤 파일들에 영향을 주는지 먼저 확인 **(테스트 파일 포함)**
2. **기존 코드 확인**: 해당 파일들을 읽어서 현재 구현 상태 파악
3. **증분 수정 수행**: 변경이 필요한 부분만 정확히 수정
4. **테스트 코드 수정**: 변경된 프로덕션 코드에 대응하는 테스트를 함께 수정하고, 새로운 분기/로직이 추가되면 테스트 케이스도 추가
5. **정적 분석 실행**: `cargo clippy --workspace` 실행하여 경고·에러가 없는지 확인 (정적 분석 통과 후 테스트 진행)
6. **테스트 실행·검증**: `cargo test --workspace` 실행하여 전체 워크스페이스 테스트 통과 확인
7. **시나리오 문서 갱신**: 컨트롤러·모델(Validator)·Service·인증/보안 설정 변경 시 `docs/test-scenarios.md` 해당 시나리오 갱신 (→ "사용자 테스트 시나리오 문서 자동 생성" 섹션 참조)
8. **일관성 검증**: 수정 후 관련 파일들과의 일관성 확인
9. **변경 이력 기록**: 커밋 메시지에 스펙 ID 포함, `docs/requirements.md` 상태 갱신

### 신규 기능 개발 시

1. **레퍼런스 확인**: 동일 레이어의 기존 파일을 먼저 확인 **(기존 테스트 파일 패턴도 확인)**
2. **패턴 복제**: 기존 파일의 구조를 그대로 따라서 생성
3. **단위별 생성**: 한 번에 하나의 파일/모듈만 생성
4. **테스트 코드 작성**: 생성한 프로덕션 코드에 대응하는 테스트 모듈을 동일 요구사항 ID로 작성
   - Service 구현체 → `crates/app/tests/services/` 단위 테스트
   - Controller → `crates/app/tests/requests/` 요청 테스트 (`loco_rs::testing::request`)
   - Model → `crates/app/tests/models/` 모델 테스트 (`boot_test`, 필요 시)
   - Core 공유 로직 → `crates/core/src/` 내 인라인 `#[cfg(test)]` 테스트
5. **정적 분석 실행**: `cargo clippy --workspace` 실행하여 경고·에러가 없는지 확인 (정적 분석 통과 후 테스트 진행)
6. **테스트 실행·검증**: `cargo test --workspace` 실행하여 전체 워크스페이스 테스트 통과 확인
7. **시나리오 문서 작성**: 컨트롤러·모델(Validator)·Service·인증/보안 설정 신규 추가 시 `docs/test-scenarios.md`에 대응 시나리오 추가 (→ "사용자 테스트 시나리오 문서 자동 생성" 섹션 참조)
8. **통합 확인**: 생성 후 호출하는 쪽과 호출받는 쪽의 인터페이스 일치 확인
9. **요구사항 완료 처리**: `docs/requirements.md` 상태를 `완료`로 갱신
10. **문서화**: `README.md` 내용 현행화

---

## 사용자 테스트 시나리오 문서 자동 생성

### 개요

코드베이스를 분석하여 **사용자 관점의 E2E 테스트 시나리오 문서**를 `docs/test-scenarios.md`에 작성·갱신합니다. 컨트롤러, 모델 검증 규칙(Validator), Service 비즈니스 로직, 인증/보안 설정을 체계적으로 분석하여 완전한 시나리오 세트를 도출합니다.

### 분석 대상 소스

| 소스 | 추출 대상 | 분석 파일 패턴 |
| ------ | ---------- | --------------- |
| 컨트롤러 (`crates/app/src/controllers/`) | 사용자 동작(Action) 흐름 | `crates/app/src/controllers/*.rs` (`routes()` 함수, `get`/`post`/`delete`/`patch` 핸들러) |
| 모델 검증 규칙 (`crates/app/src/models/`) | 입력 유효성 시나리오 | `crates/app/src/models/*.rs` (`#[derive(Validate)]`, `#[validate(length)]`, `#[validate(email)]` 등) |
| Service 비즈니스 로직 (`crates/app/src/services/`) | 예외/에러 시나리오 | `crates/app/src/services/*.rs` (`Error` 반환 지점, `Err(Error::NotFound)` 등) |
| 인증/보안 설정 (`crates/app/config/`, `crates/app/src/app.rs`) | 보안/인증 시나리오 | `crates/app/config/*.yaml` (auth, JWT 설정), `crates/app/src/app.rs` (라우트 인증 미들웨어) |

### 시나리오 분류 체계

- **TC-AUTH**: 인증 (회원가입, 로그인, 로그아웃)
- **TC-POST**: 포스트 관리 (CRUD)
- **TC-CMT**: 댓글 관리 (생성, 삭제)
- **TC-SEC**: 권한/보안
- **TC-VAL**: 입력 유효성 검증

### 각 시나리오 필수 포함 항목

```
#### TC-XXX-NNN: {시나리오 제목}

- **카테고리**: 정상 경로 | 예외 경로 | 보안 | 유효성 검증
- **사전 조건**: {테스트 전 필요한 상태}
- **테스트 단계**:
  1. {Step-by-step 사용자 동작}
- **기대 결과**:
  - {예상되는 시스템 응답}
- **코드 근거**: `{파일명}:{라인번호}` (변경 추적용)
```

### 시나리오 도출 워크플로우

1. **컨트롤러 분석**: `crates/app/src/controllers/` 하위의 모든 `routes()` 함수와 `get`/`post`/`delete`/`patch` 핸들러를 순회하여 정상 경로 시나리오 도출
2. **모델 검증 규칙 분석**: `crates/app/src/models/` 하위의 모든 `#[derive(Validate)]` 구조체와 `#[validate(length)]`, `#[validate(email)]` 등 검증 어트리뷰트를 순회하여 유효성 검증 시나리오 도출
3. **Service 비즈니스 로직 분석**: `crates/app/src/services/` 하위의 모든 `Err(Error::...)` 반환 지점을 순회하여 예외 경로 시나리오 도출
4. **인증/보안 설정 분석**: `crates/app/config/*.yaml`의 JWT/auth 설정과 `crates/app/src/app.rs`의 라우트 인증 미들웨어를 분석하여 보안 시나리오 도출
5. **시나리오 통계 작성**: 카테고리별 시나리오 수를 집계하여 문서 말미에 통계표 작성
6. **코드 소스별 매핑표 작성**: 소스 파일:라인 → 시나리오 ID 양방향 추적 매핑표 작성

### 검증 기준

- **엔드포인트 커버리지**: 모든 `routes()` 함수에 등록된 엔드포인트(`get`, `post`, `delete`, `patch`)가 최소 1개 시나리오로 커버되는지 확인
- **예외 커버리지**: 모든 `Err(Error::...)` 반환 지점이 예외 시나리오로 반영되었는지 확인
- **유효성 커버리지**: 모든 `#[derive(Validate)]` 구조체의 검증 규칙(`#[validate(length)]`, `#[validate(email)]` 등)이 유효성 시나리오로 반영되었는지 확인
- **보안 커버리지**: JWT 인증 미들웨어 및 `crates/app/config/*.yaml`의 접근 제어 규칙이 보안 시나리오로 반영되었는지 확인

### 변경 추적성 (양방향 추적)

- **코드 → 시나리오**: 각 시나리오의 "코드 근거" 필드에 `파일명:라인번호`를 명시하여, 코드 변경 시 영향받는 시나리오를 즉시 식별
- **시나리오 → 코드**: 문서 말미에 "코드 소스별 시나리오 매핑" 표를 작성하여, 특정 파일 변경 시 갱신이 필요한 시나리오 목록을 역추적

### 시나리오 문서 갱신 트리거

아래 파일이 변경될 때 `docs/test-scenarios.md`도 함께 갱신해야 합니다:

- `crates/app/src/controllers/*.rs` — 엔드포인트 추가/변경/삭제 시
- `crates/app/src/models/*.rs` — Validator 검증 규칙 추가/변경/삭제 시
- `crates/app/src/services/*.rs` — Error 반환 지점 추가/변경/삭제 시
- `crates/app/config/*.yaml`, `crates/app/src/app.rs` — 인증/접근 제어 규칙 변경 시
- `crates/core/src/**/*.rs` — 공유 도메인 타입/트레이트 변경 시 (영향받는 시나리오 확인)

---

## 금지 사항

- ❌ 기존 파일을 삭제하고 새로 작성하는 행위
- ❌ 동일 기능을 완전히 다른 방식으로 재구현하는 행위
- ❌ 프로젝트 컨벤션에 없는 새로운 패턴을 임의로 도입하는 행위
- ❌ 한 번의 작업에서 3개 이상의 파일을 동시에 대규모 변경하는 행위
- ❌ **요구사항 ID 없이 코드를 변경하는 행위 (docs/requirements.md 미등록 상태에서 작업 금지)**
- ❌ 요구사항 상태를 갱신하지 않고 작업을 종료하는 행위
- ❌ **테스트 코드 없이 프로덕션 코드만 작성·커밋하는 행위**
- ❌ 테스트가 실패하는 상태에서 작업을 완료 처리하는 행위
- ❌ **정적 분석(`cargo clippy --workspace`)을 실행하지 않고 테스트를 실행하는 행위 (반드시 clippy 통과 후 테스트 진행)**
- ❌ **컨트롤러·모델(Validator)·Service·인증/보안 설정 변경 시 `docs/test-scenarios.md`를 갱신하지 않는 행위**
- ❌ 워크스페이스 크레이트 간 순환 의존성을 만드는 행위 (`app` → `core` ← `migration` 단방향 원칙 위반)
- ❌ 루트 `Cargo.toml`의 `[workspace]` 설정을 확인하지 않고 신규 크레이트를 생성하는 행위
- ❌ `[workspace.dependencies]`에 정의된 공통 의존성을 크레이트별 `Cargo.toml`에서 별도 버전으로 지정하는 행위

---

## Loco Framework CLI 참조

### 프로젝트 초기화

```bash
# Loco CLI 설치
cargo install loco
cargo install sea-orm-cli

# 프로젝트 생성 (SaaS App 선택, SQLite DB 선택)
# ※ 모노레포에서는 crates/app 내에서 loco new 후 워크스페이스에 통합
loco new

# 개발 서버 실행 (crates/app 기준)
cargo loco start --environment development

# DB 마이그레이션
cargo loco db migrate

# DB 엔티티 자동 생성
cargo loco db entities
```

### 코드 생성 (Scaffold/Generator)

```bash
# 모델+마이그레이션+컨트롤러+테스트 일괄 생성 (crates/app 기준)
cargo loco generate scaffold post title:string content:text

# 모델+마이그레이션만 생성
cargo loco generate model post title:string content:text

# 컨트롤러만 생성
cargo loco generate controller post

# 마이그레이션만 생성
cargo loco generate migration add_status_to_posts
```

### 자주 사용하는 명령어

```bash
# 전체 워크스페이스 빌드
cargo build --workspace

# 전체 워크스페이스 테스트 실행
cargo test --workspace

# 특정 크레이트 테스트 실행
cargo test -p demo-app
cargo test -p demo-core

# 특정 테스트 실행
cargo test -p demo-app can_list_posts

# 전체 워크스페이스 린트
cargo clippy --workspace

# Loco 태스크 실행
cargo loco task <태스크명>

# 시드 데이터 로드
cargo loco task seed_data
```

### 워크스페이스 관리 명령어

```bash
# 신규 크레이트 생성 후 워크스페이스에 추가
cargo init crates/<크레이트명> --lib
# → 루트 Cargo.toml의 [workspace] members에 자동 포함 (crates/* 글로브 사용 시)

# 워크스페이스 의존성 트리 확인
cargo tree --workspace

# 특정 크레이트의 의존성 확인
cargo tree -p demo-app
```

---

## 역방향 산출물 생성 (리버스 엔지니어링)

감리 또는 고객 산출물 제출이 필요한 경우, 최종 코드를 기반으로 설계서를 재생성할 수 있습니다.

요청 예시:

```
이 코드를 기반으로 상세설계서를 작성해줘.
- 구조체/트레이트 다이어그램
- 시퀀스 다이어그램
- API 엔드포인트 명세
- 비즈니스 로직 설명
형식은 [프로젝트 산출물 템플릿]을 따라줘.
```

---
