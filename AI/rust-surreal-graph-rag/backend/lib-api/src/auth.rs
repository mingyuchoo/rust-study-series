//! 인증 관련 핸들러 (JWT 기반)
//! 모든 주석은 한국어로 작성됩니다.

use actix_web::{post, get, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::error::Error;
use crate::models::{LoginRequest, LoginResponse, RefreshResponse, MessageResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    r#type: String, // "access" | "refresh"
}

fn create_token(email: &str, ttl_secs: i64, token_type: &str, secret: &str) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::seconds(ttl_secs)).timestamp();
    let claims = Claims { sub: email.to_string(), exp, r#type: token_type.to_string() };
    let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;
    Ok(token)
}

fn verify_token(token: &str, expected_type: &str, secret: &str) -> anyhow::Result<Claims> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256))?;
    if data.claims.r#type != expected_type { anyhow::bail!("잘못된 토큰 타입"); }
    Ok(data.claims)
}

pub fn require_auth(req: &HttpRequest, cfg: &AppConfig) -> std::result::Result<String, Error> {
    let auth = req.headers().get("Authorization").and_then(|v| v.to_str().ok()).unwrap_or("");
    if !auth.starts_with("Bearer ") { return Err(Error::Unauthorized); }
    let token = &auth[7..];
    let claims = verify_token(token, "access", &cfg.jwt_secret).map_err(|_| Error::Unauthorized)?;
    Ok(claims.sub)
}

#[post("/api/auth/login")]
pub async fn login(cfg: web::Data<AppConfig>, payload: web::Json<LoginRequest>) -> Result<web::Json<LoginResponse>, Error> {
    // MVP: 데모용으로 비밀번호 검증을 단순화(실서비스에서는 해시/DB사용)
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::BadRequest("이메일/비밀번호가 비어 있습니다".into()));
    }
    // 데모: 모든 사용자 허용
    let access = create_token(&payload.email, cfg.access_token_ttl_secs, "access", &cfg.jwt_secret)
        .map_err(|e| Error::External(e.to_string()))?;
    let refresh_token = create_token(&payload.email, cfg.refresh_token_ttl_secs, "refresh", &cfg.jwt_secret)
        .map_err(|e| Error::External(e.to_string()))?;

    Ok(web::Json(LoginResponse {
        access_token: access,
        refresh_token: refresh_token,
        user_id: format!("user:{}", payload.email),
        expires_in: cfg.access_token_ttl_secs,
    }))
}

#[post("/api/auth/refresh")]
pub async fn refresh(cfg: web::Data<AppConfig>, req: HttpRequest) -> Result<web::Json<RefreshResponse>, Error> {
    let auth = req.headers().get("Authorization").and_then(|v| v.to_str().ok()).unwrap_or("");
    if !auth.starts_with("Bearer ") { return Err(Error::Unauthorized); }
    let token = &auth[7..];
    let claims = verify_token(token, "refresh", &cfg.jwt_secret).map_err(|_| Error::Unauthorized)?;

    let access = create_token(&claims.sub, cfg.access_token_ttl_secs, "access", &cfg.jwt_secret)
        .map_err(|e| Error::External(e.to_string()))?;
    Ok(web::Json(RefreshResponse { access_token: access, expires_in: cfg.access_token_ttl_secs }))
}

#[post("/api/auth/logout")]
pub async fn logout(_cfg: web::Data<AppConfig>, _req: HttpRequest) -> Result<web::Json<MessageResponse>, Error> {
    // MVP: 서버측 상태 저장을 하지 않으므로 단순 성공 반환
    Ok(web::Json(MessageResponse { message: "Successfully logged out".into() }))
}

#[get("/api/auth/me")]
pub async fn me(cfg: web::Data<AppConfig>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let sub = require_auth(&req, &cfg)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "email": sub })))
}
