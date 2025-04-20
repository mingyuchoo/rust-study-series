use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod product_info_proto {
    tonic::include_proto!("ecommerce"); // proto package
}

use product_info_proto::product_info_server::{ProductInfo, ProductInfoServer};
use product_info_proto::{Product, ProductId};

#[derive(Default)]
pub struct MyProductInfo {}

#[tonic::async_trait]
impl ProductInfo for MyProductInfo {
    async fn add_product(&self, request: Request<Product>) -> Result<Response<ProductId>, Status> {
        let response: ProductId = ProductId {
            id: request.into_inner().id,
        };

        Ok(Response::new(response))
    }

    async fn get_product(&self, request: Request<ProductId>) -> Result<Response<Product>, Status> {
        let response: Product = Product {
            id: request.into_inner().id,
            name: String::from("MacBook Air 15"),
            description: String::from("Impressively big. Impossibly thin."),
            price: 1299.9,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();

    let product_info: MyProductInfo = MyProductInfo::default();

    println!("ProductInfoServer listening on {}", addr);

    Server::builder().add_service(ProductInfoServer::new(product_info)).serve(addr).await.unwrap();

    Ok(())
}
