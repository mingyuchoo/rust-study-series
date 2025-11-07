# Makefile.toml 사용 가이드

이 프로젝트는 `cargo-make`를 사용하여 워크스페이스 작업을 자동화합니다.

## 설치

```bash
cargo install cargo-make
```

## 주요 명령어

### 개발 워크플로우

```bash
# 개발 환경 준비 (포맷 + 체크 + 린트 + 빌드)
cargo make dev

# CI 파이프라인 실행 (포맷 + 린트 + 테스트 + 빌드)
cargo make ci
```

### 빌드

```bash
# 전체 워크스페이스 빌드
cargo make build

# 릴리즈 빌드
cargo make release

# 개별 크레이트 빌드
cargo make build-domain
cargo make build-application
cargo make build-infrastructure
cargo make build-presentation
cargo make build-web
```

### 실행

```bash
# Web 애플리케이션 실행
cargo make run

# Watch 모드 (파일 변경 시 자동 재시작)
# 사전 요구사항: cargo install cargo-watch
cargo make watch-run
```

### 테스트

```bash
# 전체 워크스페이스 테스트
cargo make test

# 개별 크레이트 테스트
cargo make test-domain
cargo make test-application
cargo make test-infrastructure
cargo make test-presentation
```

### 코드 품질

```bash
# 코드 검사 (컴파일 확인)
cargo make check

# 코드 포맷팅
cargo make format

# Clippy 린트
cargo make clippy
```

### 유틸리티

```bash
# 의존성 업데이트
cargo make update

# 빌드 아티팩트 정리
cargo make clean
```

## 사용 가능한 모든 태스크 보기

```bash
# 모든 태스크 목록 출력
cargo make --list-all-steps

# 특정 카테고리의 태스크만 보기
cargo make --list-category-steps "Development"
```

## 태스크 설명

### 워크스페이스 전체 작업

| 태스크 | 설명 |
|--------|------|
| `clean` | 빌드 아티팩트 정리 |
| `check` | 전체 워크스페이스 컴파일 체크 |
| `clippy` | 전체 워크스페이스 린트 |
| `format` | 전체 코드 포맷팅 |
| `build` | 전체 워크스페이스 빌드 |
| `release` | 전체 워크스페이스 릴리즈 빌드 |
| `test` | 전체 워크스페이스 테스트 |
| `update` | 의존성 업데이트 |

### Web 애플리케이션

| 태스크 | 설명 |
|--------|------|
| `run` | Web 애플리케이션 실행 |
| `watch-run` | Watch 모드로 실행 (cargo-watch 필요) |

### 개별 크레이트 빌드

| 태스크 | 설명 |
|--------|------|
| `build-domain` | domain 크레이트만 빌드 |
| `build-application` | application 크레이트만 빌드 |
| `build-infrastructure` | infrastructure 크레이트만 빌드 |
| `build-presentation` | presentation 크레이트만 빌드 |
| `build-web` | web 애플리케이션만 빌드 |

### 개별 크레이트 테스트

| 태스크 | 설명 |
|--------|------|
| `test-domain` | domain 크레이트만 테스트 |
| `test-application` | application 크레이트만 테스트 |
| `test-infrastructure` | infrastructure 크레이트만 테스트 |
| `test-presentation` | presentation 크레이트만 테스트 |

### 유틸리티 작업

| 태스크 | 설명 |
|--------|------|
| `dev` | 개발 환경 준비 (format + check + clippy + build) |
| `ci` | CI 파이프라인 (format + clippy + test + build) |

## 워크플로우 예시

### 일반 개발 워크플로우

```bash
# 1. 코드 작성
# 2. 개발 환경 체크
cargo make dev

# 3. 애플리케이션 실행 및 테스트
cargo make run

# 4. 변경사항 커밋 전 최종 확인
cargo make ci
```

### Watch 모드 개발

```bash
# cargo-watch 설치 (최초 1회)
cargo install cargo-watch

# Watch 모드로 실행
cargo make watch-run

# 이제 파일을 수정하면 자동으로 재컴파일 및 재시작됩니다
```

### 특정 계층만 작업

```bash
# Domain 계층만 빌드 및 테스트
cargo make build-domain
cargo make test-domain

# Application 계층만 빌드 및 테스트
cargo make build-application
cargo make test-application
```

## 커스터마이징

`Makefile.toml`을 수정하여 프로젝트에 맞는 태스크를 추가할 수 있습니다:

```toml
[tasks.my-custom-task]
  description = "내 커스텀 태스크"
  command = "cargo"
  args = ["build", "-p", "domain", "--features", "my-feature"]
```

## 참고 자료

- [cargo-make 공식 문서](https://github.com/sagiegurari/cargo-make)
- [cargo-watch 공식 문서](https://github.com/watchexec/cargo-watch)
