use anyhow::{Result, anyhow};
use azure_foundry_embedding_demo::application::ports::EmbeddingServicePort;
use azure_foundry_embedding_demo::*;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // .env 파일 로드
    let _ = dotenv();

    // 환경 변수 읽기
    let endpoint = env::var("AZURE_OPENAI_ENDPOINT").map_err(|_| anyhow!("AZURE_OPENAI_ENDPOINT 환경 변수가 설정되지 않았습니다"))?;
    let api_key = env::var("AZURE_OPENAI_API_KEY").map_err(|_| anyhow!("AZURE_OPENAI_API_KEY 환경 변수가 설정되지 않았습니다"))?;
    let deployment_name = env::var("AZURE_OPENAI_DEPLOYMENT_NAME").unwrap_or_else(|_| "text-embedding-3-large".to_string());
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./data/embeddings.db".to_string());
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());

    println!("🚀 서버 시작 중...");

    // 데이터베이스 연결
    println!("📦 데이터베이스 연결 중...");
    let pool = database::create_pool(&database_url).await?;

    // 데이터베이스 초기화
    println!("🔧 데이터베이스 초기화 중...");
    database::initialize_database(&pool).await?;

    // 의존성 주입 설정
    let embedding_service = Arc::new(AzureEmbeddingService::new(endpoint, api_key, deployment_name));
    let embedding_repository = Arc::new(SqliteEmbeddingRepository::new(pool.clone()));

    // 샘플 데이터 생성
    println!("🌱 샘플 데이터 생성 중...");
    let sample_texts = vec![
        "안녕하세요, 오늘 날씨가 참 좋네요.".to_string(),
        "Hello, the weather is really nice today.".to_string(),
        "프로그래밍은 창의적이고 흥미로운 활동입니다.".to_string(),
        "Rust는 안전하고 빠른 시스템 프로그래밍 언어입니다.".to_string(),
        "Clean Architecture는 소프트웨어 설계 원칙입니다.".to_string(),
    ];

    match embedding_service.generate_embeddings(sample_texts.clone()).await {
        | Ok(vectors) => {
            let samples: Vec<(String, Vec<f32>)> = sample_texts.into_iter().zip(vectors.into_iter()).collect();
            database::seed_sample_data(&pool, samples).await?;
        },
        | Err(e) => {
            eprintln!("⚠️  샘플 데이터 생성 실패: {}", e);
            eprintln!("   서버는 계속 실행됩니다.");
        },
    }

    // 유스케이스 생성
    let create_embedding_usecase = Arc::new(CreateEmbeddingUseCase::new(embedding_service.clone(), embedding_repository.clone()));
    let search_similar_usecase = Arc::new(SearchSimilarEmbeddingsUseCase::new(embedding_service.clone(), embedding_repository.clone()));
    let get_embedding_usecase = Arc::new(GetEmbeddingUseCase::new(embedding_repository.clone()));
    let delete_embedding_usecase = Arc::new(DeleteEmbeddingUseCase::new(embedding_repository.clone()));

    // HTTP 서버 설정
    let app_state = AppState {
        create_embedding_usecase,
        search_similar_usecase,
        get_embedding_usecase,
        delete_embedding_usecase,
    };

    let app = create_router(app_state);

    // 서버 시작
    let addr = format!("{}:{}", server_host, server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("✅ 서버가 http://{} 에서 실행 중입니다", addr);
    println!("📚 API 엔드포인트:");
    println!("   GET    /health");
    println!("   POST   /embeddings");
    println!("   POST   /embeddings/batch");
    println!("   POST   /embeddings/search");
    println!("   GET    /embeddings");
    println!("   GET    /embeddings/:id");
    println!("   DELETE /embeddings/:id");

    axum::serve(listener, app).await?;

    Ok(())
}
