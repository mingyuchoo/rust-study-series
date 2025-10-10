# 주소록 앱 - Clean Architecture with Tauri + Leptos

SQLite를 사용한 주소 데이터 CRUD 기능을 제공하는 데스크톱 애플리케이션입니다.
Clean Architecture 패턴을 적용하여 각 계층을 별도의 Rust 크레이트로 분리했습니다.

## 아키텍처

```
project-root/
├── domain/                 # 도메인 계층 (엔티티, 리포지토리 인터페이스)
├── application/            # 애플리케이션 계층 (유스케이스)
├── infrastructure/         # 인프라 계층 (SQLite 구현체)
├── presentation_backend/   # 백엔드 표현 계층 (Tauri 명령어)
├── presentation_frontend/  # 프론트엔드 표현 계층 (Leptos UI)
└── tauri-entrypoint/       # Tauri 애플리케이션 진입점
```

## 기능

- 주소 추가, 조회, 수정, 삭제 (CRUD)
- SQLite 데이터베이스 저장
- 반응형 웹 UI (Leptos)
- 데스크톱 애플리케이션 (Tauri)

## 개발 환경 설정

### 필수 요구사항

- Rust (최신 stable 버전)
    - `rustup target add wasm32-unknown-unknown`
- Trunk (Leptos 빌드용): `cargo install trunk`
- SQLx CLI (데이터베이스 마이그레이션용): `cargo install sqlx-cli --no-default-features --features sqlite`

### 실행 방법

1. 의존성 설치:
```bash
cargo build
```

2. 개발 서버 실행:
```bash
cd tauri-entrypoint
cargo tauri dev
```

3. 프로덕션 빌드:
```bash
cd tauri-entrypoint
cargo tauri build
```

### 프로젝트 구조

- **domain**: 비즈니스 로직의 핵심 엔티티와 리포지토리 인터페이스 정의
- **application**: 유스케이스 구현 (비즈니스 로직 조율)
- **infrastructure**: SQLite 데이터베이스 구현체 (SQLx 사용)
- **presentation_backend**: Tauri 명령어 핸들러 및 상태 관리
- **presentation_frontend**: Leptos 기반 웹 UI (CSR 모드)
- **tauri-entrypoint**: Tauri 애플리케이션 실행 진입점

## Clean Architecture 설계

이 프로젝트는 Clean Architecture 원칙을 따라 의존성이 외부에서 내부로만 향하도록 설계되었습니다.

### 계층별 역할 및 의존성

```
┌─────────────────────────────────────────────────────────┐
│  Presentation Layer (표현 계층)                          │
│  ┌──────────────────────┐  ┌──────────────────────┐    │
│  │ presentation_frontend│  │ presentation_backend │    │
│  │  - Leptos UI         │  │  - Tauri Commands    │    │
│  │  - 사용자 인터페이스    │  │  - API 핸들러         │    │
│  └──────────────────────┘  └──────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                    ↓                    ↓
┌─────────────────────────────────────────────────────────┐
│  Application Layer (애플리케이션 계층)                    │
│  ┌──────────────────────────────────────────────────┐  │
│  │ application                                       │  │
│  │  - AddressService (유스케이스)                     │  │
│  │  - 비즈니스 로직 조율                               │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Domain Layer (도메인 계층) - 핵심 비즈니스 로직           │
│  ┌──────────────────────────────────────────────────┐  │
│  │ domain                                            │  │
│  │  - Address Entity (엔티티)                        │  │
│  │  - AddressRepository Trait (인터페이스)           │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                           ↑
┌─────────────────────────────────────────────────────────┐
│  Infrastructure Layer (인프라 계층)                      │
│  ┌──────────────────────────────────────────────────┐  │
│  │ infrastructure                                    │  │
│  │  - SqliteAddressRepository (구현체)               │  │
│  │  - 데이터베이스 연결 및 쿼리                         │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 각 계층의 상세 구조

#### 1. Domain Layer (도메인 계층)
가장 안쪽 계층으로, 외부 의존성이 전혀 없습니다.

```
domain/
├── entities/
│   └── address.rs          # Address 엔티티 정의
└── repositories/
    └── address_repository.rs  # AddressRepository trait 정의
```

- **엔티티**: 비즈니스 규칙을 담은 핵심 객체 (Address)
- **리포지토리 인터페이스**: 데이터 접근 추상화 (trait)

#### 2. Application Layer (애플리케이션 계층)
도메인 계층만 의존합니다.

```
application/
└── usecases/
    └── address_service.rs  # AddressService 구현
```

- **유스케이스**: 비즈니스 로직 조율 (CRUD 작업)
- 리포지토리 인터페이스를 통해 데이터 접근

#### 3. Infrastructure Layer (인프라 계층)
도메인 계층의 인터페이스를 구현합니다.

```
infrastructure/
└── database/
    └── sqlite_repository.rs  # SqliteAddressRepository 구현
```

- **구현체**: AddressRepository trait의 SQLite 구현
- SQLx를 사용한 실제 데이터베이스 작업

#### 4. Presentation Layer (표현 계층)
애플리케이션 계층을 사용하여 사용자와 상호작용합니다.

**Backend (presentation_backend)**
```
presentation_backend/
├── models/
│   └── dto.rs              # 데이터 전송 객체
└── commands/
    └── address_commands.rs # Tauri 명령어 핸들러
```

**Frontend (presentation_frontend)**
```
presentation_frontend/
└── src/
    ├── app.rs              # 메인 앱 컴포넌트
    ├── components/         # UI 컴포넌트
    └── api/                # Tauri API 호출
```

### Clean Architecture의 이점

1. **독립성**: 각 계층이 독립적으로 테스트 가능
2. **유연성**: 데이터베이스나 UI 프레임워크 교체 용이
3. **유지보수성**: 비즈니스 로직이 외부 기술에 종속되지 않음
4. **확장성**: 새로운 기능 추가 시 기존 코드 영향 최소화

## 권장 IDE 설정

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
