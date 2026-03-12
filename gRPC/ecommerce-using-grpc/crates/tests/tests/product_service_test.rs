use proto::product_info_client::ProductInfoClient;
use proto::product_info_server::ProductInfoServer;
use proto::{Product,
            ProductId};
use server::MyProductInfo;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tonic::Request;
use tonic::transport::Server;

#[tokio::test]
async fn test_add_product() {
    // Start the server in the background
    let server_addr = "[::1]:50052".parse::<SocketAddr>().unwrap();
    let product_info = MyProductInfo::default();

    let server_task = tokio::spawn(async move {
        Server::builder()
            .add_service(ProductInfoServer::new(product_info))
            .serve(server_addr)
            .await
            .unwrap();
    });

    // Wait for the server to become ready with a retry loop
    let server_url = format!("http://{}", server_addr);
    let mut client = {
        let mut attempts = 0u32;
        loop {
            match ProductInfoClient::connect(server_url.clone()).await {
                | Ok(c) => break c,
                | Err(_) if attempts < 20 => {
                    attempts += 1;
                    sleep(Duration::from_millis(50)).await;
                },
                | Err(e) => panic!("Failed to connect to server after retries: {}", e),
            }
        }
    };

    // Test adding a product
    let test_product = Product {
        id: 0, // Server assigns the ID
        name: String::from("Test Product"),
        description: String::from("This is a test product"),
        price: 99.99,
    };

    let request = Request::new(test_product.clone());
    let response = client.add_product(request).await.unwrap();
    let assigned_id = response.into_inner().id;

    // Verify the server assigned a valid ID
    assert!(assigned_id > 0, "Expected a positive ID, got {}", assigned_id);

    // Test getting the product by the assigned ID
    let get_request = Request::new(ProductId {
        id: assigned_id,
    });
    let get_response = client.get_product(get_request).await.unwrap();
    let retrieved_product = get_response.into_inner();

    // Verify the retrieved product matches what was stored
    assert_eq!(retrieved_product.id, assigned_id);
    assert_eq!(retrieved_product.name, test_product.name);
    assert_eq!(retrieved_product.description, test_product.description);
    assert!((retrieved_product.price - test_product.price).abs() < f32::EPSILON);

    // Test getting a non-existent product returns an error
    let missing_request = Request::new(ProductId {
        id: 9999,
    });
    assert!(
        client.get_product(missing_request).await.is_err(),
        "Expected NotFound error for missing product"
    );

    // Clean up
    server_task.abort();
}
