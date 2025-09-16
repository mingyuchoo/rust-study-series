use delete_collection::{delete_collection, delete_collection_default};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env 로드 (없어도 조용히 통과)
    let _ = dotenv();

    // 필수 환경 변수: QDRANT_URL, QDRANT_COLLECTION_NAME_1,
    // QDRANT_COLLECTION_NAME_2
    let url = env::var("QDRANT_URL").map_err(|_| "환경 변수 QDRANT_URL 이(가) 설정되어 있지 않습니다.")?;
    let collection_name_1 = env::var("QDRANT_COLLECTION_NAME_1").map_err(|_| "환경 변수 QDRANT_COLLECTION_NAME_1 이(가) 설정되어 있지 않습니다.")?;
    let collection_name_2 = env::var("QDRANT_COLLECTION_NAME_2").map_err(|_| "환경 변수 QDRANT_COLLECTION_NAME_2 이(가) 설정되어 있지 않습니다.")?;

    // 첫 번째 컬렉션 삭제 (default 함수 사용)
    println!("컬렉션 '{}' 삭제 중...", collection_name_1);
    delete_collection_default(&url, &collection_name_1).await?;
    println!("컬렉션 '{}' 삭제 완료!", collection_name_1);

    // 두 번째 컬렉션 삭제 (메인 함수 사용)
    println!("컬렉션 '{}' 삭제 중...", collection_name_2);
    delete_collection(&url, &collection_name_2).await?;
    println!("컬렉션 '{}' 삭제 완료!", collection_name_2);

    Ok(())
}
