// =============================================================================
// @trace SPEC-001
// @trace SPEC-015
// @trace PRD: PRD-001, PRD-015
// @trace FR: PRD-001/FR-1, PRD-015/FR-4, PRD-015/FR-5
// @trace file-type: impl
// =============================================================================

use clap::{Parser,
           Subcommand};
use eval_harness::{data_paths::DataPaths,
                   tui,
                   web};
use execution::{agent_registry::AgentRegistry,
                base_agent::PassthroughAgent,
                comparator::ReportComparator,
                report_renderer::ReportRenderer,
                runner::HarnessRunner};
use std::{path::Path,
          sync::Arc};

#[derive(Parser)]
#[command(name = "eval-harness", about = "AI Agent 평가 하네스 - 통합 실행 및 비교 도구")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 평가 시나리오 실행
    Run {
        #[arg(short, long, default_value = "all")]
        eval_scenario: String,
        #[arg(short, long, default_value = "passthrough")]
        agent: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long)]
        scenarios_dir: Option<String>,
        #[arg(long, default_value = "reporting_logs")]
        output_dir: String,
    },
    /// 두 리포트 비교 및 회귀 감지. 파일 인자 또는 SPEC-021 의 DB 쿼리
    /// 모드(--baseline-task/--current-task, 또는 --agent/--since/--until)
    /// 중 하나를 선택할 수 있다.
    Compare {
        /// 베이스라인 파일 경로 (DB 쿼리 모드 사용 시 생략)
        #[arg(default_value = "")]
        baseline: String,
        /// 현재 파일 경로 (DB 쿼리 모드 사용 시 생략)
        #[arg(default_value = "")]
        current: String,
        #[arg(short, long, default_value = "5.0")]
        threshold: f64,
        #[arg(short, long)]
        output: Option<String>,
        /// SPEC-021: 베이스라인 task_id (DB 조회)
        #[arg(long)]
        baseline_task: Option<String>,
        /// SPEC-021: 현재 task_id (DB 조회)
        #[arg(long)]
        current_task: Option<String>,
        /// SPEC-021: 시간 범위 평균 비교 — agent 이름
        #[arg(long)]
        agent: Option<String>,
        /// SPEC-021: 베이스라인 시간 범위 시작 (RFC3339, 예:
        /// 2026-01-01T00:00:00Z)
        #[arg(long)]
        baseline_since: Option<String>,
        /// SPEC-021: 베이스라인 시간 범위 종료
        #[arg(long)]
        baseline_until: Option<String>,
        /// SPEC-021: 현재 시간 범위 시작
        #[arg(long)]
        current_since: Option<String>,
        /// SPEC-021: 현재 시간 범위 종료
        #[arg(long)]
        current_until: Option<String>,
    },
    /// SPEC-021: 디렉토리의 trajectory_*.json / evaluation_*.json 을 DB 로
    /// 일회성 backfill.
    BackfillResults {
        #[arg(long, default_value = "reporting_trajectories")]
        trajectories_dir: String,
        #[arg(long, default_value = "reporting_logs")]
        logs_dir: String,
    },
    /// 사용 가능한 평가 시나리오 목록 표시
    List {
        #[arg(long)]
        scenarios_dir: Option<String>,
    },
    /// 저장된 리포트 표시
    Report { filepath: String },
    /// 대화형 TUI 모드 실행
    Tui {
        #[arg(long)]
        scenarios_dir: Option<String>,
        #[arg(long, default_value = "reporting_logs")]
        reports_dir: String,
    },
    /// 웹 클라이언트(HTTP 서버) 실행
    Serve {
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: String,
        #[arg(long)]
        scenarios_dir: Option<String>,
        #[arg(long, default_value = "reporting_logs")]
        reports_dir: String,
        #[arg(long)]
        golden_sets_dir: Option<String>,
        #[arg(long, default_value = "reporting_trajectories")]
        trajectories_dir: String,
    },
}

