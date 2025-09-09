use anyhow::{Result, anyhow};
use azure_foundry_embedding_demo::AzureEmbeddingClient;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // .env 파일 로드 (있을 경우)
    let _ = dotenv();

    // 환경 변수에서 설정 읽기
    let endpoint = env::var("AZURE_OPENAI_ENDPOINT").map_err(|_| anyhow!("AZURE_OPENAI_ENDPOINT 환경 변수가 설정되지 않았습니다"))?;
    let api_key = env::var("AZURE_OPENAI_API_KEY").map_err(|_| anyhow!("AZURE_OPENAI_API_KEY 환경 변수가 설정되지 않았습니다"))?;
    let deployment_name = env::var("AZURE_OPENAI_DEPLOYMENT_NAME").unwrap_or_else(|_| "text-embedding-3-large".to_string());

    // 클라이언트 생성
    let client = AzureEmbeddingClient::new(endpoint, api_key, deployment_name);

    // 예제 텍스트들
    let texts = vec![
        "안녕하세요, 오늘 날씨가 참 좋네요.".to_string(),
        "Hello, the weather is really nice today.".to_string(),
        "프로그래밍은 창의적이고 흥미로운 활동입니다.".to_string(),
    ];

    println!("텍스트 임베딩을 생성 중입니다...");

    // 임베딩 생성
    match client.get_embeddings(texts.clone()).await {
        | Ok(embeddings) => {
            println!("임베딩 생성 완료!");
            println!("총 {} 개의 임베딩이 생성되었습니다.", embeddings.len());

            for (i, embedding) in embeddings.iter().enumerate() {
                println!("텍스트 {}: '{}' - 임베딩 차원: {}", i + 1, texts[i], embedding.len());
                println!("처음 10개 값: {:?}", &embedding[.. 10.min(embedding.len())]);
            }

            // 유사도 계산 예제
            if embeddings.len() >= 2 {
                let similarity = AzureEmbeddingClient::cosine_similarity(&embeddings[0], &embeddings[1]);
                println!("\n첫 번째와 두 번째 텍스트 간의 코사인 유사도: {:.4}", similarity);
            }
        },
        | Err(e) => {
            eprintln!("오류 발생: {}", e);
        },
    }

    Ok(())
}
