use std::error::Error;
use tonic::{transport::Channel,
            Request,
            Response};
pub mod product_info_proto
{
    tonic::include_proto!("ecommerce");
}

use product_info_proto::{product_info_client::ProductInfoClient,
                         Product,
                         ProductId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let mut client: ProductInfoClient<Channel> =
        ProductInfoClient::connect("http://[::1]:50051").await
                                                        .unwrap();

    let request: Request<_> = Request::new(ProductId { id: 1, });

    let response: Response<Product> = client.get_product(request)
                                            .await
                                            .unwrap();

    println!("{:?}", response);

    Ok(())
}
