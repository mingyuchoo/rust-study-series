//! 임베딩 모듈
//! - 외부 API 연동 콜백만 지원

use anyhow::Result;
use crate::types::{Chunk, Embeddings3};

/// 임베딩 모드
pub enum EmbeddingMode<'a> {
    /// 외부 API 사용: 호출자는 텍스트 배열에 대한 임베딩 생성 콜백 제공
    External(&'a dyn Fn(&[String]) -> Result<Vec<Vec<f32>>>),
}

/// 주어진 청크 텍스트들에 대해 임베딩을 생성한다.
pub fn embed_chunks(chunks: &[Chunk], mode: EmbeddingMode) -> Result<Vec<Vec<f32>>> {
    let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
    embed_texts(&texts, mode)
}

/// 임의의 텍스트 배열에 대해 임베딩을 생성한다(엔티티/관계 등 공용).
pub fn embed_texts(texts: &[String], mode: EmbeddingMode) -> Result<Vec<Vec<f32>>> {
    match mode {
        | EmbeddingMode::External(cb) => cb(&texts.iter().cloned().collect::<Vec<_>>()),
    }
}

/// 청크에 대한 다중 관점 임베딩(semantic/structural/functional)
pub fn embed_chunks_e3(chunks: &[Chunk], mode: EmbeddingMode) -> Result<Vec<Embeddings3>> {
    let semantic = embed_chunks(chunks, mode)?;
    let mut out = Vec::with_capacity(chunks.len());
    for (i, ch) in chunks.iter().enumerate() {
        // 간단한 구조적 특징: [level, index_norm]
        let level = ch.level as f32;
        let index_norm = if chunks.is_empty() { 0.0 } else { i as f32 / chunks.len() as f32 };
        let structural = vec![level, index_norm];
        // 간단한 기능적 특징: 길이 기반 스칼라
        let functional = vec![ch.content.chars().count() as f32];
        out.push(Embeddings3 {
            semantic: semantic.get(i).cloned().unwrap_or_default(),
            structural,
            functional,
        });
    }
    Ok(out)
}

/// 텍스트 배열에 대한 다중 관점 임베딩(semantic/structural/functional)
pub fn embed_texts_e3(texts: &[String], mode: EmbeddingMode) -> Result<Vec<Embeddings3>> {
    let semantic = embed_texts(texts, mode)?;
    let mut out = Vec::with_capacity(texts.len());
    for (i, t) in texts.iter().enumerate() {
        // 구조적 특징(텍스트에는 구조 정보가 없어 간단 지표만 제공)
        let structural = vec![i as f32 / texts.len().max(1) as f32];
        // 기능적 특징: 길이
        let functional = vec![t.chars().count() as f32];
        out.push(Embeddings3 {
            semantic: semantic.get(i).cloned().unwrap_or_default(),
            structural,
            functional,
        });
    }
    Ok(out)
}

// 로컬 임베딩은 지원하지 않습니다.
