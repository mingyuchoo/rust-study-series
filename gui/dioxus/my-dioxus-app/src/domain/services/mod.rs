pub mod repositories;

use self::repositories::post_repository::PostRepository;
use self::repositories::todo_repository::TodoRepository;
use self::repositories::user_repository::UserRepository;

use self::repositories::entities::post::{Post, PostForm};
use self::repositories::entities::todo::{Todo, TodoForm};
use self::repositories::entities::user::{User, UserForm};

pub struct PostService<R: PostRepository> {
    repository: R,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(&self, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> {
        self.repository.create(post_form).await
    }

    pub async fn update(&self, id: i32, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> {
        self.repository.update(id, post_form).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.delete(id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Post, Box<dyn std::error::Error>> {
        self.repository.fetch_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        self.repository.fetch_all().await
    }
}

pub struct TodoService<R: TodoRepository> {
    repository: R,
}

impl<R: TodoRepository> TodoService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(&self, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> {
        self.repository.create(todo_form).await
    }

    pub async fn update(&self, id: i32, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> {
        self.repository.update(id, todo_form).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.delete(id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Todo, Box<dyn std::error::Error>> {
        self.repository.fetch_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        self.repository.fetch_all().await
    }
}

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(&self, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> {
        self.repository.create(user_form).await
    }

    pub async fn update(&self, id: i32, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> {
        self.repository.update(id, user_form).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.delete(id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        self.repository.fetch_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        self.repository.fetch_all().await
    }
}
    