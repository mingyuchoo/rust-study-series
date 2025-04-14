use crate::domain::services::PostService;
use crate::domain::services::TodoService;
use crate::domain::services::UserService;
use crate::domain::services::repositories::entities::post::{Post, PostForm};
use crate::domain::services::repositories::entities::todo::{Todo, TodoForm};
use crate::domain::services::repositories::entities::user::{User, UserForm};
use crate::domain::services::repositories::post_repository::PostRepository;
use crate::domain::services::repositories::todo_repository::TodoRepository;
use crate::domain::services::repositories::user_repository::UserRepository;

use crate::presentation::pages::{Home, Navbar, Posts, Todos, Users};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/users")]
    Users {},
    #[route("/todos")]
    Todos {},
    #[route("/posts")]
    Posts {},
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub struct PostApplicationService<R: PostRepository> {
    post_service: PostService<R>,
}

impl<R: PostRepository> PostApplicationService<R> {
    pub fn new(post_service: PostService<R>) -> Self {
        Self { post_service }
    }

    pub async fn create(&self, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> {
        self.post_service.create(post_form).await
    }

    pub async fn update(&self, id: i32, post_form: PostForm) -> Result<Post, Box<dyn std::error::Error>> {
        self.post_service.update(id, post_form).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.post_service.delete(id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Post, Box<dyn std::error::Error>> {
        self.post_service.find_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        self.post_service.find_all().await
    }
}

pub struct TodoApplicationService<R: TodoRepository> {
    todo_service: TodoService<R>,
}

impl<R: TodoRepository> TodoApplicationService<R> {
    pub fn new(todo_service: TodoService<R>) -> Self {
        Self { todo_service }
    }

    pub async fn create(&self, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> {
        self.todo_service.create(todo_form).await
    }

    pub async fn update(&self, id: i32, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> {
        self.todo_service.update(id, todo_form).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.todo_service.delete(id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Todo, Box<dyn std::error::Error>> {
        self.todo_service.find_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        self.todo_service.find_all().await
    }
}

pub struct UserApplicationService<R: UserRepository> {
    user_service: UserService<R>,
}

impl<R: UserRepository> UserApplicationService<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        Self { user_service }
    }

    pub async fn create(&self, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> {
        self.user_service.create(user_form).await
    }

    pub async fn update(&self, id: i32, user_form: UserForm) -> Result<User, Box<dyn std::error::Error>> {
        self.user_service.update(id, user_form).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.user_service.delete(id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        self.user_service.find_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        self.user_service.find_all().await
    }
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

#[derive(Clone, Debug)]
pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
        }
    }
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

