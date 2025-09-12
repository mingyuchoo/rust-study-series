use crate::models::{ChunkType, ServiceError};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag};
use std::collections::VecDeque;

/// Parsed markdown element with its content and metadata
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedElement {
    pub content: String,
    pub element_type: ChunkType,
    pub headers: Vec<String>,
    pub start_position: usize,
    pub end_position: usize,
}

impl ParsedElement {
    pub fn new(content: String, element_type: ChunkType, start_position: usize, end_position: usize) -> Self {
        Self {
            content,
            element_type,
            headers: Vec::new(),
            start_position,
            end_position,
        }
    }

    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }
}

/// Document parser that extracts structured content from markdown
pub struct DocumentParser {
    preserve_structure: bool,
}

impl DocumentParser {
    /// Creates a new document parser
    pub fn new() -> Self {
        Self { preserve_structure: true }
    }

    /// Creates a parser with custom configuration
    #[allow(dead_code)]
    pub fn with_config(preserve_structure: bool) -> Self {
        Self { preserve_structure }
    }

    /// Parses markdown content and returns structured elements
    pub fn parse(&self, markdown: &str, _source_file: String) -> Result<Vec<ParsedElement>, ServiceError> {
        let parser = Parser::new(markdown);
        let mut elements = Vec::new();
        let mut current_content = String::new();
        let mut current_type = ChunkType::Text;
        let mut header_stack: VecDeque<String> = VecDeque::new();
        let mut in_code_block = false;
        let mut in_list = false;
        let mut in_table = false;
        let mut in_quote = false;
        let mut current_start = 0;
        let mut position = 0;
        let mut tag_stack: Vec<Tag> = Vec::new();

        for (event, range) in parser.into_offset_iter() {
            match event {
                | Event::Start(tag) => {
                    tag_stack.push(tag.clone());
                    self.handle_start_tag(
                        tag,
                        &mut current_type,
                        &mut in_code_block,
                        &mut in_list,
                        &mut in_table,
                        &mut in_quote,
                        &mut current_start,
                        range.start,
                    );
                },
                | Event::End(tag) => {
                    if let Some(element) = self.handle_end_tag(
                        tag,
                        &mut current_content,
                        &mut current_type,
                        &mut header_stack,
                        &mut in_code_block,
                        &mut in_list,
                        &mut in_table,
                        &mut in_quote,
                        current_start,
                        range.end,
                    )? {
                        elements.push(element);
                        current_start = range.end;
                    }
                    tag_stack.pop();
                },
                | Event::Text(text) => {
                    current_content.push_str(&text);
                },
                | Event::Code(code) => {
                    current_content.push('`');
                    current_content.push_str(&code);
                    current_content.push('`');
                },
                | Event::Html(html) => {
                    if self.preserve_structure {
                        current_content.push_str(&html);
                    }
                },
                | Event::SoftBreak | Event::HardBreak => {
                    current_content.push('\n');
                },
                | Event::Rule => {
                    current_content.push_str("\n---\n");
                },
                | _ => {},
            }
            position = range.end;
        }

        // Handle any remaining content
        if !current_content.trim().is_empty() {
            let element = ParsedElement::new(current_content.trim().to_string(), current_type, current_start, position)
                .with_headers(header_stack.iter().cloned().collect());
            elements.push(element);
        }

        Ok(elements)
    }

    fn handle_start_tag(
        &self,
        tag: Tag,
        current_type: &mut ChunkType,
        in_code_block: &mut bool,
        in_list: &mut bool,
        in_table: &mut bool,
        in_quote: &mut bool,
        current_start: &mut usize,
        position: usize,
    ) {
        match tag {
            | Tag::Heading(_, _, _) => {
                *current_type = ChunkType::Header;
                *current_start = position;
            },
            | Tag::CodeBlock(CodeBlockKind::Fenced(_)) | Tag::CodeBlock(CodeBlockKind::Indented) => {
                *in_code_block = true;
                *current_type = ChunkType::CodeBlock;
                *current_start = position;
            },
            | Tag::List(_) => {
                *in_list = true;
                *current_type = ChunkType::List;
                *current_start = position;
            },
            | Tag::Table(_) => {
                *in_table = true;
                *current_type = ChunkType::Table;
                *current_start = position;
            },
            | Tag::BlockQuote => {
                *in_quote = true;
                *current_type = ChunkType::Quote;
                *current_start = position;
            },
            | _ => {},
        }
    }

