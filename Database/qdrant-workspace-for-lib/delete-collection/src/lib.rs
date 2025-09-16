use qdrant_client::{Qdrant, QdrantError};

pub async fn delete_collection_default(url: &str, collection_name: &str) -> Result<(), QdrantError> { delete_collection(url, collection_name).await }

pub async fn delete_collection(url: &str, collection_name: &str) -> Result<(), QdrantError> {
    let client = Qdrant::from_url(url).build()?;

    client.delete_collection(collection_name).await?;

    Ok(())
}
