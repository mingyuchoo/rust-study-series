//! PDF 처리 모듈
//! - 계층적 청킹(제목/섹션/문단) 구현
//! - 중복 영역 최소화한 분할(간단 휴리스틱)
//!
//! 주: PDF에서 텍스트 추출은 다양한 외부 라이브러리 의존이 필요합니다.
//! 본 구현은 MVP로서 간단히 PDF 확장자 파일을 바이너리로 읽은 뒤,
//! 텍스트 추출이 불가한 경우 파일명을 기반으로 더미 본문을 구성합니다.
//! 실제 서비스에서는 `pdfium-render`, `lopdf` 등으로 교체하세요.

use anyhow::Result;
use regex::Regex;
use serde_json::json;

use crate::types::{Chunk, ChunkKind};

/// PDF를 읽어 텍스트를 얻고, 계층적 청킹을 수행한다.
pub fn process_pdf(path: &str) -> Result<Vec<Chunk>> {
    // 실제 PDF 파싱 시도 → 실패 시 시뮬레이션 텍스트로 대체
    let simulated_text = match extract_text_from_pdf(path) {
        | Ok(t) if !t.trim().is_empty() => t,
        | _ => simulate_extract_text_from_pdf(path),
    };

    // 1) 문서 → 섹션 → 문단 계층으로 분해
    let sections = split_sections(&simulated_text);
    let mut chunks: Vec<Chunk> = Vec::new();

    // 문서 제목(가정): 첫 줄 또는 파일명
    if let Some(title_line) = simulated_text.lines().next() {
        chunks.push(Chunk {
            content: title_line.trim().to_string(),
            level: 3,
            kind: ChunkKind::Title,
            index: 0,
            metadata: json!({"source": path}),
        });
    }

    // 섹션과 문단을 순회하며 청크 생성
    let mut idx = 1;
    let mut doc_summary_snippets: Vec<String> = Vec::new();
    for (sec_idx, sec) in sections.iter().enumerate() {
        // 섹션 제목 노드(레벨 2)
        chunks.push(Chunk {
            content: sec.title.clone(),
            level: 2,
            kind: ChunkKind::Section,
            index: idx,
            metadata: json!({"source": path, "section_index": sec_idx}),
        });
        idx += 1;

        // 섹션 본문 문단 → 토큰 기반 200-500 토큰 범위로 분할 + 30 토큰 겹침
        let mut section_tokens: Vec<String> = Vec::new();
        for p in &sec.paragraphs {
            let toks = tokenize(p);
            if !toks.is_empty() {
                section_tokens.extend(toks);
            }
        }

        // 섹션 요약(간단): 첫 문장 또는 처음 30 토큰
        let section_summary = make_simple_summary(&section_tokens, 30);
        if !section_summary.is_empty() {
            doc_summary_snippets.push(section_summary.clone());
            chunks.push(Chunk {
                content: format!("[섹션 요약] {}", section_summary),
                level: 2,
                kind: ChunkKind::Text,
                index: idx,
                metadata: json!({"source": path, "section_index": sec_idx, "summary": true}),
            });
            idx += 1;
        }

        for part in split_tokens_with_overlap(&section_tokens, 200, 500, 30) {
            let content = part.join(" ");
            if content.trim().is_empty() {
                continue;
            }
            chunks.push(Chunk {
                content,
                level: 1,
                kind: ChunkKind::Paragraph,
                index: idx,
                metadata: json!({"source": path, "section_index": sec_idx}),
            });
            idx += 1;
        }
    }

    // 문서 요약 노드(간단): 섹션 요약들을 합쳐 상위 요약 구성
    if !doc_summary_snippets.is_empty() {
        let doc_sum = doc_summary_snippets.join(" … ");
        chunks.push(Chunk {
            content: format!("[문서 요약] {}", doc_sum),
            level: 3,
            kind: ChunkKind::Text,
            index: idx,
            metadata: json!({"source": path, "summary": true}),
        });
    }

    Ok(chunks)
}

/// 기능 플래그에 따라 실제 PDF 텍스트를 추출한다.
#[allow(unused_variables)]
fn extract_text_from_pdf(path: &str) -> Result<String> {
    // pdfium 우선 사용
    #[cfg(feature = "pdfium")]
    {
        use pdfium_render::prelude::*;
        let pdfium = Pdfium::new(Pdfium::bind_to_system_library()?);
        let doc = pdfium.load_pdf_from_file(path, None)?;
        let mut out = String::new();
        for (i, page) in doc.pages().iter().enumerate() {
            let text_page = page.text()?;
            let page_text = text_page.all();
            out.push_str(&format!("\n[Page {}]\n{}\n", i + 1, page_text));
        }
        return Ok(out);
    }

    // lopdf 사용(간단 모드)
    #[cfg(all(not(feature = "pdfium"), feature = "lopdf"))]
    {
        let doc = lopdf::Document::load(path)?;
        // lopdf만으로 텍스트를 완전 추출하기는 까다로움 → 간단히 페이지 객체 순회하며 문자열 객체를 수집(제약 존재)
        let mut out = String::new();
        for (page_num, _page_id) in doc.get_pages().keys().enumerate() {
            out.push_str(&format!("\n[Page {}]\n", page_num + 1));
            if let Ok(content) = doc.extract_text(&[*doc.get_pages().get(&(page_num as u32 + 1)).unwrap()]) {
                out.push_str(&content);
            }
        }
        return Ok(out);
    }

    // 어떤 기능도 없으면 오류
    #[cfg(all(not(feature = "pdfium"), not(feature = "lopdf")))]
    {
        Err(anyhow::anyhow!(
            "PDF 파서 기능이 비활성화됨: Cargo features 'pdfium' 또는 'lopdf'를 활성화하세요. 경로: {path}"
        ))
    }
}

