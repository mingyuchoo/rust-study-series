# ai-agent-eval-harness

Rust로 구현된 AI 에이전트 평가(Evaluation) 하네스입니다. 다양한 도메인 시나리오에서 AI 에이전트의 도구 호출 정확도, 성능, 회귀 여부를 자동으로 평가합니다.

## 특징

- **다중 도메인 지원**: 고객 서비스, 금융 등 YAML 기반 시나리오 정의
- **멀티턴 대화 추적**: 다중 턴 대화 컨텍스트 관리
- **폴트 인젝션**: 도구 실패 시나리오 테스트
- **LLM 기반 채점**: LLM Judge를 활용한 응답 품질 평가
- **골든셋 검증**: 기준 응답과의 비교 검증
- **리포트 비교**: 두 실행 결과를 비교하여 회귀를 감지
- **Azure OpenAI 지원**: Azure OpenAI API를 통한 LLM 연동

## 프로젝트 구조

```
ai-agent-eval-harness/
├── src/
│   ├── agent_core/          # 에이전트 코어 (LLM 클라이언트, 설정, 모델)
│   ├── data_scenarios/      # 시나리오 로더 및 모델
│   ├── domains/
│   │   ├── customer_service/ # 고객 서비스 도구 (분류, 환불, 에스컬레이션)
│   │   └── financial/        # 금융 도구 (단리/복리 계산, 거래 검증)
│   ├── execution/           # 실행 엔진 (러너, 비교기, 에이전트 레지스트리)
│   ├── execution_fault_injection/ # 폴트 인젝션 (실패 모드, 래퍼)
│   ├── execution_multi_turn/ # 멀티턴 대화 관리
│   ├── execution_tools/     # 실행 도구 (파일 도구, 레지스트리)
│   ├── reporting/           # 로깅 및 리포트 출력
│   ├── scoring/             # 채점 (평가기, 골든셋 검증기)
│   ├── scoring_llm_judge/   # LLM 기반 채점
│   ├── lib.rs
│   └── main.rs
├── eval_data/
│   ├── scenarios/           # 도메인별 시나리오 YAML
│   │   ├── customer_service.yaml
│   │   └── financial.yaml
│   └── golden_sets/         # 골든셋 기준 응답 JSON
│       ├── customer_service.json
│       └── financial.json
├── Cargo.toml
├── Makefile.toml
└── rust-toolchain.toml
```

## 요구사항

- Rust (rust-toolchain.toml에 지정된 버전)
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

PPA 에이전트(Azure OpenAI 연동)를 사용하려면 `.env` 파일을 생성하세요:

```env
AZURE_OPENAI_ENDPOINT=https://<your-resource>.openai.azure.com/
AZURE_OPENAI_API_KEY=<your-api-key>
AZURE_OPENAI_DEPLOYMENT=gpt-5.3-chat
AZURE_OPENAI_API_VERSION=2025-04-01-preview
AZURE_OPENAI_REGION=koreacentral
AZURE_OPENAI_TEMPERATURE=1.0
AZURE_OPENAI_MAX_TOKENS=4096
```

## 사용법

### 시나리오 목록 조회

```bash
cargo run -- list
```

### 벤치마크 실행

```bash
# 기본 (passthrough 에이전트, 전체 스위트)
cargo run -- run

# 특정 스위트 실행
cargo run -- run --suite customer_service

# PPA 에이전트로 실행
cargo run -- run --agent ppa --suite financial

# 출력 파일 지정
cargo run -- run --agent ppa --output report.json
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
```

## 에이전트

| 에이전트 | 설명 |
|----------|------|
| `passthrough` | 항상 빈 응답을 반환하는 기본 에이전트 (베이스라인용) |
| `ppa` | Azure OpenAI를 사용하는 실제 LLM 에이전트 |

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
# 포맷 + 린트 + 빌드
cargo make build

# 테스트
cargo make test

# 파일 변경 시 자동 재실행
cargo make watch-run
```

## 라이선스

BSD-3-Clause
