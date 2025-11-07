// apps/web/main.rs - 애플리케이션 진입점

#[tokio::main]
async fn main() {
    presentation::web::run_server().await;
}
