# Redis Embedding Cache (Rust + Azure OpenAI)

> 모든 문서/주석은 한국어로 제공됩니다.

## 개요

- 임베딩 벡터 캐싱, 응답 캐싱, 세션 대화 기록 저장을 통해 Azure OpenAI 기반 AI 서비스의 응답 속도를 크게 향상합니다.
- Redis 인메모리 캐시를 사용해 반복 작업을 제거하고, LRU 정책과 TTL로 효율적으로 메모리를 관리합니다.

## 사용 기술

- Rust, Tokio(async)
- Redis (String/List) + Lua 스크립트 삭제
- Azure OpenAI (임베딩/챗)

## 환경 변수

- REDIS_URL (기본: redis://127.0.0.1:6379)
- CACHE_TTL (기본: 3600)
- REDIS_MAX_MEMORY (기본: 512mb)
- AZURE_OPENAI_ENDPOINT (예: <https://YOUR_RESOURCE.openai.azure.com>)
- AZURE_OPENAI_API_KEY
- AZURE_OPENAI_EMBEDDINGS_DEPLOYMENT (예: text-embedding-3-large 등 배포명)
- AZURE_OPENAI_CHAT_DEPLOYMENT (예: gpt-4o-mini 등 배포명)

## 빌드 및 실행

1) 의존성 설치/빌드

```bash
cargo build --release
```

2) Redis Docker 준비(선택, Docker 필요)

```bash
cargo run -- setup --memory 512mb
```

3) 연결 상태 확인

```bash
cargo run -- status
```

4) 데모 실행 (임베딩/응답 캐시, 세션 저장)

```bash
cargo run -- test --text "안녕하세요, Redis 임베딩 캐시입니다!"
```

5) 통계 확인

```bash
cargo run -- stats
```

6) 전체 캐시 삭제

```bash
cargo run -- clear
```

## 주요 모듈 경로

- `src/embedding_cache.rs` 기본 임베딩 캐시
- `src/compressed_cache.rs` 압축 임베딩 캐시
- `src/azure_openai.rs` Azure OpenAI REST 클라이언트
- `src/response_cache.rs` 질의-응답 캐시
- `src/session_store.rs` 세션/대화 컨텍스트 저장소
- `src/config.rs` 설정 로딩
- `src/utils.rs` Redis Docker 실행/연결 테스트
- `src/main.rs` CLI 및 데모

## 주의 사항

- Docker, 네트워크 호출은 환경에 따라 실패할 수 있습니다. 에러 메시지를 참고하여 설정을 확인하세요.
- Azure OpenAI 호출은 과금이 발생할 수 있으므로 실제 키/엔드포인트를 설정한 후 사용하세요.

---

# 🦀 Rust Redis 임베딩 캐시
Redis를 활용한 고성능 임베딩 벡터 캐싱 라이브러리입니다. AI/ML 애플리케이션에서 임베딩 생성 비용을 절약하고 응답 속도를 크게 향상시킵니다.

## ✨ 주요 특징
- 빠른 성능: 메모리 기반으로 밀리초 단위 응답
- 메모리 효율: gzip 압축으로 50% 이상 메모리 절약
- 비동기 지원: Tokio 기반 완전 비동기 처리
- 타입 안전: Rust의 타입 시스템 활용
- 배치 처리: 대량 데이터 효율적 처리
- 통계 제공: 히트율 및 성능 모니터링

## 🚀 빠른 시작
1) 자동 설치
```bash
curl -sSL https://raw.githubusercontent.com/your-repo/setup.sh | bash
```

2) 수동 설치
```bash
# 프로젝트 생성
cargo new redis-embedding-cache --bin
cd redis-embedding-cache

# 의존성 추가 (Cargo.toml 참조)
cargo add redis tokio anyhow serde bincode

# Redis 서버 시작
docker run -d -p 6379:6379 --name redis-embedding \
  redis:alpine redis-server --maxmemory 512mb --maxmemory-policy allkeys-lru
```

3) 기본 사용법
```rust
use caching_with_redis::EmbeddingCache;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 캐시 초기화
    let cache = EmbeddingCache::new("redis://127.0.0.1:6379", 3600).await?;

    // 임베딩 함수 정의
    let compute_embedding = |text: String| async move {
        // 실제로는 OpenAI API나 로컬 모델 호출
        Ok(vec![0.1, 0.2, 0.3, 0.4, 0.5])
    };

    // 캐시를 통한 임베딩 조회/계산
    let embedding = cache
        .get_or_compute(
            "안녕하세요, Redis 캐시입니다!",
            "test-model",
            compute_embedding,
        )
        .await?;

    println!("임베딩 차원: {}", embedding.len());
    Ok(())
}
```

## 📊 성능 비교(예시)
| 방식 | 첫 호출 | 캐시 히트 | 속도 향상 |
|---|---:|---:|---:|
| OpenAI API | ~200ms | ~2ms | 100x |
| 로컬 모델 | ~50ms | ~1ms | 50x |
| 압축 캐시 | ~60ms | ~3ms | 20x |

## 🎯 사용 사례
### OpenAI API 통합
```rust
use caching_with_redis::EmbeddingCache;
use reqwest::Client;
use serde_json::json;
use anyhow::Result;

struct OpenAIClient {
    client: Client,
    api_key: String,
    cache: EmbeddingCache,
}

impl OpenAIClient {
    async fn get_embedding_cached(&self, text: &str) -> Result<Vec<f32>> {
        let embedding_func = |text: String| async {
            // OpenAI API 호출(예시)
            let _resp = self.client
                .post("https://api.openai.com/v1/embeddings")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&json!({"model": "text-embedding-ada-002", "input": text}))
                .send()
                .await?;
            // 실제 응답 파싱은 생략하고, 예시 벡터 반환
            Ok::<_, anyhow::Error>(vec![0.1f32; 1536])
        };
        self.cache.get_or_compute(text, "openai-ada-002", embedding_func).await
    }
}
```

