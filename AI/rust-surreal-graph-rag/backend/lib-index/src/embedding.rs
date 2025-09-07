//! 임베딩 모듈
//! - 로컬 TF-IDF 임베딩
//! - 외부 API 연동 옵션(스텁)

use anyhow::Result;
use hashbrown::HashMap;

use crate::types::{Chunk, Embeddings3};

/// 임베딩 모드
pub enum EmbeddingMode<'a> {
    /// 로컬 TF-IDF 사용
    Tfidf,
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
        EmbeddingMode::Tfidf => tfidf_embed(texts),
        EmbeddingMode::External(cb) => cb(&texts.iter().cloned().collect::<Vec<_>>()),
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

/// 간단한 TF-IDF 벡터라이저
fn tfidf_embed(docs: &[String]) -> Result<Vec<Vec<f32>>> {
    if docs.is_empty() {
        return Ok(Vec::new());
    }

    // 토큰화(아주 단순: 공백 기준, 소문자화)
    let tokenized: Vec<Vec<String>> = docs
        .iter()
        .map(|d| tokenize(d))
        .collect();

    // vocabulary
    let mut vocab: HashMap<String, usize> = HashMap::new();
    for doc in &tokenized {
        for t in doc {
            if !vocab.contains_key(t) {
                let id = vocab.len();
                vocab.insert(t.clone(), id);
            }
        }
    }
    let vsize = vocab.len();

    // DF
    let mut df = vec![0usize; vsize];
    for doc in &tokenized {
        let mut seen: HashMap<usize, bool> = HashMap::new();
        for t in doc {
            if let Some(&idx) = vocab.get(t) {
                if !seen.contains_key(&idx) {
                    df[idx] += 1;
                    seen.insert(idx, true);
                }
            }
        }
    }

    let n_docs = docs.len() as f32;
    let idf: Vec<f32> = df
        .iter()
        .map(|&d| ((n_docs + 1.0) / (d as f32 + 1.0)).ln() + 1.0)
        .collect();

    // TF-IDF
    let mut out = Vec::with_capacity(docs.len());
    for doc in &tokenized {
        let mut tf: HashMap<usize, f32> = HashMap::new();
        for t in doc {
            if let Some(&idx) = vocab.get(t) {
                *tf.entry(idx).or_insert(0.0) += 1.0;
            }
        }
        let doc_len = doc.len() as f32;
        let mut vec = vec![0f32; vsize];
        for (idx, cnt) in tf {
            let tf_val = cnt / doc_len;
            vec[idx] = tf_val * idf[idx];
        }
        l2_normalize(&mut vec);
        out.push(vec);
    }

    Ok(out)
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn l2_normalize(v: &mut [f32]) {
    let sumsq: f32 = v.iter().map(|x| x * x).sum();
    if sumsq > 0.0 {
        let inv = 1.0 / sumsq.sqrt();
        for x in v.iter_mut() { *x *= inv; }
    }
}
