use crate::{markdown::render_to_text,
            model::{CalendarState,
                    CalendarSubMode,
                    EditorMode,
                    EditorState,
                    EditorSubMode,
                    Model,
                    Screen}};
use unicode_width::UnicodeWidthChar;

use ratatui::{Frame,
              layout::{Alignment,
                       Constraint,
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
                        Clear,
                        Paragraph,
                        Wrap}};

/// 선택 영역 타입: ((시작 라인, 시작 컬럼), (끝 라인, 끝 컬럼))
type SelectionRange = Option<((usize, usize), (usize, usize))>;

/// 달력 화면의 현재 모드에 맞는 키바인딩 도움말 텍스트 생성
pub fn build_calendar_keybindings(state: &CalendarState) -> String {
    match state.submode {
        | None => "hjkl:이동 | e:편집 | space:명령 | q:종료".to_string(),
        | Some(CalendarSubMode::Space) => "n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소".to_string(),
    }
}

/// 에디터 화면의 현재 모드에 맞는 키바인딩 도움말 텍스트 생성
pub fn build_editor_keybindings(state: &EditorState) -> String {
    match state.mode {
        | EditorMode::Normal => match &state.submode {
            | None => "hjkl:이동 | w/b/e:단어 | i/a/o/O:입력 | v/x:선택 | d/c/y/p:편집 | u/U:실행취소 | g/space//:모드 | Esc:뒤로".to_string(),
            | Some(EditorSubMode::Goto) => "g:문서시작 | e:문서끝 | h:줄시작 | l:줄끝 | Esc:취소".to_string(),
            | Some(EditorSubMode::SpaceCommand) => "w:저장 | q:뒤로 | x:저장후뒤로 | Esc:취소".to_string(),
            | Some(EditorSubMode::Search) => "입력:검색어 | Enter:실행 | n/N:다음/이전 | Esc:취소".to_string(),
        },
        | EditorMode::Insert => "입력중... | Enter:새줄 | Backspace:삭제 | Esc:Normal모드".to_string(),
    }
}

pub fn view(f: &mut Frame, model: &Model) {
    match model.screen {
        | Screen::Calendar => render_calendar(f, model),
        | Screen::Editor => render_editor(f, model),
    }

    // 에러 팝업
    if model.show_error_popup {
        render_error_popup(f, model);
    }
}

fn render_calendar(f: &mut Frame, model: &Model) {
    // 메인 레이아웃: 수평 분할 (50:50)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // 왼쪽: 달력
            Constraint::Percentage(50), // 오른쪽: 미리보기
        ])
        .split(f.size());

    // 왼쪽: 달력 영역 (기존 레이아웃)
    let calendar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 달력
            Constraint::Length(2), // 상태바
        ])
        .split(main_chunks[0]);

    // 헤더
    let header = Paragraph::new(format!("{}년 {}월", model.calendar_state.current_year, model.calendar_state.current_month))
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, calendar_chunks[0]);

    // 달력 그리드
    render_calendar_grid(f, calendar_chunks[1], model);

    // 상태바 - 동적 키바인딩
    let keybindings = build_calendar_keybindings(&model.calendar_state);
    let statusbar = Paragraph::new(keybindings).alignment(Alignment::Center);
    f.render_widget(statusbar, calendar_chunks[2]);

    // 오른쪽: 미리보기 영역
    let selected_date = model.calendar_state.selected_date;
    let preview_content = match model.storage.load(selected_date) {
        | Ok(content) => content,
        | Err(_) => "📝 작성된 다이어리가 없습니다.\n\nEnter를 눌러 새로 작성하세요.".to_string(),
    };

    render_preview_pane(f, main_chunks[1], &preview_content, &format!("다이어리: {}", selected_date));
}

fn render_preview_pane(f: &mut Frame, area: Rect, content: &str, title: &str) {
    let text = Paragraph::new(content)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap {
            trim: false,
        })
        .style(Style::default());

    f.render_widget(text, area);
}

