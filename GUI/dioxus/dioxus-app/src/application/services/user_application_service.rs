use crate::domain::services::repositories::entities::user::{User, UserForm};
use crate::domain::services::repositories::user_repository::UserRepository;
use crate::domain::services::user_service::UserService;

pub struct UserApplicationService<R: UserRepository> {
    user_service: UserService<R>,
}

impl<R: UserRepository> UserApplicationService<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        Self {
            user_service,
        }
    }

    pub async fn create(&self, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> { self.user_service.create(user_form).await }

    pub async fn update(&self, id: i32, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> { self.user_service.update(id, user_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.user_service.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> { self.user_service.find_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> { self.user_service.find_all().await }
}

#[derive(Clone, Debug)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub email: String,
}
impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