fn resolve_data_paths(scenarios: Option<&str>, golden_sets: Option<&str>) -> DataPaths {
    let base = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    match DataPaths::load(&base) {
        | Ok(p) => p.with_overrides(scenarios, golden_sets),
        | Err(e) => {
            eprintln!("설정 파일 오류: {e}");
            std::process::exit(1);
        },
    }
}

/// SPEC-017: 기동 시 SQLite 저장소를 전역 설치한다. 멱등 — 이미 설치되었으면
/// 이전 인스턴스를 재사용한다.
///
/// @trace SPEC: SPEC-017
/// @trace FR: PRD-017/FR-5
fn install_data_store(paths: &DataPaths) {
    use data_scenarios::loader::{ScenarioLoader,
                                 try_installed_store};
    if let Err(e) = ScenarioLoader::install(&paths.db_path) {
        eprintln!("[warn] SQLite 저장소 초기화 실패: {e} — 인메모리 fallback 모드");
        return;
    }
    println!("[store] SQLite DB: {}", paths.db_path.display());

    // SPEC-022: 부트스트랩 도메인의 라우터 키워드를 DB 에 시드한다(멱등). 기존
    // 사용자가 키워드를 수정한 경우는 INSERT OR IGNORE 로 보존된다.
    if let Some(store) = try_installed_store() {
        let pairs = agent_core::domain_router::default_keywords_flat();
        let store_clone = store.clone();
        let result = match tokio::runtime::Handle::try_current() {
            | Ok(handle) => tokio::task::block_in_place(|| handle.block_on(async move { store_clone.seed_domain_keywords(&pairs).await })),
            | Err(_) => tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .ok()
                .map(|rt| rt.block_on(async move { store_clone.seed_domain_keywords(&pairs).await }))
                .unwrap_or_else(|| Ok(0)),
        };
        match result {
            | Ok(n) if n > 0 => println!("[store] 부트스트랩 키워드 {n}개 시드"),
            | Ok(_) => {},
            | Err(e) => eprintln!("[warn] 키워드 시드 실패: {e}"),
        }
    }
}

