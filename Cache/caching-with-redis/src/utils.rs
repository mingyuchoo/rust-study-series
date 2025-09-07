// Docker 기반 Redis 실행 및 연결 테스트 유틸
use anyhow::Result;
use std::process::Command;

pub fn setup_redis_docker(max_memory: &str) -> Result<()> {
    println!("🚀 Redis Docker 컨테이너 시작...");

    // 기존 컨테이너 중지 및 제거 (실패해도 무시)
    let _ = Command::new("docker")
        .args(["stop", "redis-embedding"])
        .output();
    let _ = Command::new("docker")
        .args(["rm", "redis-embedding"])
        .output();

    // 새 Redis 컨테이너 시작 (allkeys-lru, appendonly 비활성, RDB 저장 없음)
    let output = Command::new("docker")
        .args([
            "run",
            "-d",
            "--name",
            "redis-embedding",
            "--restart",
            "unless-stopped",
            "-p",
            "6379:6379",
            "redis:alpine",
            "redis-server",
            "--maxmemory",
            max_memory,
            "--maxmemory-policy",
            "allkeys-lru",
            "--save",
            "",
            "--appendonly",
            "no",
            "--timeout",
            "300",
        ])
        .output()?;

    if output.status.success() {
        println!("✅ Redis 컨테이너가 성공적으로 시작되었습니다!");
        std::thread::sleep(std::time::Duration::from_secs(2));
        Ok(())
    } else {
        anyhow::bail!(
            "Redis 컨테이너 시작 실패: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub async fn test_redis_connection(redis_url: &str) -> Result<()> {
    use redis::AsyncCommands;

    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_async_connection().await?;

    // ping 테스트
    let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
    if pong.to_uppercase() != "PONG" {
        anyhow::bail!("PING 실패: {}", pong);
    }

    // 간단한 set/get
    conn.set::<_, _, ()>("test_key", "test_value").await?;
    let result: String = conn.get("test_key").await?;
    let _: () = conn.del::<_, ()>("test_key").await?;

    if result == "test_value" {
        println!("✅ Redis 연결 및 테스트 성공!");
        Ok(())
    } else {
        anyhow::bail!("Redis 테스트 실패");
    }
}
