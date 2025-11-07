use anyhow::{Context, Result};
use foundry_local::FoundryLocalManager;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    println!("안녕하세요 Foundry Local!");
    println!("===================");

    // Create a FoundryLocalManager instance using the builder pattern
    println!("\nFoundry Local 매니저 초기화 중...");
    let manager = FoundryLocalManager::builder()
        .bootstrap(true) // Start the service if not running
        .build()
        .await?;
    
    // Use the actual model ID from the service
    // Available models:
    // - gpt-oss-20b-cuda-gpu:1
    // - Phi-4-mini-instruct-cuda-gpu:4
    // - qwen2.5-7b-instruct-cuda-gpu:3
    let model_id = "qwen2.5-7b-instruct-cuda-gpu:3";
    println!("\n사용 중인 모델: {}", model_id);

    // Build the prompt
    let prompt = "황금비율이란 무엇인가요?";
    println!("\n프롬프트: {prompt}");

    // Use the OpenAI compatible API to interact with the model
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120)) // 2분 타임아웃 설정
        .build()
        .context("Failed to build HTTP client")?;

    // API 요청 데이터 준비
    let endpoint = format!("{}/chat/completions", manager.endpoint()?);
    let request_body = serde_json::json!({
        "model": model_id,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.7,
        "max_tokens": 500
    });

    println!("\n모델에 요청 전송 중...");
    println!("엔드포인트: {}", endpoint);
    println!(
        "요청 본문: {}",
        serde_json::to_string_pretty(&request_body).unwrap_or_default()
    );

    let response = client
        .post(&endpoint)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request to API")?;

    // 응답 상태 코드 확인
    let status = response.status();
    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "<응답 본문을 읽을 수 없음>".to_string());
        println!("\nAPI 오류 응답 (상태 코드 {}):\n{}", status, error_text);
        anyhow::bail!("API 요청 실패: {}", status);
    }

    // 응답 본문 텍스트로 먼저 가져오기
    let response_text = response
        .text()
        .await
        .context("Failed to read response body as text")?;

    if response_text.trim().is_empty() {
        println!("\n오류: API에서 빈 응답을 받았습니다.");
        anyhow::bail!("빈 API 응답");
    }

    // 텍스트를 JSON으로 파싱
    println!("\nJSON 응답 파싱 중...");
    let result: serde_json::Value = serde_json::from_str(&response_text)
        .with_context(|| format!("JSON 파싱 실패. 응답 본문: {}", response_text))?;

    // 응답 내용 추출
    if let Some(content) = result
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
    {
        println!("\n응답:\n{content}");
    } else {
        println!("\n오류: API 결과에서 응답 내용을 추출할 수 없습니다.");
        println!("전체 API 응답: {}", result);
        anyhow::bail!("API 응답에서 content를 찾을 수 없음");
    }

    Ok(())
}
