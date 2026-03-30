use demo_rust::handler::app;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("서버 시작: http://localhost:3000");
    axum::serve(listener, app()).await.unwrap();
}
