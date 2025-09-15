use create_collection::{create_collection, create_collection_default};
use qdrant_client::qdrant::Distance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:6334";
    
    // Create a collection with default parameters
    println!("Creating collection 'my_collection' with default parameters...");
    create_collection_default(url, "my_collection").await?;
    println!("Collection 'my_collection' created successfully!");

    // Create a collection with custom parameters
    println!("Creating collection 'custom_collection' with custom parameters...");
    create_collection(
        url,
        "custom_collection",
        Some(384), // vector size
        Some(Distance::Dot), // distance metric
    ).await?;
    println!("Collection 'custom_collection' created successfully!");

    Ok(())
}