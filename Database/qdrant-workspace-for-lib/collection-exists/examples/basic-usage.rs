use collection_exists::{collection_exists, collection_exists_default};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env 로드 (없으면 무시)
    let _ = dotenv();

    // 필수 환경 변수 로드
    let url = env::var("QDRANT_URL").map_err(|_| "환경 변수 QDRANT_URL 이(가) 설정되어 있지 않습니다.")?;
    let collection_name_1 = env::var("QDRANT_COLLECTION_NAME_1").map_err(|_| "환경 변수 QDRANT_COLLECTION_NAME_1 이(가) 설정되어 있지 않습니다.")?;
    let collection_name_2 = env::var("QDRANT_COLLECTION_NAME_2").map_err(|_| "환경 변수 QDRANT_COLLECTION_NAME_2 이(가) 설정되어 있지 않습니다.")?;

    // 컬렉션 1 존재 여부 확인 (default 래퍼 사용 예)
    let exists_1 = collection_exists_default(&url, &collection_name_1).await?;
    println!("컬렉션 '{}' 존재 여부: {}", collection_name_1, exists_1);

    // 컬렉션 2 존재 여부 확인 (메인 함수 사용 예)
    let exists_2 = collection_exists(&url, &collection_name_2).await?;
    println!("컬렉션 '{}' 존재 여부: {}", collection_name_2, exists_2);

    Ok(())
}
