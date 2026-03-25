//! AI Agent 테스트 평가 TUI
//!
//! ratatui + crossterm 기반 터미널 UI.
//! 평가 도구 선택 → Enter로 실행 → 로그 실시간 확인.
//!
//! # stdout 분리 전략
//!
//! eval 함수 내부의 `println!()` 이 TUI 화면을 오염시키는 문제를 방지하기 위해
//! ratatui 백엔드는 `/dev/tty` 를 직접 사용하고, fd 1(stdout) 은 `/dev/null` 로
//! 리다이렉트한다. crossterm 의 터미널 크기 감지는 macOS 에서 stdin(fd 0) 을
//! 사용하므로 fd 1 리다이렉트의 영향을 받지 않는다.

mod app;
mod ui;

use anyhow::Result;
use app::{App,
          EvalTool,
          LogMsg,
          RunState,
          TOOLS};
use crossterm::{cursor::{Hide,
                         Show},
                event::{Event,
                        EventStream,
                        KeyCode,
                        KeyModifiers},
                execute,
                terminal::{EnterAlternateScreen,
                           LeaveAlternateScreen,
                           disable_raw_mode,
                           enable_raw_mode}};
use eval_runner::{config::EvalConfig,
                  legacy_golden_dataset,
                  load_test_cases,
                  run_langfuse_evaluation,
                  run_llm_as_judge,
                  run_promptfoo_evaluation,
                  run_ragas_evaluation,
                  run_safety_evaluation,
                  save_results};
use futures::StreamExt;
use ratatui::{Terminal,
              backend::CrosstermBackend};
use std::{fs::File,
          time::Duration};
use tokio::sync::mpsc;

// ── 터미널 헬퍼 ──────────────────────────────────────────────────────────────

/// `/dev/tty` 를 쓰기 전용으로 열어 반환한다.
fn open_tty() -> Result<File> { Ok(std::fs::OpenOptions::new().write(true).open("/dev/tty")?) }

/// fd 1(stdout) 을 `/dev/null` 로 리다이렉트하고, 원래 fd 를 반환한다.
///
/// eval 함수 내부의 `println!()` 이 TUI 화면에 출력되지 않도록 한다.
fn suppress_stdout() -> i32 {
    unsafe {
        let null_fd = libc::open(c"/dev/null".as_ptr(), libc::O_WRONLY);
        let saved = libc::dup(1);
        libc::dup2(null_fd, 1);
        libc::close(null_fd);
        saved
    }
}

/// `suppress_stdout` 으로 저장한 fd 로 stdout 을 복원한다.
fn restore_stdout(saved: i32) {
    unsafe {
        libc::dup2(saved, 1);
        libc::close(saved);
    }
}

// ── 진입점 ───────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("오류: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let _ = dotenvy::dotenv();

    enable_raw_mode()?;

    // 얼터네이트 스크린 진입 및 커서 숨김 (/dev/tty 에 직접 씀)
    let mut tty = open_tty()?;
    execute!(tty, EnterAlternateScreen, Hide)?;

    // eval 함수의 println! 을 /dev/null 로 억제
    let saved_stdout = suppress_stdout();

    // ratatui 백엔드는 /dev/tty 사용 (fd 1 과 독립)
    let backend = CrosstermBackend::new(open_tty()?);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app).await;

    // stdout 복원 후 정리
    restore_stdout(saved_stdout);

    let mut tty = open_tty()?;
    execute!(tty, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    result
}

// ── 이벤트 루프 ──────────────────────────────────────────────────────────────

async fn run_app(terminal: &mut Terminal<CrosstermBackend<File>>, app: &mut App) -> Result<()> {
    let mut events = EventStream::new();

    loop {
        app.drain_logs();
        // /dev/tty 에 직접 렌더링 — terminal.clear() 불필요, 깜빡임 없음
        terminal.draw(|f| ui::render(f, app))?;

        tokio::select! {
            maybe_event = events.next() => {
                match maybe_event {
                    | Some(Ok(Event::Key(key))) => {
                        if is_quit(&key) {
                            break;
                        }
                        handle_key(app, key.code).await;
                    },
                    | None => break,
                    | _ => {},
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(50)) => {}
        }
    }

    Ok(())
}

fn is_quit(key: &crossterm::event::KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char('q')) || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
}

async fn handle_key(app: &mut App, code: KeyCode) {
    match code {
        | KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        | KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        | KeyCode::Char('s') => app.toggle_save(),
        | KeyCode::Char('g') => app.toggle_golden_json(),
        | KeyCode::Enter =>
            if app.run_state != RunState::Running {
                start_evaluation(app).await;
            },
        | _ => {},
    }
}

// ── 평가 실행 ────────────────────────────────────────────────────────────────

