# wasm-diary

Rust WASM + React로 구현한 다이어리 웹 애플리케이션입니다.

## 주요 기능

- RBAC(역할 기반 접근 제어) 사용자 관리 (관리자 / 일반 사용자)
- 회원가입 및 로그인 (SHA-256 비밀번호 해싱)
- 사용자 ID: 영문/숫자/언더스코어 전용, 닉네임 선택 지원
- 일기 CRUD (생성, 조회, 수정, 삭제)
- 감정(8종) 및 날씨(6종) 기록
- 키워드 검색, 감정/날씨/날짜 범위 필터링
- 통계 대시보드 (총 일기 수, 글자 수, 어절 수, 감정/날씨 분포)
- 일반 사용자: 자신의 일기만 관리
- 관리자: 모든 일기 조회/삭제, 사용자 관리

## 프로젝트 구조

```
wasm-diary/
├── wasm-lib/       # Rust WASM 라이브러리 (비즈니스 로직)
│   └── src/
│       ├── model.rs         # DiaryEntry, Mood, Weather
│       ├── user_model.rs    # Role, UserAccount
│       ├── manager.rs       # DiaryManager (일기 CRUD, 검색, 필터)
│       ├── user_manager.rs  # UserManager (회원가입, 로그인, 권한)
│       ├── hash_util.rs     # SHA-256 비밀번호 해싱
│       ├── stats.rs         # 통계 계산
│       ├── validation.rs    # 입력 유효성 검사
│       ├── date_util.rs     # 날짜 유틸리티
│       └── id_gen.rs        # ID 생성
├── react-app/      # React + TypeScript 프론트엔드
│   └── src/
│       ├── hooks/           # useWasm, useAuth, useDiary
│       ├── components/      # AuthForms, DiaryEntryForm, 등
│       ├── types/           # TypeScript 타입 정의
│       └── constants/       # 감정, 날씨 매핑
├── run.sh          # 빌드 스크립트 (Linux/macOS)
└── run.ps1         # 빌드 스크립트 (Windows PowerShell)
```

## 사전 준비

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` 타겟
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) (`cargo install wasm-pack`)
- [Bun](https://bun.sh/)

## 실행 방법

### Linux / macOS

```bash
./run.sh dev     # 개발 서버 실행
./run.sh build   # 프로덕션 빌드
./run.sh clean   # 빌드 산출물 정리
```

### Windows (PowerShell)

```powershell
.\run.ps1 dev     # 개발 서버 실행
.\run.ps1 build   # 프로덕션 빌드
.\run.ps1 clean   # 빌드 산출물 정리
```

## 기본 계정

| 역할 | 사용자 ID | 비밀번호 |
|------|----------|---------|
| 관리자 | admin | admin123 |

## 권한 매트릭스

| 기능 | 일반 사용자 | 관리자 |
|------|-----------|-------|
| 자기 일기 CRUD | O | O |
| 모든 일기 조회 | X | O |
| 사용자 관리 | X | O |
| 통계 조회 | 자기 것만 | 전체 |

## 기술 스택

- **WASM:** Rust + wasm-pack + sha2
- **프론트엔드:** React 19 + TypeScript + Vite
- **런타임/패키지 매니저:** Bun
- **테스트:** Vitest + Testing Library
