use std::env;
use time::Duration;

pub struct AppConfig {
    pub jwt_secret: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            eprintln!("\n❌ 환경 설정 오류: JWT_SECRET 환경 변수를 찾을 수 없습니다.\n");
            eprintln!("📋 해결 방법:");
            eprintln!("   1. 프로젝트 루트 디렉토리에 .env 파일을 생성하세요.");
            eprintln!("   2. .env.example 파일을 참고하여 필요한 환경 변수를 설정하세요.");
            eprintln!("   3. 다음 명령어로 .env 파일을 생성할 수 있습니다:");
            eprintln!("      cp .env.example .env  (Linux/Mac)");
            eprintln!("      copy .env.example .env  (Windows)\n");
            eprintln!("⚠️  .env 파일에 JWT_SECRET 값을 반드시 변경하세요!\n");
            panic!("필수 환경 변수가 설정되지 않았습니다. 위의 안내를 따라 .env 파일을 설정해주세요.");
        });

        let refresh_token_ttl_days: i64 = env::var("REFRESH_TOKEN_TTL_DAYS").unwrap_or("30".to_string()).parse().unwrap_or_else(|_| {
            eprintln!("\n❌ 환경 설정 오류: REFRESH_TOKEN_TTL_DAYS는 유효한 숫자여야 합니다.\n");
            panic!("REFRESH_TOKEN_TTL_DAYS 값을 확인해주세요.");
        });

        let access_token_ttl_secs: i64 = env::var("ACCESS_TOKEN_TTL_SECS").unwrap_or("30".to_string()).parse().unwrap_or_else(|_| {
            eprintln!("\n❌ 환경 설정 오류: ACCESS_TOKEN_TTL_SECS는 유효한 숫자여야 합니다.\n");
            panic!("ACCESS_TOKEN_TTL_SECS 값을 확인해주세요.");
        });

        Self {
            jwt_secret,
            access_token_ttl: Duration::seconds(access_token_ttl_secs),
            refresh_token_ttl: Duration::days(refresh_token_ttl_days),
        }
    }
}
