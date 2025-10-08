# 주소록 앱 (Address Book App)

SQLite를 사용하는 Clean Architecture 기반의 크로스 플랫폼 주소록 데스크톱 애플리케이션입니다. Tauri와 Dioxus를 사용하여 프론트엔드와 백엔드 모두 Rust로 개발되었습니다.

## Project Structure

```
tauri-dioxus-app/
├── src/                    # Dioxus frontend source
│   ├── main.rs            # Frontend entry point
│   ├── app.rs             # Main app component
│   ├── types.rs           # Type definitions
│   ├── components/        # UI components
│   │   ├── mod.rs
│   │   ├── contact_form.rs    # Contact form component
│   │   └── contact_list.rs    # Contact list component
│   └── services/          # Backend communication services
│       ├── mod.rs
│       └── contact_service.rs # Contact service for Tauri commands
├── src-tauri/             # Tauri backend source (Clean Architecture)
│   ├── src/
│   │   ├── main.rs        # Backend entry point
│   │   ├── lib.rs         # Tauri setup and configuration
│   │   ├── domain/        # Domain layer
│   │   │   ├── entities/  # Domain entities
│   │   │   ├── repositories/ # Repository interfaces
│   │   │   └── errors.rs  # Domain errors
│   │   ├── application/   # Application layer
│   │   │   └── use_cases/ # Business logic use cases
│   │   ├── infrastructure/ # Infrastructure layer
│   │   │   └── database/  # SQLite repository implementations
│   │   └── presentation/  # Presentation layer
│   │       ├── dto/       # Data transfer objects
│   │       └── commands/  # Tauri command handlers
│   ├── Cargo.toml         # Backend dependencies
│   └── tauri.conf.json    # Tauri configuration
├── assets/                # Static assets (CSS, images)
├── Cargo.toml            # Frontend dependencies
├── Dioxus.toml           # Dioxus configuration
└── Makefile.toml         # Build tasks
```

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started): `cargo install dioxus-cli`
- [Tauri CLI](https://tauri.app/start/prerequisites/): `cargo install tauri-cli`

## Development

### Quick Start

```shell
# Clone and navigate to project
cd tauri-dioxus-app

# Install dependencies (handled by cargo)
# Run development server
cargo tauri dev
```

### Available Commands

Using cargo-make (install with `cargo install cargo-make`):

```shell
# Development
cargo make run              # Start development server
cargo make watch-run        # Start with file watching

# Code Quality
cargo make check            # Check code
cargo make clippy           # Run linter
cargo make format           # Format code
cargo make test             # Run tests

# Build
cargo make build            # Development build
cargo make release          # Production build
cargo make clean            # Clean build artifacts
```

### Direct Commands

```shell
# Development
cargo tauri dev             # Start development server
dx serve --port 1420        # Start Dioxus dev server only

# Build
cargo tauri build           # Build for production
dx bundle --release         # Build Dioxus bundle only
```

## 기능 (Features)

- **연락처 관리**: 이름, 이메일, 전화번호, 주소 정보 저장
- **검색 기능**: 모든 필드에서 연락처 검색 가능
- **CRUD 작업**: 연락처 생성, 조회, 수정, 삭제
- **SQLite 데이터베이스**: 로컬 데이터 저장
- **Clean Architecture**: 도메인, 애플리케이션, 인프라, 프레젠테이션 계층 분리
- **크로스 플랫폼**: Windows, macOS, Linux에서 실행
- **Rust 풀스택**: 프론트엔드(Dioxus)와 백엔드(Tauri) 모두 Rust 사용
- **타입 안전성**: 전체 스택에서 Rust 타입 안전성 보장

## Configuration

- **Dioxus Config**: `Dioxus.toml` - Frontend build settings
- **Tauri Config**: `src-tauri/tauri.conf.json` - App metadata and build settings
- **Build Tasks**: `Makefile.toml` - Development workflow automation

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) with extensions:
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Dioxus](https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus)

## Learn More

