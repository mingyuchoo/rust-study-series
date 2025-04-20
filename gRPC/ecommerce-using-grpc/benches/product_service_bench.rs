#![feature(test)]

extern crate test;

use ecommerce_using_grpc::product_info_proto::product_info_client::ProductInfoClient;
use ecommerce_using_grpc::product_info_proto::product_info_server::ProductInfoServer;
use ecommerce_using_grpc::product_info_proto::{Product, ProductId};
use ecommerce_using_grpc::MyProductInfo;
use std::net::SocketAddr;
use std::time::Duration;
use test::Bencher;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use tonic::Request;
use tonic::transport::Server;

// Helper function to set up the test environment
fn setup_test_env() -> (Runtime, String) {
    let rt = Runtime::new().unwrap();
    let server_addr = "[::1]:50053";

    // Start the server
    let addr = server_addr.parse::<SocketAddr>().unwrap();
    let product_info = MyProductInfo::default();

    rt.spawn(async move {
        Server::builder().add_service(ProductInfoServer::new(product_info)).serve(addr).await.unwrap();
    });

    // Give the server a moment to start
    rt.block_on(async {
        sleep(Duration::from_millis(100)).await;
    });

    (rt, format!("http://{}", server_addr))
}

#[bench]
fn bench_add_product(b: &mut Bencher) {
    let (rt, server_url) = setup_test_env();

    b.iter(|| {
        rt.block_on(async {
            let mut client = ProductInfoClient::connect(server_url.clone()).await.unwrap();

            let test_product = Product {
                id: 4,
                name: String::from("Benchmark Product"),
                description: String::from("This is a benchmark product"),
                price: 199.99,
            };

            let request = Request::new(test_product);
            let _response = client.add_product(request).await.unwrap();
        });
    });
}

#[bench]
fn bench_get_product(b: &mut Bencher) {
    let (rt, server_url) = setup_test_env();

    b.iter(|| {
        rt.block_on(async {
            let mut client = ProductInfoClient::connect(server_url.clone()).await.unwrap();

            let request = Request::new(ProductId {
                id: 1,
            });
            let _response = client.get_product(request).await.unwrap();
        });
    });
}
