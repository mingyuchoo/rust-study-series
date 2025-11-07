# 아키텍처 개요

## 워크스페이스 구조

이 프로젝트는 Cargo 워크스페이스(모노레포)로 구성되어 있으며, 재사용 가능한 컴포넌트로 모듈화되어 있습니다.

```
plugin-architecture/
├─ Cargo.toml              # 워크스페이스 루트 (virtual manifest)
├─ Cargo.lock              # 통합 의존성 잠금 파일
├─ crates/                 # 재사용 가능한 라이브러리
│  ├─ foundry-types/       # 공통 타입 정의
│  └─ foundry-client/      # API 클라이언트 라이브러리
└─ apps/                   # 애플리케이션
   └─ cli/                 # CLI 예제 애플리케이션
```

## 크레이트 설명

### foundry-types
**목적**: 공통 타입 정의 및 데이터 구조

**주요 타입**:
- `FoundryModel`: 사용 가능한 AI 모델 열거형
- `ChatMessage`: 채팅 메시지 (user, system, assistant)
- `ChatCompletionRequest`: API 요청 구조체
- `ChatCompletionResponse`: API 응답 구조체
- `ChatCompletionRequestBuilder`: 빌더 패턴 구현

**의존성**: serde, serde_json, thiserror

### foundry-client
**목적**: Foundry Local API와의 통신을 담당하는 클라이언트 라이브러리

**주요 기능**:
- `FoundryClient`: HTTP 클라이언트 및 매니저 래퍼
- `FoundryClientBuilder`: 빌더 패턴으로 클라이언트 생성
- 구조화된 에러 처리 (`FoundryClientError`)
- 구조화된 로깅 (tracing)

**의존성**: foundry-types, foundry-local, reqwest, tokio, tracing

### foundry-cli (apps/cli)
**목적**: Foundry Local 사용 예제 CLI 애플리케이션

**기능**:
- 클라이언트 초기화
- 채팅 완성 요청 생성 및 전송
- 응답 처리 및 출력

**의존성**: foundry-types, foundry-client, tokio, tracing

## 주요 개선사항

### 1. 모듈화 및 재사용성
- 각 크레이트는 독립적으로 사용 가능
- 다른 프로젝트에서 `foundry-client`와 `foundry-types`를 라이브러리로 사용 가능
- 워크스페이스 의존성 공유로 버전 일관성 유지

### 2. 타입 안전성
- `FoundryModel` 열거형으로 모델 선택 시 타입 안전성 보장
- 빌더 패턴으로 유연하고 안전한 객체 생성
- 편의 메서드 (`ChatMessage::user()`, `ChatMessage::system()` 등)

### 3. 에러 처리
- `thiserror`를 사용한 구조화된 에러 타입
- 각 에러 케이스에 대한 명확한 메시지
- 에러 컨텍스트 보존

### 4. 로깅
- `tracing` 크레이트를 사용한 구조화된 로깅
- 환경 변수(`RUST_LOG`)로 로그 레벨 제어
- 디버깅 및 모니터링 용이

### 5. 빌더 패턴
- `ChatCompletionRequestBuilder`: 유연한 요청 생성
- `FoundryClientBuilder`: 커스터마이징 가능한 클라이언트 생성
- 선택적 매개변수 지원

## 사용 예제

### 기본 사용법

```rust
use foundry_client::FoundryClient;
use foundry_types::{ChatCompletionRequestBuilder, ChatMessage, FoundryModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 클라이언트 초기화
    let client = FoundryClient::new().await?;
    
    // 요청 생성
    let request = ChatCompletionRequestBuilder::new(FoundryModel::Phi4MiniInstruct.as_str())
        .message(ChatMessage::user("안녕하세요"))
        .temperature(0.7)
        .max_tokens(500)
        .build();
    
    // API 호출
    let response = client.chat_completion(request).await?;
    
    // 응답 처리
    if let Some(content) = response.first_content() {
        println!("응답: {}", content);
    }
    
    Ok(())
}
```

### 고급 사용법

```rust
// 커스텀 타임아웃 설정
let client = FoundryClient::builder()
    .bootstrap(true)
    .timeout(Duration::from_secs(600))
    .build()
    .await?;

// 여러 메시지로 대화 구성
let request = ChatCompletionRequestBuilder::new(model)
    .message(ChatMessage::system("당신은 친절한 AI 어시스턴트입니다."))
    .message(ChatMessage::user("황금비율에 대해 설명해주세요."))
    .temperature(0.7)
    .max_tokens(1000)
    .build();
```

## 빌드 및 실행

```bash
# 전체 워크스페이스 빌드
cargo build --workspace

# CLI 실행
cargo run -p foundry-cli

# 릴리스 빌드
cargo build --workspace --release

# 테스트
cargo test --workspace

# Makefile.toml 사용
cargo make run
cargo make test
cargo make release
```

## 확장 가능성

이 아키텍처는 다음과 같은 확장이 용이합니다:

1. **새로운 애플리케이션 추가**: `apps/` 디렉터리에 새 크레이트 추가
2. **새로운 라이브러리 추가**: `crates/` 디렉터리에 새 크레이트 추가
3. **플러그인 시스템**: 동적 로딩을 위한 trait 기반 플러그인 시스템 구현
4. **추가 API 지원**: `foundry-client`에 새로운 메서드 추가
5. **다른 모델 지원**: `FoundryModel` 열거형에 새 변형 추가
