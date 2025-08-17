use crate::domain::services::repositories::entities::post::{Post, PostForm};
use crate::domain::services::repositories::post_repository::PostRepository;

pub struct PostService<R: PostRepository> {
    repository: R,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
        }
    }

    pub async fn create(&self, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> { self.repository.create(post_form).await }

    pub async fn update(&self, id: i32, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> { self.repository.update(id, post_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.repository.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Post, Box<dyn std::error::Error>> { self.repository.fetch_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error>> { self.repository.fetch_all().await }
}
