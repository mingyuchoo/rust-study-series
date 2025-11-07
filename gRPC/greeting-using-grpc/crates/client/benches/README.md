# Client Benchmarks

이 디렉토리는 gRPC 클라이언트 성능 벤치마크를 포함합니다.

## 벤치마크 목록

### 1. client_benchmark.rs
기본 클라이언트 요청 성능을 측정합니다.

**실행 방법:**
```bash
cargo bench --bench client_benchmark -p greeting-client
```

**측정 항목:**
- `grpc_client_request`: 단일 gRPC 요청의 전체 처리 시간 (연결, 요청, 응답 처리)

### 2. concurrent_benchmark.rs
동시 요청 처리 성능을 측정합니다.

**실행 방법:**
```bash
cargo bench --bench concurrent_benchmark -p greeting-client
```

**측정 항목:**
- `concurrent_requests`: 다양한 동시성 레벨(1, 5, 10, 20)에서의 성능
- `connection_establishment`: 서버 연결 수립 오버헤드

## 사전 요구사항

벤치마크 실행 전 gRPC 서버가 실행 중이어야 합니다:

```bash
# 터미널 1: 서버 시작
cargo run -p greeting-server --bin server
```

```bash
# 터미널 2: 벤치마크 실행
cargo bench -p greeting-client
```

## 벤치마크 결과

결과는 `target/criterion/` 디렉토리에 저장됩니다.

HTML 리포트 확인:
```bash
open target/criterion/report/index.html
```

## 모든 벤치마크 실행

```bash
cargo bench -p greeting-client
```

## 주의사항

벤치마크는 `[::1]:50051`에서 실행 중인 서버가 필요합니다. 서버가 실행 중이지 않으면 연결 오류가 발생합니다.
