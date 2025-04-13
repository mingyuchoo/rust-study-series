//
// infrastructure/api.rs - REST API 구현 (예시)
//

use crate::application::services::UserApplicationService;
use crate::domain::services::repositories::UserRepository;
pub struct UserApiController<R: UserRepository> {
    application_service: UserApplicationService<R>,
}

impl<R: UserRepository> UserApiController<R> {
    pub fn new(application_service: UserApplicationService<R>) -> Self {
        Self {
            application_service,
        }
    }

    pub fn register_user(
        &self,
        id: String,
        username: String,
        email: String,
    ) -> Result<String, String> {
        match self.application_service.register_user(id, username, email) {
            Ok(user_dto) => Ok(format!("User created: {}", user_dto.username)),
            Err(e) => Err(format!("Failed to create user: {}", e)),
        }
    }

    pub fn get_user(&self, id: &str) -> Result<String, String> {
        match self.application_service.get_user_details(id) {
            Some(user) => Ok(format!(
                "User: {}, Email: {}, Active: {}",
                user.username, user.email, user.active
            )),
            None => Err(format!("User with id {} not found", id)),
        }
    }

    pub fn deactivate_user(&self, id: &str) -> Result<String, String> {
        match self.application_service.deactivate_user(id) {
            Ok(_) => Ok(format!("User {} deactivated", id)),
            Err(e) => Err(e),
        }
    }
}
