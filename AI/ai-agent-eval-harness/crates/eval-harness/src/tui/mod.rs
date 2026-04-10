// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2, FR-3, FR-4
// @trace file-type: impl
// =============================================================================

pub mod state;
pub mod view;

use crossterm::{event::{self,
                        Event},
                execute,
                terminal::{EnterAlternateScreen,
                           LeaveAlternateScreen,
                           disable_raw_mode,
                           enable_raw_mode}};
use ratatui::{Terminal,
              backend::CrosstermBackend};
use state::TuiState;
use std::{io,
          path::Path,
          time::Duration};

/// 터미널을 raw 모드로 전환하고 TUI 메인 루프를 실행한다.
///
/// @trace SPEC: SPEC-001
/// @trace TC: TC-1
/// @trace FR: PRD-001/FR-1
pub fn run_tui(scenarios_dir: &Path, reports_dir: &Path) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, scenarios_dir, reports_dir);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn event_loop<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, scenarios_dir: &Path, reports_dir: &Path) -> io::Result<()> {
    let mut state = TuiState::new(scenarios_dir, reports_dir)?;
    loop {
        terminal.draw(|f| view::draw(f, &state))?;
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                state.handle_key(key.code);
            }
        }
        if state.should_quit {
            return Ok(());
        }
    }
}
