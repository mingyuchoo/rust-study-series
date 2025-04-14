use crate::domain::services::repositories::entities::post::{Post, PostForm};
use crate::domain::services::repositories::entities::todo::{Todo, TodoForm};
use crate::domain::services::repositories::entities::user::{User, UserForm};
use crate::domain::services::repositories::post_repository::PostRepository;
use crate::domain::services::repositories::todo_repository::TodoRepository;
use crate::domain::services::repositories::user_repository::UserRepository;
use crate::infrastructure::api::constants::API_BASE_URL;
use reqwest::Client;
use std::error::Error;

// User functions
pub async fn fetch_users() -> Result<Vec<User>, Box<dyn Error>> {
    let repo = JsonPlaceholderUserRepository::new();
    repo.fetch_all().await
}

pub async fn create_user(user: UserForm) -> Result<User, Box<dyn Error>> {
    let repo = JsonPlaceholderUserRepository::new();
    repo.create(user).await
}

pub async fn update_user(id: i32, user: UserForm) -> Result<User, Box<dyn Error>> {
    let repo = JsonPlaceholderUserRepository::new();
    repo.update(id, user).await
}

pub async fn delete_user(id: i32) -> Result<(), Box<dyn Error>> {
    let repo = JsonPlaceholderUserRepository::new();
    repo.delete(id).await
}

// Todo functions
pub async fn fetch_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let repo = JsonPlaceholderTodoRepository::new();
    repo.fetch_all().await
}

pub async fn create_todo(todo: TodoForm) -> Result<Todo, Box<dyn Error>> {
    let repo = JsonPlaceholderTodoRepository::new();
    repo.create(todo).await
}

pub async fn update_todo(id: i32, todo: TodoForm) -> Result<Todo, Box<dyn Error>> {
    let repo = JsonPlaceholderTodoRepository::new();
    repo.update(id, todo).await
}

pub async fn delete_todo(id: i32) -> Result<(), Box<dyn Error>> {
    let repo = JsonPlaceholderTodoRepository::new();
    repo.delete(id).await
}

// Post functions
pub async fn fetch_posts() -> Result<Vec<Post>, Box<dyn Error>> {
    let repo = JsonPlaceholderPostRepository::new();
    repo.fetch_all().await
}

pub async fn create_post(post: PostForm) -> Result<Post, Box<dyn Error>> {
    let repo = JsonPlaceholderPostRepository::new();
    repo.create(post).await
}

pub async fn update_post(id: i32, post: PostForm) -> Result<Post, Box<dyn Error>> {
    let repo = JsonPlaceholderPostRepository::new();
    repo.update(id, post).await
}

pub async fn delete_post(id: i32) -> Result<(), Box<dyn Error>> {
    let repo = JsonPlaceholderPostRepository::new();
    repo.delete(id).await
}

// User Repository Implementation
pub struct JsonPlaceholderUserRepository {
    client: Client,
}

impl JsonPlaceholderUserRepository {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl UserRepository for JsonPlaceholderUserRepository {
    async fn fetch_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let users = self.client.get(format!("{}/users", API_BASE_URL)).send().await?.json::<Vec<User>>().await?;
        Ok(users)
    }

    async fn fetch_by_id(&self, id: i32) -> Result<User, Box<dyn Error>> {
        let user = self.client.get(format!("{}/users/{}", API_BASE_URL, id)).send().await?.json::<User>().await?;
        Ok(user)
    }

    async fn create(&self, user: UserForm) -> Result<User, Box<dyn Error>> {
        let new_user = self
            .client
            .post(format!("{}/users", API_BASE_URL))
            .json(&user)
            .send()
            .await?
            .json::<User>()
            .await?;
        Ok(new_user)
    }

    async fn update(&self, id: i32, user: UserForm) -> Result<User, Box<dyn Error>> {
        let updated_user = self
            .client
            .put(format!("{}/users/{}", API_BASE_URL, id))
            .json(&user)
            .send()
            .await?
            .json::<User>()
            .await?;
        Ok(updated_user)
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        self.client.delete(format!("{}/users/{}", API_BASE_URL, id)).send().await?;
        Ok(())
    }
}

// Todo Repository Implementation
pub struct JsonPlaceholderTodoRepository {
    client: Client,
}

impl JsonPlaceholderTodoRepository {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl TodoRepository for JsonPlaceholderTodoRepository {
    async fn fetch_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let todos = self.client.get(format!("{}/todos", API_BASE_URL)).send().await?.json::<Vec<Todo>>().await?;
        Ok(todos)
    }

    async fn fetch_by_id(&self, id: i32) -> Result<Todo, Box<dyn Error>> {
        let todo = self.client.get(format!("{}/todos/{}", API_BASE_URL, id)).send().await?.json::<Todo>().await?;
        Ok(todo)
    }

    async fn create(&self, todo: TodoForm) -> Result<Todo, Box<dyn Error>> {
        let new_todo = self
            .client
            .post(format!("{}/todos", API_BASE_URL))
            .json(&todo)
            .send()
            .await?
            .json::<Todo>()
            .await?;
        Ok(new_todo)
    }

    async fn update(&self, id: i32, todo: TodoForm) -> Result<Todo, Box<dyn Error>> {
        let updated_todo = self
            .client
            .put(format!("{}/todos/{}", API_BASE_URL, id))
            .json(&todo)
            .send()
            .await?
            .json::<Todo>()
            .await?;
        Ok(updated_todo)
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        self.client.delete(format!("{}/todos/{}", API_BASE_URL, id)).send().await?;
        Ok(())
    }
}

// Post Repository Implementation
pub struct JsonPlaceholderPostRepository {
    client: Client,
}

impl JsonPlaceholderPostRepository {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl PostRepository for JsonPlaceholderPostRepository {
    async fn fetch_all(&self) -> Result<Vec<Post>, Box<dyn Error>> {
        let posts = self.client.get(format!("{}/posts", API_BASE_URL)).send().await?.json::<Vec<Post>>().await?;
        Ok(posts)
    }

    async fn fetch_by_id(&self, id: i32) -> Result<Post, Box<dyn Error>> {
        let post = self.client.get(format!("{}/posts/{}", API_BASE_URL, id)).send().await?.json::<Post>().await?;
        Ok(post)
    }

    async fn create(&self, post: PostForm) -> Result<Post, Box<dyn Error>> {
        let new_post = self
            .client
            .post(format!("{}/posts", API_BASE_URL))
            .json(&post)
            .send()
            .await?
            .json::<Post>()
            .await?;
        Ok(new_post)
    }

    async fn update(&self, id: i32, post: PostForm) -> Result<Post, Box<dyn Error>> {
        let updated_post = self
            .client
            .put(format!("{}/posts/{}", API_BASE_URL, id))
            .json(&post)
            .send()
            .await?
            .json::<Post>()
            .await?;
        Ok(updated_post)
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        self.client.delete(format!("{}/posts/{}", API_BASE_URL, id)).send().await?;
        Ok(())
    }
}
