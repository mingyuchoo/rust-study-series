use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use tonic::{Request, Response, Status};
use tracing::{info, instrument};

pub mod proto {
    tonic::include_proto!("auth");
}

use proto::auth_service_server::AuthService;
use proto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};

// ─── DB structs ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct NewUser {
    user_id: String,
    username: String,
    email: String,
    password_hash: String,
}

#[derive(Debug, Deserialize)]
struct UserRecord {
    user_id: String,
    #[allow(dead_code)]
    username: String,
    #[allow(dead_code)]
    email: String,
    password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewSession {
    user_id: String,
    token: String,
}

// ─── AuthDb ───────────────────────────────────────────────────────────────────

pub struct AuthDb {
    pub db: Surreal<Db>,
}

impl AuthDb {
    pub async fn new() -> anyhow::Result<Self> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("greeting").use_db("auth").await?;

        // Schema definition
        db.query(
            "
            DEFINE TABLE user SCHEMAFULL;
            DEFINE FIELD user_id       ON user TYPE string;
            DEFINE FIELD username      ON user TYPE string;
            DEFINE FIELD email         ON user TYPE string;
            DEFINE FIELD password_hash ON user TYPE string;
            DEFINE INDEX user_email    ON user FIELDS email UNIQUE;

            DEFINE TABLE session SCHEMAFULL;
            DEFINE FIELD user_id ON session TYPE string;
            DEFINE FIELD token   ON session TYPE string;
            ",
        )
        .await?;

        Ok(Self { db })
    }
}

// ─── AuthServiceImpl ──────────────────────────────────────────────────────────

pub struct AuthServiceImpl {
    db: Arc<AuthDb>,
}

impl AuthServiceImpl {
    pub fn new(db: Arc<AuthDb>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    /// 회원가입
    #[instrument(skip(self))]
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        info!("Register request for email: {}", req.email);

        // 이메일 중복 확인
        let mut result = self
            .db
            .db
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", req.email.clone()))
            .await
            .map_err(|e| Status::internal(format!("DB query error: {e}")))?;

        let existing: Vec<UserRecord> = result
            .take(0)
            .map_err(|e| Status::internal(format!("DB result error: {e}")))?;

        if !existing.is_empty() {
            return Ok(Response::new(RegisterResponse {
                success: false,
                message: "이미 등록된 이메일입니다.".to_string(),
                user_id: String::new(),
            }));
        }

        // 비밀번호 해시 (블로킹 작업 → 스레드 풀)
        let password = req.password.clone();
        let password_hash = tokio::task::spawn_blocking(move || {
            bcrypt::hash(password, bcrypt::DEFAULT_COST)
        })
        .await
        .map_err(|e| Status::internal(format!("Task join error: {e}")))?
        .map_err(|e| Status::internal(format!("Hash error: {e}")))?;

        // 사용자 생성
        let user_id = uuid::Uuid::new_v4().to_string();
        let _: Option<NewUser> = self
            .db
            .db
            .create(("user", &user_id))
            .content(NewUser {
                user_id: user_id.clone(),
                username: req.username,
                email: req.email,
                password_hash,
            })
            .await
            .map_err(|e| Status::internal(format!("DB create error: {e}")))?;

        info!("User registered with id: {}", user_id);
        Ok(Response::new(RegisterResponse {
            success: true,
            message: "회원가입이 완료되었습니다.".to_string(),
            user_id,
        }))
    }

    /// 로그인
    #[instrument(skip(self))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        info!("Login request for email: {}", req.email);

        // 이메일로 사용자 조회
        let mut result = self
            .db
            .db
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", req.email.clone()))
            .await
            .map_err(|e| Status::internal(format!("DB query error: {e}")))?;

        let users: Vec<UserRecord> = result
            .take(0)
            .map_err(|e| Status::internal(format!("DB result error: {e}")))?;

        let user = match users.into_iter().next() {
            Some(u) => u,
            None => {
                return Ok(Response::new(LoginResponse {
                    success: false,
                    message: "이메일 또는 비밀번호가 올바르지 않습니다.".to_string(),
                    token: String::new(),
                    user_id: String::new(),
                }));
            }
        };

        // 비밀번호 검증 (블로킹 작업 → 스레드 풀)
        let password = req.password.clone();
        let hash = user.password_hash.clone();
        let valid = tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash))
            .await
            .map_err(|e| Status::internal(format!("Task join error: {e}")))?
            .map_err(|e| Status::internal(format!("Verify error: {e}")))?;

        if !valid {
            return Ok(Response::new(LoginResponse {
                success: false,
                message: "이메일 또는 비밀번호가 올바르지 않습니다.".to_string(),
                token: String::new(),
                user_id: String::new(),
            }));
        }

        // 세션 토큰 생성 및 저장
        let token = uuid::Uuid::new_v4().to_string();
        let _: Option<NewSession> = self
            .db
            .db
            .create(("session", &token))
            .content(NewSession {
                user_id: user.user_id.clone(),
                token: token.clone(),
            })
            .await
            .map_err(|e| Status::internal(format!("DB create error: {e}")))?;

        info!("User logged in: {}", user.user_id);
        Ok(Response::new(LoginResponse {
            success: true,
            message: "로그인에 성공하였습니다.".to_string(),
            token,
            user_id: user.user_id,
        }))
    }
}
