use crate::{error::AppError,
            DB};
use actix_web::{get,
                web::Json};

#[get("/session")]
pub async fn session() -> Result<Json<String>, AppError> {
    let res: Option<String> = DB.query("RETURN <string>$session")
                                .await?
                                .take(0)?;
    Ok(Json(res.unwrap_or("No session data found!".into())))
}
