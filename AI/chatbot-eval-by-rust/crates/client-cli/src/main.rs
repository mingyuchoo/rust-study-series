#![allow(clippy::doc_markdown)]

//! AI Agent 테스트 평가 CLI
//!
//! # 사용 예시
//! ```bash
//! # LLM-as-Judge 평가 (기본)
//! cargo run -- --tool llm-judge --save
//!
//! # 안전성 테스트
//! cargo run -- --tool safety --use-golden-json --save
//!
//! # RAGAS 평가
//! cargo run -- --tool ragas --save
//!
//! # 전체 평가
//! cargo run -- --tool all --save
//! ```

use anyhow::Result;
use clap::{Parser,
           ValueEnum};
use eval_runner::{config::EvalConfig,
                  legacy_golden_dataset,
                  load_test_cases,
                  run_langfuse_evaluation,
                  run_llm_as_judge,
                  run_promptfoo_evaluation,
                  run_ragas_evaluation,
                  run_safety_evaluation,
                  save_results};

/// AI Agent 테스트 평가 도구
#[derive(Parser)]
#[command(name = "rust-eval-demo")]
#[command(about = "AI Agent 테스트 평가 CLI (Rust 포팅)")]
struct Cli {
    /// 사용할 평가 도구
    #[arg(long, default_value = "llm-judge")]
    tool: EvalTool,

    /// 결과를 파일로 저장
    #[arg(long, default_value_t = false)]
    save: bool,

    /// golden_dataset.json 사용 (기본: 레거시 드래곤볼 데이터)
    #[arg(long, default_value_t = false)]
    use_golden_json: bool,
}

/// 평가 도구 선택
#[derive(Clone, ValueEnum)]
enum EvalTool {
    Ragas,
    Safety,
    LlmJudge,
    Langfuse,
    Promptfoo,
    All,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("오류: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    // 환경변수 로드
    let _ = dotenvy::dotenv();

    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();
    let eval_config = EvalConfig::from_cwd();

    // 데이터셋 선택
    let samples = if cli.use_golden_json {
        load_test_cases(&eval_config.golden_dataset_path).map_or_else(
            |_| {
                println!("golden_dataset.json을 찾을 수 없습니다. 레거시 데이터 사용.");
                legacy_golden_dataset()
            },
            |samples| {
                println!("[데이터셋] golden_dataset.json ({}개 테스트)", samples.len());
                samples
            },
        )
    } else {
        let dataset = legacy_golden_dataset();
        println!("[데이터셋] 레거시 드래곤볼 ({}개 테스트)", dataset.len());
        dataset
    };

    println!("{}", "=".repeat(50));
    println!("AI Agent 테스트 평가 시작");
    println!("{}", "=".repeat(50));

    match cli.tool {
        | EvalTool::Ragas => {
            let results = run_ragas_evaluation(&samples).await?;
            if cli.save {
                let json = serde_json::to_value(&results)?;
                save_results(&json, "ragas_results.json", &eval_config.results_dir)?;
            }
        },
        | EvalTool::Safety => {
            let results = run_safety_evaluation(None, None).await?;
            if cli.save {
                let json = serde_json::to_value(&results)?;
                save_results(&json, "safety_results.json", &eval_config.results_dir)?;
            }
        },
        | EvalTool::LlmJudge => {
            let results = run_llm_as_judge(&samples).await?;
            if cli.save {
                let json = serde_json::to_value(&results)?;
                save_results(&json, "llm_judge_results.json", &eval_config.results_dir)?;
            }
        },
        | EvalTool::Langfuse => {
            let results = run_langfuse_evaluation(&samples).await?;
            if cli.save {
                let json = serde_json::to_value(&results)?;
                save_results(&json, "langfuse_results.json", &eval_config.results_dir)?;
            }
        },
        | EvalTool::Promptfoo => {
            let results = run_promptfoo_evaluation(None).await?;
            if cli.save {
                save_results(&results, "promptfoo_results.json", &eval_config.results_dir)?;
            }
        },
        | EvalTool::All => {
            println!("\n--- LLM-as-a-Judge 평가 ---");
            let llm_results = run_llm_as_judge(&samples).await?;

            println!("\n--- 안전성 평가 ---");
            let safety_results = run_safety_evaluation(None, None).await?;

            if cli.save {
                let llm_json = serde_json::to_value(&llm_results)?;
                save_results(&llm_json, "llm_judge_results.json", &eval_config.results_dir)?;

                let safety_json = serde_json::to_value(&safety_results)?;
                save_results(&safety_json, "safety_results.json", &eval_config.results_dir)?;
            }
        },
    }

    println!("\n{}", "=".repeat(50));
    println!("평가 완료");
    println!("{}", "=".repeat(50));

    Ok(())
}
