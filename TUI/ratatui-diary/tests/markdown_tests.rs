use ratatui_diary::markdown::render_to_text;

#[test]
fn test_render_empty_string() {
    let result = render_to_text("");
    assert!(!result.lines.is_empty()); // 최소한 빈 줄은 있어야 함
}

#[test]
fn test_render_plain_text() {
    let result = render_to_text("Hello, World!");
    // 텍스트가 포함되어 있는지 확인 (정확한 형식은 구현 후 조정)
    assert!(result.lines.len() > 0);
}
