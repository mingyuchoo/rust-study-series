# ecommerce-using-grpc

Rust 기반 gRPC 전자상거래 상품 관리 서비스입니다. Railway Oriented Programming, tracing을 사용한 구조화 로깅, 인메모리 저장소, Cargo 워크스페이스(모노레포) 아키텍처를 적용하였습니다.

## 사전 요구사항

### Protocol Buffers 컴파일러

**Ubuntu**
```bash
sudo apt install protobuf-compiler
```

**Fedora**
```bash
sudo dnf install protobuf-compiler
```

**macOS**
```bash
brew install protobuf
```

## 프로젝트 구조

Cargo 워크스페이스(모노레포) 구조입니다:

```
ecommerce-using-grpc/
├── Cargo.toml          # 워크스페이스 설정
├── crates/
│   ├── proto/          # 공유 Protocol Buffer 정의
│   │   ├── proto/
│   │   │   └── ProductInfo.proto
│   │   ├── build.rs
│   │   └── src/lib.rs
│   ├── server/         # gRPC 서버 구현
│   │   └── src/
│   │       ├── lib.rs  # 서비스 로직, 인메모리 저장소, 에러 처리
│   │       └── main.rs # 서버 바이너리
│   ├── client/         # gRPC 클라이언트 구현
│   │   └── src/
│   │       └── main.rs # 클라이언트 바이너리
│   └── tests/          # 통합 테스트
│       └── tests/
│           └── product_service_test.rs
└── proto/              # 원본 proto 파일 (참조용)
    └── ProductInfo.proto
```

## 주요 기능

- **Cargo 워크스페이스(모노레포) 아키텍처** - 여러 크레이트로 조직화
- **gRPC 기반 상품 관리 서비스** - 효율적인 클라이언트-서버 통신
- **상품 추가 및 조회** - 핵심 CRUD 작업
- **인메모리 저장소** - `Arc<Mutex<>>` 기반 스레드 안전 `HashMap`
- **자동 증가 ID** - 서버가 atomic 카운터로 상품 ID 할당
- **Railway Oriented Programming** - 깔끔한 에러 처리 패턴
- **tracing을 사용한 구조화 로깅** - 프로덕션 수준의 관측 가능성
- **입력 유효성 검사** - 상품명(비어있지 않음) 및 가격(양수) 검증

## 주요 의존성

| 패키지 | 버전 | 설명 |
|--------|------|------|
| `tonic` | 0.14.5 | gRPC 프레임워크 |
| `prost` | 0.14.3 | Protocol Buffers 구현 |
| `tokio` | 1.50.0 | 비동기 런타임 |
| `anyhow` | 1.0.102 | 에러 처리 |
| `thiserror` | 2.0.18 | 커스텀 에러 타입 |
| `tracing` | 0.1.44 | 구조화 로깅 |
| `tracing-subscriber` | 0.3.22 | 로그 출력 |

## 빌드

```bash
# 프로젝트 빌드
cargo build

# 릴리스 빌드
cargo build --release
```

## 실행

### 서버 시작

```bash
cargo run -p server
```

서버는 `[::1]:50051` (IPv6 localhost)에서 시작됩니다.

### 클라이언트 실행

다른 터미널에서:

```bash
cargo run -p client
```

## 테스트

```bash
# 전체 워크스페이스 테스트
cargo test

# 특정 크레이트 테스트
cargo test -p tests
```

## API

### AddProduct

시스템에 새 상품을 추가합니다. 서버가 자동으로 고유 ID를 할당합니다.

**요청:** `Product`
- `id` (int32): 무시됨 -- 서버가 새 자동 증가 ID를 할당
- `name` (string): 상품명 (필수, 비어있지 않아야 함)
- `description` (string): 상품 설명
- `price` (float): 상품 가격 (필수, 양수여야 함)

**응답:** `ProductId`
- `id` (int32): 서버가 할당한 상품 ID

### GetProduct

ID로 상품 정보를 조회합니다.

**요청:** `ProductId`
- `id` (int32): 상품 ID (양수, 해당 ID의 상품이 없으면 `NOT_FOUND` 반환)

**응답:** `Product`
- 저장된 전체 상품 정보

## 에러 처리

Railway Oriented Programming 패턴으로 커스텀 에러 타입을 사용합니다:

| 에러 | gRPC 상태 | 조건 |
|------|-----------|------|
| `ServiceError::NotFound` | `NOT_FOUND` | 해당 ID의 상품이 없음 |
| `ServiceError::InvalidData` | `INVALID_ARGUMENT` | 빈 이름 또는 양수가 아닌 가격 |
| `ServiceError::Internal` | `INTERNAL` | 예기치 않은 서버 측 오류 |

## License

프로젝트 라이선스 파일을 참조하세요.
