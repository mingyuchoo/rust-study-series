# ai-agent-eval-harness

Rust로 구현된 AI 에이전트 평가(Evaluation) 하네스입니다. 다양한 도메인 시나리오에서 AI 에이전트의 도구 호출 정확도, 성능, 회귀 여부를 자동으로 평가합니다.

## 특징

- **3종 프론트엔드**: CLI 서브커맨드, 대화형 TUI, 내장 HTTP 서버 + SPA
- **다중 도메인 지원**: 고객 서비스, 금융 등 YAML 기반 시나리오 정의
- **멀티턴 대화 추적**: 다중 턴 대화 컨텍스트 관리
- **폴트 인젝션**: 도구 실패 시나리오 테스트
- **LLM 기반 채점**: LLM Judge를 활용한 응답 품질 평가
- **골든셋 검증**: 기준 응답과의 비교 검증
- **리포트 비교**: 두 실행 결과를 비교하여 회귀를 감지
- **세분화된 HTTP API**: 에이전트 실행, 단일 도구 호출, 폴트 주입, 궤적 채점까지 웹에서 직접 실행
- **TDD 추적성**: PRD → SPEC → TC → 구현 함수까지 `@trace` 태그로 양방향 추적
- **Azure OpenAI 지원**: Azure OpenAI API를 통한 LLM 연동

## 빠른 시작

```bash
# 1) 빌드
cargo build

# 2) CLI로 벤치마크 실행 (Azure 설정 불필요)
cargo run -- run --suite customer_service --agent passthrough

# 3) 결과를 웹에서 조회 + 추가 실행
cargo run -- serve
# → http://127.0.0.1:8080 접속
```

## 프로젝트 구조

Cargo 워크스페이스로 구성되어 있으며, 12개의 전문화된 크레이트로 이루어집니다.

```
ai-agent-eval-harness/
├── crates/
│   ├── agent-models/            # 에이전트 인터페이스 및 모델 (BaseAgent 트레이트, 궤적)
│   ├── agent-core/              # LLM 클라이언트, Azure OpenAI 설정, PPA 에이전트
│   ├── data-scenarios/          # YAML 시나리오 로더 및 도메인 설정 모델
│   ├── domains/                 # 도메인별 도구 구현
│   │   ├── customer_service/    #   고객 서비스 도구 (분류, 환불, 에스컬레이션)
│   │   └── financial/           #   금융 도구 (단리/복리 계산, 거래 검증)
│   ├── execution/               # 실행 엔진 (HarnessRunner, 비교기, 에이전트 레지스트리)
│   ├── execution-tools/         # 기본 도구 프레임워크 (BaseTool, ToolRegistry, 파일 도구)
│   ├── execution-fault-injection/ # 폴트 인젝션 (실패 모드 시뮬레이션)
│   ├── execution-multi-turn/    # 멀티턴 대화 컨텍스트 관리
│   ├── scoring/                 # 궤적 평가기, 골든셋 검증기
│   ├── scoring-llm-judge/       # LLM 기반 채점 모델
│   ├── reporting/               # 궤적 로거, 컬러 콘솔 출력
│   └── eval-harness/            # CLI 바이너리 (main 진입점)
│       └── src/
│           ├── main.rs          # clap 서브커맨드 정의
│           ├── tui/             # ratatui 기반 대화형 TUI (PRD-001)
│           └── web/             # Axum HTTP 서버 + SPA (PRD-002 ~ PRD-004)
│               ├── mod.rs       #   AppState, build_router, run_server
│               ├── handlers.rs  #   조회 API + index.html 임베드
│               ├── api.rs       #   전체 스위트 실행/비교 API
│               ├── api_exec.rs  #   세분화된 실행 API (crates 4~10)
│               └── index.html   #   정적 SPA
├── eval_data/
│   ├── scenarios/               # 도메인별 시나리오 YAML
│   │   ├── customer_service.yaml
│   │   └── financial.yaml
│   └── golden_sets/             # 골든셋 기준 응답 JSON
│       ├── customer_service.json
│       └── financial.json
├── reporting_logs/              # 평가 리포트 JSON (자동 생성)
├── reporting_trajectories/      # 실행 궤적 JSON (자동 생성)
├── docs/                        # TDD 추적성 산출물
│   ├── prd/                     #   PRD-001 ~ PRD-004
│   ├── spec/                    #   SPEC-001 ~ SPEC-004
│   ├── registry/                #   파편화 레지스트리 (counters/entries/trace)
│   └── traceability-matrix.md   #   자동 생성 매트릭스
├── .tdd-config.json             # TDD 워크플로우 경로 설정
├── Cargo.toml                   # 워크스페이스 설정
├── Makefile.toml                # cargo-make 태스크
├── rust-toolchain.toml          # Rust 툴체인 (nightly)
└── .env.example                 # 환경 변수 템플릿
```

