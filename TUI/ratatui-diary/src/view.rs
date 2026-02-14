use crate::model::{Model, Screen, EditorMode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn view(f: &mut Frame, model: &Model) {
    match model.screen {
        Screen::Calendar => render_calendar(f, model),
        Screen::Editor => render_editor(f, model),
    }
}

fn render_calendar(f: &mut Frame, model: &Model) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // 헤더
            Constraint::Min(0),      // 달력
            Constraint::Length(2),   // 상태바
        ])
        .split(f.size());

    // 헤더
    let header = Paragraph::new(format!(
        "{}년 {}월",
        model.calendar_state.current_year,
        model.calendar_state.current_month
    ))
    .alignment(Alignment::Center)
    .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // 달력 그리드
    render_calendar_grid(f, chunks[1], model);

    // 상태바
    let statusbar = Paragraph::new("h/l: 달 | H/L: 연도 | Enter: 작성 | q: 종료")
        .alignment(Alignment::Center);
    f.render_widget(statusbar, chunks[2]);
}

fn render_calendar_grid(f: &mut Frame, area: Rect, model: &Model) {
    use chrono::{Datelike, NaiveDate};

    let year = model.calendar_state.current_year;
    let month = model.calendar_state.current_month;

    // 요일 헤더
    let weekdays = vec!["일", "월", "화", "수", "목", "금", "토"];
    let mut lines = vec![Line::from(
        weekdays.iter()
            .map(|&day| Span::styled(format!("{:^4}", day), Style::default()))
            .collect::<Vec<_>>()
    )];

    // 월의 첫날
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let weekday = first_day.weekday().num_days_from_sunday() as usize;

    // 달력 생성
    let days_in_month = first_day
        .with_month(month + 1)
        .unwrap_or_else(|| first_day.with_year(year + 1).unwrap().with_month(1).unwrap())
        .pred_opt()
        .unwrap()
        .day();

    let mut week = vec![Span::raw("    "); 7];
    let mut day = 1;

    // 첫 주 빈 칸 채우기
    for i in weekday..7 {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        week[i] = format_day(day, date, model);
        day += 1;
    }
    lines.push(Line::from(week.clone()));

    // 나머지 주
    while day <= days_in_month {
        week = vec![Span::raw("    "); 7];
        for i in 0..7 {
            if day <= days_in_month {
                let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                week[i] = format_day(day, date, model);
                day += 1;
            }
        }
        lines.push(Line::from(week.clone()));
    }

    let calendar = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(calendar, area);
}

fn format_day(day: u32, date: chrono::NaiveDate, model: &Model) -> Span<'static> {
    let has_entry = model.diary_entries.entries.contains(&date);
    let is_selected = date == model.calendar_state.selected_date;
    let is_today = date == chrono::Local::now().date_naive();

    let mut style = Style::default();

    if has_entry {
        style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
    }
    if is_selected {
        style = style.bg(Color::Blue);
    }
    if is_today {
        style = style.add_modifier(Modifier::UNDERLINED);
    }

    let marker = if has_entry { "●" } else { " " };
    Span::styled(format!("{:>2}{} ", day, marker), style)
}

fn render_editor(f: &mut Frame, model: &Model) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),   // 날짜 헤더
            Constraint::Min(0),      // 에디터 영역
            Constraint::Length(1),   // 모드 표시
        ])
        .split(f.size());

    // 헤더: 날짜
    let header = Paragraph::new(model.editor_state.date.to_string())
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // 에디터 내용
    let content = model.editor_state.get_content();
    let text = Paragraph::new(content)
        .wrap(Wrap { trim: false });
    f.render_widget(text, chunks[1]);

    // 커서 표시 (Insert 모드)
    if model.editor_state.mode == EditorMode::Insert {
        // 커서 위치 계산
        let cursor_x = chunks[1].x + model.editor_state.cursor_col as u16;
        let cursor_y = chunks[1].y + model.editor_state.cursor_line as u16;
        f.set_cursor(cursor_x, cursor_y);
    }

    // 하단바: 모드 표시
    let mode_text = match &model.editor_state.mode {
        EditorMode::Normal => "-- NORMAL --".to_string(),
        EditorMode::Insert => "-- INSERT --".to_string(),
        EditorMode::Command(cmd) => format!(":{}", cmd),
    };
    let statusbar = Paragraph::new(mode_text)
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(statusbar, chunks[2]);
}
