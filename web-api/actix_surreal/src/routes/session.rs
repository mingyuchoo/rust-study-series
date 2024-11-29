use crate::routes::ServerError;
use actix_surreal::db::DB;
use actix_web::get;
use actix_web::web::Json;

#[get("/session")]
pub async fn session() -> Result<Json<String>, ServerError> {
    let res: Option<String> = DB.query("RETURN <string>$session")
                                .await?
                                .take(0)?;
    Ok(Json(res.unwrap_or("No session data found!".into())))
}
