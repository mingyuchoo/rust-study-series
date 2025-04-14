use crate::domain::services::repositories::entities::user::{User, UserForm};
use crate::domain::services::repositories::user_repository::UserRepository;

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
        }
    }

    pub async fn create(&self, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> { self.repository.create(user_form).await }

    pub async fn update(&self, id: i32, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> { self.repository.update(id, user_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.repository.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> { self.repository.fetch_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> { self.repository.fetch_all().await }
}