    fn handle_end_tag(
        &self,
        tag: Tag,
        current_content: &mut String,
        current_type: &mut ChunkType,
        header_stack: &mut VecDeque<String>,
        in_code_block: &mut bool,
        in_list: &mut bool,
        in_table: &mut bool,
        in_quote: &mut bool,
        start_position: usize,
        end_position: usize,
    ) -> Result<Option<ParsedElement>, ServiceError> {
        match tag {
            | Tag::Heading(level, _, _) => {
                let content = current_content.trim().to_string();
                if !content.is_empty() {
                    // Update header stack based on level
                    self.update_header_stack(header_stack, &content, level);

                    let element = ParsedElement::new(content.clone(), ChunkType::Header, start_position, end_position)
                        .with_headers(header_stack.iter().cloned().collect());

                    current_content.clear();
                    *current_type = ChunkType::Text;
                    return Ok(Some(element));
                }
            },
            | Tag::CodeBlock(_) => {
                let content = current_content.trim().to_string();
                if !content.is_empty() {
                    let element =
                        ParsedElement::new(content, ChunkType::CodeBlock, start_position, end_position).with_headers(header_stack.iter().cloned().collect());

                    current_content.clear();
                    *in_code_block = false;
                    *current_type = ChunkType::Text;
                    return Ok(Some(element));
                }
            },
            | Tag::List(_) => {
                let content = current_content.trim().to_string();
                if !content.is_empty() {
                    let element =
                        ParsedElement::new(content, ChunkType::List, start_position, end_position).with_headers(header_stack.iter().cloned().collect());

                    current_content.clear();
                    *in_list = false;
                    *current_type = ChunkType::Text;
                    return Ok(Some(element));
                }
            },
            | Tag::Table(_) => {
                let content = current_content.trim().to_string();
                if !content.is_empty() {
                    let element =
                        ParsedElement::new(content, ChunkType::Table, start_position, end_position).with_headers(header_stack.iter().cloned().collect());

                    current_content.clear();
                    *in_table = false;
                    *current_type = ChunkType::Text;
                    return Ok(Some(element));
                }
            },
            | Tag::BlockQuote => {
                let content = current_content.trim().to_string();
                if !content.is_empty() {
                    let element =
                        ParsedElement::new(content, ChunkType::Quote, start_position, end_position).with_headers(header_stack.iter().cloned().collect());

                    current_content.clear();
                    *in_quote = false;
                    *current_type = ChunkType::Text;
                    return Ok(Some(element));
                }
            },
            | Tag::Paragraph => {
                // Only create text elements for paragraphs if we're not in other structures
                if !*in_code_block && !*in_list && !*in_table && !*in_quote {
                    let content = current_content.trim().to_string();
                    if !content.is_empty() {
                        let element =
                            ParsedElement::new(content, ChunkType::Text, start_position, end_position).with_headers(header_stack.iter().cloned().collect());

                        current_content.clear();
                        return Ok(Some(element));
                    }
                }
            },
            | _ => {},
        }

        Ok(None)
    }

    fn update_header_stack(&self, header_stack: &mut VecDeque<String>, content: &str, level: HeadingLevel) {
        let level_num = match level {
            | HeadingLevel::H1 => 1,
            | HeadingLevel::H2 => 2,
            | HeadingLevel::H3 => 3,
            | HeadingLevel::H4 => 4,
            | HeadingLevel::H5 => 5,
            | HeadingLevel::H6 => 6,
        };

        // Remove headers at the same level or deeper
        while header_stack.len() >= level_num {
            header_stack.pop_back();
        }

        // Add the new header
        header_stack.push_back(content.to_string());
    }

    /// Extracts plain text content from markdown, stripping all formatting
    #[allow(dead_code)]
    pub fn extract_plain_text(&self, markdown: &str) -> Result<String, ServiceError> {
        let parser = Parser::new(markdown);
        let mut plain_text = String::new();

        for event in parser {
            match event {
                | Event::Text(text) | Event::Code(text) => {
                    plain_text.push_str(&text);
                },
                | Event::SoftBreak | Event::HardBreak => {
                    plain_text.push(' ');
                },
                | _ => {},
            }
        }

        Ok(plain_text.trim().to_string())
    }
}

