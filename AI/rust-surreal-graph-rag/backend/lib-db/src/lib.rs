use log::error;
use log::info;
use std::env;
use std::sync::LazyLock;
use std::time::Duration;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::types::RecordId;
use surrealdb::{Error, Surreal};
use tokio::time::sleep;

pub type Id = RecordId;
pub type SurrealDbError = Error;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn setup_database() -> anyhow::Result<()> {
    let url = env::var("SURREALDB_URL").unwrap_or_else(|_| "localhost:8000".to_string());
    let username = env::var("SURREALDB_USERNAME").unwrap_or_else(|_| "root".to_string());
    let password = env::var("SURREALDB_PASSWORD").unwrap_or_else(|_| "root".to_string());
    let ns = env::var("SURREALDB_NS").unwrap_or_else(|_| "namespace".to_string());
    let db = env::var("SURREALDB_DB").unwrap_or_else(|_| "database".to_string());

    info!("SurrealDB 접속 시도: url={url}, ns={ns}, db={db}, user={username}");

    // 1) 커넥션 — LazyLock<Surreal<Client>>는 한 번만 connect 가능
    let mut last_err: Option<anyhow::Error> = None;
    for attempt in 1..=5 {
        match DB.connect::<Ws>(&url).await {
            | Ok(_) => {
                info!("SurrealDB 연결 성공({attempt}/5)");
                last_err = None;
                break;
            },
            | Err(e) => {
                error!("SurrealDB 연결 실패({attempt}/5): {e}");
                last_err = Some(anyhow::Error::new(e));
                sleep(Duration::from_millis(800)).await;
            },
        }
    }
    if let Some(e) = last_err {
        return Err(e);
    }

    // 2) 인증 및 네임스페이스/DB 선택 — 연결 이후 재시도
    let mut last_err: Option<anyhow::Error> = None;
    for attempt in 1..=5 {
        if let Err(e) = DB
            .signin(Root {
                username: username.clone(),
                password: password.clone(),
            })
            .await
        {
            error!("SurrealDB 서명 실패({attempt}/5): {e}");
            last_err = Some(anyhow::Error::new(e));
            sleep(Duration::from_millis(800)).await;
            continue;
        }

        if let Err(e) = DB.use_ns(&ns).use_db(&db).await {
            error!("SurrealDB ns/db 선택 실패({attempt}/5): {e}");
            last_err = Some(anyhow::Error::new(e));
            sleep(Duration::from_millis(800)).await;
            continue;
        }

        info!("SurrealDB 초기화 완료");

        // 스키마 파일이 있으면 적용
        let schema_path = env::var("SURREALDB_SCHEMA_PATH").unwrap_or_else(|_| "schema.surql".to_string());
        if let Ok(schema) = std::fs::read_to_string(&schema_path) {
            info!("스키마 적용 중: {schema_path}");
            if let Err(e) = DB.query(&schema).await {
                error!("스키마 적용 실패: {e}");
            } else {
                info!("스키마 적용 완료");
            }
        } else {
            info!("스키마 파일 없음(건너뜀): {schema_path}");
        }

        return Ok(());
    }

    Err(last_err.unwrap_or_else(|| anyhow::anyhow!("SurrealDB 초기화 실패(알 수 없는 오류)")))
}
