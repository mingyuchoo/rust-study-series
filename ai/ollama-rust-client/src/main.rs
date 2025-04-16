use reqwest::Client;
use serde::{Deserialize, Serialize};

// Ollama API 요청 본문 구조체 (직렬화용)
#[derive(Serialize, Debug)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool, /* 스트리밍 응답을 받을지 여부 (false면 전체 응답 한 번에 받음)
                   * 필요한 경우 다른 옵션 추가 가능 (예: options 필드)
                   * options: Option<serde_json::Value>, */
}

// Ollama API 응답 본문 구조체 (역직렬화용)
// 스트리밍이 아닐 때의 응답 구조에 초점
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct GenerateResponse {
    model: String,
    created_at: String,
    response: String, // 생성된 텍스트
    done: bool,
    // context, total_duration 등 다른 필드도 필요하면 추가
}

// 비동기 메인 함수 설정
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama_api_url = "http://localhost:11434/api/generate"; // Ollama API 엔드포인트
    let model_name = "phi4"; // 사용할 모델 이름 (Ollama에 로드된 모델 이름과 일치해야 함)
    const SYSTEM_PROMPT: &str = "You are a helpful, concise assistant."; // 기본 시스템 프롬프트

    // 사용자 프롬프트 입력 안내
    println!("Enter your prompt (press Enter to submit):");
    let mut user_prompt = String::new();
    std::io::stdin().read_line(&mut user_prompt).expect("Failed to read line");
    let user_prompt = user_prompt.trim();

    // 시스템 프롬프트와 사용자 프롬프트 결합
    let full_prompt = format!("[SYSTEM]\n{}\n[USER]\n{}", SYSTEM_PROMPT, user_prompt);

    // HTTP 클라이언트 생성
    let client = Client::new();

    // 요청 본문 데이터 생성
    let request_data = GenerateRequest {
        model: model_name.to_string(),
        prompt: full_prompt,
        stream: true, /* 스트리밍 응답을 받을지 여부 (true로 설정하여 스트리밍 모드 활성화)
                       * 필요한 경우 다른 옵션 추가 가능 (예: options 필드)
                       * options: Option<serde_json::Value>, */
    };

    println!("Sending request to Ollama (Model: {})...", model_name);

    // POST 요청 보내기 (스트리밍 응답 처리)
    let res = client.post(ollama_api_url).json(&request_data).send().await?;

    // 응답 상태 확인
    if res.status().is_success() {
        use futures_util::StreamExt;

        println!("Received streaming response.\n--- Ollama Response ---");
        let mut stream = res.bytes_stream();
        let mut buffer = Vec::new();
        while let Some(chunk) = stream.next().await {
            match chunk {
                | Ok(bytes) => {
                    buffer.extend_from_slice(&bytes);
                    // 스트림에서 온 데이터는 여러 줄일 수 있으므로 줄 단위로 파싱
                    while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                        let line = buffer.drain(..= pos).collect::<Vec<u8>>();
                        if let Ok(line_str) = std::str::from_utf8(&line) {
                            let line_trimmed = line_str.trim();
                            if !line_trimmed.is_empty() {
                                // 각 줄을 GenerateResponse로 역직렬화
                                if let Ok(resp) = serde_json::from_str::<GenerateResponse>(line_trimmed) {
                                    print!("{}", resp.response);
                                }
                            }
                        }
                    }
                },
                | Err(e) => {
                    eprintln!("Stream error: {}", e);
                    break;
                },
            }
        }
        println!("\n-----------------------\n");
    } else {
        println!("Error: Request failed with status: {}", res.status());
        let error_body = res.text().await?;
        println!("Error body: {}", error_body);
    }

    Ok(())
}