## 요구사항

- Rust nightly (rust-toolchain.toml에 지정된 버전)
- [cargo-make](https://github.com/sagiegurari/cargo-make) (선택, Makefile.toml 사용 시)
- Azure OpenAI 계정 (PPA 에이전트 사용 시)

## 설치

```bash
cargo build
```

릴리즈 빌드:

```bash
cargo build --release
```

## 환경 변수 설정

`.env.example`을 복사하여 `.env` 파일을 생성하고 값을 채우세요:

```bash
cp .env.example .env
```

```env
AZURE_OPENAI_ENDPOINT=https://your-resource.cognitiveservices.azure.com
AZURE_OPENAI_API_KEY=your-key
AZURE_OPENAI_API_VERSION=2025-04-01-preview
AZURE_OPENAI_DEPLOYMENT=gpt-5.3-chat
AZURE_OPENAI_REGION=koreacentral
AZURE_OPENAI_TEMPERATURE=1.0
AZURE_OPENAI_MAX_TOKENS=4096
```

> PPA 에이전트를 사용하지 않는 경우(passthrough 에이전트만 사용) `.env` 파일 없이도 실행 가능합니다.

## 사용법

`eval-harness` 바이너리는 6개의 서브커맨드를 제공합니다.

| 명령 | 용도 |
|------|------|
| `list` | 도메인/시나리오 및 등록된 에이전트 목록 출력 |
| `run` | 벤치마크 스위트 실행 및 리포트 저장 |
| `report <file>` | 저장된 리포트 JSON을 컬러 콘솔로 렌더 |
| `compare <baseline> <current>` | 두 리포트 비교, 회귀 감지 |
| `tui` | 대화형 TUI 2-패널 뷰 (조회 전용) |
| `serve` | Axum HTTP 서버 + 브라우저 SPA |

> 모든 명령은 `cargo run -- <command>` 또는 릴리즈 빌드 후 `./target/release/eval-harness <command>` 형태로 실행합니다.

### 데이터 경로 설정 (`eval-harness.toml`)

`eval_data/` 계열 디렉토리(시나리오, 골든셋)는 다음 4단계 우선순위로 해석됩니다 (높음 → 낮음):

1. **CLI 인자** — `--scenarios-dir`, `--golden-sets-dir`
2. **환경변수** — `EVAL_HARNESS_SCENARIOS_DIR`, `EVAL_HARNESS_GOLDEN_SETS_DIR`
3. **설정 파일** — 프로젝트 루트의 `eval-harness.toml`
4. **내장 기본값** — `eval_data/scenarios`, `eval_data/golden_sets`

설정 파일 예시 (`eval-harness.toml`):

```toml
[data]
# 상대 경로는 이 설정 파일이 위치한 디렉토리를 기준으로 해석됩니다.
scenarios_dir   = "eval_data/scenarios"
# 절대 경로도 가능합니다.
golden_sets_dir = "/var/lib/eval/golden"
```

설정 파일이 없으면 기존 동작과 동일하게 내장 기본값(CWD 기준)이 사용됩니다. desktop 앱은 워크스페이스 루트에서 동일한 설정 파일을 검색합니다. 자세한 명세는 `docs/spec/SPEC-015.md` 참조.

### 시나리오 목록 조회

```bash
cargo run -- list

# 시나리오 디렉토리 직접 지정
cargo run -- list --scenarios-dir eval_data/scenarios
```

### 벤치마크 실행

```bash
# 기본 (passthrough 에이전트, 전체 스위트)
cargo run -- run

# 특정 스위트 실행
cargo run -- run --suite customer_service
cargo run -- run --suite financial

# PPA 에이전트로 실행
cargo run -- run --agent ppa --suite financial

# 출력 파일 및 디렉토리 지정
cargo run -- run --agent ppa --output report.json --output-dir reporting_logs
```

### 리포트 조회

```bash
cargo run -- report reporting_logs/<report-file>.json
```

### 리포트 비교 (회귀 감지)

```bash
cargo run -- compare baseline.json current.json

# 허용 임계값 지정 (기본 5.0%)
cargo run -- compare baseline.json current.json --threshold 3.0

# 비교 결과 파일 저장
cargo run -- compare baseline.json current.json --output comparison.json
```

### TUI 모드 (대화형)

시나리오 목록과 저장된 리포트를 한 화면에서 탐색할 수 있는 2-패널 TUI를 실행합니다.

```bash
cargo run -- tui

# 디렉토리 직접 지정
cargo run -- tui --scenarios-dir eval_data/scenarios --reports-dir reporting_logs
```

**키 바인딩**

| 키 | 동작 |
|----|------|
| `↑` / `k` | 이전 항목 선택 |
| `↓` / `j` | 다음 항목 선택 |
| `Tab` | 시나리오 ↔ 리포트 패널 전환 |
| `q` / `Esc` | TUI 종료 |

> TUI는 현재 **조회 전용**입니다. 벤치마크 실행과 리포트 상세 뷰는 `run`/`report` 서브커맨드를 사용하세요.

### 웹 클라이언트 (HTTP 서버)

Axum 기반 내장 HTTP 서버를 띄우면 **7-탭 SPA**가 함께 제공되어, 브라우저에서 모든 REST API(조회·실행·비교·채점·폴트주입)를 직접 사용할 수 있습니다. `index.html`과 `help.html`은 바이너리에 임베드되며 **한국어/영어 다국어 토글**을 지원합니다 (헤더의 `[한][EN]` 버튼, 선택은 `localStorage`에 저장).

**탭 구성**

| 탭 | 기능 | 사용 API |
|----|------|----------|
| **Run** | 스위트 + 에이전트 + 출력 파일명 선택 후 실행 | `POST /api/run` |
| **Scenarios** | 도메인/시나리오 트리 탐색 + 단일 시나리오 실행 | `GET /api/list`, `GET /api/scenarios/:d/:id`, `POST /api/scenarios/:d/:id/run` |
| **Tools** | 도구 선택 + params JSON 편집 + Invoke/Fault 시뮬레이션 | `GET /api/tools`, `POST /api/tools/:n/invoke`, `POST /api/tools/:n/simulate-fault` |
| **Agents** | 에이전트 선택 + task/env 입력 후 직접 실행 | `POST /api/agents/:n/execute` |
| **Reports** | 리포트 조회 + 두 리포트 비교 | `GET /api/reports`, `POST /api/compare` |
| **Trajectories** | 저장된 궤적 목록/상세 + 채점 | `GET /api/trajectories[/:name]`, `POST /api/score` |
| **Goldens** | 전체 파일 조회 + 단일 엔트리 조회 | `GET /api/golden-sets[/:d/:sid]` |

```bash
cargo run -- serve

# 옵션 지정 (모든 기본값 명시)
cargo run -- serve --addr 127.0.0.1:8080 \
                   --scenarios-dir eval_data/scenarios \
                   --reports-dir reporting_logs \
                   --golden-sets-dir eval_data/golden_sets \
                   --trajectories-dir reporting_trajectories
```

실행 후 브라우저에서 `http://127.0.0.1:8080` 접속.

**HTTP API**

| 메서드 | 경로 | 설명 |
|--------|------|------|
| GET | `/` | SPA (정적 HTML) |
| GET | `/help` | 사용안내 페이지 (SPA 헤더 **📖 사용안내** 버튼에서 새 탭으로 열림) |
| GET | `/api/scenarios` | 도메인/시나리오 목록 JSON |
| GET | `/api/reports` | 저장된 리포트 파일명 목록 |
| GET | `/api/reports/:name` | 리포트 JSON 원문 (경로 순회 차단) |
| GET | `/api/list` | 도메인/시나리오 + 에이전트 집계 (CLI `list` 대응) |
| GET | `/api/agents` | 등록된 에이전트 이름 배열 (`passthrough`, 옵션으로 `ppa`) |
| GET | `/api/tools` | 전체 도메인 도구 메타데이터 (`execution-tools::ToolRegistry::get_tools_metadata`) |
| GET | `/api/golden-sets` | `eval_data/golden_sets/` 하위 골든셋 파일 배열 |
| GET | `/api/scenarios/:domain/:id` | 단일 시나리오 상세. 없으면 404 |
| POST | `/api/run` | 벤치마크 실행. body `{"suite","agent","output"?}` → `{report, saved_to}` |
| POST | `/api/compare` | 리포트 비교. body `{"baseline","current","threshold"?,"output"?}` → `{result, saved_to}` |

**POST 요청 예시**

```bash
# 벤치마크 실행 (파일명 지정 저장, CLI `run --output` 대응)
curl -X POST http://127.0.0.1:8080/api/run \
  -H 'content-type: application/json' \
  -d '{"suite":"customer_service","agent":"passthrough","output":"my_report.json"}'

# 리포트 비교 + 저장 (CLI `compare --output` 대응)
curl -X POST http://127.0.0.1:8080/api/compare \
  -H 'content-type: application/json' \
  -d '{"baseline":"a.json","current":"b.json","threshold":5.0,"output":"cmp.json"}'

# 도메인 + 에이전트 집계 (CLI `list` 대응)
curl http://127.0.0.1:8080/api/list
```

> `POST /api/run`은 항상 aggregate report를 `reports_dir`에 저장하며, `output` 생략 시 `evaluation_report_<timestamp>.json` 기본 파일명을 사용합니다. `POST /api/compare`는 `output` 지정 시에만 파일로 저장합니다.
>
> `POST /api/run`과 `/api/compare`는 `tokio::task::spawn_blocking` 으로 실행되어 서버 블로킹을 방지합니다. 리포트 이름은 `reports_dir` 내부 파일만 허용(경로 순회 차단).

**세분화된 실행 API (PRD-004)**

평가 파이프라인의 개별 단위를 웹에서 실행할 수 있습니다.

| 메서드 | 경로 | 설명 |
|--------|------|------|
| POST | `/api/scenarios/:domain/:id/run` | 단일 시나리오 실행. body `{"agent":"..."}` → `EvaluationResult` |
| POST | `/api/agents/:name/execute` | 에이전트 직접 호출. body `{"task":"...","environment":{...}}` → `Trajectory` |
| POST | `/api/tools/:name/invoke` | 단일 도구 호출. body `{"params":{...}}` → 결과 JSON |
| POST | `/api/tools/:name/simulate-fault` | 폴트 주입된 도구 호출. body `{"params":{...},"config":FaultInjectionConfig}` |
| GET | `/api/golden-sets/:domain/:scenario_id` | 골든셋 엔트리 상세 (없으면 404) |
| POST | `/api/score` | 궤적 채점. body `{"trajectory":{...}}` → `EvaluationResult` |
| GET | `/api/trajectories` | 저장된 궤적 파일명 목록 (`reporting_trajectories/`) |
| GET | `/api/trajectories/:name` | 궤적 JSON 원문 (경로 순회 차단) |

**예시**

```bash
# 단일 도구 호출
curl -X POST http://127.0.0.1:8080/api/tools/classify_inquiry/invoke \
  -H 'content-type: application/json' \
  -d '{"params":{"inquiry_text":"환불 요청","customer_id":"C1"}}'

# 단일 시나리오 실행
curl -X POST http://127.0.0.1:8080/api/scenarios/customer_service/cs_001/run \
  -H 'content-type: application/json' \
  -d '{"agent":"passthrough"}'

# 에이전트 직접 호출
curl -X POST http://127.0.0.1:8080/api/agents/passthrough/execute \
  -H 'content-type: application/json' \
  -d '{"task":"hello"}'

# 저장된 궤적 조회
curl http://127.0.0.1:8080/api/trajectories
```

> 범위 외: 멀티턴 대화 API(`ConversationManager`) 및 PpaAgent 전체 스위트에 대한 폴트 주입 연동은 다음 PRD로 분리되었습니다.

> 웹 API는 인증/인가 기능이 없습니다. 외부 노출 시 리버스 프록시에서 보호하세요.

### 데스크톱 앱 (Tauri)

`desktop/` 디렉토리는 **Tauri 2.x 기반 크로스 플랫폼 데스크톱 래퍼**입니다. 내장 Axum 서버를 무료 로컬 포트에 기동한 뒤, 시스템 WebView 로 동일한 7-탭 SPA 를 로드합니다. 웹 버전과 **100% 동일한 기능**을 네이티브 윈도우에서 사용할 수 있습니다.

> 이 프로젝트는 워크스페이스에서 `exclude` 되어 있어, 기본 `cargo build` 는 Tauri 시스템 의존성이 없어도 성공합니다.

**시스템 의존성**

| OS | 필수 패키지 |
|----|------------|
| Ubuntu 22.04+ | `sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev libayatana-appindicator3-dev librsvg2-dev` |
| macOS | `xcode-select --install` (Xcode Command Line Tools) |
| Windows | WebView2 Runtime (Win11 기본 포함) + MSVC 빌드 도구 |

**Tauri CLI 설치**

```bash
cargo install tauri-cli --version "^2.0"
```

**개발 실행** (데스크톱 윈도우 자동 오픈)

```bash
cd desktop
cargo tauri dev
```

**릴리즈 빌드** (플랫폼 네이티브 바이너리 생성)

```bash
cd desktop
cargo tauri build
```

또는 워크스페이스 루트에서 `cargo-make` 로 플랫폼별 번들을 한 번에 생성할 수 있습니다 (PRD-012 / SPEC-012):

```bash
# Windows (MSI/NSIS, 타겟: x86_64-pc-windows-msvc)
cargo make desktop-release-windows

# Linux (Deb/AppImage, 타겟: x86_64-unknown-linux-gnu)
cargo make desktop-release-linux

# macOS (.app/.dmg, 타겟: universal-apple-darwin)
cargo make desktop-release-macos

# 세 플랫폼을 연속 실행
cargo make desktop-release-all
```

> 위 태스크는 `desktop/` 크레이트로 `cwd` 진입 후 `cargo tauri build` 를 호출합니다. 크로스 타겟 toolchain(mingw, osxcross 등)과 `cargo-tauri` CLI 가 설치되어 있어야 해당 타겟을 실제로 빌드할 수 있으며, `install_crate` 메커니즘이 `cargo-tauri` 를 자동 설치합니다.

> `desktop/icons/icon.png` 에는 32×32 플레이스홀더(#f0c419 단색)가 포함되어 있어 `cargo tauri dev` / `cargo build` 가 즉시 동작합니다. 릴리즈 번들용 실제 아이콘 세트가 필요하면 `desktop/icons/README.md` 의 `cargo tauri icon` 가이드를 참조하고 `tauri.conf.json` 의 `bundle.active` 를 `true` 로 변경하세요.

**아키텍처**

```
┌─────────────────────────────────────────────┐
│  Tauri Desktop Window (system WebView)      │
│  └─ loads http://127.0.0.1:<auto_port>      │
│       └─ 7-탭 SPA + 한/영 i18n 그대로 재사용 │
└──────────────────────┬──────────────────────┘
                       │
              ┌────────▼────────┐
              │ 내장 Axum 서버  │  (detached thread)
              │ (같은 프로세스) │
              └─────────────────┘
```

> 백엔드 변경 없이 웹 UI 를 네이티브 앱으로 감싸는 패턴입니다. 향후 Tauri IPC(`invoke()`) 기반 직접 호출로 전환하는 것은 별도 PRD 대상입니다.

## 에이전트

| 에이전트 | 설명 |
|----------|------|
| `passthrough` | 항상 빈 응답을 반환하는 기본 에이전트 (베이스라인용) |
| `ppa` | Azure OpenAI를 사용하는 PPA(Perceive-Policy-Action) 루프 에이전트 |

## 시나리오 도메인

### 고객 서비스 (`customer_service`)

| 시나리오 | 난이도 | 도구 |
|----------|--------|------|
| cs_001: 고객 문의 분류 | easy | classify_inquiry |
| cs_002: 환불 요청 처리 | medium | classify_inquiry, process_refund |
| cs_003: 불만 고객 에스컬레이션 | medium | classify_inquiry, escalate_issue |
| cs_004: 복합 고객 서비스 워크플로우 | hard | classify_inquiry, process_refund, escalate_issue |
| cs_005: 문의 데이터 파일 분석 | hard | read_file, classify_inquiry, write_file |

### 금융 (`financial`)

| 시나리오 | 난이도 | 도구 |
|----------|--------|------|
| fin_001: 단리 이자 계산 | easy | calculate_simple_interest |
| fin_002: 복리 이자 계산 | medium | calculate_compound_interest, calculate_simple_interest |
| fin_003: 대액 출금 검증 | medium | validate_transaction |
| fin_004: 예금 데이터 파일 분석 | hard | read_file, calculate_simple_interest, write_file |
| fin_005: 종합 금융 분석 | hard | calculate_simple_interest, calculate_compound_interest, write_file |

## 커스텀 시나리오 추가

`eval_data/scenarios/` 디렉토리에 YAML 파일을 추가하세요:

```yaml
name: my_domain
description: 나의 도메인 설명

tools:
  - class_name: MyTool
    module_path: domains.my_domain.tools

scenarios:
  - id: my_001
    name: 시나리오 이름
    description: 시나리오 설명
    task_description: >
      에이전트에게 전달할 태스크 설명
    initial_environment:
      key: value
    expected_tools:
      - my_tool
    success_criteria:
      result_key: expected_value
    difficulty: easy  # easy | medium | hard
```

## 개발

```bash
# 포맷 + 검사 + 린트 + 빌드
cargo make build

# 포맷 + 검사 + 린트 + 릴리즈 빌드
cargo make release

# 포맷 + 테스트
cargo make test

# 파일 변경 시 자동 재실행
cargo make watch-run

# 포맷만
cargo make format

# 린트만
cargo make clippy

# 데스크톱 릴리즈 번들 (Windows/Linux/macOS, SPEC-012)
cargo make desktop-release-windows
cargo make desktop-release-linux
cargo make desktop-release-macos
cargo make desktop-release-all
```

## 아키텍처

```
Frontends (eval-harness binary)
 ├── CLI           (clap 서브커맨드: list | run | report | compare | tui | serve)
 ├── TUI           (ratatui + crossterm, 2-패널 조회)
 └── Web server    (Axum + 임베드 SPA)
        ├── 조회 API  : /api/{scenarios,reports,agents,tools,golden-sets,trajectories}
        └── 실행 API  : /api/{run,compare,score}, /api/scenarios/:d/:id/run,
                        /api/agents/:n/execute, /api/tools/:n/{invoke,simulate-fault}

Core (workspace crates)
 └── execution::HarnessRunner
        ├── AgentRegistry           → BaseAgent (passthrough | ppa)
        ├── ToolRegistry            → Domain Tools (customer_service, financial)
        ├── scoring::TrajectoryEvaluator
        │    └── GoldenSetValidator
        ├── execution-fault-injection::FaultInjector
        ├── execution-multi-turn::ConversationManager
        └── reporting::TrajectoryLogger
              ├── reporting_logs/          (EvaluationResult / Report JSON)
              └── reporting_trajectories/  (Trajectory JSON)
```

## TDD 추적성

본 프로젝트는 PRD → SPEC → 테스트 케이스(TC) → 구현 함수까지 `@trace` 태그로 양방향 추적됩니다.

```
docs/prd/PRD-001.md  ──┐
docs/prd/PRD-002.md    │  docs/spec/SPEC-00{1..4}.md
docs/prd/PRD-003.md    │     │
docs/prd/PRD-004.md  ──┘     └─ TC → @trace (테스트/구현 함수)
                                    │
                                    └─ docs/traceability-matrix.md (자동 생성)
```

**PRD 목록**

| PRD | 내용 | SPEC |
|-----|------|------|
| PRD-001 | TUI 모드 (ratatui 2-패널) | SPEC-001 |
| PRD-002 | 웹 서버 기본 조회 API + SPA | SPEC-002 |
| PRD-003 | 전체 크레이트 능력 HTTP 노출 (agents/tools/goldens/run/compare) | SPEC-003 |
| PRD-004 | 세분화된 실행 API (단일 시나리오/에이전트/도구/채점/폴트/궤적) | SPEC-004 |
| PRD-005 | CLI↔Web 완전 동등화 (`/api/list`, `run/compare` 의 `output` 필드) | SPEC-005 |
| PRD-006 | 탭 기반 SPA UI — 7개 탭에서 전체 API 사용 | SPEC-006 |
| PRD-007 | `/help` 사용안내 페이지 + SPA 헤더 버튼 | SPEC-007 |
| PRD-008 | 한/영 다국어 토글 (헤더 버튼, localStorage 영속화) | SPEC-008 |
| PRD-009 | Tauri 데스크톱 앱 (`desktop/`, 내장 Axum 서버 + 시스템 WebView) | SPEC-009 |
| PRD-010 | IBM Plex Sans KR/Sans/Mono 타이포그래피 적용 | SPEC-010 |
| PRD-011 | select/option 다크 테마 스타일 (color-scheme, 커스텀 화살표) | SPEC-011 |
| PRD-012 | 데스크톱 앱 크로스 플랫폼 릴리즈 스크립트 (Makefile.toml) | SPEC-012 |
| PRD-014 | 웹 UI 라이트/다크 테마 토글 (CSS 변수 기반, localStorage 영속화) | SPEC-014 |

**추적성 검증**

```bash
# 추적성 검사만
python3 ~/.claude/skills/tdd-workflow/scripts/verify_trace.py

# 검사 + 매트릭스 재생성
python3 ~/.claude/skills/tdd-workflow/scripts/verify_trace.py --matrix
```

## 라이선스

BSD-3-Clause
