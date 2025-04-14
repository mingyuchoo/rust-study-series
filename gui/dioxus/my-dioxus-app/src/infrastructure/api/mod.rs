pub mod repositories;

use crate::application::services::post_application_service::PostApplicationService;
use crate::application::services::todo_application_service::TodoApplicationService;
use crate::application::services::user_application_service::UserApplicationService;
use crate::domain::services::repositories::entities::post::{Post, PostForm};
use crate::domain::services::repositories::entities::todo::{Todo, TodoForm};
use crate::domain::services::repositories::entities::user::{User, UserForm};
use crate::domain::services::repositories::post_repository::PostRepository;
use crate::domain::services::repositories::todo_repository::TodoRepository;
use crate::domain::services::repositories::user_repository::UserRepository;
use std::error::Error;

pub struct PostApiController<R: PostRepository> {
    application_service: PostApplicationService<R>,
}

impl<R: PostRepository> PostApiController<R> {
    pub fn new(application_service: PostApplicationService<R>) -> Self {
        Self {
            application_service,
        }
    }

    pub async fn create(&self, post_form: PostForm) -> Result<Post, Box<dyn Error>> { self.application_service.create(post_form).await }

    pub async fn update(&self, id: i32, post_form: PostForm) -> Result<Post, Box<dyn Error>> { self.application_service.update(id, post_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> { self.application_service.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Post, Box<dyn Error>> { self.application_service.find_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Post>, Box<dyn Error>> { self.application_service.find_all().await }
}

pub struct TodoApiController<R: TodoRepository> {
    application_service: TodoApplicationService<R>,
}

impl<R: TodoRepository> TodoApiController<R> {
    pub fn new(application_service: TodoApplicationService<R>) -> Self {
        Self {
            application_service,
        }
    }

    pub async fn create(&self, todo_form: TodoForm) -> Result<Todo, Box<dyn Error>> { self.application_service.create(todo_form).await }

    pub async fn update(&self, id: i32, todo_form: TodoForm) -> Result<Todo, Box<dyn Error>> { self.application_service.update(id, todo_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> { self.application_service.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Todo, Box<dyn Error>> { self.application_service.find_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> { self.application_service.find_all().await }
}

pub struct UserApiController<R: UserRepository> {
    application_service: UserApplicationService<R>,
}

impl<R: UserRepository> UserApiController<R> {
    pub fn new(application_service: UserApplicationService<R>) -> Self {
        Self {
            application_service,
        }
    }

    pub async fn create(&self, user_form: UserForm) -> Result<User, Box<dyn Error>> { self.application_service.create(user_form).await }

    pub async fn update(&self, id: i32, user_form: UserForm) -> Result<User, Box<dyn Error>> { self.application_service.update(id, user_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> { self.application_service.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<User, Box<dyn Error>> { self.application_service.find_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> { self.application_service.find_all().await }
}
