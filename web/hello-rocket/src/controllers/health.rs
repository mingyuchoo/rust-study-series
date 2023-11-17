use rocket::{get, http::Status, serde::json::Json};

use crate::models::GenericResponse;

type HealthResult<T, E> = std::result::Result<Json<T>, E>;

#[get("/health")]
pub async fn health() -> HealthResult<GenericResponse, Status> {
    let response = GenericResponse {
        status: "success".to_owned(),
        message: "I'm healthy.".to_owned(),
    };
    Ok(Json(response))
}