/// @trace SPEC: SPEC-016
/// @trace FR: PRD-016/FR-3
fn build_registry() -> AgentRegistry {
    let mut registry = AgentRegistry::new();
    registry.register("passthrough", Arc::new(PassthroughAgent));

    dotenvy::dotenv().ok();
    if let Ok(llm_config) = agent_core::config::AzureOpenAiConfig::from_env() {
        let base = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let eval_config = match eval_harness::data_paths::load_evaluation_config(&base) {
            | Ok(cfg) => cfg,
            | Err(e) => {
                eprintln!("[warn] eval-harness.toml [evaluation] 파싱 실패: {e} — 기본값 사용");
                agent_core::config::EvaluationConfig::default()
            },
        };
        println!(
            "[cfg] PPA 설정: max_iterations={}, early_stop_threshold={}",
            eval_config.max_iterations, eval_config.early_stop_threshold
        );
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
            eval_scenario,
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

            let paths = resolve_data_paths(scenarios_dir.as_deref(), None);
            install_data_store(&paths);
            let scenarios_dir = paths.scenarios_dir.to_string_lossy().into_owned();
            let mut runner = HarnessRunner::new(&output_dir);
            let report = match runner.run_eval_scenario(&eval_scenario, agent_impl.as_ref(), &scenarios_dir) {
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
            baseline_task,
            current_task,
            agent,
            baseline_since,
            baseline_until,
            current_since,
            current_until,
        } => {
            let comparator = ReportComparator::new(threshold);
            // SPEC-021: 입력 모드 결정. 우선순위 = 시간 범위 > task_id > 파일.
            let result = if agent.is_some() && baseline_since.is_some() {
                // 시간 범위 모드
                let paths = resolve_data_paths(None, None);
                install_data_store(&paths);
                let agent = agent.unwrap();
                let bs = baseline_since.unwrap();
                let bu = baseline_until.expect("--baseline-until 필요");
                let cs = current_since.expect("--current-since 필요");
                let cu = current_until.expect("--current-until 필요");
                match eval_harness::compare_db::compare_windows(&agent, &bs, &bu, &cs, &cu, threshold) {
                    | Ok(r) => r,
                    | Err(e) => {
                        eprintln!("비교 오류: {}", e);
                        std::process::exit(1);
                    },
                }
            } else if let (Some(bt), Some(ct)) = (baseline_task.as_ref(), current_task.as_ref()) {
                // task_id 모드
                let paths = resolve_data_paths(None, None);
                install_data_store(&paths);
                match eval_harness::compare_db::compare_two_tasks(bt, ct, threshold) {
                    | Ok(r) => r,
                    | Err(e) => {
                        eprintln!("비교 오류: {}", e);
                        std::process::exit(1);
                    },
                }
            } else if !baseline.is_empty() && !current.is_empty() {
                // 파일 모드 (기존)
                match comparator.compare_files(&baseline, &current) {
                    | Ok(r) => r,
                    | Err(e) => {
                        eprintln!("비교 오류: {}", e);
                        std::process::exit(1);
                    },
                }
            } else {
                eprintln!(
                    "compare: 파일 인자 두 개, --baseline-task/--current-task, 또는 --agent/--baseline-since/--baseline-until/--current-since/--current-until 중 하나의 모드를 지정하세요."
                );
                std::process::exit(2);
            };

            comparator.print_comparison(&result);
            if let Some(out) = output {
                comparator.save_comparison(&result, &out).ok();
            }
            if result.verdict == "fail" {
                std::process::exit(1);
            }
        },

        | Commands::BackfillResults {
            trajectories_dir,
            logs_dir,
        } => {
            let paths = resolve_data_paths(None, None);
            install_data_store(&paths);
            let report = eval_harness::backfill::backfill_results(Path::new(&trajectories_dir), Path::new(&logs_dir));
            println!(
                "[backfill] trajectories: {} 성공 / {} 실패, evaluations: {} 성공 / {} 실패",
                report.trajectories_imported, report.trajectories_failed, report.evaluations_imported, report.evaluations_failed
            );
        },

        | Commands::List {
            scenarios_dir,
        } => {
            use data_scenarios::loader::ScenarioLoader;

            let paths = resolve_data_paths(scenarios_dir.as_deref(), None);
            install_data_store(&paths);
            let scenarios_dir = paths.scenarios_dir.to_string_lossy().into_owned();
            let loader = ScenarioLoader::new();
            if !std::path::Path::new(&scenarios_dir).exists() {
                eprintln!("디렉토리 없음: {}", scenarios_dir);
                std::process::exit(1);
            }

            match loader.load_all_domains(&scenarios_dir) {
                | Ok(configs) => {
                    if configs.is_empty() {
                        println!("등록된 평가 시나리오가 없습니다.");
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

        | Commands::Tui {
            scenarios_dir,
            reports_dir,
        } => {
            let paths = resolve_data_paths(scenarios_dir.as_deref(), None);
            install_data_store(&paths);
            if let Err(e) = tui::run_tui(&paths.scenarios_dir, Path::new(&reports_dir)) {
                eprintln!("TUI 오류: {}", e);
                std::process::exit(1);
            }
        },

        | Commands::Serve {
            addr,
            scenarios_dir,
            reports_dir,
            golden_sets_dir,
            trajectories_dir,
        } => {
            let socket: std::net::SocketAddr = match addr.parse() {
                | Ok(a) => a,
                | Err(e) => {
                    eprintln!("주소 파싱 오류: {}", e);
                    std::process::exit(1);
                },
            };
            let paths = resolve_data_paths(scenarios_dir.as_deref(), golden_sets_dir.as_deref());
            install_data_store(&paths);
            if let Err(e) = web::run_server(
                socket,
                paths.scenarios_dir,
                reports_dir.into(),
                paths.golden_sets_dir,
                trajectories_dir.into(),
                Some(paths.db_path.clone()),
            ) {
                eprintln!("서버 오류: {}", e);
                std::process::exit(1);
            }
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
