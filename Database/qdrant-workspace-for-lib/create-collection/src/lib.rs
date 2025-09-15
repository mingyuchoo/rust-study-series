use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};
use qdrant_client::{Qdrant, QdrantError};


pub async fn create_collection(url: &str, collection_name: &str, vector_size: Option<u64>, distance: Option<Distance>) -> Result<(), QdrantError> {
    let client = Qdrant::from_url(url).build()?;

    let size = vector_size.unwrap_or(100);
    let dist = distance.unwrap_or(Distance::Cosine);

    client
        .create_collection(CreateCollectionBuilder::new(collection_name).vectors_config(VectorParamsBuilder::new(size, dist)))
        .await?;

    Ok(())
}

pub async fn create_collection_default(url: &str, collection_name: &str) -> Result<(), QdrantError> {
    create_collection(url, collection_name, None, None).await
}
