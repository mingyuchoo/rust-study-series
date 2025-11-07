# Foundry Local Workspace

Cargo 워크스페이스 구조로 구성된 Foundry Local SDK 사용 예제입니다.

## 프로젝트 구조

```
plugin-architecture/
├─ Cargo.toml              # 워크스페이스 루트 (virtual manifest)
├─ Cargo.lock
├─ crates/
│  ├─ foundry-types/       # 공통 타입 정의 (재사용 가능)
│  └─ foundry-client/      # API 클라이언트 라이브러리
└─ apps/
   └─ cli/                 # CLI 애플리케이션 예제
```

## 주요 개선사항

### 1. 모듈화된 구조
- **foundry-types**: 공통 타입 정의 (재사용 가능)
- **foundry-client**: API 클라이언트 라이브러리
- **cli**: 사용 예제 애플리케이션

### 2. 개선된 에러 처리
- `thiserror`를 사용한 구조화된 에러 타입
- 명확한 에러 메시지와 컨텍스트

### 3. 타입 안전성
- `FoundryModel` 열거형으로 타입 안전성 확보
- 빌더 패턴을 통한 유연한 객체 생성
- 편의 메서드 제공 (`ChatMessage::user()` 등)

### 4. 구조화된 로깅
- `tracing` 크레이트 사용
- 환경 변수를 통한 로그 레벨 제어

### 5. 재사용성
- 라이브러리로 분리하여 다른 프로젝트에서도 사용 가능
- 워크스페이스 의존성 공유

## 빌드 및 실행

```bash
# 전체 워크스페이스 빌드
cargo build --workspace

# CLI 애플리케이션 실행
cargo run -p foundry-cli

# 또는 Makefile.toml 사용
cargo make run
```

## 개발

```bash
# 포맷팅 및 린팅
cargo make format

# 테스트
cargo make test

# 릴리스 빌드
cargo make release
```

## 로그 레벨 설정

```bash
# 디버그 로그 활성화
RUST_LOG=debug cargo run -p foundry-cli

# 특정 모듈만 디버그
RUST_LOG=foundry_client=debug cargo run -p foundry-cli
```
