// main.rs - 애플리케이션 진입점
//

// Import the crate itself
use onion_arch_example::{// Application layer
                         application::services::user_application_service::UserApplicationService,
                         // Infrastructure layer
                         infrastructure::api::user_api_controller::UserApiController
};


fn main() -> Result<(), String> {
    Ok(())
}