async fn start_evaluation(app: &mut App) {
    app.run_state = RunState::Running;
    app.logs.push(format!("─ {} 평가 시작 ─", TOOLS[app.selected].label()));

    let tool = app.selected_tool();
    let save = app.save;
    let use_golden = app.use_golden_json;
    let tx = app.log_tx.clone();

    tokio::spawn(async move {
        if let Err(e) = run_evaluation(tool, save, use_golden, tx.clone()).await {
            let _ = tx.send(LogMsg::Failed(e.to_string()));
        } else {
            let _ = tx.send(LogMsg::Done);
        }
    });
}

async fn run_evaluation(tool: EvalTool, save: bool, use_golden: bool, tx: mpsc::UnboundedSender<LogMsg>) -> Result<()> {
    let eval_config = EvalConfig::from_cwd();

    let samples = if use_golden {
        match load_test_cases(&eval_config.golden_dataset_path) {
            | Ok(s) => {
                let _ = tx.send(LogMsg::Line(format!("데이터셋: golden_dataset.json ({}개)", s.len())));
                s
            },
            | Err(_) => {
                let _ = tx.send(LogMsg::Line("golden_dataset.json 없음 → 레거시 사용".into()));
                legacy_golden_dataset()
            },
        }
    } else {
        let s = legacy_golden_dataset();
        let _ = tx.send(LogMsg::Line(format!("데이터셋: 레거시 드래곤볼 ({}개)", s.len())));
        s
    };

    match tool {
        | EvalTool::LlmJudge => {
            let _ = tx.send(LogMsg::Line("LLM-as-Judge 평가 중...".into()));
            let results = run_llm_as_judge(&samples).await?;
            let _ = tx.send(LogMsg::Line(format!("결과: {}개 샘플 평가 완료", results.len())));
            if save {
                let json = serde_json::to_value(&results)?;
                save_results(&json, "llm_judge_results.json", &eval_config.results_dir)?;
                let _ = tx.send(LogMsg::Line("llm_judge_results.json 저장됨".into()));
            }
        },

        | EvalTool::Ragas => {
            let _ = tx.send(LogMsg::Line("RAGAS 평가 중...".into()));
            let result = run_ragas_evaluation(&samples).await?;
            if let Some(q) = result.response_quality {
                let _ = tx.send(LogMsg::Line(format!("response_quality: {q:.3}")));
            }
            if save {
                let json = serde_json::to_value(&result)?;
                save_results(&json, "ragas_results.json", &eval_config.results_dir)?;
                let _ = tx.send(LogMsg::Line("ragas_results.json 저장됨".into()));
            }
        },

        | EvalTool::Safety => {
            let _ = tx.send(LogMsg::Line("Safety 평가 중...".into()));
            let report = run_safety_evaluation(None, None).await?;
            let _ = tx.send(LogMsg::Line(format!(
                "통과: {}/{} ({:.1}%)",
                report.passed,
                report.total,
                report.pass_rate * 100.0,
            )));
            if save {
                let json = serde_json::to_value(&report)?;
                save_results(&json, "safety_results.json", &eval_config.results_dir)?;
                let _ = tx.send(LogMsg::Line("safety_results.json 저장됨".into()));
            }
        },

        | EvalTool::Langfuse => {
            let _ = tx.send(LogMsg::Line("Langfuse 평가 중...".into()));
            let result = run_langfuse_evaluation(&samples).await?;
            let _ = tx.send(LogMsg::Line(format!("평균 키워드 일치율: {:.1}%", result.avg_keyword_overlap * 100.0,)));
            if save {
                let json = serde_json::to_value(&result)?;
                save_results(&json, "langfuse_results.json", &eval_config.results_dir)?;
                let _ = tx.send(LogMsg::Line("langfuse_results.json 저장됨".into()));
            }
        },

        | EvalTool::Promptfoo => {
            let _ = tx.send(LogMsg::Line("Promptfoo 평가 중...".into()));
            let result = run_promptfoo_evaluation(None).await?;
            if save {
                save_results(&result, "promptfoo_results.json", &eval_config.results_dir)?;
                let _ = tx.send(LogMsg::Line("promptfoo_results.json 저장됨".into()));
            }
        },

        | EvalTool::All => {
            let _ = tx.send(LogMsg::Line("LLM-as-Judge 평가 중...".into()));
            let llm = run_llm_as_judge(&samples).await?;
            let _ = tx.send(LogMsg::Line(format!("LLM-as-Judge: {}개 완료", llm.len())));

            let _ = tx.send(LogMsg::Line("Safety 평가 중...".into()));
            let safety = run_safety_evaluation(None, None).await?;
            let _ = tx.send(LogMsg::Line(format!("Safety: {}/{} 통과", safety.passed, safety.total)));

            if save {
                let json = serde_json::to_value(&llm)?;
                save_results(&json, "llm_judge_results.json", &eval_config.results_dir)?;
                let json = serde_json::to_value(&safety)?;
                save_results(&json, "safety_results.json", &eval_config.results_dir)?;
                let _ = tx.send(LogMsg::Line("결과 파일 저장됨".into()));
            }
        },
    }

    Ok(())
}
