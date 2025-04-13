// main.rs - 애플리케이션 진입점
//

// Import the crate itself
use onion_arch_example::{
    // Domain layer
    domain::{
        models::User,
        repositories::UserRepository,
        services::UserService,
    },
    // Application layer
    application::services::{UserApplicationService, UserDto},
    // Infrastructure layer
    infrastructure::{
        repositories::InMemoryUserRepository,
        api::UserApiController,
    },
};

fn main() {
    // 인프라스트럭처 계층 초기화
    let repository = InMemoryUserRepository::new();

    // 애플리케이션 서비스 초기화
    let app_service = UserApplicationService::new(repository);

    // API 컨트롤러 초기화
    let api_controller = UserApiController::new(app_service);

    // API 사용 예시
    let result = api_controller.register_user("1".to_string(), "john_doe".to_string(), "john@example.com".to_string());
    println!("{:?}", result);

    let user_info = api_controller.get_user("1");
    println!("{:?}", user_info);

    let deactivate_result = api_controller.deactivate_user("1");
    println!("{:?}", deactivate_result);

    let user_info_after = api_controller.get_user("1");
    println!("{:?}", user_info_after);
}
