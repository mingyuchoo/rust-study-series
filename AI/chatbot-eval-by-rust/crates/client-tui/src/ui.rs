//! TUI 렌더링

use crate::app::{App,
                 RunState,
                 Screen,
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

// ── 진입점 ───────────────────────────────────────────────────────────────────

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // 수직 분할: 제목(3) | 본문(가변) | 상태바(1)
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(1)])
        .split(area);

    render_title(f, root[0], app);

    match app.screen {
        | Screen::Run => render_run_screen(f, root[1], app),
        | Screen::Results => render_results_screen(f, root[1], app),
    }

    render_status_bar(f, root[2], app);
}

// ── 공통 위젯 ────────────────────────────────────────────────────────────────

fn render_title(f: &mut Frame, area: Rect, app: &App) {
    let (run_style, res_style) = match app.screen {
        | Screen::Run => (
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
            Style::default().fg(Color::DarkGray),
        ),
        | Screen::Results => (
            Style::default().fg(Color::DarkGray),
            Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
    };

    let title_line = Line::from(vec![
        Span::styled("  AI Agent 테스트 평가 TUI  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(" 평가 실행 ", run_style),
        Span::raw("  "),
        Span::styled(" 결과 보기 ", res_style),
        Span::styled("  (Tab 전환)", Style::default().fg(Color::DarkGray)),
    ]);

    let title = Paragraph::new(title_line).block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let hint = match app.screen {
        | Screen::Run => " Tab:결과보기  ↑↓/jk:도구선택  Enter:실행  s:저장  g:데이터셋  q:종료",
        | Screen::Results => " Tab:실행화면  ↑↓/jk:파일선택  PgUp/PgDn:스크롤  r:새로고침  q:종료",
    };
    let bar = Paragraph::new(hint).style(Style::default().bg(Color::DarkGray).fg(Color::White));
    f.render_widget(bar, area);
}

// ── 실행 화면 ────────────────────────────────────────────────────────────────

fn render_run_screen(f: &mut Frame, area: Rect, app: &App) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(30)])
        .split(area);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(body[1]);

    render_tool_menu(f, body[0], app);
    render_options(f, right[0], app);
    render_logs(f, right[1], app);
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
    let visible = area.height.saturating_sub(2) as usize;
    let total = app.logs.len();
    let start = total.saturating_sub(visible);
    let lines: Vec<Line> = app.logs[start ..].iter().map(|l| Line::from(format!("  {l}"))).collect();
    let para = Paragraph::new(lines).block(Block::default().title(" 로그 ").borders(Borders::ALL)).wrap(Wrap {
        trim: false,
    });
    f.render_widget(para, area);
}

// ── 결과 화면 ────────────────────────────────────────────────────────────────

fn render_results_screen(f: &mut Frame, area: Rect, app: &App) {
    // 상단(파일목록+통계) | 하단(상세)
    let vsplit = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(5)])
        .split(area);

    // 상단: 파일 목록 | 통계
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(30)])
        .split(vsplit[0]);

    render_result_file_list(f, top[0], app);
    render_result_stats(f, top[1], app);
    render_result_detail(f, vsplit[1], app);
}

fn render_result_file_list(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = if app.result_files.is_empty() {
        vec![ListItem::new("  (결과 파일 없음)").style(Style::default().fg(Color::DarkGray))]
    } else {
        app.result_files
            .iter()
            .enumerate()
            .map(|(i, rf)| {
                let has_data = rf.data.is_some();
                let icon = if has_data { "○" } else { "×" };
                let label = format!(" {icon} {}", rf.name);
                if i == app.result_selected {
                    ListItem::new(label).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                } else {
                    let color = if has_data { Color::White } else { Color::DarkGray };
                    ListItem::new(label).style(Style::default().fg(color))
                }
            })
            .collect()
    };

    let list = List::new(items).block(Block::default().title(" 결과 파일 ").borders(Borders::ALL));
    f.render_widget(list, area);
}

fn render_result_stats(f: &mut Frame, area: Rect, app: &App) {
    let lines: Vec<Line> = if app.result_files.is_empty() {
        vec![Line::from(Span::styled(
            "  eval_results/ 디렉토리에 결과 파일이 없습니다.",
            Style::default().fg(Color::DarkGray),
        ))]
    } else if let Some(rf) = app.result_files.get(app.result_selected) {
        rf.stats_lines()
            .into_iter()
            .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::Green))))
            .collect()
    } else {
        vec![]
    };

    let title = app
        .result_files
        .get(app.result_selected)
        .map(|rf| format!(" 통계 — {} ", rf.name))
        .unwrap_or_else(|| " 통계 ".into());

    let para = Paragraph::new(lines).block(Block::default().title(title).borders(Borders::ALL)).wrap(Wrap {
        trim: false,
    });
    f.render_widget(para, area);
}

fn render_result_detail(f: &mut Frame, area: Rect, app: &App) {
    let visible = area.height.saturating_sub(2) as usize;

    let (detail_lines, title) = if let Some(rf) = app.result_files.get(app.result_selected) {
        (rf.detail_lines(), format!(" 상세 내용 — {} ", rf.name))
    } else {
        (vec!["파일을 선택하세요.".into()], " 상세 내용 ".into())
    };

    let total_lines = detail_lines.len();
    // 스크롤 범위 클램프
    let scroll = app.detail_scroll.min(total_lines.saturating_sub(visible));

    let lines: Vec<Line> = detail_lines[scroll ..].iter().map(|l| Line::from(format!("  {l}"))).collect();

    // 스크롤 위치 표시
    let scroll_indicator = if total_lines > visible {
        format!(" {scroll}/{}", total_lines.saturating_sub(visible))
    } else {
        String::new()
    };
    let full_title = format!("{title}{scroll_indicator}");

    let para = Paragraph::new(lines)
        .block(Block::default().title(full_title).borders(Borders::ALL))
        .wrap(Wrap {
            trim: false,
        });
    f.render_widget(para, area);
}
