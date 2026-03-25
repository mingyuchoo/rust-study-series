//! Promptfoo 평가 모듈
//!
//! Promptfoo를 사용한 프롬프트 단위 테스트.
//! `promptfoo.yaml` 설정 파일을 기반으로 테스트 실행.
//!
//! 사전 요구사항:
//! - `npm install -g promptfoo`
//! - `promptfoo.yaml` 설정 파일

use crate::config::EvalConfig;
use std::{path::Path,
          process::Command};

/// Promptfoo를 사용한 프롬프트 단위 테스트를 수행한다.
///
/// # Errors
///
/// Promptfoo 미설치, 설정 파일 누락, 실행 실패 시 에러를 반환한다.
pub async fn run_promptfoo_evaluation(config_path: Option<&Path>) -> anyhow::Result<serde_json::Value> {
    let eval_config = EvalConfig::from_cwd();
    let config_path = config_path.unwrap_or(&eval_config.promptfoo_config_path);

    if !config_path.exists() {
        anyhow::bail!("Promptfoo 설정 파일을 찾을 수 없습니다: {}", config_path.display());
    }

    // promptfoo 설치 확인
    let version_check = Command::new("npx").args(["promptfoo", "--version"]).output();

    match version_check {
        | Ok(output) if output.status.success() => {},
        | _ => {
            anyhow::bail!("Promptfoo가 설치되지 않았습니다. npm install -g promptfoo");
        },
    }

    println!("Promptfoo 평가 실행 중...");
    println!("  설정 파일: {}", config_path.display());

    let output = Command::new("npx")
        .args(["promptfoo", "eval", "-c", &config_path.to_string_lossy(), "--output", "json"])
        .current_dir(eval_config.data_dir.parent().unwrap_or_else(|| Path::new(".")))
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Promptfoo 실행 실패: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("\n=== Promptfoo 평가 결과 ===");

    serde_json::from_str::<serde_json::Value>(&stdout).map_or_else(
        |_| {
            println!("{stdout}");
            Ok(serde_json::json!({ "output": stdout.to_string() }))
        },
        Ok,
    )
}