- [Tauri Documentation](https://tauri.app/)
- [Dioxus Documentation](https://dioxuslabs.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 아키텍처 (Architecture)

이 애플리케이션은 Clean Architecture 패턴을 따릅니다:

### 백엔드 구조 (Backend Structure)
```
src-tauri/src/
├── domain/                 # 도메인 계층
│   ├── entities/          # 도메인 엔티티
│   │   └── contact.rs     # Contact 엔티티
│   ├── repositories/      # 리포지토리 인터페이스
│   │   └── contact_repository.rs # Contact 리포지토리 트레이트
│   └── errors.rs          # 도메인 에러 정의
├── application/           # 애플리케이션 계층
│   └── use_cases/         # 비즈니스 로직 유스케이스
│       ├── create_contact.rs    # 연락처 생성
│       ├── get_contact.rs       # 연락처 조회
│       ├── list_contacts.rs     # 연락처 목록
│       ├── update_contact.rs    # 연락처 수정
│       ├── delete_contact.rs    # 연락처 삭제
│       └── search_contacts.rs   # 연락처 검색
├── infrastructure/        # 인프라 계층
│   └── database/          # 데이터베이스 구현
│       └── sqlite_contact_repository.rs # SQLite 리포지토리 구현
└── presentation/          # 프레젠테이션 계층
    ├── dto/               # 데이터 전송 객체
    │   └── contact_dto.rs # Contact DTO 정의
    └── commands/          # Tauri 명령어 핸들러
        └── contact_commands.rs # Contact 관련 명령어
```

### 프론트엔드 구조 (Frontend Structure)
```
src/
├── main.rs                # 애플리케이션 진입점
├── app.rs                 # 메인 앱 컴포넌트 (상태 관리 및 라우팅)
├── types.rs               # 타입 정의 (Contact, Request/Response 타입)
├── components/            # UI 컴포넌트
│   ├── mod.rs            # 컴포넌트 모듈 정의
│   ├── contact_form.rs    # 연락처 추가/수정 폼
│   └── contact_list.rs    # 연락처 목록 및 카드 컴포넌트
└── services/              # 백엔드 통신 서비스
    ├── mod.rs            # 서비스 모듈 정의
    └── contact_service.rs # Tauri 명령어 호출 서비스
```

## 사용법 (Usage)

### 연락처 추가
1. "새 연락처" 버튼 클릭
2. 이름 (필수), 이메일, 전화번호, 주소 입력
3. "추가" 버튼 클릭

### 연락처 검색
1. 상단 검색창에 검색어 입력
2. "검색" 버튼 클릭 또는 Enter 키 입력
3. 이름, 이메일, 전화번호, 주소에서 검색됩니다

### 연락처 수정
1. 연락처 카드에서 "수정" 버튼 클릭
2. 정보 수정 후 "수정" 버튼 클릭

### 연락처 삭제
1. 연락처 카드에서 "삭제" 버튼 클릭

## 데이터베이스 (Database)

SQLite 데이터베이스가 애플리케이션 실행 시 자동으로 생성됩니다:
- 파일명: `contacts.db`
- 테이블: `contacts`
- 필드: id (UUID), name, email, phone, address, created_at, updated_at

### 데이터 타입
- **Frontend**: Contact 타입은 `String` ID와 `Option<String>` 필드 사용
- **Backend**: Domain 엔티티는 `Uuid` 타입과 `chrono::DateTime<Utc>` 사용
- **변환**: DTO 계층에서 프론트엔드와 백엔드 간 타입 변환 처리

## 주요 기능 구현

### Clean Architecture 패턴
- **도메인 계층**: 비즈니스 규칙과 엔티티 정의
- **애플리케이션 계층**: 유스케이스별 비즈니스 로직 구현
- **인프라 계층**: SQLite 데이터베이스 연동
- **프레젠테이션 계층**: Tauri 명령어와 DTO 변환

### 프론트엔드 아키텍처
- **컴포넌트 기반**: 재사용 가능한 UI 컴포넌트
- **서비스 계층**: 백엔드 API 호출 추상화
- **상태 관리**: Dioxus signals를 통한 반응형 상태 관리
- **타입 안전성**: Rust 타입 시스템을 통한 컴파일 타임 검증