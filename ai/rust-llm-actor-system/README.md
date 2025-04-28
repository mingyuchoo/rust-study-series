# Rust Multi-LLM Agent System

이 프로젝트는 Rust 생태계에서 ractor-supervisor 크레이트를 활용한 액터 모델 기반 설계의 DynamicSupervisor를 사용하여 멀티 LLM 에이전트 시스템을 구현합니다.

## 아키텍처

```
[DynamicSupervisor]
    ├── [LLM Agent 1] (ractor::Actor)
    ├── [LLM Agent 2] (ractor::Actor)
    └── [Task Supervisor] (비동기 작업 관리)
```

## 주요 컴포넌트

### LLMActor

- 실제 LLM 처리 로직 구현
- 다양한 LLM 모델 지원 (gpt-4, gpt-3.5-turbo 등)
- 비동기 프롬프트 처리

### DynamicSupervisor

- 런타임에 에이전트 동적 추가/제거 가능
- OneForOne 전략으로 개별 에이전트 관리
- 최대 자식 수(max_children) 설정 가능

### AgentRouter

- 메시지 라우팅 관리
- 키워드 기반 에이전트 선택
- 기본 에이전트 설정 가능

### HealthMonitor

- 에이전트 상태 추적
- 주기적인 헬스 체크
- 장애 감지 및 보고

## 메트릭스

- Prometheus 통합
- 요청 처리 시간 측정
- 에러율 모니터링
- 활성 에이전트 수 추적

## 사용 방법

### 의존성 설치

```bash
cargo build
```

### 실행

```bash
cargo run
```

## 예제

```rust
// 에이전트 생성
let (reply_tx, reply_rx) = ractor::call_rpc();
supervisor_ref.send_message(SupervisorMessage::SpawnAgent(
    "gpt-4o".to_string(),
    "gpt-4".to_string(),
    reply_tx,
))?

// 프롬프트 전송
let (reply_tx, reply_rx) = ractor::call_rpc();
router_ref.send_message(RouterMessage::RoutePrompt(
    "프랑스의 수도는 어디인가요?".to_string(),
    reply_tx,
))?
```

## 장애 복구 메커니즘

- 자동 재시작 정책
- 최대 재시작 횟수 제한
- 시간 기반 재시작 윈도우

## 성능 최적화

- 비동기 통신을 위한 `crossbeam-channel` 활용
- 메시지 직렬화/역직렬화에 `bincode` 사용
- 토큰 기반 비동기 처리

## 라이센스

MIT
