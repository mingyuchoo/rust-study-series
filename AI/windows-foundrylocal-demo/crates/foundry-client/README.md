# foundry-client

Foundry Local API와 상호작용하기 위한 클라이언트 라이브러리입니다.

## 기능

- Foundry Local 매니저 자동 초기화
- 채팅 완성 API 지원
- 구조화된 에러 처리
- 구조화된 로깅

## 사용 예제

```rust
use foundry_client::FoundryClient;
use foundry_types::{ChatCompletionRequestBuilder, ChatMessage, FoundryModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FoundryClient::new().await?;
    
    let request = ChatCompletionRequestBuilder::new(FoundryModel::Phi4MiniInstruct.as_str())
        .message(ChatMessage::user("황금비율이란?"))
        .temperature(0.7)
        .max_tokens(500)
        .build();
    
    let response = client.chat_completion(request).await?;
    
    if let Some(content) = response.first_content() {
        println!("응답: {}", content);
    }
    
    Ok(())
}
```
