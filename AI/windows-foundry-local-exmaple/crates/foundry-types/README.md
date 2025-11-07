# foundry-types

Foundry API와 상호작용하기 위한 공통 타입 정의 라이브러리입니다.

## 주요 타입

- `FoundryModel`: 사용 가능한 모델 열거형
- `ChatMessage`: 채팅 메시지 구조체
- `ChatCompletionRequest`: 채팅 완성 요청
- `ChatCompletionResponse`: 채팅 완성 응답

## 사용 예제

```rust
use foundry_types::{ChatMessage, ChatCompletionRequestBuilder, FoundryModel};

let request = ChatCompletionRequestBuilder::new(FoundryModel::Phi4MiniInstruct.as_str())
    .message(ChatMessage::user("안녕하세요"))
    .temperature(0.7)
    .max_tokens(500)
    .build();
```