fn render_calendar_grid(f: &mut Frame, area: Rect, model: &Model) {
    use chrono::{Datelike,
                 NaiveDate};

    let year = model.calendar_state.current_year;
    let month = model.calendar_state.current_month;

    // 요일 헤더
    let weekdays = ["일", "월", "화", "수", "목", "금", "토"];
    let mut lines = vec![Line::from(
        weekdays
            .iter()
            .map(|&day| Span::styled(format!("{:^4}", day), Style::default()))
            .collect::<Vec<_>>(),
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
    for slot in week.iter_mut().take(7).skip(weekday) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        *slot = format_day(day, date, model);
        day += 1;
    }
    lines.push(Line::from(week.clone()));

    // 나머지 주
    while day <= days_in_month {
        week = vec![Span::raw("    "); 7];
        for slot in week.iter_mut().take(7) {
            if day <= days_in_month {
                let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                *slot = format_day(day, date, model);
                day += 1;
            }
        }
        lines.push(Line::from(week.clone()));
    }

    let calendar = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
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
    // 메인 레이아웃: 수평 분할 (50:50)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // 왼쪽: 에디터
            Constraint::Percentage(50), // 오른쪽: Markdown 미리보기
        ])
        .split(f.size());

    // 왼쪽: 에디터 영역 (기존 레이아웃)
    let editor_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // 날짜 헤더
            Constraint::Min(0),    // 에디터 영역
            Constraint::Length(1), // 모드 표시
        ])
        .split(main_chunks[0]);

    // 헤더: 날짜
    let header = Paragraph::new(model.editor_state.date.to_string()).style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, editor_chunks[0]);

    // 에디터 내용 - 스타일이 적용된 라인들로 렌더링
    let styled_lines = render_editor_content(&model.editor_state);
    let text = Paragraph::new(styled_lines).wrap(Wrap {
        trim: false,
    });
    f.render_widget(text, editor_chunks[1]);

    // 커서 표시 (Insert와 Normal 모드 모두)
    // CJK 문자(한글 등)는 터미널에서 2칸을 차지하므로, cursor_col(문자 수)이 아닌
    // 실제 표시 너비(display width)를 사용하여 커서 X 좌표를 계산해야 합니다.
    let display_width: u16 = if model.editor_state.cursor_line < model.editor_state.content.len() {
        model.editor_state.content[model.editor_state.cursor_line]
            .chars()
            .take(model.editor_state.cursor_col)
            .map(|c| UnicodeWidthChar::width(c).unwrap_or(0) as u16)
            .sum()
    } else {
        0
    };
    let cursor_x = editor_chunks[1].x + display_width;
    let cursor_y = editor_chunks[1].y + model.editor_state.cursor_line as u16;
    f.set_cursor(cursor_x, cursor_y);

    // 하단바: 모드 정보와 키바인딩 표시
    let mode_text = build_status_text(&model.editor_state);
    let keybindings = build_editor_keybindings(&model.editor_state);
    let status_text = format!("{} | {}", mode_text, keybindings);
    let statusbar = Paragraph::new(status_text).style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(statusbar, editor_chunks[2]);

    // 오른쪽: Markdown 미리보기
    let content = model.editor_state.get_content();
    render_markdown_preview(f, main_chunks[1], &content);
}

fn render_markdown_preview(f: &mut Frame, area: Rect, markdown: &str) {
    let rendered_text = render_to_text(markdown);

    let preview = Paragraph::new(rendered_text)
        .block(
            Block::default()
                .title("Markdown 미리보기")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap {
            trim: false,
        });

    f.render_widget(preview, area);
}

