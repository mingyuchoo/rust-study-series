use crate::domain::services::post_service::PostService;
use crate::domain::services::repositories::entities::post::{Post, PostForm};
use crate::domain::services::repositories::post_repository::PostRepository;

pub struct PostApplicationService<R: PostRepository> {
    post_service: PostService<R>,
}

impl<R: PostRepository> PostApplicationService<R> {
    pub fn new(post_service: PostService<R>) -> Self {
        Self {
            post_service,
        }
    }

    pub async fn create(&self, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> { self.post_service.create(post_form).await }

    pub async fn update(&self, id: i32, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> { self.post_service.update(id, post_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.post_service.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Post, Box<dyn std::error::Error>> { self.post_service.find_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error>> { self.post_service.find_all().await }
}

#[derive(Clone, Debug)]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub body: String,
}
impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            body: post.body,
        }
    }
}
