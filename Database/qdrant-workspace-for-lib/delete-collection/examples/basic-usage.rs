use delete_collection::{delete_collection, delete_collection_default};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:6334";

    // Delete a collection using the default function
    println!("Deleting collection 'my_collection'...");
    delete_collection_default(url, "my_collection").await?;
    println!("Collection 'my_collection' deleted successfully!");

    // Delete another collection using the main function
    println!("Deleting collection 'custom_collection'...");
    delete_collection(url, "custom_collection").await?;
    println!("Collection 'custom_collection' deleted successfully!");

    Ok(())
}
