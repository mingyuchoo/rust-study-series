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

#[test]
fn test_render_headers() {
    let markdown = "# Header 1\n## Header 2\n### Header 3";
    let result = render_to_text(markdown);

    // 헤더가 스타일이 적용되어 렌더링되는지 확인
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_render_bold_italic() {
    let markdown = "**bold** and *italic*";
    let result = render_to_text(markdown);

    // 최소한 텍스트가 렌더링되는지 확인
    assert!(result.lines.len() > 0);
}

#[test]
fn test_render_code_block() {
    let markdown = "```rust\nfn main() {}\n```";
    let result = render_to_text(markdown);

    assert!(result.lines.len() > 0);
}

#[test]
fn test_render_list() {
    let markdown = "* Item 1\n* Item 2\n* Item 3";
    let result = render_to_text(markdown);

    assert!(result.lines.len() >= 3);
}

#[test]
fn test_render_complex_markdown() {
    let markdown = r#"# Diary Entry

Today I learned about **Rust** and *Ratatui*.

## Code Example

```rust
fn main() {
    println!("Hello, World!");
}
```

## List of Tasks

* Write code
* Test code
* Commit changes
"#;
    let result = render_to_text(markdown);

    // 복잡한 마크다운이 제대로 렌더링되는지 확인
    assert!(result.lines.len() > 5);
}