### RAG 시스템
```rust
struct RAGPipeline {
    cache: caching_with_redis::EmbeddingCache,
    documents: Vec<Document>,
}

impl RAGPipeline {
    async fn search(&self, query: &str, top_k: usize) -> anyhow::Result<Vec<SearchResult>> {
        // 쿼리 임베딩 (캐시 활용)
        let query_embedding = self
            .cache
            .get_or_compute(query, "sentence-transformer", |text| async move {
                Ok::<Vec<f32>, anyhow::Error>(vec![0.0; 384])
            })
            .await?;
        // 유사도 검색(예시)
        let results = vec![]; // self.vector_search(query_embedding, top_k)
        Ok(results)
    }
}
```

### 배치 처리
```rust
async fn process_documents(
    cache: &caching_with_redis::EmbeddingCache,
    texts: Vec<String>,
) -> anyhow::Result<Vec<Vec<f32>>> {
    let batch_compute = |batch: Vec<String>| async move {
        // 배치로 임베딩 계산(예시)
        Ok::<Vec<Vec<f32>>, anyhow::Error>(batch.into_iter().map(|_| vec![0.0f32; 384]).collect())
    };
    cache.get_or_compute_batch(&texts, "sentence-transformer", batch_compute).await
}
```

## ⚙️ 설정 옵션
### 환경 변수
```bash
export REDIS_URL="redis://127.0.0.1:6379"
export CACHE_TTL=3600                    # TTL (초)
export REDIS_MAX_MEMORY="512mb"          # 최대 메모리
export OPENAI_API_KEY="sk-..."           # OpenAI API 키
```

### Redis 최적화
```conf
maxmemory 512mb
maxmemory-policy allkeys-lru
save ""
appendonly no
timeout 300
```

## 🛠️ CLI 도구
```bash
# Docker로 빠르게 상태 확인
pwsh ./scripts/setup.ps1 up
# 또는
./scripts/setup.sh up

# 상태 확인
cargo run -- status

# 캐시 통계
cargo run -- stats

# 테스트 실행
cargo run -- test --text "테스트 문장"

# 캐시 삭제
cargo run -- clear
```

## 📈 모니터링
```rust
let stats = cache.get_stats().await;
println!("히트율: {:.1}%", stats.hit_rate() * 100.0);
println!("총 키: {}", stats.total_keys);

// 압축 통계
let compression_stats = compressed_cache.get_compression_stats().await?;
println!("압축률: {:.2}x", compression_stats.compression_ratio);
println!("메모리 절약: {:.1}%", compression_stats.memory_saved_percent());
```

## 🐳 Docker 배포
현재 리포지토리에는 `Dockerfile`, `docker-compose.yml`이 포함되어 있습니다.
```yaml
version: '3.8'
services:
  redis:
    image: redis:7-alpine
    command: >
      redis-server --maxmemory 512mb --maxmemory-policy allkeys-lru --save "" --appendonly no --timeout 300 --tcp-keepalive 60
    ports: ["6379:6379"]
    volumes: ["redis_data:/data"]
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 30s
      timeout: 10s
      retries: 3

  embedding-cache:
    build: .
    image: caching_with_redis:latest
    depends_on:
      redis:
        condition: service_healthy
    environment:
      - REDIS_URL=redis://redis:6379
      - CACHE_TTL=3600
      - RUST_LOG=info
    restart: unless-stopped
    command: ["status"]

  redis-commander:
    image: rediscommander/redis-commander:latest
    environment:
      - REDIS_HOSTS=local:redis:6379
    ports: ["8081:8081"]

volumes:
  redis_data: { driver: local }
```

## 🧪 테스트 및 예제 실행
```bash
# 단위 테스트
cargo test

# 예제 실행
cargo run --example openai_integration
cargo run --example rag_pipeline
cargo run --example performance_comparison
```

## 🤝 기여하기
1) Fork the project
2) Create your feature branch (git checkout -b feature/amazing-feature)
3) Commit your changes (git commit -m 'Add some amazing feature')
4) Push to the branch (git push origin feature/amazing-feature)
5) Open a Pull Request

## 📄 라이선스
이 프로젝트는 MIT 라이선스 하에 있습니다. LICENSE 파일을 참조하세요.

## 🔗 관련 링크
- Redis 공식 문서
- Tokio 비동기 런타임
- OpenAI API 문서
- Sentence Transformers

## ❓ FAQ
Q: 어떤 임베딩 모델을 지원하나요?
A: 모든 임베딩 모델을 지원합니다. 반환값이 Vec<f32> 형태면 캐시할 수 있습니다.

Q: 메모리 사용량은 어느 정도인가요?
A: OpenAI 임베딩 기준으로 1000개당 약 6MB (압축 시 더 절약) 사용합니다.

Q: 프로덕션에서 사용해도 되나요?
A: 네, 타입 안전성과 에러 처리가 잘 되어있어 프로덕션 환경에 적합합니다.

Q: 다른 언어 바인딩이 있나요?
A: 현재는 Rust만 지원하지만, FFI를 통해 다른 언어에서도 사용 가능합니다.
