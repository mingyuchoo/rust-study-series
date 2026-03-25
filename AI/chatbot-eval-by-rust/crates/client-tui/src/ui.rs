//! TUI 렌더링

use crate::app::{App,
                 RunState,
                 TOOLS};
use ratatui::{Frame,
              layout::{Constraint,
                       Direction,
                       Layout,
                       Rect},
              style::{Color,
                      Modifier,
                      Style},
              text::{Line,
                     Span},
              widgets::{Block,
                        Borders,
                        List,
                        ListItem,
                        Paragraph,
                        Wrap}};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // 수직 분할: 제목(3) | 본문(가변) | 상태바(1)
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(1)])
        .split(area);

    render_title(f, root[0]);

    // 본문 수평 분할: 도구 메뉴(24) | 오른쪽 패널(가변)
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(30)])
        .split(root[1]);

    // 오른쪽 수직 분할: 옵션(8) | 로그(가변)
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(body[1]);

    render_tool_menu(f, body[0], app);
    render_options(f, right[0], app);
    render_logs(f, right[1], app);
    render_status_bar(f, root[2]);
}

fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("  AI Agent 테스트 평가 TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_tool_menu(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = TOOLS
        .iter()
        .enumerate()
        .map(|(i, tool)| {
            let label = tool.label();
            if i == app.selected {
                ListItem::new(format!(" ▶ {label}")).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                ListItem::new(format!("   {label}")).style(Style::default().fg(Color::White))
            }
        })
        .collect();

    let list = List::new(items).block(Block::default().title(" 평가 도구 ").borders(Borders::ALL));

    f.render_widget(list, area);
}

fn render_options(f: &mut Frame, area: Rect, app: &App) {
    let on_style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);
    let off_style = Style::default().fg(Color::DarkGray);

    let (state_label, state_color) = match &app.run_state {
        | RunState::Idle => ("대기 중", Color::Gray),
        | RunState::Running => ("실행 중...", Color::Yellow),
        | RunState::Done => ("완료", Color::Green),
        | RunState::Failed(_) => ("실패", Color::Red),
    };

    let text = vec![
        Line::from(vec![
            Span::raw("  결과 저장  : "),
            Span::styled(if app.save { "[ON ]" } else { "[OFF]" }, if app.save { on_style } else { off_style }),
            Span::styled("  (s)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::raw("  Golden JSON: "),
            Span::styled(
                if app.use_golden_json { "[ON ]" } else { "[OFF]" },
                if app.use_golden_json { on_style } else { off_style },
            ),
            Span::styled("  (g)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  상태: "),
            Span::styled(state_label, Style::default().fg(state_color).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled("  Enter: 실행   q/Ctrl+C: 종료", Style::default().fg(Color::DarkGray))),
    ];

    let para = Paragraph::new(text).block(Block::default().title(" 옵션 ").borders(Borders::ALL)).wrap(Wrap {
        trim: false,
    });
    f.render_widget(para, area);
}

fn render_logs(f: &mut Frame, area: Rect, app: &App) {
    // 테두리 제외한 표시 가능 행 수
    let visible = area.height.saturating_sub(2) as usize;
    let total = app.logs.len();
    let start = total.saturating_sub(visible);

    let lines: Vec<Line> = app.logs[start ..].iter().map(|l| Line::from(format!("  {l}"))).collect();

    let para = Paragraph::new(lines).block(Block::default().title(" 로그 ").borders(Borders::ALL)).wrap(Wrap {
        trim: false,
    });
    f.render_widget(para, area);
}

fn render_status_bar(f: &mut Frame, area: Rect) {
    let bar =
        Paragraph::new(" ↑↓/jk: 선택   Enter: 실행   s: 저장 토글   g: 데이터셋 토글   q: 종료").style(Style::default().bg(Color::DarkGray).fg(Color::White));
    f.render_widget(bar, area);
}
