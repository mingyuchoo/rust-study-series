pub mod product_info_proto {
    tonic::include_proto!("ecommerce");
}

use product_info_proto::product_info_client::ProductInfoClient;
use product_info_proto::ProductId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProductInfoClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    let request = tonic::Request::new(ProductId { id: 1 });

    let response = client.get_product(request).await.unwrap();

    println!("{:?}", response);

    Ok(())
}
