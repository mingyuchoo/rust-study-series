// main.rs - 애플리케이션 진입점
//

use onion_arch_example::ui::web;

#[tokio::main]
async fn main() {
    // Call the web UI module's main function
    web::main().await;
}