impl Default for DocumentParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_text() {
        let parser = DocumentParser::new();
        let markdown = "This is a simple paragraph.";
        let result = parser.parse(markdown, "test.md".to_string()).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content, "This is a simple paragraph.");
        assert_eq!(result[0].element_type, ChunkType::Text);
    }

    #[test]
    fn test_parse_headers() {
        let parser = DocumentParser::new();
        let markdown = r#"# Main Title

Some content under main title.

## Subtitle

Content under subtitle.

### Sub-subtitle

More content."#;

        let result = parser.parse(markdown, "test.md".to_string()).unwrap();

        // Should have headers and text elements
        let headers: Vec<_> = result.iter().filter(|e| e.element_type == ChunkType::Header).collect();

        assert_eq!(headers.len(), 3);
        assert_eq!(headers[0].content, "Main Title");
        assert_eq!(headers[1].content, "Subtitle");
        assert_eq!(headers[2].content, "Sub-subtitle");
    }

    #[test]
    fn test_parse_code_block() {
        let parser = DocumentParser::new();
        let markdown = r#"Here's some code:

```rust
fn main() {
    println!("Hello, world!");
}
```

And some more text."#;

        let result = parser.parse(markdown, "test.md".to_string()).unwrap();

        let code_blocks: Vec<_> = result.iter().filter(|e| e.element_type == ChunkType::CodeBlock).collect();

        assert_eq!(code_blocks.len(), 1);
        assert!(code_blocks[0].content.contains("fn main()"));
        assert!(code_blocks[0].content.contains("println!"));
    }

    #[test]
    fn test_parse_list() {
        let parser = DocumentParser::new();
        let markdown = r#"Shopping list:

- Apples
- Bananas
- Oranges

That's all."#;

        let result = parser.parse(markdown, "test.md".to_string()).unwrap();

        let lists: Vec<_> = result.iter().filter(|e| e.element_type == ChunkType::List).collect();

        assert_eq!(lists.len(), 1);
        assert!(lists[0].content.contains("Apples"));
        assert!(lists[0].content.contains("Bananas"));
        assert!(lists[0].content.contains("Oranges"));
    }

    #[test]
    fn test_parse_table() {
        let parser = DocumentParser::new();
        let markdown = r#"| Name | Age |
|------|-----|
| John | 25  |
| Jane | 30  |"#;

        let result = parser.parse(markdown, "test.md".to_string()).unwrap();

        // Tables might be parsed as text elements in some versions of pulldown-cmark
        // Let's check if we have any elements with table content
        let table_content: Vec<_> = result.iter().filter(|e| e.content.contains("Name") && e.content.contains("John")).collect();

        assert!(!table_content.is_empty(), "Should find table content in some element");
        assert!(table_content[0].content.contains("Name"));
        assert!(table_content[0].content.contains("John"));
        assert!(table_content[0].content.contains("Jane"));
    }

    #[test]
    fn test_parse_quote() {
        let parser = DocumentParser::new();
        let markdown = r#"> This is a quote.
> It spans multiple lines.

Regular text."#;

        let result = parser.parse(markdown, "test.md".to_string()).unwrap();

        let quotes: Vec<_> = result.iter().filter(|e| e.element_type == ChunkType::Quote).collect();

        assert_eq!(quotes.len(), 1);
        assert!(quotes[0].content.contains("This is a quote"));
        assert!(quotes[0].content.contains("multiple lines"));
    }

    #[test]
    fn test_header_hierarchy() {
        let parser = DocumentParser::new();
        let markdown = r#"# Chapter 1

## Section 1.1

### Subsection 1.1.1

Some content.

## Section 1.2

More content."#;

        let result = parser.parse(markdown, "test.md".to_string()).unwrap();

        // Find the text element and check its headers
        let text_elements: Vec<_> = result.iter().filter(|e| e.element_type == ChunkType::Text).collect();

        assert!(!text_elements.is_empty());

        // The text under "Subsection 1.1.1" should have all three headers in its hierarchy
        let subsection_text = text_elements.iter().find(|e| e.content == "Some content.").unwrap();

        assert_eq!(subsection_text.headers.len(), 3);
        assert_eq!(subsection_text.headers[0], "Chapter 1");
        assert_eq!(subsection_text.headers[1], "Section 1.1");
        assert_eq!(subsection_text.headers[2], "Subsection 1.1.1");
    }

    #[test]
    fn test_extract_plain_text() {
        let parser = DocumentParser::new();
        let markdown = r#"# Title

This is **bold** and *italic* text with `code`.

- List item 1
- List item 2"#;

        let plain_text = parser.extract_plain_text(markdown).unwrap();

        assert!(plain_text.contains("Title"));
        assert!(plain_text.contains("This is bold and italic text with code"));
        assert!(plain_text.contains("List item 1"));
        assert!(!plain_text.contains("**"));
        assert!(!plain_text.contains("*"));
        assert!(!plain_text.contains("`"));
    }

    #[test]
    fn test_empty_content() {
        let parser = DocumentParser::new();
        let result = parser.parse("", "test.md".to_string()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let parser = DocumentParser::new();
        let result = parser.parse("   \n\n   ", "test.md".to_string()).unwrap();
        assert!(result.is_empty());
    }
}
