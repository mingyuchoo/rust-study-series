// infrastructure/api.rs - REST API 구현 (예시)
//


use crate::application::services::user_application_service::UserApplicationService;
use crate::domain::services::repositories::user_repository::UserRepository;

pub struct UserApiController<R: UserRepository> {
    application_service: UserApplicationService<R>,
}

impl<R: UserRepository> UserApiController<R> {
    pub fn new(application_service: UserApplicationService<R>) -> Self {
        Self {
            application_service,
        }
    }
    
    pub fn new_with_repository(repository: R) -> Self {
        Self::new(UserApplicationService::new(repository))
    }
    
    pub fn register_user(&self, id: String, username: String, email: String) -> Result<String, String> {
        match self.application_service.register_user(id, username, email) {
            | Ok(user_dto) => Ok(format!("User created: {}", user_dto.username)),
            | Err(e) => Err(format!("Failed to create user: {}", e)),
        }
    }

    pub fn get_user(&self, id: &str) -> Result<String, String> {
        let user = self
            .application_service
            .get_user_details(id)
            .ok_or_else(|| format!("User with id {} not found", id))?;
        Ok(format!("User: {}, Email: {}, Active: {}", user.username, user.email, user.active))
    }

    pub fn deactivate_user(&self, id: &str) -> Result<String, String> {
        match self.application_service.deactivate_user(id) {
            | Ok(_) => Ok(format!("User {} deactivated", id)),
            | Err(e) => Err(e),
        }
    }

    pub fn list_all_users(&self) -> Result<String, String> {
        let users = self.application_service.list_all_users()?;
        if users.is_empty() {
            return Ok("No users found".to_string());
        }

        let mut result = String::from("Users:\n");
        for user in users {
            result.push_str(&format!(
                "- {} ({}): {}\n",
                user.username,
                user.email,
                if user.active { "active" } else { "inactive" }
            ));
        }

        Ok(result)
    }
}
