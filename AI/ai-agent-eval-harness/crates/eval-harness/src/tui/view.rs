// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-2, FR-3
// @trace file-type: impl
// =============================================================================

use super::state::{Focus,
                   TuiState};
use ratatui::{Frame,
              layout::{Constraint,
                       Direction,
                       Layout},
              style::{Color,
                      Modifier,
                      Style},
              text::Line,
              widgets::{Block,
                        Borders,
                        List,
                        ListItem,
                        ListState,
                        Paragraph}};

/// TUI 화면을 렌더링한다.
///
/// @trace SPEC: SPEC-001
/// @trace TC: -
/// @trace FR: PRD-001/FR-2, PRD-001/FR-3
pub fn draw(f: &mut Frame, state: &TuiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(f.area());

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    render_list(f, body[0], "Scenarios", &state.scenarios, state.scenario_idx, state.focus == Focus::Scenarios);
    render_list(f, body[1], "Reports", &state.reports, state.report_idx, state.focus == Focus::Reports);

    let help = Paragraph::new(Line::from("↑/↓ or j/k: move  |  Tab: switch panel  |  q/Esc: quit")).block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[1]);
}

fn render_list(f: &mut Frame, area: ratatui::layout::Rect, title: &str, items: &[String], selected: usize, focused: bool) {
    let list_items: Vec<ListItem> = items.iter().map(|s| ListItem::new(s.as_str())).collect();
    let border_style = if focused { Style::default().fg(Color::Yellow) } else { Style::default() };
    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title).border_style(border_style))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default();
    if !items.is_empty() {
        list_state.select(Some(selected));
    }
    f.render_stateful_widget(list, area, &mut list_state);
}
