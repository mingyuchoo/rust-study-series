use crate::domain::services::repositories::entities::user::{User, UserForm};
use std::error::Error;

#[async_trait::async_trait(?Send)]
pub trait UserRepository {
    async fn fetch_all(&self) -> Result<Vec<User>, Box<dyn Error>>;
    #[allow(dead_code)]
    async fn fetch_by_id(&self, id: i32) -> Result<User, Box<dyn Error>>;
    async fn create(&self, user: UserForm) -> Result<User, Box<dyn Error>>;
    async fn update(&self, id: i32, user: UserForm) -> Result<User, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
