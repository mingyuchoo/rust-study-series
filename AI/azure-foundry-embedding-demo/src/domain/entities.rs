use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 임베딩 엔티티
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub id: i64,
    pub text: String,
    pub vector: Vec<f32>,
    pub created_at: DateTime<Utc>,
}

impl Embedding {
    /// 새로운 임베딩 생성
    pub fn new(id: i64, text: String, vector: Vec<f32>) -> Self {
        Self {
            id,
            text,
            vector,
            created_at: Utc::now(),
        }
    }

    /// 코사인 유사도 계산
    pub fn cosine_similarity(&self, other: &Self) -> f32 { cosine_similarity(&self.vector, &other.vector) }
}

/// 두 벡터 간의 코사인 유사도 계산
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}
