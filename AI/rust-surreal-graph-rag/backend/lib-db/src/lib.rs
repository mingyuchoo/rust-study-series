use log::error;
use log::info;
use std::env;
use std::sync::LazyLock;
use std::time::Duration;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, RecordId, Surreal};
use tokio::time::sleep;

pub type Id = RecordId;
pub type SurrealDbError = Error;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn setup_database() -> anyhow::Result<()> {
    // 환경변수에서 접속 정보 로드(기본값 제공)
    let url = env::var("SURREALDB_URL").unwrap_or_else(|_| "localhost:8000".to_string());
    let username = env::var("SURREALDB_USERNAME").unwrap_or_else(|_| "root".to_string());
    let password = env::var("SURREALDB_PASSWORD").unwrap_or_else(|_| "root".to_string());
    let ns = env::var("SURREALDB_NS").unwrap_or_else(|_| "namespace".to_string());
    let db = env::var("SURREALDB_DB").unwrap_or_else(|_| "database".to_string());

    info!("SurrealDB 접속 시도: url={url}, ns={ns}, db={db}, user={username}");

    // 간단한 재시도 로직 (최대 5회)
    let mut last_err: Option<anyhow::Error> = None;
    for attempt in 1..=5 {
        // 1) 커넥션
        match DB.connect::<Ws>(&url).await {
            | Ok(_) => {
                info!("SurrealDB 연결 성공({attempt}/5)");
            },
            | Err(e) => {
                error!("SurrealDB 연결 실패({attempt}/5): {e}");
                last_err = Some(anyhow::Error::new(e));
                sleep(Duration::from_millis(800)).await;
                continue;
            },
        }

        // 2) 루트 서명
        if let Err(e) = DB
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await
        {
            error!("SurrealDB 서명 실패({attempt}/5): {e}");
            last_err = Some(anyhow::Error::new(e));
            sleep(Duration::from_millis(800)).await;
            continue;
        }

        // 3) 네임스페이스/DB 선택
        if let Err(e) = DB.use_ns(&ns).use_db(&db).await {
            error!("SurrealDB ns/db 선택 실패({attempt}/5): {e}");
            last_err = Some(anyhow::Error::new(e));
            sleep(Duration::from_millis(800)).await;
            continue;
        }

        info!("SurrealDB 초기화 완료");
        return Ok(());
    }

    // 모든 재시도 실패 시 마지막 에러 반환
    Err(last_err.unwrap_or_else(|| anyhow::anyhow!("SurrealDB 초기화 실패(알 수 없는 오류)")))
}
