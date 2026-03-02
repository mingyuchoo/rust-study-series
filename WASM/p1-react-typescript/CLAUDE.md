# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust WASM 라이브러리를 React + TypeScript 프론트엔드에서 호출하는 예제 프로젝트. 두 개의 독립적인 서브 프로젝트로 구성된다.

## Architecture

```
p1-react-typescript/
├── wasm-lib/          # Rust → WASM 크레이트 (cdylib)
│   └── pkg/           # wasm-pack 빌드 결과물 (git-ignored)
└── react-app/         # React 19 + Vite 프론트엔드
    └── wasm-lib dep   # file:../wasm-lib/pkg 로컬 의존성
```

**데이터 흐름:** `wasm-lib/src/lib.rs`에서 `#[wasm_bindgen]`으로 export한 함수를 `react-app`에서 `import init, { fn } from "wasm-lib"` 패턴으로 호출. WASM 초기화는 `init()` 호출 후 사용해야 한다.

## Build & Run Commands

### wasm-lib (Rust)

```bash
cd wasm-lib

# 빌드 전제: wasm-pack, wasm32-unknown-unknown target 필요
# 최초 설정
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# WASM 패키지 빌드 (react-app에서 참조하는 pkg/ 생성)
wasm-pack build --target web --out-dir pkg

# 또는 build.sh 스크립트 사용
./build.sh

# Rust 단위 테스트
cargo test

# cargo-make 태스크 (check → clippy → fmt 체인 포함)
cargo make build     # dev 프로파일 빌드
cargo make test      # format 후 테스트
cargo make format    # check + clippy + fmt
cargo make clippy
```

### react-app (React + Vite)

```bash
cd react-app

# 의존성 설치 (Bun 사용)
bun install

# 개발 서버
bun run dev

# 프로덕션 빌드
bun run build

# 테스트
bun run test         # watch 모드
bun run test:run     # 단일 실행
```

**빌드 순서:** 반드시 `wasm-lib`를 먼저 빌드(`wasm-pack build`)하여 `pkg/`를 생성한 후 `react-app`의 의존성 설치 및 빌드를 진행해야 한다.

## Key Conventions

### Rust (wasm-lib)
- **Nightly 툴체인** 사용 (`rust-toolchain.toml`)
- **Edition 2024**
- `rustfmt.toml`: `max_width = 160`, `fn_single_line = true`, `imports_granularity = "Crate"`, `group_imports = "One"`
- `taplo`로 TOML 포맷팅 (align entries/comments 활성화)
- `cargo-make`로 태스크 관리 (`Makefile.toml`) — format 태스크는 check + clippy를 선행 의존성으로 가짐

### React (react-app)
- **Bun** 패키지 매니저/런타임
- **Vite** 빌드 도구 + `vite-plugin-wasm`, `vite-plugin-top-level-await` 플러그인
- **Vitest** 테스트 (globals 활성화, jsdom 환경, `@testing-library/react`)
- WASM 모듈 테스트 시 `vi.mock("wasm-lib", ...)` 패턴으로 모킹
- TypeScript strict 모드, `noUnusedLocals`, `noUnusedParameters` 활성화
