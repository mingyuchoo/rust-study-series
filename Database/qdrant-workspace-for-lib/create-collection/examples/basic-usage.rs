use create_collection::{create_collection, create_collection_default};
use qdrant_client::qdrant::Distance;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env 로드 (없어도 조용히 통과)
    let _ = dotenv();

    // 필수 환경 변수: QDRANT_URL, QDRANT_COLLECTION_NAME_1, QDRANT_COLLECTION_NAME_2
    let url = env::var("QDRANT_URL")
        .map_err(|_| "환경 변수 QDRANT_URL 이(가) 설정되어 있지 않습니다.")?;
    let collection_name_1 = env::var("QDRANT_COLLECTION_NAME_1")
        .map_err(|_| "환경 변수 QDRANT_COLLECTION_NAME_1 이(가) 설정되어 있지 않습니다.")?;
    let collection_name_2 = env::var("QDRANT_COLLECTION_NAME_2")
        .map_err(|_| "환경 변수 QDRANT_COLLECTION_NAME_2 이(가) 설정되어 있지 않습니다.")?;

    // 선택 환경 변수: QDRANT_VECTOR_SIZE (기본값 384)
    let vector_size: u64 = env::var("QDRANT_VECTOR_SIZE")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(384);

    // 기본 파라미터로 컬렉션 생성
    println!("기본 파라미터로 컬렉션 '{}' 생성 중...", collection_name_1);
    create_collection_default(&url, &collection_name_1).await?;
    println!("컬렉션 '{}' 생성 완료!", collection_name_1);

    // 커스텀 파라미터로 컬렉션 생성 (벡터 크기와 거리 메트릭)
    println!(
        "커스텀 파라미터로 컬렉션 '{}' 생성 중... (vector_size={}, distance=Dot)",
        collection_name_2, vector_size
    );
    create_collection(
        &url,
        &collection_name_2,
        Some(vector_size),
        Some(Distance::Dot),
    )
    .await?;
    println!("컬렉션 '{}' 생성 완료!", collection_name_2);

    Ok(())
}
