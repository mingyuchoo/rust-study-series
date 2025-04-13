use crate::models::{Post, PostForm, Todo, TodoForm, User, UserForm};
use reqwest::Client;
use std::error::Error;

const API_BASE_URL: &str = "https://jsonplaceholder.typicode.com";

// User API functions
pub async fn fetch_users() -> Result<Vec<User>, Box<dyn Error>> {
    let users = Client::new()
        .get(format!("{}/users", API_BASE_URL))
        .send()
        .await?
        .json::<Vec<User>>()
        .await?;
    Ok(users)
}

pub async fn fetch_user(id: i32) -> Result<User, Box<dyn Error>> {
    let user = Client::new()
        .get(format!("{}/users/{}", API_BASE_URL, id))
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(user)
}

pub async fn create_user(user: UserForm) -> Result<User, Box<dyn Error>> {
    let new_user = Client::new()
        .post(format!("{}/users", API_BASE_URL))
        .json(&user)
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(new_user)
}

pub async fn update_user(id: i32, user: UserForm) -> Result<User, Box<dyn Error>> {
    let updated_user = Client::new()
        .put(format!("{}/users/{}", API_BASE_URL, id))
        .json(&user)
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(updated_user)
}

pub async fn delete_user(id: i32) -> Result<(), Box<dyn Error>> {
    Client::new()
        .delete(format!("{}/users/{}", API_BASE_URL, id))
        .send()
        .await?;
    Ok(())
}

// Todo API functions
pub async fn fetch_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let todos = Client::new()
        .get(format!("{}/todos", API_BASE_URL))
        .send()
        .await?
        .json::<Vec<Todo>>()
        .await?;
    Ok(todos)
}

pub async fn fetch_todo(id: i32) -> Result<Todo, Box<dyn Error>> {
    let todo = Client::new()
        .get(format!("{}/todos/{}", API_BASE_URL, id))
        .send()
        .await?
        .json::<Todo>()
        .await?;
    Ok(todo)
}

pub async fn create_todo(todo: TodoForm) -> Result<Todo, Box<dyn Error>> {
    let new_todo = Client::new()
        .post(format!("{}/todos", API_BASE_URL))
        .json(&todo)
        .send()
        .await?
        .json::<Todo>()
        .await?;
    Ok(new_todo)
}

pub async fn update_todo(id: i32, todo: TodoForm) -> Result<Todo, Box<dyn Error>> {
    let updated_todo = Client::new()
        .put(format!("{}/todos/{}", API_BASE_URL, id))
        .json(&todo)
        .send()
        .await?
        .json::<Todo>()
        .await?;
    Ok(updated_todo)
}

pub async fn delete_todo(id: i32) -> Result<(), Box<dyn Error>> {
    Client::new()
        .delete(format!("{}/todos/{}", API_BASE_URL, id))
        .send()
        .await?;
    Ok(())
}

// Post API functions
pub async fn fetch_posts() -> Result<Vec<Post>, Box<dyn Error>> {
    let posts = Client::new()
        .get(format!("{}/posts", API_BASE_URL))
        .send()
        .await?
        .json::<Vec<Post>>()
        .await?;
    Ok(posts)
}

pub async fn fetch_post(id: i32) -> Result<Post, Box<dyn Error>> {
    let post = Client::new()
        .get(format!("{}/posts/{}", API_BASE_URL, id))
        .send()
        .await?
        .json::<Post>()
        .await?;
    Ok(post)
}

pub async fn create_post(post: PostForm) -> Result<Post, Box<dyn Error>> {
    let new_post = Client::new()
        .post(format!("{}/posts", API_BASE_URL))
        .json(&post)
        .send()
        .await?
        .json::<Post>()
        .await?;
    Ok(new_post)
}

pub async fn update_post(id: i32, post: PostForm) -> Result<Post, Box<dyn Error>> {
    let updated_post = Client::new()
        .put(format!("{}/posts/{}", API_BASE_URL, id))
        .json(&post)
        .send()
        .await?
        .json::<Post>()
        .await?;
    Ok(updated_post)
}

pub async fn delete_post(id: i32) -> Result<(), Box<dyn Error>> {
    Client::new()
        .delete(format!("{}/posts/{}", API_BASE_URL, id))
        .send()
        .await?;
    Ok(())
}
