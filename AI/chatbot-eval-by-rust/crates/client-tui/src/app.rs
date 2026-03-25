//! TUI 앱 상태 및 이벤트 타입

use tokio::sync::mpsc;

/// 평가 도구 목록
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EvalTool {
    LlmJudge,
    Ragas,
    Safety,
    Langfuse,
    Promptfoo,
    All,
}

pub const TOOLS: &[EvalTool] = &[
    EvalTool::LlmJudge,
    EvalTool::Ragas,
    EvalTool::Safety,
    EvalTool::Langfuse,
    EvalTool::Promptfoo,
    EvalTool::All,
];

impl EvalTool {
    pub fn label(self) -> &'static str {
        match self {
            | Self::LlmJudge => "LLM-as-Judge",
            | Self::Ragas => "RAGAS",
            | Self::Safety => "Safety",
            | Self::Langfuse => "Langfuse",
            | Self::Promptfoo => "Promptfoo",
            | Self::All => "전체 평가",
        }
    }
}

/// 실행 상태
#[derive(Clone, PartialEq)]
pub enum RunState {
    Idle,
    Running,
    Done,
    Failed(String),
}

/// 백그라운드 태스크 → UI 메시지
pub enum LogMsg {
    Line(String),
    Done,
    Failed(String),
}

/// TUI 앱 전체 상태
pub struct App {
    pub selected: usize,
    pub save: bool,
    pub use_golden_json: bool,
    pub run_state: RunState,
    pub logs: Vec<String>,
    pub log_tx: mpsc::UnboundedSender<LogMsg>,
    pub log_rx: mpsc::UnboundedReceiver<LogMsg>,
}

impl App {
    pub fn new() -> Self {
        let (log_tx, log_rx) = mpsc::unbounded_channel();
        Self {
            selected: 0,
            save: false,
            use_golden_json: false,
            run_state: RunState::Idle,
            logs: vec!["도구를 선택하고 Enter를 눌러 평가를 실행하세요.".to_string()],
            log_tx,
            log_rx,
        }
    }

    pub fn selected_tool(&self) -> EvalTool { TOOLS[self.selected] }

    pub fn move_up(&mut self) { self.selected = self.selected.saturating_sub(1); }

    pub fn move_down(&mut self) {
        if self.selected + 1 < TOOLS.len() {
            self.selected += 1;
        }
    }

    pub fn toggle_save(&mut self) { self.save = !self.save; }

    pub fn toggle_golden_json(&mut self) { self.use_golden_json = !self.use_golden_json; }

    /// 채널에서 로그 메시지를 수신해 상태에 반영한다.
    pub fn drain_logs(&mut self) {
        while let Ok(msg) = self.log_rx.try_recv() {
            match msg {
                | LogMsg::Line(line) => self.logs.push(line),
                | LogMsg::Done => {
                    self.logs.push("─".repeat(44));
                    self.logs.push("완료".to_string());
                    self.run_state = RunState::Done;
                },
                | LogMsg::Failed(e) => {
                    self.logs.push(format!("오류: {e}"));
                    self.run_state = RunState::Failed(e);
                },
            }
        }
    }
}
