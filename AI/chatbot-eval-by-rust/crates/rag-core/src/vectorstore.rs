//! 인메모리 코사인 유사도 벡터스토어

use crate::{embedding::EmbeddingClient,
            error::RagError};
use models::DocumentMeta;

/// 인메모리 벡터스토어
pub struct VectorStore {
    embedding_client: EmbeddingClient,
    documents: Vec<DocumentMeta>,
    embeddings: Vec<Vec<f32>>,
}

impl VectorStore {
    /// 새 벡터스토어를 생성한다.
    #[must_use]
    pub const fn new(embedding_client: EmbeddingClient) -> Self {
        Self {
            embedding_client,
            documents: Vec::new(),
            embeddings: Vec::new(),
        }
    }

    /// 문서를 벡터스토어에 추가한다.
    ///
    /// # Errors
    ///
    /// 임베딩 생성 실패 시 에러를 반환한다.
    pub async fn add_documents(&mut self, documents: &[DocumentMeta]) -> Result<(), RagError> {
        let texts: Vec<String> = documents.iter().map(|d| d.content.clone()).collect();
        let new_embeddings = self.embedding_client.embed(&texts).await?;

        self.documents.extend(documents.iter().cloned());
        self.embeddings.extend(new_embeddings);

        Ok(())
    }

    /// 질문과 가장 유사한 Top-K 문서를 검색한다.
    ///
    /// # Errors
    ///
    /// 문서 미로드 또는 임베딩 실패 시 에러를 반환한다.
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<DocumentMeta>, RagError> {
        if self.documents.is_empty() {
            return Err(RagError::NoDocuments);
        }

        let query_embedding = self.embedding_client.embed_one(query).await?;

        let mut scored: Vec<(usize, f64)> = self
            .embeddings
            .iter()
            .enumerate()
            .map(|(i, emb)| (i, cosine_similarity(&query_embedding, emb)))
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(scored.into_iter().take(top_k).map(|(i, _)| self.documents[i].clone()).collect())
    }

    /// 저장된 문서 수를 반환한다.
    #[must_use]
    pub fn len(&self) -> usize { self.documents.len() }

    /// 벡터스토어가 비어있는지 확인한다.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.documents.is_empty() }
}

/// 두 벡터 간 코사인 유사도를 계산한다.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| f64::from(*x) * f64::from(*y)).sum();
    let norm_a: f64 = a.iter().map(|x| f64::from(*x) * f64::from(*x)).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| f64::from(*x) * f64::from(*x)).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 동일한_벡터의_코사인_유사도는_1이다() {
        let v = vec![1.0_f32, 2.0, 3.0];
        let sim = cosine_similarity(&v, &v);
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn 직교_벡터의_코사인_유사도는_0이다() {
        let a = vec![1.0_f32, 0.0, 0.0];
        let b = vec![0.0_f32, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6);
    }

    #[test]
    fn 반대_벡터의_코사인_유사도는_마이너스_1이다() {
        let a = vec![1.0_f32, 0.0];
        let b = vec![-1.0_f32, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim + 1.0).abs() < 1e-6);
    }

    #[test]
    fn 영벡터의_코사인_유사도는_0이다() {
        let a = vec![0.0_f32, 0.0];
        let b = vec![1.0_f32, 1.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6);
    }
}
