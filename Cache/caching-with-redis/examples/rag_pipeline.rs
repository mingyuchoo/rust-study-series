// RAG 파이프라인 + 캐시 데모
use anyhow::Result;
use caching_with_redis::EmbeddingCache;
use std::cmp::Ordering;

struct Document {
    id: String,
    content: String,
    embedding: Option<Vec<f32>>,
}

struct RAGPipeline {
    cache: EmbeddingCache,
    documents: Vec<Document>,
}

impl RAGPipeline {
    async fn new(redis_url: &str) -> Result<Self> {
        let cache = EmbeddingCache::new(redis_url, 7200).await?; // 2시간 캐시
        Ok(Self {
            cache,
            documents: Vec::new(),
        })
    }

    async fn add_document(&mut self, id: String, content: String) -> Result<()> {
        let embedding_func = |text: String| async move {
            // 실제 임베딩 계산 대신 더미 딜레이 + 규칙성 있는 벡터 생성
            tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
            Ok::<Vec<f32>, anyhow::Error>(
                (0..384)
                    .map(|i| (i as f32 * 0.001) + text.len() as f32 * 0.01)
                    .collect(),
            )
        };
        let embedding = self
            .cache
            .get_or_compute(&content, "sentence-transformer", embedding_func)
            .await?;
        self.documents.push(Document {
            id,
            content,
            embedding: Some(embedding),
        });
        Ok(())
    }

    async fn search(&self, query: &str, top_k: usize) -> Result<Vec<(String, f32)>> {
        let query_embedding_func = |text: String| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
            Ok::<Vec<f32>, anyhow::Error>(
                (0..384)
                    .map(|i| (i as f32 * 0.001) + text.len() as f32 * 0.01)
                    .collect(),
            )
        };
        let query_embedding = self
            .cache
            .get_or_compute(query, "sentence-transformer", query_embedding_func)
            .await?;

        let mut scores: Vec<(String, f32)> = Vec::new();
        for doc in &self.documents {
            if let Some(ref doc_emb) = doc.embedding {
                let sim = cosine_similarity(&query_embedding, doc_emb);
                scores.push((doc.id.clone(), sim));
            }
        }
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        scores.truncate(top_k);
        Ok(scores)
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (na * nb)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 RAG 파이프라인 임베딩 캐시 데모\n=====================================");
    let mut rag = RAGPipeline::new("redis://127.0.0.1:6379").await?;

    let documents = vec![
        (
            "doc1",
            "Rust는 메모리 안전성과 성능을 보장하는 시스템 프로그래밍 언어입니다",
        ),
        (
            "doc2",
            "Redis는 고성능 인메모리 데이터베이스로 캐싱에 자주 사용됩니다",
        ),
        (
            "doc3",
            "임베딩 벡터는 자연어 처리에서 텍스트를 수치화하는 방법입니다",
        ),
        (
            "doc4",
            "RAG는 검색 증강 생성으로 외부 지식을 활용한 AI 기법입니다",
        ),
        (
            "doc5",
            "캐싱은 반복적인 연산을 줄여 애플리케이션 성능을 크게 향상시킵니다",
        ),
    ];

    println!("\n📚 문서 인덱싱 중...");
    for (id, content) in documents {
        rag.add_document(id.to_string(), content.to_string())
            .await?;
    }

    let queries = vec![
        "프로그래밍 언어에 대해 알려줘",
        "데이터베이스 성능 최적화",
        "AI 기법과 자연어 처리",
    ];

    println!("\n🔍 검색 테스트:");
    for query in queries {
        println!("\n쿼리: \"{}\"", query);
        let results = rag.search(query, 3).await?;
        for (i, (doc_id, score)) in results.iter().enumerate() {
            println!("  {}. {} (유사도: {:.3})", i + 1, doc_id, score);
        }
    }

    let stats = rag.cache.get_stats().await;
    println!(
        "\n📊 최종 캐시 통계:\n  총 키: {}\n  히트: {}\n  미스: {}\n  히트율: {:.1}%",
        stats.total_keys,
        stats.hit_count,
        stats.miss_count,
        stats.hit_rate() * 100.0
    );
    Ok(())
}
