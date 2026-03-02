# wasm-diary

Rust WASM + React로 구현한 다이어리 웹 애플리케이션입니다.

## 프로젝트 구조

```
wasm-diary/
├── wasm-lib/       # Rust WASM 라이브러리 (비즈니스 로직)
├── react-app/      # React + TypeScript 프론트엔드
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

## 명령어 설명

| 명령어 | 설명 |
|--------|------|
| `dev`  | WASM 빌드(dev) + 의존성 설치 + Vite 개발 서버 실행 |
| `build`| WASM 빌드(release) + React 프로덕션 빌드 |
| `clean`| 모든 빌드 산출물 제거 (pkg, target, node_modules, dist) |

## 기술 스택

- **WASM:** Rust + wasm-pack
- **프론트엔드:** React 19 + TypeScript + Vite
- **런타임/패키지 매니저:** Bun
- **테스트:** Vitest + Testing Library
