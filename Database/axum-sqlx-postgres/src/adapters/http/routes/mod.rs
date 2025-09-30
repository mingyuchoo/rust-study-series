pub mod user;

use crate::adapters::http::app_state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> { Router::new().nest("/user", user::router()) }
