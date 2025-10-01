use anyhow::Result;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;

/// 데이터베이스 연결 풀 생성
pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    // SQLite 파일 경로 추출 및 디렉터리 생성
    let final_url = if let Some(file_path) = database_url.strip_prefix("sqlite://").or_else(|| database_url.strip_prefix("sqlite:")) {
        let path = Path::new(file_path);

        // 절대 경로로 변환 및 정규화
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        // 경로 정규화 (canonicalize는 파일이 존재해야 하므로 수동으로 처리)
        let normalized = abs_path.components().fold(std::path::PathBuf::new(), |mut acc, component| {
            match component {
                | std::path::Component::CurDir => {},
                | _ => acc.push(component),
            }
            acc
        });

        // 부모 디렉터리가 있으면 생성
        if let Some(parent) = normalized.parent() {
            std::fs::create_dir_all(parent)?;
            println!("📁 디렉터리 생성: {:?}", parent);
        }

        // 절대 경로로 URL 재구성 (Windows에서는 슬래시로 변환)
        let path_str = normalized.to_string_lossy().replace('\\', "/");
        // mode=rwc: 읽기/쓰기 모드, 파일이 없으면 생성
        format!("sqlite://{}?mode=rwc", path_str)
    } else {
        database_url.to_string()
    };

    println!("🔗 데이터베이스 URL: {}", final_url);

    let pool = SqlitePoolOptions::new().max_connections(5).connect(&final_url).await?;

    Ok(pool)
}

/// 데이터베이스 초기화 (테이블 생성)
pub async fn initialize_database(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS embeddings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            vector BLOB NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// 샘플 데이터 생성
pub async fn seed_sample_data(pool: &SqlitePool, samples: Vec<(String, Vec<f32>)>) -> Result<()> {
    // 기존 데이터가 있는지 확인
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM embeddings").fetch_one(pool).await?;

    if count.0 > 0 {
        println!("샘플 데이터가 이미 존재합니다. 건너뜁니다.");
        return Ok(());
    }

    // 샘플 데이터 삽입
    for (text, vector) in samples {
        let vector_bytes = vector_to_bytes(&vector);
        let created_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO embeddings (text, vector, created_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(text)
        .bind(vector_bytes)
        .bind(created_at)
        .execute(pool)
        .await?;
    }

    println!("샘플 데이터 생성 완료");
    Ok(())
}

/// 벡터를 바이트로 변환
pub fn vector_to_bytes(vector: &[f32]) -> Vec<u8> { vector.iter().flat_map(|&f| f.to_le_bytes()).collect() }

/// 바이트를 벡터로 변환
pub fn bytes_to_vector(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}
