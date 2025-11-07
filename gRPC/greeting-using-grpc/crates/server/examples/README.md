# Server Examples

이 디렉토리는 gRPC 서버 사용 예제를 포함합니다.

## 예제 목록

### 1. simple_server.rs
기본적인 gRPC 서버 실행 방법을 보여줍니다.

**실행 방법:**
```bash
cargo run --example simple_server -p greeting-server
```

**설명:**
- 기본 포트(50051)에서 서버 시작
- 클라이언트 요청 대기
- Ctrl+C로 종료

### 2. custom_port_server.rs
사용자 지정 포트에서 서버를 실행하는 방법을 보여줍니다.

**실행 방법:**
```bash
cargo run --example custom_port_server -p greeting-server
```

**설명:**
- 포트 50052에서 서버 시작
- 클라이언트는 연결 문자열을 `http://[::1]:50052`로 변경해야 함

### 3. greeting_logic.rs
서버를 시작하지 않고 greeting 로직을 테스트합니다.

**실행 방법:**
```bash
cargo run --example greeting_logic -p greeting-server
```

**설명:**
- 비즈니스 로직만 독립적으로 테스트
- 다양한 입력값에 대한 테스트 케이스
- 에러 처리 확인

## 클라이언트 연결

서버가 실행 중일 때 클라이언트를 연결할 수 있습니다:

```bash
cargo run --bin client -p greeting-client
```

또는 클라이언트 예제를 사용:

```bash
cargo run --example simple_client -p greeting-client
```
