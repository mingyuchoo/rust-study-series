# 주소록 앱 - Clean Architecture (크레이트 분리)

## 🎯 프로젝트 개요

이 프로젝트는 **Clean Architecture 원칙**을 따르는 Rust 기반 주소록 애플리케이션입니다.
각 계층이 **독립적인 크레이트(Crate)**로 분리되어 있으며, **의존성 규칙이 Cargo 워크스페이스를 통해 컴파일 타임에 강제**됩니다.

- **프론트엔드**: Dioxus 0.6 (WebAssembly)
- **백엔드**: Tauri 2.0
- **데이터베이스**: SQLite (SQLx)
- **아키텍처**: Clean Architecture (5개 독립 크레이트)

## 📁 프로젝트 구조

```
project-root/
├── Cargo.toml                    # 워크스페이스 설정
│
├── domain/                       # 1️⃣ 도메인 계층 (의존성 없음)
│   ├── Cargo.toml
│   └── src/
│       ├── entities/            # Contact 엔티티
│       ├── repositories/        # ContactRepository 트레이트
│       └── errors.rs            # DomainError
│
├── application/                  # 2️⃣ 애플리케이션 계층 (domain만 의존)
│   ├── Cargo.toml
│   └── src/
│       └── usecases/            # 6개 유스케이스
│           ├── create_contact.rs
│           ├── get_contact.rs
│           ├── list_contacts.rs
│           ├── update_contact.rs
│           ├── delete_contact.rs
│           └── search_contacts.rs
│
├── infrastructure/               # 3️⃣ 인프라 계층 (domain 의존)
│   ├── Cargo.toml
│   └── src/
│       └── database/
│           └── sqlite_contact_repository.rs
│
├── presentation_backend/         # 4️⃣ 백엔드 표현 계층 (모든 계층 의존)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── capabilities/
│   ├── icons/
│   └── src/
│       ├── routes/              # Tauri 명령어
│       └── models/              # DTO
│
└── presentation_frontend/        # 5️⃣ 프론트엔드 표현 계층 (독립)
    ├── Cargo.toml
    ├── assets/
    └── src/
        ├── app.rs               # 메인 앱
        ├── components/          # UI 컴포넌트
        ├── services/            # 백엔드 통신
        └── models/              # 프론트엔드 타입
```

## 🔄 의존성 그래프

```
presentation_frontend (독립)
    │
    └─ (Tauri IPC)
    
presentation_backend
    ├─ domain
    ├─ application
    │   └─ domain
    └─ infrastructure
        └─ domain
```

**핵심**: 의존성은 항상 안쪽(Domain)을 향합니다!

## 🚀 빠른 시작

### 필수 요구사항

- Rust 1.70+ (Edition 2024)
- Node.js (Dioxus CLI용)
- SQLite

### 설치

```bash
# Dioxus CLI 설치
cargo install dioxus-cli

# 프로젝트 클론
git clone <repository-url>
cd tauri-dioxus-app
```

### 개발 모드 실행

```bash
# 전체 워크스페이스 빌드
cargo build --workspace

# 백엔드 + 프론트엔드 개발 서버 실행
cargo tauri dev --manifest-path presentation_backend/Cargo.toml
```

### 프로덕션 빌드

```bash
cargo tauri build --manifest-path presentation_backend/Cargo.toml
```

## 🧪 테스트

```bash
# 전체 워크스페이스 테스트
cargo test --workspace

# 계층별 테스트
cargo test -p domain
cargo test -p application
cargo test -p infrastructure

# 코드 검증
cargo check --workspace
cargo clippy --workspace --all-targets
cargo fmt --all -- --check
```

## 📊 Clean Architecture 원칙

### 1. 의존성 규칙 (Dependency Rule)

| 계층 | 의존성 | 검증 방법 |
|------|--------|-----------|
| Domain | 없음 | `cargo tree -p domain --depth 1` |
| Application | domain만 | `cargo tree -p application --depth 1` |
| Infrastructure | domain만 | `cargo tree -p infrastructure --depth 1` |
| Presentation Backend | 모든 계층 | `cargo tree -p presentation_backend --depth 1` |
| Presentation Frontend | 독립 | `cargo tree -p presentation_frontend --depth 1` |

### 2. 계층별 책임

#### Domain (도메인 계층)
- **책임**: 비즈니스 규칙, 엔티티, 리포지토리 인터페이스
- **특징**: 외부 프레임워크/라이브러리에 의존하지 않음
- **예시**: `Contact` 엔티티, `ContactRepository` 트레이트

#### Application (애플리케이션 계층)
- **책임**: 유스케이스, 비즈니스 로직 조율
- **특징**: Domain 인터페이스만 사용
- **예시**: `CreateContactUseCase`, `ListContactsUseCase`

#### Infrastructure (인프라 계층)
- **책임**: 외부 기술 구현 (DB, API 등)
- **특징**: Domain 인터페이스를 구현
- **예시**: `SqliteContactRepository`

