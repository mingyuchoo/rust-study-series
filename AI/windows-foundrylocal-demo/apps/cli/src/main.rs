use anyhow::Result;
use foundry_client::FoundryClient;
use foundry_types::{ChatCompletionRequestBuilder, ChatMessage, FoundryModel};

#[tokio::main]
async fn main() -> Result<()> {
    // 구조화된 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("안녕하세요 Foundry Local!");
    println!("===================");

    // Foundry 클라이언트 초기화
    println!("\nFoundry Local 클라이언트 초기화 중...");
    let client = FoundryClient::new().await?;

    // 모델 선택
    let model = FoundryModel::Phi4MiniInstruct;
    println!("\n사용 중인 모델: {}", model.as_str());

    // 프롬프트 준비
    let prompt = "황금비율이란 무엇인가요?";
    println!("\n프롬프트: {prompt}");

    // 채팅 완성 요청 생성
    let request = ChatCompletionRequestBuilder::new(model.as_str())
        .message(ChatMessage::user(prompt))
        .temperature(0.7)
        .max_tokens(500)
        .build();

    println!("\n모델에 요청 전송 중...");
    println!("엔드포인트: {}", client.endpoint());

    // API 호출
    let response = client.chat_completion(request).await?;

    // 응답 출력
    if let Some(content) = response.first_content() {
        println!("\n응답:\n{content}");
    } else {
        println!("\n오류: API 결과에서 응답 내용을 추출할 수 없습니다.");
    }

    Ok(())
}