fn render_error_popup(f: &mut Frame, model: &Model) {
    let area = centered_rect(60, 20, f.size());

    let error_msg = model.error_message.as_deref().unwrap_or("알 수 없는 에러");
    let popup = Paragraph::new(error_msg)
        .block(
            Block::default()
                .title("Error")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .style(Style::default().bg(Color::Black))
        .wrap(Wrap {
            trim: true,
        });

    f.render_widget(Clear, area);
    f.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// 에디터 내용을 스타일이 적용된 Line 벡터로 변환
fn render_editor_content(editor_state: &EditorState) -> Vec<Line<'static>> {
    let selection_range = editor_state.get_selection_range();

    editor_state
        .content
        .iter()
        .enumerate()
        .map(|(line_idx, line_text)| {
            let mut spans = Vec::new();
            let chars: Vec<char> = line_text.chars().collect();

            let mut col_idx = 0;
            while col_idx < chars.len() {
                let ch = chars[col_idx];

                // 현재 위치의 스타일 결정
                let style = get_char_style(
                    line_idx,
                    col_idx,
                    &selection_range,
                    &editor_state.search_matches,
                    editor_state.current_match_index,
                    &editor_state.search_pattern,
                );

                spans.push(Span::styled(ch.to_string(), style));
                col_idx += 1;
            }

            // 빈 줄 처리
            if spans.is_empty() {
                spans.push(Span::raw(" "));
            }

            Line::from(spans)
        })
        .collect()
}

/// 문자의 스타일 결정 (선택 영역, 검색 매치 등)
fn get_char_style(
    line: usize,
    col: usize,
    selection_range: &SelectionRange,
    search_matches: &[(usize, usize)],
    current_match_index: usize,
    search_pattern: &str,
) -> Style {
    let pattern_len = search_pattern.len();

    // 검색 매치 확인
    let (is_match, is_current) = is_search_match(line, col, search_matches, current_match_index, pattern_len);

    // 선택 영역 확인
    let in_selection = is_in_selection(line, col, selection_range);

    // 우선순위: 현재 검색 매치 > 선택 영역 > 다른 검색 매치 > 기본
    if is_current {
        Style::default().bg(Color::LightYellow).fg(Color::Black).add_modifier(Modifier::BOLD)
    } else if in_selection {
        Style::default().bg(Color::DarkGray).fg(Color::White)
    } else if is_match {
        Style::default().bg(Color::Yellow).fg(Color::Black)
    } else {
        Style::default()
    }
}

/// 특정 위치가 선택 영역 내에 있는지 확인
fn is_in_selection(line: usize, col: usize, selection_range: &SelectionRange) -> bool {
    if let Some(((start_line, start_col), (end_line, end_col))) = selection_range {
        if line < *start_line || line > *end_line {
            return false;
        }

        if *start_line == *end_line {
            // 같은 줄
            col >= *start_col && col < *end_col
        } else if line == *start_line {
            // 시작 줄
            col >= *start_col
        } else if line == *end_line {
            // 끝 줄
            col < *end_col
        } else {
            // 중간 줄
            true
        }
    } else {
        false
    }
}

/// 특정 위치가 검색 매치인지 확인 (현재 매치인지도 함께 반환)
fn is_search_match(line: usize, col: usize, matches: &[(usize, usize)], current_match_index: usize, pattern_len: usize) -> (bool, bool) {
    for (idx, (match_line, match_col)) in matches.iter().enumerate() {
        if *match_line == line && col >= *match_col && col < match_col + pattern_len {
            let is_current = idx == current_match_index;
            return (true, is_current);
        }
    }
    (false, false)
}

/// 상태바 텍스트 생성 (모드, submode, 검색 패턴 등)
fn build_status_text(editor_state: &EditorState) -> String {
    let mode_text = match &editor_state.mode {
        | EditorMode::Normal => "-- NORMAL --",
        | EditorMode::Insert => "-- INSERT --",
    };

    // Submode 표시
    let submode_text = match &editor_state.submode {
        | Some(EditorSubMode::Goto) => " [GOTO]",
        | Some(EditorSubMode::SpaceCommand) => " [SPACE]",
        | Some(EditorSubMode::Search) => {
            return format!("/{}", editor_state.search_pattern);
        },
        | None => "",
    };

    // 검색 매치 정보 표시
    let search_info = if !editor_state.search_matches.is_empty() {
        format!(" | 검색: {}/{} 매치", editor_state.current_match_index + 1, editor_state.search_matches.len())
    } else {
        String::new()
    };

    format!("{}{}{}", mode_text, submode_text, search_info)
}
