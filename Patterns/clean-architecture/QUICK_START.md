# Quick Start Guide

## 빌드 및 실행

```bash
# 전체 워크스페이스 빌드
cargo build --workspace

# Web 애플리케이션 실행
cargo run -p web

# 브라우저에서 접속
# http://localhost:3000
```

## 개발 명령어

```bash
# 특정 크레이트만 빌드
cargo build -p domain
cargo build -p application
cargo build -p infrastructure
cargo build -p presentation

# 전체 테스트 실행
cargo test --workspace

# 특정 크레이트 테스트
cargo test -p domain

# 코드 검사 (컴파일 확인)
cargo check --workspace

# 포맷팅
cargo fmt --all

# Clippy 린트
cargo clippy --workspace
```

## 워크스페이스 구조

```
clean-architecture/
├── Cargo.toml              # 워크스페이스 루트 설정
├── crates/                 # 라이브러리 크레이트
│   ├── domain/             # 핵심 비즈니스 로직 (의존성 없음)
│   ├── application/        # 유스케이스 (domain에만 의존)
│   ├── infrastructure/     # 외부 시스템 연동 (domain, application에 의존)
│   └── presentation/       # UI/API (모든 계층 사용 가능)
└── apps/                   # 실행 가능한 애플리케이션
    └── web/                # Web 서버 애플리케이션
```

## 의존성 방향

```
apps/web
    ↓
presentation
    ↓
infrastructure
    ↓
application
    ↓
domain (핵심, 의존성 없음)
```

## 새 크레이트 추가하기

1. `crates/` 또는 `apps/` 아래에 새 디렉터리 생성
2. `Cargo.toml` 생성:
   ```toml
   [package]
   name = "my-crate"
   edition.workspace = true
   version.workspace = true
   
   [dependencies]
   # 필요한 의존성 추가
   ```
3. 루트 `Cargo.toml`의 `members`에 추가:
   ```toml
   [workspace]
   members = [
       # ... 기존 멤버들
       "crates/my-crate",
   ]
   ```

## 트러블슈팅

### SQLite 링크 오류 (Windows)
```toml
# Cargo.toml에서 bundled 기능 사용
rusqlite = {version = "0.29", features = ["bundled"]}
```

### 빌드 캐시 문제
```bash
cargo clean
cargo build --workspace
```

### 의존성 업데이트
```bash
cargo update
```
