use crate::domain::entities::post::{Post, PostForm};
use std::error::Error;

#[async_trait::async_trait]
pub trait PostRepository {
    async fn fetch_all(&self) -> Result<Vec<Post>, Box<dyn Error>>;
    async fn fetch_by_id(&self, id: i32) -> Result<Post, Box<dyn Error>>;
    async fn create(&self, post: PostForm) -> Result<Post, Box<dyn Error>>;
    async fn update(&self, id: i32, post: PostForm) -> Result<Post, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
