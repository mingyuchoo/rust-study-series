//! Web handlers module
//! Contains handlers for web routes

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Serve the index.html file
pub async fn serve_index() -> impl IntoResponse {
    let html = include_str!("../../../static/index.html");
    (StatusCode::OK, [("Content-Type", "text/html")], html)
}
