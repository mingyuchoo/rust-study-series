use ratatui::text::{Line, Span, Text};
use ratatui::style::{Color, Modifier, Style};

/// Markdown 문자열을 ratatui Text로 렌더링
pub fn render_to_text(markdown: &str) -> Text<'static> {
    if markdown.is_empty() {
        return Text::from(vec![Line::from("")]);
    }

    // 임시: 단순 텍스트로 반환 (termimad 통합 전)
    let lines: Vec<Line> = markdown
        .lines()
        .map(|line| Line::from(line.to_string()))
        .collect();

    Text::from(lines)
}
