//! PDF 처리 모듈
//! - 계층적 청킹(제목/섹션/문단) 구현
//! - 한글/다국어 텍스트를 글자 수 기반으로 분할
//!
//! 주: PDF에서 텍스트 추출은 다양한 외부 라이브러리 의존이 필요합니다.
//! 본 구현은 MVP로서 간단히 PDF 확장자 파일을 바이너리로 읽은 뒤,
//! 텍스트 추출이 불가한 경우 파일명을 기반으로 더미 본문을 구성합니다.
//! 실제 서비스에서는 `pdfium-render`, `lopdf` 등으로 교체하세요.

use anyhow::Result;
use regex::Regex;
use serde_json::json;

use crate::types::{Chunk, ChunkKind};

/// 글자 수 기반 청킹 파라미터
const CHUNK_MIN_CHARS: usize = 400;
const CHUNK_MAX_CHARS: usize = 1000;
const CHUNK_OVERLAP_CHARS: usize = 60;

/// PDF를 읽어 텍스트를 얻고, 계층적 청킹을 수행한다.
pub fn process_pdf(path: &str) -> Result<Vec<Chunk>> {
    // 실제 PDF 파싱 시도 → 실패 시 시뮬레이션 텍스트로 대체
    let raw_text = match extract_text_from_pdf(path) {
        | Ok(t) if !t.trim().is_empty() => t,
        | _ => simulate_extract_text_from_pdf(path),
    };

    // 추출된 텍스트를 정규화(연속 공백/줄바꿈 축소)
    let text = normalize_text(&raw_text);

    // 1) 문서 → 섹션 → 문단 계층으로 분해
    let sections = split_sections(&text);
    let mut chunks: Vec<Chunk> = Vec::new();

    // 문서 제목(가정): 첫 줄 또는 파일명
    if let Some(title_line) = text.lines().next() {
        let title = title_line.trim();
        if !title.is_empty() {
            chunks.push(Chunk {
                content: title.to_string(),
                level: 3,
                kind: ChunkKind::Title,
                index: 0,
                metadata: json!({"source": path}),
            });
        }
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

        // 섹션 본문 문단들을 하나의 문자열로 결합
        let section_text: String = sec
            .paragraphs
            .iter()
            .filter(|p| !p.trim().is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");

        // 섹션 요약(간단): 첫 문장 또는 처음 80자
        let section_summary = make_simple_summary(&section_text, 80);
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

        // 글자 수 기반 청킹(한글 호환)
        for part in split_text_with_overlap(
            &section_text,
            CHUNK_MIN_CHARS,
            CHUNK_MAX_CHARS,
            CHUNK_OVERLAP_CHARS,
        ) {
            let content = part.trim().to_string();
            if content.is_empty() {
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

/// PDF 추출 텍스트를 정규화한다.
/// - 연속된 공백 줄을 하나로 축소
/// - 캐리지 리턴 제거
/// - 줄 앞뒤 공백 제거
fn normalize_text(text: &str) -> String {
    let re_blank_lines = Regex::new(r"\n{3,}").unwrap();
    let cleaned = text.replace('\r', "");
    let normalized = re_blank_lines.replace_all(&cleaned, "\n\n");
    normalized
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join("\n")
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

/// 글자 수 기반으로 텍스트를 분할한다 (한글/다국어 호환).
///
/// `split_whitespace()` 토큰 기반 대신 **글자 수(char count)** 기준으로
/// 문장 경계에서 분할하여 한글 텍스트도 균일한 크기로 청킹한다.
fn split_text_with_overlap(
    text: &str,
    min_chars: usize,
    max_chars: usize,
    overlap_chars: usize,
) -> Vec<String> {
    if text.trim().is_empty() {
        return vec![];
    }

    let min_chars = min_chars.max(1);
    let max_chars = max_chars.max(min_chars);
    let overlap = overlap_chars.min(max_chars.saturating_sub(1));

    // 문장 단위로 먼저 분리
    let sentences = split_into_sentences(text);
    if sentences.is_empty() {
        return vec![];
    }

    let mut chunks: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut current_len: usize = 0;

    for sentence in &sentences {
        let sent_len = sentence.chars().count();

        // 단일 문장이 max_chars를 초과하면 강제 분할
        if sent_len > max_chars {
            // 현재 버퍼가 있으면 먼저 청크로 저장
            if !current.trim().is_empty() {
                chunks.push(current.clone());
                current.clear();
                current_len = 0;
            }
            // 긴 문장을 글자 수 기준으로 강제 분할
            let chars: Vec<char> = sentence.chars().collect();
            let mut pos = 0;
            while pos < chars.len() {
                let end = (pos + max_chars).min(chars.len());
                let slice: String = chars[pos..end].iter().collect();
                chunks.push(slice);
                pos = end.saturating_sub(overlap);
            }
            continue;
        }

        // 현재 버퍼 + 이 문장이 max_chars를 초과하면 청크 확정
        if current_len + sent_len > max_chars && current_len >= min_chars {
            chunks.push(current.clone());

            // 겹침 영역: 현재 청크 끝부분에서 overlap_chars만큼 가져오기
            let overlap_text = take_tail_chars(&current, overlap);
            current = overlap_text;
            current_len = current.chars().count();
        }

        if !current.is_empty() && !current.ends_with('\n') {
            current.push(' ');
            current_len += 1;
        }
        current.push_str(sentence);
        current_len += sent_len;
    }

    // 남은 버퍼 처리
    if !current.trim().is_empty() {
        // 남은 텍스트가 너무 짧으면 이전 청크에 병합
        if current_len < min_chars && !chunks.is_empty() {
            let last = chunks.last_mut().unwrap();
            last.push(' ');
            last.push_str(current.trim());
        } else {
            chunks.push(current);
        }
    }

    chunks
}

/// 텍스트를 문장 단위로 분리한다.
/// 한글/영문 구두점(`.` `!` `?` `。`)과 줄바꿈을 문장 경계로 인식한다.
fn split_into_sentences(text: &str) -> Vec<String> {
    let re = Regex::new(r"[.!?。]\s+|\n+").unwrap();
    let mut sentences = Vec::new();
    let mut last = 0;

    for m in re.find_iter(text) {
        let end = m.end();
        let segment = text[last..end].trim();
        if !segment.is_empty() {
            sentences.push(segment.to_string());
        }
        last = end;
    }

    // 마지막 잔여 텍스트
    let tail = text[last..].trim();
    if !tail.is_empty() {
        sentences.push(tail.to_string());
    }

    sentences
}

/// 문자열의 끝에서 최대 n글자를 추출한다.
fn take_tail_chars(s: &str, n: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let start = chars.len().saturating_sub(n);
    chars[start..].iter().collect()
}

/// 텍스트의 첫 문장 또는 처음 max_chars 글자를 요약으로 반환한다.
fn make_simple_summary(text: &str, max_chars: usize) -> String {
    if text.trim().is_empty() {
        return String::new();
    }
    // 첫 문장 경계 찾기
    let sentences = split_into_sentences(text);
    if let Some(first) = sentences.first() {
        let char_count = first.chars().count();
        if char_count <= max_chars {
            return first.clone();
        }
    }
    // 첫 문장이 너무 길면 글자 수 기준으로 자르기
    let chars: Vec<char> = text.chars().collect();
    let end = chars.len().min(max_chars);
    let mut result: String = chars[..end].iter().collect();
    result.push('…');
    result
}

#[derive(Debug)]
struct Section {
    title: String,
    paragraphs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_sentences_korean() {
        let text = "이것은 첫 번째 문장입니다. 이것은 두 번째 문장입니다. 세 번째!";
        let sentences = split_into_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert!(sentences[0].contains("첫 번째"));
        assert!(sentences[1].contains("두 번째"));
    }

    #[test]
    fn test_split_into_sentences_newline() {
        let text = "첫째 줄\n둘째 줄\n셋째 줄";
        let sentences = split_into_sentences(text);
        assert_eq!(sentences.len(), 3);
    }

    #[test]
    fn test_split_text_with_overlap_korean() {
        // 한글 텍스트가 글자 수 기준으로 적절히 분할되는지 확인
        let text = "가".repeat(800); // 800글자
        let chunks = split_text_with_overlap(&text, 400, 1000, 60);
        assert_eq!(chunks.len(), 1); // 1000자 이내이므로 1개 청크
    }

    #[test]
    fn test_split_text_with_overlap_long_korean() {
        // 충분히 긴 한글 텍스트를 생성하여 여러 청크로 분할 확인
        let sentences: Vec<String> = (0..100)
            .map(|i| format!("이것은 {}번째로 작성된 테스트용 한글 문장이며 청킹 로직을 검증합니다.", i))
            .collect();
        let text = sentences.join(" ");
        assert!(text.chars().count() > 1000, "테스트 텍스트가 충분히 길어야 합니다");
        let chunks = split_text_with_overlap(&text, 400, 1000, 60);
        assert!(chunks.len() >= 2, "긴 한글 텍스트는 여러 청크로 분할되어야 합니다");
        for chunk in &chunks {
            let char_count = chunk.chars().count();
            assert!(
                char_count <= 2000,
                "청크 글자 수 {}가 합리적 범위를 초과합니다",
                char_count
            );
        }
    }

    #[test]
    fn test_split_text_with_overlap_empty() {
        let result = split_text_with_overlap("", 400, 1000, 60);
        assert!(result.is_empty());
    }

    #[test]
    fn test_split_text_with_overlap_preserves_content() {
        let text = "첫 번째 문장입니다. 두 번째 문장입니다. 세 번째 문장입니다.";
        let chunks = split_text_with_overlap(text, 5, 50, 5);
        // 모든 원본 문장이 어딘가에 포함되어야 함
        let joined = chunks.join(" ");
        assert!(joined.contains("첫 번째"));
        assert!(joined.contains("두 번째"));
        assert!(joined.contains("세 번째"));
    }

    #[test]
    fn test_normalize_text() {
        let text = "줄 1\r\n\n\n\n줄 2\n  앞뒤 공백  ";
        let result = normalize_text(text);
        assert!(!result.contains('\r'));
        assert!(!result.contains("\n\n\n"));
        assert!(result.contains("앞뒤 공백"));
    }

    #[test]
    fn test_make_simple_summary_korean() {
        let text = "이것은 요약 대상 문장입니다. 두 번째 문장은 포함되지 않아야 합니다.";
        let summary = make_simple_summary(text, 80);
        assert!(summary.contains("요약 대상"));
    }

    #[test]
    fn test_make_simple_summary_empty() {
        assert_eq!(make_simple_summary("", 80), "");
    }

    #[test]
    fn test_take_tail_chars() {
        assert_eq!(take_tail_chars("가나다라마", 3), "다라마");
        assert_eq!(take_tail_chars("ab", 5), "ab");
    }

    #[test]
    fn test_split_sections() {
        let text = "서론 내용\n1. 개요\n개요 본문입니다.\n2. 결론\n결론 본문입니다.";
        let sections = split_sections(text);
        assert!(sections.len() >= 2);
    }

    #[test]
    fn test_process_pdf_simulated() {
        let chunks = process_pdf("test_doc.pdf").unwrap();
        assert!(!chunks.is_empty());
        assert!(chunks.iter().any(|c| c.kind == ChunkKind::Title));
        // 문단 청크가 생성되어야 한다
        assert!(chunks.iter().any(|c| c.kind == ChunkKind::Paragraph));
    }

    #[test]
    fn test_process_pdf_chunks_contain_korean() {
        let chunks = process_pdf("한글_문서.pdf").unwrap();
        // 한글 내용이 포함된 청크가 존재해야 한다
        let has_korean = chunks.iter().any(|c| c.content.contains("프로젝트") || c.content.contains("문서"));
        assert!(has_korean, "한글 텍스트가 청크에 포함되어야 합니다");
    }
}
