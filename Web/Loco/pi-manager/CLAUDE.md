# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

```bash
# 개발 서버 시작 (http://localhost:5150)
cargo loco start

# 테스트 실행
cargo test

# 단일 테스트 실행
cargo test <test_name>

# 마이그레이션 생성
cargo loco generate migration <name>

# 모델 생성
cargo loco generate model <name> <field:type>...

# 컨트롤러 생성
cargo loco generate controller <name>

# 빌드
cargo build
```

## Architecture Overview

**Loco 프레임워크** 기반 SaaS 애플리케이션으로, 성과지표(Performance Indicators) 관리 시스템입니다.

### 핵심 도메인 모델

```
PerformanceIndicator (성과지표)
├── InputIndices (투입지표) - has_many
├── ProcessIndices (과정지표) - has_many
├── OutputIndices (산출지표) - has_many
└── OutcomeIndices (성과지표) - has_many
```

각 하위 지표는 `performance_indicator_id`로 상위 성과지표와 연결됩니다.

### 디렉토리 구조

- `src/app.rs` - 애플리케이션 진입점, 라우트 등록, 훅(Hooks) 정의
- `src/controllers/` - HTTP 핸들러 (REST API + 페이지 렌더링)
- `src/models/` - SeaORM 엔티티 및 비즈니스 로직
  - `_entities/` - SeaORM 자동 생성 엔티티 (수정 금지)
  - 루트 레벨 파일 - 커스텀 쿼리 및 비즈니스 로직
- `src/views/` - JSON 응답 포맷터
- `src/workers/` - 백그라운드 작업 (DownloadWorker)
- `src/initializers/` - 앱 초기화 (ViewEngine 설정)
- `migration/` - SeaORM 마이그레이션

### API 엔드포인트

- `/api/performance-indicators` - 성과지표 CRUD
- `/api/input-indices/by-indicator/{pi_id}` - 투입지표
- `/api/process-indices/by-indicator/{pi_id}` - 과정지표
- `/api/output-indices/by-indicator/{pi_id}` - 산출지표
- `/api/outcome-indices/by-indicator/{pi_id}` - 성과지표
- `/api/auth/*` - JWT 인증

### 주요 기술 스택

- **ORM**: SeaORM (SQLite/PostgreSQL 지원)
- **인증**: JWT 기반
- **템플릿**: Tera + fluent-templates (i18n)
- **테스트**: rstest, insta (스냅샷), serial_test

### 설정 파일

- `config/development.yaml` - 개발 환경 (기본 SQLite)
- `config/test.yaml` - 테스트 환경
- `config/production.yaml` - 프로덕션 환경

데이터베이스 URI는 `DATABASE_URL` 환경변수로 오버라이드 가능.
