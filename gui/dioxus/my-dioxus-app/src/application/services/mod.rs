use crate::domain::services::PostService;
use crate::domain::services::TodoService;
use crate::domain::services::UserService;

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

pub struct PostApplicationService<R: PostService> {
    post_service: PostService<R>,
}

impl<R: PostService> PostApplicationService<R> {
    pub fn new(post_service: PostService<R>) -> Self {
        Self { post_service }
    }

    pub fn create(&self, post: Post) -> Result<Post, R::Error> {
        self.post_service.create(post)
    }

    pub fn update(&self, post: Post) -> Result<Post, R::Error> {
        self.post_service.update(post)
    }

    pub fn delete(&self, post: Post) -> Result<(), R::Error> {
        self.post_service.delete(post)
    }

    pub fn find_by_id(&self, id: i32) -> Result<Post, R::Error> {
        self.post_service.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<Post>, R::Error> {
        self.post_service.find_all()
    }
}

pub struct TodoApplicationService<R: TodoService> {
    todo_service: TodoService<R>,
}

impl<R: TodoService> TodoApplicationService<R> {
    pub fn new(todo_service: TodoService<R>) -> Self {
        Self { todo_service }
    }

    pub fn create(&self, todo: Todo) -> Result<Todo, R::Error> {
        self.todo_service.create(todo)
    }

    pub fn update(&self, todo: Todo) -> Result<Todo, R::Error> {
        self.todo_service.update(todo)
    }

    pub fn delete(&self, todo: Todo) -> Result<(), R::Error> {
        self.todo_service.delete(todo)
    }

    pub fn find_by_id(&self, id: i32) -> Result<Todo, R::Error> {
        self.todo_service.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<Todo>, R::Error> {
        self.todo_service.find_all()
    }
}

pub struct UserApplicationService<R: UserService> {
    user_service: UserService<R>,
}

impl<R: UserService> UserApplicationService<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        Self { user_service }
    }

    pub fn create(&self, user: User) -> Result<User, R::Error> {
        self.user_service.create(user)
    }

    pub fn update(&self, user: User) -> Result<User, R::Error> {
        self.user_service.update(user)
    }

    pub fn delete(&self, user: User) -> Result<(), R::Error> {
        self.user_service.delete(user)
    }

    pub fn find_by_id(&self, id: i32) -> Result<User, R::Error> {
        self.user_service.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<User>, R::Error> {
        self.user_service.find_all()
    }
}

#[derive(Clone, Debug)]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub content: String,
}
impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub content: String,
}
impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            content: todo.content,
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