#### Presentation Backend (백엔드 표현 계층)
- **책임**: Tauri 명령어, DTO 변환
- **특징**: 모든 계층을 조합
- **예시**: `create_contact` 명령어, `ContactDto`

#### Presentation Frontend (프론트엔드 표현 계층)
- **책임**: UI 컴포넌트, 사용자 인터페이스
- **특징**: Tauri IPC를 통해 백엔드와 통신
- **예시**: `ContactForm`, `ContactList`

## 🎓 주요 개념

### 의존성 역전 원칙 (DIP)

```rust
// Application 계층은 인터페이스에 의존
pub struct CreateContactUseCase {
    repository: Arc<dyn ContactRepository>,  // 트레이트
}

// Infrastructure 계층이 인터페이스를 구현
pub struct SqliteContactRepository { ... }

impl ContactRepository for SqliteContactRepository {
    async fn create(&self, contact: Contact) -> Result<Contact, DomainError> {
        // SQLite 구현
    }
}
```

**장점**: 데이터베이스를 PostgreSQL로 변경해도 Application 계층은 수정 불필요!

### Cargo 워크스페이스를 통한 의존성 강제

```toml
# application/Cargo.toml
[dependencies]
domain = { path = "../domain" }  # ✅ 허용
# infrastructure = { path = "../infrastructure" }  # ❌ 컴파일 에러!
```

Cargo가 컴파일 타임에 잘못된 의존성을 방지합니다.

## 🔧 개발 가이드

### 새로운 유스케이스 추가

1. `application/src/usecases/` 에 파일 생성
2. Domain 인터페이스만 사용
3. `application/src/usecases/mod.rs` 에 export 추가

```rust
// application/src/usecases/export_contacts.rs
use domain::{Contact, ContactRepository, DomainError};
use std::sync::Arc;

pub struct ExportContactsUseCase {
    repository: Arc<dyn ContactRepository>,
}

impl ExportContactsUseCase {
    pub fn new(repository: Arc<dyn ContactRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, format: &str) -> Result<String, DomainError> {
        let contacts = self.repository.get_all().await?;
        // Export logic
        Ok(format!("Exported {} contacts", contacts.len()))
    }
}
```

### 새로운 인프라 구현체 추가

```rust
// infrastructure/src/database/postgres_contact_repository.rs
use domain::{Contact, ContactRepository, DomainError};

pub struct PostgresContactRepository {
    pool: PgPool,
}

#[async_trait]
impl ContactRepository for PostgresContactRepository {
    // PostgreSQL 구현
}
```

**변경 필요**: Infrastructure 계층만
**변경 불필요**: Domain, Application 계층

## 📚 문서

- **README_REFACTORED.md**: 전체 프로젝트 개요
- **MIGRATION_GUIDE.md**: 단계별 마이그레이션 가이드
- **CLEAN_ARCHITECTURE_SUMMARY.md**: 아키텍처 요약
- **DEPENDENCY_DIAGRAM.md**: 의존성 다이어그램

## 🎯 핵심 장점

### 1. 컴파일 타임 의존성 검증
```bash
# Domain이 Infrastructure에 의존하려고 하면?
error[E0432]: unresolved import `infrastructure`
```

### 2. 독립적인 테스트
```bash
# Mock 리포지토리로 Application 테스트
cargo test -p application
```

### 3. 확장 용이성
- 새로운 DB: Infrastructure 계층만 추가
- 새로운 UI: Presentation 계층만 추가
- 비즈니스 로직 변경: Application 계층만 수정

### 4. 유지보수성
- 각 계층의 책임이 명확
- 변경 사항의 영향 범위 최소화

## 🔄 확장 시나리오

### PostgreSQL 추가
```bash
# infrastructure/src/database/postgres_contact_repository.rs 생성
# presentation_backend/src/lib.rs 에서 선택적으로 사용
```

### REST API 추가
```bash
# 새 크레이트 생성
mkdir presentation_rest_api
# domain, application, infrastructure 재사용
```

### CLI 추가
```bash
# 새 크레이트 생성
mkdir presentation_cli
# 기존 비즈니스 로직 재사용
```

## 🛠️ 기술 스택

| 계층 | 기술 |
|------|------|
| Domain | Rust (순수) |
| Application | Rust + async-trait |
| Infrastructure | SQLx + SQLite |
| Presentation Backend | Tauri 2.0 |
| Presentation Frontend | Dioxus 0.6 |

## 📝 라이선스

MIT License

## 🤝 기여

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📞 문의

- Issues: GitHub Issues
- Email: [your-email]

---

**Clean Architecture = 의존성 방향 제어 + 계층 분리 + 컴파일 타임 검증**

이 프로젝트는 Rust의 강력한 타입 시스템과 Cargo 워크스페이스를 활용하여
Clean Architecture 원칙을 실제로 강제하는 방법을 보여줍니다. 🎉
