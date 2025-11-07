# Client Examples

이 디렉토리는 gRPC 클라이언트 사용 예제를 포함합니다.

## 예제 목록

### 1. simple_client.rs
기본적인 gRPC 클라이언트 사용법을 보여줍니다.

**실행 방법:**
```bash
cargo run --example simple_client -p greeting-client
```

**설명:**
- 서버에 연결
- 단일 greeting 요청 전송
- 응답 처리 및 출력

### 2. multiple_requests.rs
순차적으로 여러 요청을 보내는 방법을 보여줍니다.

**실행 방법:**
```bash
cargo run --example multiple_requests -p greeting-client
```

**설명:**
- 여러 이름에 대해 순차적으로 요청
- 각 요청의 응답을 순서대로 처리

### 3. concurrent_requests.rs
동시에 여러 요청을 보내는 방법을 보여줍니다.

**실행 방법:**
```bash
cargo run --example concurrent_requests -p greeting-client
```

**설명:**
- `JoinSet`을 사용한 동시 요청 처리
- 비동기 태스크 관리
- 결과 수집 및 출력

## 사전 요구사항

예제를 실행하기 전에 서버가 실행 중이어야 합니다:

```bash
cargo run --bin server -p greeting-server
```

또는 서버 예제를 사용:

```bash
cargo run --example simple_server -p greeting-server
```
