use rocket::{get, http::Status, serde::json::Json};

use crate::controllers::GenericResponse;

type HealthResult<T, E> = Result<T, E>;

#[get("/health")]
pub async fn health() -> HealthResult<Json<GenericResponse>, Status> {
    let response = GenericResponse {
        status: "success".into(),
        message: "I'm healthy.".into(),
    };
    Ok(Json(response))
}
