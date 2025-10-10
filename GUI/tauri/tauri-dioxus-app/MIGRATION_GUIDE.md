# Clean Architecture 마이그레이션 가이드

이 문서는 기존 단일 크레이트 구조에서 Clean Architecture 기반의 멀티 크레이트 구조로 마이그레이션하는 방법을 설명합니다.

## 📋 목차

1. [마이그레이션 개요](#마이그레이션-개요)
2. [의존성 규칙 검증](#의존성-규칙-검증)
3. [단계별 마이그레이션](#단계별-마이그레이션)
4. [빌드 및 테스트](#빌드-및-테스트)
5. [문제 해결](#문제-해결)

## 마이그레이션 개요

### 기존 구조
```
tauri-dioxus-app/
├── src/                    # 프론트엔드
└── src-tauri/              # 백엔드 (모든 계층이 하나의 크레이트)
    └── src/
        ├── domain/
        ├── application/
        ├── infrastructure/
        └── presentation/
```

### 새로운 구조
```
tauri-dioxus-app/
├── domain/                 # 독립 크레이트
├── application/            # 독립 크레이트 (domain에만 의존)
├── infrastructure/         # 독립 크레이트 (domain에 의존)
├── presentation_backend/   # 독립 크레이트 (모든 계층 의존)
└── presentation_frontend/  # 독립 크레이트 (백엔드와 독립)
```

## 의존성 규칙 검증

Clean Architecture의 핵심은 **의존성 규칙**입니다. Cargo 워크스페이스를 통해 이를 강제합니다:

### ✅ 허용되는 의존성

```
presentation_backend → infrastructure → domain
                    → application → domain
                    → domain

presentation_frontend (독립)
```

### ❌ 금지되는 의존성

```
domain → application (X)
domain → infrastructure (X)
application → infrastructure (X)
infrastructure → application (X)
```

이러한 규칙은 `Cargo.toml`의 `[dependencies]` 섹션에서 강제됩니다.

## 단계별 마이그레이션

### 1단계: 워크스페이스 설정

루트 `Cargo.toml` 생성:

```toml
[workspace]
resolver = "2"
members = [
    "domain",
    "application",
    "infrastructure",
    "presentation_backend",
    "presentation_frontend",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["you"]

[workspace.dependencies]
# 공통 의존성 정의
serde = { version = "1", features = ["derive"] }
# ... 기타 의존성
```

### 2단계: Domain 계층 분리

```bash
# 디렉터리 생성
mkdir -p domain/src/{entities,repositories}

# 파일 이동
cp src-tauri/src/domain/entities/contact.rs domain/src/entities/
cp src-tauri/src/domain/repositories/contact_repository.rs domain/src/repositories/
cp src-tauri/src/domain/errors.rs domain/src/
```

`domain/Cargo.toml` 생성:

```toml
[package]
name = "domain"
version.workspace = true
edition.workspace = true

[dependencies]
# 외부 계층에 의존하지 않음
serde.workspace = true
chrono.workspace = true
uuid.workspace = true
thiserror.workspace = true
async-trait.workspace = true
```

**중요**: Domain 계층은 다른 계층에 의존하지 않습니다!

### 3단계: Application 계층 분리

```bash
mkdir -p application/src/usecases
cp src-tauri/src/application/use_cases/*.rs application/src/usecases/
```

`application/Cargo.toml` 생성:

```toml
[package]
name = "application"
version.workspace = true
edition.workspace = true

[dependencies]
# domain 크레이트에만 의존
domain = { path = "../domain" }
async-trait.workspace = true
uuid.workspace = true
```

**핵심**: `domain` 크레이트만 의존합니다.

### 4단계: Infrastructure 계층 분리

```bash
mkdir -p infrastructure/src/database
cp src-tauri/src/infrastructure/database/*.rs infrastructure/src/database/
```

`infrastructure/Cargo.toml` 생성:

```toml
[package]
name = "infrastructure"
version.workspace = true
edition.workspace = true

[dependencies]
# domain 크레이트에 의존
domain = { path = "../domain" }
sqlx.workspace = true
tokio.workspace = true
async-trait.workspace = true
chrono.workspace = true
uuid.workspace = true
```

**주의**: `application` 크레이트에 의존하지 않습니다!

### 5단계: Presentation Backend 분리

```bash
mkdir -p presentation_backend/src/{routes,models}
cp src-tauri/src/presentation/commands/*.rs presentation_backend/src/routes/
cp src-tauri/src/presentation/dto/*.rs presentation_backend/src/models/
cp src-tauri/src/lib.rs presentation_backend/src/
cp src-tauri/src/main.rs presentation_backend/src/
cp src-tauri/build.rs presentation_backend/
cp src-tauri/tauri.conf.json presentation_backend/
cp -r src-tauri/icons presentation_backend/
cp -r src-tauri/capabilities presentation_backend/
```

`presentation_backend/Cargo.toml` 생성:

```toml
[package]
name = "presentation_backend"
version.workspace = true
edition.workspace = true

[lib]
name = "presentation_backend_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[dependencies]
# 모든 하위 계층에 의존 가능
domain = { path = "../domain" }
application = { path = "../application" }
infrastructure = { path = "../infrastructure" }

tauri.workspace = true
tauri-plugin-opener.workspace = true
# ... 기타 의존성
```

### 6단계: Presentation Frontend 분리

```bash
mkdir -p presentation_frontend/src/{components,services,models}
cp src/app.rs presentation_frontend/src/
cp src/main.rs presentation_frontend/src/
cp src/types.rs presentation_frontend/src/models/contact.rs
cp src/components/*.rs presentation_frontend/src/components/
cp src/services/*.rs presentation_frontend/src/services/
cp -r assets presentation_frontend/
```

`presentation_frontend/Cargo.toml` 생성:

```toml
[package]
name = "presentation_frontend"
version.workspace = true
edition.workspace = true

[dependencies]
dioxus.workspace = true
dioxus-logger.workspace = true
# ... 기타 프론트엔드 의존성
```

## 빌드 및 테스트

### 전체 워크스페이스 빌드

```bash
cargo build --workspace
```

### 개별 크레이트 빌드

```bash
# Domain 계층만
cargo build -p domain

# Application 계층만
cargo build -p application

# Infrastructure 계층만
cargo build -p infrastructure
```

### 의존성 규칙 검증

```bash
# Domain이 다른 크레이트에 의존하지 않는지 확인
cargo tree -p domain --depth 1

# Application이 domain에만 의존하는지 확인
cargo tree -p application --depth 1

# Infrastructure가 domain에만 의존하는지 확인
cargo tree -p infrastructure --depth 1
```

### 테스트 실행

```bash
# 전체 워크스페이스 테스트
cargo test --workspace

# 계층별 테스트
cargo test -p domain
cargo test -p application
cargo test -p infrastructure
```

### 애플리케이션 실행

```bash
# 개발 모드
cargo tauri dev --manifest-path presentation_backend/Cargo.toml

# 프로덕션 빌드
cargo tauri build --manifest-path presentation_backend/Cargo.toml
```

## 문제 해결

### 1. 순환 의존성 오류

**증상**:
```
error: cyclic package dependency: package `application` depends on itself
```

**해결**:
- `Cargo.toml`의 `[dependencies]` 섹션을 확인
- 의존성 방향이 올바른지 검증 (안쪽 → 바깥쪽만 허용)

### 2. 타입 불일치 오류

**증상**:
```
error: mismatched types
expected struct `domain::Contact`
found struct `Contact`
```

**해결**:
- `use domain::Contact;` 추가
- 각 크레이트에서 올바른 타입을 import

### 3. 트레이트 구현 오류

**증상**:
```
error[E0117]: only traits defined in the current crate can be implemented
```

**해결**:
- Orphan Rule 위반
- `From` 트레이트 대신 직접 변환 함수 사용
- 예: `.map_err(|e| DomainError::DatabaseError(e.to_string()))`

### 4. 에셋 경로 오류

**증상**:
```
error: Asset at /assets/styles.css doesn't exist
```

**해결**:
```bash
mkdir -p presentation_frontend/assets
cp assets/styles.css presentation_frontend/assets/
```

### 5. Tauri 설정 오류

**증상**:
```
error: unable to read Tauri config file
```

**해결**:
- `presentation_backend/tauri.conf.json` 존재 확인
- `frontendDist` 경로가 올바른지 확인
- `$schema` 필드 추가

## 마이그레이션 체크리스트

- [ ] 워크스페이스 `Cargo.toml` 생성
- [ ] Domain 크레이트 생성 및 파일 이동
- [ ] Application 크레이트 생성 및 파일 이동
- [ ] Infrastructure 크레이트 생성 및 파일 이동
- [ ] Presentation Backend 크레이트 생성 및 파일 이동
- [ ] Presentation Frontend 크레이트 생성 및 파일 이동
- [ ] 각 크레이트의 `Cargo.toml` 의존성 확인
- [ ] Import 경로 수정 (`crate::` → `domain::`, `application::` 등)
- [ ] 에셋 파일 복사
- [ ] Tauri 설정 파일 복사 및 수정
- [ ] `cargo check --workspace` 성공 확인
- [ ] `cargo test --workspace` 성공 확인
- [ ] `cargo tauri dev` 실행 확인
- [ ] 의존성 규칙 검증 (`cargo tree`)

## 추가 리소스

- [Cargo Workspaces 문서](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Clean Architecture 원칙](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Rust Orphan Rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules)

## 결론

이 마이그레이션을 통해:

1. **의존성 규칙이 컴파일 타임에 강제**됩니다
2. **각 계층을 독립적으로 테스트**할 수 있습니다
3. **코드 재사용성과 유지보수성**이 향상됩니다
4. **새로운 구현체 추가**가 용이해집니다

Clean Architecture의 핵심은 **비즈니스 로직(Domain)을 외부 기술로부터 독립**시키는 것입니다. 
이제 데이터베이스를 SQLite에서 PostgreSQL로 변경하거나, 새로운 프레젠테이션 계층(CLI, Web API)을 추가하는 것이 훨씬 쉬워졌습니다!
