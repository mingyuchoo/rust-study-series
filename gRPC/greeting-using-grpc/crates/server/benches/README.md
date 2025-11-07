# Server Benchmarks

이 디렉토리는 gRPC 서버 성능 벤치마크를 포함합니다.

## 벤치마크 목록

### 1. server_benchmark.rs
서버 비즈니스 로직의 성능을 측정합니다.

**실행 방법:**
```bash
cargo bench --bench server_benchmark -p greeting-server
```

**측정 항목:**
- `process_greeting_request`: 단일 greeting 요청 처리 시간
- `process_multiple_greetings`: 여러 greeting 요청 순차 처리 시간

**설명:**
- 서버를 시작하지 않고 비즈니스 로직만 벤치마크
- 네트워크 오버헤드 없이 순수 처리 성능 측정

## 벤치마크 결과

벤치마크 결과는 `target/criterion/` 디렉토리에 저장됩니다.

HTML 리포트 확인:
```bash
open target/criterion/report/index.html
```

## 모든 벤치마크 실행

```bash
cargo bench -p greeting-server
```

## 성능 팁

1. **Release 모드**: 벤치마크는 항상 release 모드에서 실행됩니다
2. **반복 측정**: Criterion은 자동으로 여러 번 측정하여 통계적으로 유의미한 결과를 제공합니다
3. **비교**: 코드 변경 전후 벤치마크를 실행하여 성능 변화를 확인할 수 있습니다
