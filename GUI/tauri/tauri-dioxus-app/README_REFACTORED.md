# 주소록 앱 - Clean Architecture 리팩토링

SQLite를 사용하는 Clean Architecture 기반의 크로스 플랫폼 주소록 데스크톱 애플리케이션입니다. 
각 계층이 **별도의 크레이트(Crate)**로 분리되어 의존성 규칙이 Cargo 워크스페이스를 통해 강제됩니다.

## 🏗️ 프로젝트 구조

```
project-root/
├── Cargo.toml                    # 워크스페이스 설정
├── domain/                       # 1️⃣ 도메인 계층 (가장 안쪽 원)
│   ├── Cargo.toml               # 외부 의존성 없음
│   └── src/
│       ├── lib.rs
│       ├── entities/            # 엔티티 (핵심 비즈니스 객체)
│       │   ├── mod.rs
│       │   └── contact.rs
│       ├── repositories/        # 리포지토리 인터페이스 (트레이트)
│       │   ├── mod.rs
│       │   └── contact_repository.rs
│       └── errors.rs            # 도메인 에러
│
├── application/                  # 2️⃣ 애플리케이션 계층 (유스케이스)
│   ├── Cargo.toml               # domain 크레이트에만 의존
│   └── src/
│       ├── lib.rs
│       └── usecases/            # 유스케이스 구현
│           ├── mod.rs
│           ├── create_contact.rs
│           ├── get_contact.rs
│           ├── list_contacts.rs
│           ├── update_contact.rs
│           ├── delete_contact.rs
│           └── search_contacts.rs
│
├── infrastructure/               # 3️⃣ 인프라 계층 (외부 기술)
│   ├── Cargo.toml               # domain 크레이트에 의존
│   └── src/
│       ├── lib.rs
│       └── database/            # 데이터베이스 구현체
│           ├── mod.rs
│           └── sqlite_contact_repository.rs
│
├── presentation_backend/         # 4️⃣ 표현 계층 - 백엔드 (Tauri)
│   ├── Cargo.toml               # 모든 하위 계층에 의존 가능
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/
│   ├── icons/
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── routes/              # Tauri 명령어 핸들러
│       │   ├── mod.rs
│       │   └── contact_commands.rs
│       └── models/              # 요청/응답 DTO
│           ├── mod.rs
│           └── contact_dto.rs
│
└── presentation_frontend/        # 5️⃣ 표현 계층 - 프론트엔드 (Dioxus)
    ├── Cargo.toml
    └── src/
        ├── main.rs
        ├── app.rs               # 메인 앱 컴포넌트
        ├── models/              # 프론트엔드 타입 정의
        │   ├── mod.rs
        │   └── contact.rs
        ├── components/          # UI 컴포넌트
        │   ├── mod.rs
        │   ├── contact_form.rs
        │   └── contact_list.rs
        └── services/            # 백엔드 통신 서비스
            ├── mod.rs
            └── contact_service.rs
```

## 🎯 Clean Architecture 의존성 규칙

각 계층의 의존성은 Cargo.toml을 통해 강제됩니다:

### 1. Domain (도메인 계층)
- **의존성**: 없음 (순수 비즈니스 로직)
- **역할**: 엔티티, 리포지토리 인터페이스, 도메인 에러 정의
- **특징**: 외부 프레임워크나 라이브러리에 의존하지 않음

### 2. Application (애플리케이션 계층)
- **의존성**: `domain` 크레이트만 의존
- **역할**: 비즈니스 로직 유스케이스 구현
- **특징**: 인프라나 프레젠테이션 계층을 알지 못함

### 3. Infrastructure (인프라 계층)
- **의존성**: `domain` 크레이트에 의존
- **역할**: 리포지토리 구현체 (SQLite, 외부 API 등)
- **특징**: 도메인 인터페이스를 구현

### 4. Presentation Backend (표현 계층 - 백엔드)
- **의존성**: `domain`, `application`, `infrastructure` 모두 의존 가능
- **역할**: Tauri 명령어 핸들러, DTO 변환
- **특징**: 가장 바깥쪽 계층, 모든 계층 조합

### 5. Presentation Frontend (표현 계층 - 프론트엔드)
- **의존성**: 백엔드와 독립적
- **역할**: Dioxus UI 컴포넌트, 사용자 인터페이스
- **특징**: Tauri 명령어를 통해 백엔드와 통신

## 🚀 빌드 및 실행

### 개발 모드
```bash
# 전체 워크스페이스 빌드
cargo build

# 백엔드 개발 서버 실행
cargo tauri dev --manifest-path presentation_backend/Cargo.toml

# 프론트엔드만 개발 (Dioxus)
dx serve --port 1420
```

### 프로덕션 빌드
```bash
# 전체 애플리케이션 빌드
cargo tauri build --manifest-path presentation_backend/Cargo.toml
```

### 개별 크레이트 테스트
```bash
# 도메인 계층 테스트
cargo test -p domain

# 애플리케이션 계층 테스트
cargo test -p application

# 인프라 계층 테스트
cargo test -p infrastructure
```

## 📦 의존성 관리

워크스페이스 레벨에서 공통 의존성을 관리합니다 (`Cargo.toml`):

```toml
[workspace.dependencies]
# 도메인 계층
serde = { version = "1", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
thiserror = "1.0"

# 애플리케이션 계층
async-trait = "0.1"

# 인프라 계층
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid"] }
tokio = { version = "1", features = ["full"] }

# 표현 계층
tauri = { version = "2", features = [] }
dioxus = { features = ["web"], version = "0.6" }
```

## 🔑 핵심 장점

### 1. 의존성 규칙 강제
- Cargo 워크스페이스가 컴파일 타임에 의존성 위반을 방지
- 예: `domain` 크레이트는 `infrastructure`를 import할 수 없음

### 2. 테스트 용이성
- 각 계층을 독립적으로 테스트 가능
- Mock 구현체를 쉽게 주입 가능

### 3. 유지보수성
- 각 계층의 책임이 명확히 분리
- 변경 사항이 다른 계층에 영향을 최소화

### 4. 확장성
- 새로운 인프라 구현체 추가 용이 (예: PostgreSQL)
- 새로운 프레젠테이션 계층 추가 가능 (예: CLI, Web API)

## 🛠️ 기술 스택

- **언어**: Rust (Edition 2024)
- **프론트엔드**: Dioxus 0.6
- **백엔드**: Tauri 2.0
- **데이터베이스**: SQLite (SQLx)
- **비동기**: Tokio
- **아키텍처**: Clean Architecture (Crate 분리)

## 📝 마이그레이션 가이드

기존 `src-tauri/` 구조에서 새로운 크레이트 구조로 마이그레이션:

1. **도메인 계층**: `src-tauri/src/domain/` → `domain/src/`
2. **애플리케이션 계층**: `src-tauri/src/application/` → `application/src/`
3. **인프라 계층**: `src-tauri/src/infrastructure/` → `infrastructure/src/`
4. **백엔드 표현**: `src-tauri/src/presentation/` → `presentation_backend/src/`
5. **프론트엔드 표현**: `src/` → `presentation_frontend/src/`

## 🎓 학습 자료

- [Clean Architecture (Robert C. Martin)](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Rust Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Tauri Documentation](https://tauri.app/)
- [Dioxus Documentation](https://dioxuslabs.com/)

## 📄 라이선스

MIT License
