use crate::models::{_entities::users, users::RegisterParams};
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub pid: String,
    pub email: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&users::Model> for UserResponse {
    fn from(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            email: user.email.clone(),
            name: user.name.clone(),
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserParams {
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

/// Get all users
#[debug_handler]
async fn index(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let users = users::Entity::find().all(&ctx.db).await?;
    let response: Vec<UserResponse> = users.iter().map(UserResponse::from).collect();
    format::json(response)
}

/// Get a specific user by ID
#[debug_handler]
async fn show(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &id).await?;
    format::json(UserResponse::from(&user))
}

/// Create a new user
#[debug_handler]
async fn create(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let user = users::Model::create_with_password(&ctx.db, &params).await?;
    
    // Automatically verify the user since we're creating it from the admin panel
    let user = user
        .into_active_model()
        .verified(&ctx.db)
        .await?;
    
    format::json(UserResponse::from(&user))
}

/// Update an existing user
#[debug_handler]
async fn update(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<String>,
    Json(params): Json<UpdateUserParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &id).await?;
    let mut user_active = user.into_active_model();
    
    if let Some(email) = params.email {
        user_active.email = sea_orm::ActiveValue::Set(email);
    }
    
    if let Some(name) = params.name {
        user_active.name = sea_orm::ActiveValue::Set(name);
    }
    
    // If password is provided, update it
    if let Some(password) = params.password {
        user_active = user_active.reset_password(&ctx.db, &password).await?.into_active_model();
    } else {
        // Otherwise just update the other fields
        user_active = user_active.update(&ctx.db).await?.into_active_model();
    }
    
    let updated_user = user_active.update(&ctx.db).await?;
    format::json(UserResponse::from(&updated_user))
}

/// Delete a user
#[debug_handler]
async fn delete(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<String>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &id).await?;
    user.delete(&ctx.db).await?;
    format::empty_json()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/users")
        .add("/", axum::routing::get(index))
        .add("/", axum::routing::post(create))
        .add("/{id}", axum::routing::get(show))
        .add("/{id}", axum::routing::put(update))
        .add("/{id}", axum::routing::delete(delete))
}
