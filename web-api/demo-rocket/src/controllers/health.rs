use crate::controllers::GenericResponse;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;

type HealthResult<T, E> = Result<T, E>;

#[get("/health")]
pub async fn health() -> HealthResult<Json<GenericResponse>, Status> {
    let response = GenericResponse { status:  "success".to_string(),
                                     message: "I'm healthy.".to_string(), };
    Ok(Json(response))
}
