# wasm-game-of-life

Conway's Game of Life를 Rust + WebAssembly로 구현한 프로젝트입니다.

## 소개

Rust로 작성한 Game of Life 시뮬레이션을 WebAssembly로 컴파일하고, Canvas 기반 웹 프론트엔드에서 실시간으로 렌더링합니다.

- **패키지명**: `wasm-game-of-life`
- **버전**: `0.1.0`
- **Rust 에디션**: 2024

## 주요 기능

- **Conway's Game of Life 시뮬레이션**: 64×64 격자에서 셀 생존/탄생/사망 규칙 적용
- **Canvas 렌더링**: WASM 선형 메모리를 직접 참조하는 제로카피 렌더링
- **인터랙티브 조작**: 셀 클릭 토글, Play/Pause, 속도 조절 (1~30 fps)
- **토러스 토폴로지**: 격자 경계가 반대편과 연결

## 프로젝트 구조

```text
src/
├── lib.rs              # Game of Life 핵심 로직 (Cell, Universe)
└── utils.rs            # 유틸리티 함수 (panic hook 설정)
tests/
└── web.rs              # WASM 환경 테스트 (빈 우주, Blinker 패턴 검증)
www/
├── index.html          # 웹 페이지 (Canvas + 컨트롤 UI)
├── index.js            # Canvas 렌더링, 애니메이션 루프, 클릭 상호작용
├── bootstrap.js        # 비동기 WASM 로딩 래퍼
├── package.json        # NPM 설정 (Webpack 기반)
└── webpack.config.js   # Webpack 번들러 설정 (asyncWebAssembly)
```

## 핵심 구조

### Cell (`src/lib.rs`)

- `Dead = 0`, `Alive = 1`로 표현되는 `#[repr(u8)]` 열거형
- JS에서 `Uint8Array`로 직접 접근 가능
- `toggle()` 메서드로 상태 전환

### Universe (`src/lib.rs`)

- `width`, `height`, `cells: Vec<Cell>`로 구성
- `Default` trait 구현 (`new()`와 동일)

| 메서드 | 설명 |
|--------|------|
| `new()` | 64×64 랜덤 패턴으로 우주 생성 (`js_sys::Math::random()` 사용) |
| `tick()` | Conway 규칙에 따라 다음 세대 계산 (더블 버퍼링) |
| `width()` / `height()` | 우주 크기 getter |
| `cells()` | WASM 선형 메모리 포인터 반환 (`*const Cell`) |
| `toggle_cell(row, col)` | 셀 상태 토글 |
| `set_width(w)` / `set_height(h)` | 크기 변경 (모든 셀 Dead로 초기화) |
| `render()` | `Display` trait을 통한 텍스트 렌더링 (`◻`/`◼`) |
| `get_cells()` | 셀 슬라이스 반환 (테스트용, wasm_bindgen 미적용) |
| `set_cells(&[(row, col)])` | 지정 좌표를 Alive로 설정 (테스트용) |

### Conway 규칙

1. 살아있는 셀의 이웃이 2개 미만이면 죽음 (과소)
2. 살아있는 셀의 이웃이 2~3개이면 생존
3. 살아있는 셀의 이웃이 3개 초과이면 죽음 (과밀)
4. 죽은 셀의 이웃이 정확히 3개이면 탄생

## 사용 방법

### 사전 요구 사항

```bash
rustup default stable
rustup update stable
cargo install cargo-make
cargo install wasm-pack
```

Bun이 설치되어 있어야 합니다. (`curl -fsSL https://bun.sh/install | bash`)

### cargo-make 태스크

| 태스크 | 설명 |
|--------|------|
| `cargo make check` | 코드 검사 |
| `cargo make clippy` | Lint 검사 |
| `cargo make format` | 코드 포맷팅 (`check` + `clippy` 선행) |
| `cargo make build` | 개발용 빌드 |
| `cargo make release` | 릴리스 빌드 |
| `cargo make test` | Rust 단위 테스트 실행 |
| `cargo make wasm-build` | WASM 패키지 빌드 (`pkg/` 생성) |
| `cargo make wasm-build-release` | WASM 릴리스 빌드 |
| `cargo make www-install` | 웹 프론트엔드 의존성 설치 (`wasm-build` 선행) |
| `cargo make www-serve` | 개발 서버 실행 (`www-install` 선행) |
| `cargo make www-build` | 웹 프론트엔드 프로덕션 빌드 |
| `cargo make run` | `www-serve`의 별칭 |
| `cargo make clean` | 빌드 산출물 정리 |

### 빠른 시작

```bash
# 한 번에 WASM 빌드 → npm install → 개발 서버 실행
cargo make run
```

### 수동 실행

```bash
# 1. WASM 빌드
wasm-pack build

# 2. 웹 프론트엔드 설정
cd www
bun install

# 3. 개발 서버 실행
bun run serve
```

브라우저에서 `http://localhost:8080`으로 접속하면 Game of Life 시뮬레이션을 확인할 수 있습니다.

### 조작 방법

- **Play/Pause 버튼**: 시뮬레이션 실행/정지
- **속도 슬라이더**: 1~30 fps 사이 속도 조절
- **셀 클릭**: 개별 셀 상태 토글 (Dead ↔ Alive)

## 의존성

### Rust (Cargo.toml)

| 패키지 | 버전 | 설명 |
|--------|------|------|
| [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) | 0.2.95 | WebAssembly ↔ JavaScript 간 통신 |
| [`js-sys`](https://docs.rs/js-sys) | 0.3 | JavaScript 전역 API 바인딩 (`Math.random()`) |
| [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) | 0.1.7 | 패닉 메시지를 콘솔에 로깅 (기본 활성화) |
| [`wee_alloc`](https://github.com/rustwasm/wee_alloc) | 0.4.5 | 경량 메모리 할당자 (선택적) |
| [`wasm-bindgen-test`](https://github.com/rustwasm/wasm-bindgen) | 0.3.45 | WASM 테스트 프레임워크 (dev) |

### 웹 프론트엔드 (www/package.json)

| 패키지 | 버전 | 설명 |
|--------|------|------|
| `webpack` | ^5.97.0 | 모듈 번들러 |
| `webpack-cli` | ^6.0.0 | Webpack CLI |
| `webpack-dev-server` | ^5.2.0 | 개발 서버 |
| `copy-webpack-plugin` | ^12.0.0 | 정적 파일 복사 |

## 참고 자료

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-pack 튜토리얼](https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html)
- [Conway's Game of Life (Wikipedia)](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
