# 주소록 앱 - Clean Architecture with Tauri + Leptos

SQLite를 사용한 주소 데이터 CRUD 기능을 제공하는 데스크톱 애플리케이션입니다.
Clean Architecture 패턴을 적용하여 각 계층을 별도의 Rust 크레이트로 분리했습니다.

## 아키텍처

```
project-root/
├── domain/             # 도메인 계층 (엔티티, 리포지토리 인터페이스)
├── application/        # 애플리케이션 계층 (유스케이스)
├── infrastructure/     # 인프라 계층 (SQLite 구현체)
├── presentation_backend/   # 백엔드 표현 계층 (Tauri 명령어)
└── presentation_frontend/  # 프론트엔드 표현 계층 (Leptos UI)
```

## 기능

- 주소 추가, 조회, 수정, 삭제 (CRUD)
- SQLite 데이터베이스 저장
- 반응형 웹 UI (Leptos)
- 데스크톱 애플리케이션 (Tauri)

## 개발 환경 설정

### 필수 요구사항

- Rust (최신 stable 버전)
- Node.js (Tauri 개발용)
- Trunk (Leptos 빌드용): `cargo install trunk`

### 실행 방법

1. 의존성 설치:
```bash
cargo build
```

2. 개발 서버 실행:
```bash
cargo tauri dev
```

3. 프로덕션 빌드:
```bash
cargo tauri build
```

## 권장 IDE 설정

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
