# Benchmarks and Examples

이 문서는 Cargo 워크스페이스의 벤치마크와 예제 구조를 설명합니다.

## 디렉토리 구조

```
crates/
├── client/
│   ├── benches/           # 클라이언트 성능 벤치마크
│   │   ├── client_benchmark.rs
│   │   ├── concurrent_benchmark.rs
│   │   └── README.md
│   └── examples/          # 클라이언트 사용 예제
│       ├── simple_client.rs
│       ├── multiple_requests.rs
│       ├── concurrent_requests.rs
│       └── README.md
└── server/
    ├── benches/           # 서버 성능 벤치마크
    │   ├── server_benchmark.rs
    │   └── README.md
    └── examples/          # 서버 사용 예제
        ├── simple_server.rs
        ├── custom_port_server.rs
        ├── greeting_logic.rs
        └── README.md
```

## 빠른 시작

### 예제 실행

#### 서버 예제
```bash
# 기본 서버 시작
cargo run --example simple_server -p greeting-server

# 커스텀 포트 서버
cargo run --example custom_port_server -p greeting-server

# 비즈니스 로직 테스트
cargo run --example greeting_logic -p greeting-server
```

#### 클라이언트 예제
```bash
# 서버가 실행 중이어야 합니다
cargo run --bin server -p greeting-server

# 다른 터미널에서:
cargo run --example simple_client -p greeting-client
cargo run --example multiple_requests -p greeting-client
cargo run --example concurrent_requests -p greeting-client
```

### 벤치마크 실행

#### 서버 벤치마크
```bash
# 서버 비즈니스 로직 벤치마크 (서버 실행 불필요)
cargo bench -p greeting-server
```

#### 클라이언트 벤치마크
```bash
# 서버가 실행 중이어야 합니다
cargo run --bin server -p greeting-server

# 다른 터미널에서:
cargo bench -p greeting-client
```

## 벤치마크 상세

### Client Benchmarks

1. **client_benchmark.rs**
   - 단일 gRPC 요청 성능 측정
   - 연결, 요청, 응답 처리 전체 사이클

2. **concurrent_benchmark.rs**
   - 동시 요청 처리 성능 (1, 5, 10, 20 동시성 레벨)
   - 연결 수립 오버헤드 측정

### Server Benchmarks

1. **server_benchmark.rs**
   - 단일 greeting 요청 처리 시간
   - 다중 greeting 요청 순차 처리 시간

## 예제 상세

### Client Examples

1. **simple_client.rs**
   - 기본 클라이언트 연결 및 요청
   - 단일 요청/응답 처리

2. **multiple_requests.rs**
   - 순차적 다중 요청 처리
   - 여러 이름에 대한 greeting

3. **concurrent_requests.rs**
   - 동시 다중 요청 처리
   - `JoinSet`을 사용한 비동기 태스크 관리

### Server Examples

1. **simple_server.rs**
   - 기본 서버 시작 (포트 50051)
   - 표준 설정

2. **custom_port_server.rs**
   - 커스텀 포트 서버 (포트 50052)
   - 포트 설정 변경 예제

3. **greeting_logic.rs**
   - 비즈니스 로직 독립 테스트
   - 다양한 입력 케이스 검증

## 성능 측정 결과

벤치마크 결과는 다음 위치에 저장됩니다:
```
target/criterion/
```

HTML 리포트 보기:
```bash
open target/criterion/report/index.html
```

## 개발 워크플로우

### 새 기능 개발
1. 비즈니스 로직 구현
2. `examples/`에 사용 예제 추가
3. `benches/`에 성능 벤치마크 추가
4. README 업데이트

### 성능 최적화
1. 현재 성능 벤치마크 실행 (베이스라인)
2. 코드 최적화
3. 벤치마크 재실행
4. Criterion 리포트에서 성능 변화 확인

## 추가 정보

각 디렉토리의 README.md 파일에서 더 자세한 정보를 확인할 수 있습니다:
- `crates/client/benches/README.md`
- `crates/client/examples/README.md`
- `crates/server/benches/README.md`
- `crates/server/examples/README.md`
