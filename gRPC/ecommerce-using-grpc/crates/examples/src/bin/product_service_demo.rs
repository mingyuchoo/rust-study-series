use proto::product_info_client::ProductInfoClient;
use proto::{Product, ProductId};
use std::error::Error;
use tonic::{Request, Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the server
    let mut client = ProductInfoClient::connect("http://[::1]:50051")
        .await?
        .max_encoding_message_size(16 * 1024 * 1024);

    // Create a new product
    let new_product = Product {
        id: 2,
        name: String::from("Google Pixel 7"),
        description: String::from("The latest Google Pixel smartphone"),
        price: 599.99,
    };

    // Add the product
    let add_request = Request::new(new_product);
    let add_response = client.add_product(add_request).await?;
    println!("Added product with ID: {}", add_response.into_inner().id);

    // Get the product details
    let get_request = Request::new(ProductId { id: 2 });
    let get_response: Response<Product> = client.get_product(get_request).await?;
    let product = get_response.into_inner();

    // Display the product details
    println!("\nProduct Details:");
    println!("ID: {}", product.id);
    println!("Name: {}", product.name);
    println!("Description: {}", product.description);
    println!("Price: ${:.2}", product.price);

    Ok(())
}
