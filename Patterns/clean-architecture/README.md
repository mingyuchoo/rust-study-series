# Clean Architecture Example (Cargo Workspace)

이 프로젝트는 Rust에서 Clean Architecture(Onion Architecture)를 Cargo 워크스페이스(모노레포) 구조로 구현한 예제입니다.

## 프로젝트 구조

```
clean-architecture/
├─ Cargo.toml              # workspace 루트 (virtual manifest)
├─ Cargo.lock
├─ crates/                 # 라이브러리 크레이트
│  ├─ domain/              # 가장 내부 계층 (핵심 비즈니스 로직)
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ models.rs      # 도메인 엔티티
│  │     ├─ repositories.rs # 저장소 인터페이스
│  │     └─ services.rs    # 도메인 서비스
│  ├─ application/         # 애플리케이션 서비스 계층
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     └─ services.rs    # 유스케이스 구현
│  ├─ infrastructure/      # 가장 외부 계층 (외부 의존성)
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ repositories.rs # 저장소 구현 (SQLite)
│  │     └─ controllers.rs  # DB 컨트롤러
│  └─ presentation/        # 프레젠테이션 계층
│     ├─ Cargo.toml
│     └─ src/
│        ├─ web.rs         # Web 서버 구현
│        └─ static/
│           └─ index.html  # UI
└─ apps/                   # 실행 가능한 애플리케이션
   └─ web/                 # Web 애플리케이션
      ├─ Cargo.toml
      └─ src/
         └─ main.rs        # 애플리케이션 진입점
```

## 계층 구조

1. **Domain (domain)**: 핵심 비즈니스 로직과 엔티티
   - 외부 의존성 없음
   - 순수한 비즈니스 규칙만 포함

2. **Application (application)**: 유스케이스 구현
   - Domain에만 의존
   - 비즈니스 로직 조율

3. **Infrastructure (infrastructure)**: 외부 시스템 연동
   - Domain과 Application에 의존
   - 데이터베이스, 외부 API 등 구현

4. **Presentation (presentation)**: 사용자 인터페이스
   - 모든 계층에 의존 가능
   - Web UI, API 엔드포인트 등

5. **Apps (apps/web)**: 실행 가능한 애플리케이션
   - Presentation 계층을 사용하여 서버 실행

## 빌드 및 실행

### 빠른 시작 (cargo-make 사용 권장)

```bash
# cargo-make 설치 (최초 1회)
cargo install cargo-make

# 개발 환경 준비 (포맷 + 체크 + 린트 + 빌드)
cargo make dev

# Web 애플리케이션 실행
cargo make run

# 브라우저에서 접속
# http://localhost:3000
```

### Cargo 직접 사용

```bash
# 전체 워크스페이스 빌드
cargo build --workspace

# Web 애플리케이션 실행
cargo run -p web

# 특정 크레이트만 빌드
cargo build -p domain
cargo build -p application
cargo build -p infrastructure
cargo build -p presentation

# 테스트 실행
cargo test --workspace
```

## 문서

- **[QUICK_START.md](QUICK_START.md)** - 빠른 시작 가이드 및 명령어 레퍼런스
- **[MAKEFILE_GUIDE.md](MAKEFILE_GUIDE.md)** - cargo-make 사용 가이드 및 워크플로우
- **[MIGRATION.md](MIGRATION.md)** - 모놀리식에서 워크스페이스로의 마이그레이션 가이드
- **[.cleanup-old-structure.md](.cleanup-old-structure.md)** - 기존 구조 정리 방법

## 의존성 관리

워크스페이스 루트의 `Cargo.toml`에서 공통 의존성을 관리합니다:
- `workspace.dependencies`: 모든 크레이트에서 공유하는 의존성
- `workspace.package`: 공통 메타데이터 (edition, version 등)

각 크레이트는 필요한 의존성만 선택적으로 사용합니다.

## 장점

1. **명확한 의존성 방향**: 내부 계층은 외부 계층을 알지 못함
2. **테스트 용이성**: 각 계층을 독립적으로 테스트 가능
3. **재사용성**: 크레이트 단위로 다른 프로젝트에서 재사용 가능
4. **확장성**: 새로운 프레젠테이션 계층(CLI, gRPC 등) 추가 용이
5. **모듈화**: 각 크레이트가 독립적으로 컴파일되어 빌드 시간 최적화