/// 간이 PDF 텍스트 추출(데모)
fn simulate_extract_text_from_pdf(path: &str) -> String {
    let filename = std::path::Path::new(path).file_name().and_then(|s| s.to_str()).unwrap_or("문서");
    format!(
        "{title}\n1. 개요\n이 문서는 {title} 에 대한 테스트 문서입니다. Rust 기반 GraphRAG 파이프라인 검증 용도입니다.\n\n2. 본문\n조직 A는 2024-01-01에 프로젝트를 시작하였습니다. 장소는 서울입니다.\n인물 홍길동이 프로젝트를 이끌고, 주식회사 샘플이 참여했습니다.\n\n3. 결론\n본 문서는 MVP이므로 실제 PDF 파서로 교체가 필요합니다.",
        title = filename
    )
}

/// 섹션 단위 분리
fn split_sections(text: &str) -> Vec<Section> {
    // 예: "1. 제목", "2. 제목" 형태를 섹션 제목으로 간주
    let re = Regex::new(r"(?m)^(\d+)\.\s*(.+)$").unwrap();
    let mut sections = Vec::new();
    let mut last_pos = 0;
    let mut last_title = String::from("서론");

    for cap in re.captures_iter(text) {
        let m = cap.get(0).unwrap();
        let sec_title = cap.get(0).unwrap().as_str().to_string();
        let start = m.start();
        if start > last_pos {
            let body = &text[last_pos..start];
            if !body.trim().is_empty() {
                sections.push(Section {
                    title: last_title.clone(),
                    paragraphs: body.split('\n').map(|s| s.to_string()).collect(),
                });
            }
        }
        last_pos = m.end();
        last_title = sec_title;
    }

    let tail = &text[last_pos..];
    if !tail.trim().is_empty() {
        sections.push(Section {
            title: last_title,
            paragraphs: tail.split('\n').map(|s| s.to_string()).collect(),
        });
    }

    sections
}

/// 토큰 시퀀스를 [min_tokens, max_tokens] 범위로 분할하며, window_overlap 토큰 겹침을 적용
fn split_tokens_with_overlap(tokens: &[String], min_tokens: usize, max_tokens: usize, window_overlap: usize) -> Vec<Vec<String>> {
    if tokens.is_empty() {
        return vec![];
    }
    let min_tokens = min_tokens.max(1);
    let max_tokens = max_tokens.max(min_tokens);
    let overlap = window_overlap.min(max_tokens.saturating_sub(1));

    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut start = 0usize;
    while start < tokens.len() {
        let mut end = (start + max_tokens).min(tokens.len());
        if end - start < min_tokens {
            // 남은 토큰이 너무 작다면 마지막 청크로 추가하고 종료
            chunks.push(tokens[start..tokens.len()].to_vec());
            break;
        }
        // 문장 경계로 조정(가능하면 뒤에서부터 마침표 위치 찾기)
        if let Some(rel) = find_sentence_boundary(&tokens[start..end]) {
            let cand = start + rel;
            if cand - start >= min_tokens {
                end = cand;
            }
        }
        chunks.push(tokens[start..end].to_vec());
        if end == tokens.len() {
            break;
        }
        start = end.saturating_sub(overlap);
    }
    chunks
}

fn tokenize(s: &str) -> Vec<String> {
    s.split_whitespace().map(|w| w.to_string()).collect()
}

fn find_sentence_boundary(tokens: &[String]) -> Option<usize> {
    for i in (0..tokens.len()).rev() {
        let t = tokens[i].as_str();
        if t.ends_with('.') || t.ends_with('!') || t.ends_with('?') || t.ends_with('…') {
            return Some(i + 1);
        }
    }
    None
}

fn make_simple_summary(tokens: &[String], max_tokens: usize) -> String {
    if tokens.is_empty() {
        return String::new();
    }
    let end = tokens.len().min(max_tokens.max(1));
    tokens[..end].join(" ")
}

#[derive(Debug)]
struct Section {
    title: String,
    paragraphs: Vec<String>,
}
