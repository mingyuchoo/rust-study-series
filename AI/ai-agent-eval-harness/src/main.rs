use clap::{Parser,
           Subcommand};
use std::sync::Arc;

mod agent_core;
mod data_scenarios;
mod domains;
mod execution;
mod execution_fault_injection;
mod execution_multi_turn;
mod execution_tools;
mod reporting;
mod scoring;
mod scoring_llm_judge;

use execution::{agent_registry::AgentRegistry,
                base_agent::PassthroughAgent,
                comparator::ReportComparator,
                report_renderer::ReportRenderer,
                runner::HarnessRunner};

#[derive(Parser)]
#[command(name = "eval-harness", about = "AI Agent 평가 하네스 - 통합 실행 및 비교 도구")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 벤치마크 스위트 실행
    Run {
        #[arg(short, long, default_value = "all")]
        suite: String,
        #[arg(short, long, default_value = "passthrough")]
        agent: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long, default_value = "eval_data/scenarios")]
        scenarios_dir: String,
        #[arg(long, default_value = "reporting_logs")]
        output_dir: String,
    },
    /// 두 리포트 비교 및 회귀 감지
    Compare {
        baseline: String,
        current: String,
        #[arg(short, long, default_value = "5.0")]
        threshold: f64,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// 사용 가능한 스위트/시나리오 목록 표시
    List {
        #[arg(long, default_value = "eval_data/scenarios")]
        scenarios_dir: String,
    },
    /// 저장된 리포트 표시
    Report { filepath: String },
}

fn build_registry() -> AgentRegistry {
    let mut registry = AgentRegistry::new();
    registry.register("passthrough", Arc::new(PassthroughAgent));

    dotenvy::dotenv().ok();
    if let Ok(llm_config) = agent_core::config::AzureOpenAiConfig::from_env() {
        let eval_config = agent_core::config::EvaluationConfig::default();
        let llm = agent_core::llm_client::LlmClient::new(llm_config);
        let agent = agent_core::agent::PpaAgent::new(llm, eval_config);
        registry.register("ppa", Arc::new(agent));
    } else {
        eprintln!("PPA 에이전트 초기화 실패 (LLM 설정 확인 필요) - passthrough만 사용 가능");
    }

    registry
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        | Commands::Run {
            suite,
            agent,
            output,
            scenarios_dir,
            output_dir,
        } => {
            let registry = build_registry();
            let agent_impl = match registry.get_agent(&agent) {
                | Some(a) => a,
                | None => {
                    eprintln!("등록되지 않은 에이전트: {}", agent);
                    eprintln!("사용 가능: {:?}", registry.get_agent_names());
                    std::process::exit(1);
                },
            };

            let mut runner = HarnessRunner::new(&output_dir);
            let report = match runner.run_suite(&suite, agent_impl.as_ref(), &scenarios_dir) {
                | Ok(r) => r,
                | Err(e) => {
                    eprintln!("실행 오류: {}", e);
                    std::process::exit(1);
                },
            };

            runner.save_report(&report, output.as_deref()).ok();
            let renderer = ReportRenderer::new();
            renderer.render(&report);
        },

        | Commands::Compare {
            baseline,
            current,
            threshold,
            output,
        } => {
            let comparator = ReportComparator::new(threshold);
            let result = match comparator.compare_files(&baseline, &current) {
                | Ok(r) => r,
                | Err(e) => {
                    eprintln!("비교 오류: {}", e);
                    std::process::exit(1);
                },
            };

            comparator.print_comparison(&result);
            if let Some(out) = output {
                comparator.save_comparison(&result, &out).ok();
            }
            if result.verdict == "fail" {
                std::process::exit(1);
            }
        },

        | Commands::List {
            scenarios_dir,
        } => {
            use data_scenarios::loader::ScenarioLoader;

            let loader = ScenarioLoader::new();
            if !std::path::Path::new(&scenarios_dir).exists() {
                eprintln!("디렉토리 없음: {}", scenarios_dir);
                std::process::exit(1);
            }

            match loader.load_all_domains(&scenarios_dir) {
                | Ok(configs) => {
                    if configs.is_empty() {
                        println!("등록된 스위트가 없습니다.");
                        return;
                    }
                    for config in &configs {
                        println!("\n{}: {}", config.name, config.description);
                        for s in &config.scenarios {
                            println!("  - [{}] {}: {}", s.difficulty, s.id, s.name);
                        }
                    }
                },
                | Err(e) => {
                    eprintln!("로드 오류: {}", e);
                    std::process::exit(1);
                },
            }

            let registry = build_registry();
            println!("\n등록된 에이전트: {:?}", registry.get_agent_names());
        },

        | Commands::Report {
            filepath,
        } => {
            let renderer = ReportRenderer::new();
            match renderer.load_report(&filepath) {
                | Ok(report) => renderer.render(&report),
                | Err(e) => {
                    eprintln!("파일 오류: {}", e);
                    std::process::exit(1);
                },
            }
        },
    }
}
