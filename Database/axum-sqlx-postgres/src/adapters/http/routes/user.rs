use crate::adapters::http::app_state::AppState;
use crate::app_error::AppResult;
use crate::use_cases::user::UserUseCases;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, instrument};

pub fn router() -> Router<AppState> { Router::new().route("/register", post(register)) }

#[derive(Debug, Clone, Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: SecretString,
}

#[derive(Debug, Clone, Serialize)]
struct RegisterResponse {
    success: bool,
}

/// Creates a new user based on the submitted credentials.
#[instrument(skip(user_use_cases))]
async fn register(State(user_use_cases): State<Arc<UserUseCases>>, Json(payload): Json<RegisterPayload>) -> AppResult<impl IntoResponse> {
    info!("Register user called");
    user_use_cases.add(&payload.username, &payload.email, &payload.password).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            success: true,
        }),
    ))
}
