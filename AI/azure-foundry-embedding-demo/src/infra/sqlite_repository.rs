use crate::application::ports::EmbeddingRepositoryPort;
use crate::domain::entities::Embedding;
use crate::domain::value_objects::SimilarityResult;
use crate::infra::database::{bytes_to_vector, vector_to_bytes};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

/// SQLite 임베딩 저장소 구현
pub struct SqliteEmbeddingRepository {
    pool: SqlitePool,
}

impl SqliteEmbeddingRepository {
    /// 새로운 저장소 생성
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
        }
    }
}

#[async_trait]
impl EmbeddingRepositoryPort for SqliteEmbeddingRepository {
    async fn save(&self, text: String, vector: Vec<f32>) -> Result<Embedding> {
        let vector_bytes = vector_to_bytes(&vector);
        let created_at = Utc::now();
        let created_at_str = created_at.to_rfc3339();

        let result = sqlx::query(
            r#"
            INSERT INTO embeddings (text, vector, created_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&text)
        .bind(&vector_bytes)
        .bind(&created_at_str)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid();

        Ok(Embedding {
            id,
            text,
            vector,
            created_at,
        })
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Embedding>> {
        let row: Option<(i64, String, Vec<u8>, String)> = sqlx::query_as(
            r#"
            SELECT id, text, vector, created_at
            FROM embeddings
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(id, text, vector_bytes, created_at_str)| {
            let vector = bytes_to_vector(&vector_bytes);
            let created_at = DateTime::parse_from_rfc3339(&created_at_str).unwrap().with_timezone(&Utc);

            Embedding {
                id,
                text,
                vector,
                created_at,
            }
        }))
    }

    async fn find_all(&self) -> Result<Vec<Embedding>> {
        let rows: Vec<(i64, String, Vec<u8>, String)> = sqlx::query_as(
            r#"
            SELECT id, text, vector, created_at
            FROM embeddings
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let embeddings = rows
            .into_iter()
            .map(|(id, text, vector_bytes, created_at_str)| {
                let vector = bytes_to_vector(&vector_bytes);
                let created_at = DateTime::parse_from_rfc3339(&created_at_str).unwrap().with_timezone(&Utc);

                Embedding {
                    id,
                    text,
                    vector,
                    created_at,
                }
            })
            .collect();

        Ok(embeddings)
    }

    async fn find_similar(&self, vector: &[f32], limit: usize) -> Result<Vec<SimilarityResult>> {
        // 모든 임베딩 조회
        let embeddings = self.find_all().await?;

        // 코사인 유사도 계산 및 정렬
        let mut results: Vec<SimilarityResult> = embeddings
            .into_iter()
            .map(|embedding| {
                let similarity = crate::domain::entities::cosine_similarity(vector, &embedding.vector);
                SimilarityResult::new(embedding.id, embedding.text, similarity)
            })
            .collect();

        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        // 상위 N개만 반환
        results.truncate(limit);

        Ok(results)
    }

    async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM embeddings
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
