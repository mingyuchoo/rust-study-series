//! 공통 유틸리티 함수

use std::path::Path;

/// NaN/Infinity를 `None`으로 변환한다.
#[must_use]
pub const fn safe_float(value: f64) -> Option<f64> { if value.is_nan() || value.is_infinite() { None } else { Some(value) } }

/// 평가 결과를 JSON 파일로 저장한다.
///
/// # Errors
///
/// 디렉토리 생성 실패 또는 파일 쓰기 실패 시 에러를 반환한다.
pub fn save_results(results: &serde_json::Value, filename: &str, output_dir: &Path) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(output_dir)?;
    let filepath = output_dir.join(filename);

    let content = serde_json::to_string_pretty(results)?;
    std::fs::write(&filepath, content)?;

    println!("\n결과 저장: {}", filepath.display());
    Ok(filepath)
}

/// `EvalResult` 리스트를 JSONL 형식으로 저장한다.
///
/// # Errors
///
/// 디렉토리 생성 실패, JSON 직렬화 실패, 또는 파일 쓰기 실패 시 에러를
/// 반환한다.
pub fn save_results_jsonl(results: &[serde_json::Value], filename: &str, output_dir: &Path) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(output_dir)?;
    let filepath = output_dir.join(filename);

    let mut lines = Vec::with_capacity(results.len());
    for result in results {
        lines.push(serde_json::to_string(result)?);
    }
    std::fs::write(&filepath, lines.join("\n"))?;

    println!("\n결과 저장: {}", filepath.display());
    Ok(filepath)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nan을_none으로_변환한다() {
        assert!(safe_float(f64::NAN).is_none());
        assert!(safe_float(f64::INFINITY).is_none());
        assert_eq!(safe_float(0.85), Some(0.85));
        assert_eq!(safe_float(0.0), Some(0.0));
    }
}
