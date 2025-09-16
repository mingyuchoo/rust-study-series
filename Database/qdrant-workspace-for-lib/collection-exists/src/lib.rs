use qdrant_client::{Qdrant, QdrantError};

pub async fn collection_exists_default(url: &str, collection_name: &str) -> Result<bool, QdrantError> { collection_exists(url, collection_name).await }
pub async fn collection_exists(url: &str, collection_name: &str) -> Result<bool, QdrantError> {
    let client = Qdrant::from_url(url).build()?;
    client.collection_exists(collection_name).await
}
