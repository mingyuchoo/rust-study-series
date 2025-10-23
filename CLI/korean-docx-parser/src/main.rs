use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input/애국가.docx")?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    // styles.xml 파싱
    let styles = parse_styles(&mut archive)?;

    // numbering.xml 파싱
    let numbering = parse_numbering(&mut archive)?;

    // document.xml 파싱 및 Markdown 변환
    let markdown = parse_document(&mut archive, &styles, &numbering)?;

    // output 디렉토리 생성
    std::fs::create_dir_all("output")?;

    // Markdown 파일로 저장
    let mut output = File::create("output/애국가.md")?;
    output.write_all(markdown.as_bytes())?;

    println!("✅ Markdown 파일이 생성되었습니다: 애국가.md");
    Ok(())
}

fn parse_styles(
    archive: &mut ZipArchive<BufReader<File>>,
) -> Result<HashMap<String, StyleInfo>, Box<dyn std::error::Error>> {
    let mut styles = HashMap::new();

    if let Ok(mut file) = archive.by_name("word/styles.xml") {
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut reader = Reader::from_str(&content);
        let mut buf = Vec::new();
        let mut current_style_id = String::new();
        let mut current_style = StyleInfo::default();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"w:style" => {
                        if let Some(id) = get_attribute(&e, b"w:styleId") {
                            current_style_id = id;
                        }
                    }
                    b"w:b" => current_style.bold = true,
                    b"w:i" => current_style.italic = true,
                    _ => {}
                },
                Ok(Event::End(e)) if e.name().as_ref() == b"w:style" => {
                    if !current_style_id.is_empty() {
                        styles.insert(current_style_id.clone(), current_style.clone());
                        current_style = StyleInfo::default();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => eprintln!("스타일 파싱 경고: {}", e),
                _ => {}
            }
            buf.clear();
        }
    }

    Ok(styles)
}

fn parse_numbering(
    archive: &mut ZipArchive<BufReader<File>>,
) -> Result<HashMap<String, NumberingInfo>, Box<dyn std::error::Error>> {
    let mut numbering = HashMap::new();

    if let Ok(mut file) = archive.by_name("word/numbering.xml") {
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut reader = Reader::from_str(&content);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"w:num" {
                        if let Some(id) = get_attribute(&e, b"w:numId") {
                            numbering.insert(id, NumberingInfo { _is_ordered: true });
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => eprintln!("번호 매기기 파싱 경고: {}", e),
                _ => {}
            }
            buf.clear();
        }
    }

    Ok(numbering)
}

fn parse_document(
    archive: &mut ZipArchive<BufReader<File>>,
    _styles: &HashMap<String, StyleInfo>,
    numbering: &HashMap<String, NumberingInfo>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut xml_file = archive.by_name("word/document.xml")?;
    let mut xml_content = String::new();
    xml_file.read_to_string(&mut xml_content)?;

    let mut reader = Reader::from_str(&xml_content);
    let mut buf = Vec::new();
    let mut markdown = String::new();

    let mut in_text = false;
    let mut in_table = false;
    let mut in_table_cell = false;
    let mut is_bold = false;
    let mut is_italic = false;
    let mut num_id = None;
    let mut current_para = String::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"w:p" => {
                        current_para.clear();
                        num_id = None;
                    }
                    b"w:tbl" => in_table = true,
                    b"w:tr" => {
                        current_row.clear();
                    }
                    b"w:tc" => {
                        in_table_cell = true;
                        current_cell.clear();
                    }
                    b"w:t" => in_text = true,
                    b"w:b" => is_bold = true,
                    b"w:i" => is_italic = true,
                    b"w:numPr" => {
                        // 목록 번호 추출 시도
                    }
                    b"w:numId" => {
                        if let Some(val) = get_attribute(&e, b"w:val") {
                            num_id = Some(val);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                if in_text {
                    let text = String::from_utf8_lossy(e.as_ref());
                    let mut formatted = text.to_string();

                    if is_bold && is_italic {
                        formatted = format!("***{}***", formatted);
                    } else if is_bold {
                        formatted = format!("**{}**", formatted);
                    } else if is_italic {
                        formatted = format!("*{}*", formatted);
                    }

                    if in_table_cell {
                        current_cell.push_str(&formatted);
                    } else {
                        current_para.push_str(&formatted);
                    }
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"w:p" => {
                    if !in_table && !current_para.trim().is_empty() {
                        if let Some(id) = &num_id {
                            if numbering.contains_key(id) {
                                markdown.push_str(&format!("- {}\n", current_para.trim()));
                            } else {
                                markdown.push_str(&format!("{}\n\n", current_para.trim()));
                            }
                        } else {
                            markdown.push_str(&format!("{}\n\n", current_para.trim()));
                        }
                    }
                }
                b"w:tbl" => {
                    in_table = false;
                    if !table_rows.is_empty() {
                        markdown.push_str(&format_table(&table_rows));
                        table_rows.clear();
                    }
                }
                b"w:tr" => {
                    if !current_row.is_empty() {
                        table_rows.push(current_row.clone());
                    }
                }
                b"w:tc" => {
                    in_table_cell = false;
                    current_row.push(current_cell.trim().to_string());
                    current_cell.clear();
                }
                b"w:t" => in_text = false,
                b"w:b" => is_bold = false,
                b"w:i" => is_italic = false,
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML 파싱 에러: {}", e).into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(markdown)
}

fn format_table(rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // 헤더 행
    if let Some(header) = rows.first() {
        result.push_str("| ");
        for cell in header {
            result.push_str(&format!("{} | ", cell));
        }
        result.push('\n');

        // 구분선
        result.push_str("|");
        for _ in 0..col_count {
            result.push_str(" --- |");
        }
        result.push('\n');
    }

    // 데이터 행
    for row in rows.iter().skip(1) {
        result.push_str("| ");
        for cell in row {
            result.push_str(&format!("{} | ", cell));
        }
        result.push('\n');
    }

    result.push('\n');
    result
}

fn get_attribute(element: &BytesStart, attr_name: &[u8]) -> Option<String> {
    element
        .attributes()
        .filter_map(|a| a.ok())
        .find(|a| a.key.as_ref() == attr_name)
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok())
}

#[derive(Clone, Default)]
struct StyleInfo {
    bold: bool,
    italic: bool,
}

#[derive(Clone)]
struct NumberingInfo {
    _is_ordered: bool,
}
