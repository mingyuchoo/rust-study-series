# Text Converter Plugin

텍스트 파일 인코딩 변환 플러그인

## 개요

Text Converter Plugin은 텍스트 파일의 문자 인코딩을 변환하는 기능을 제공합니다. 이 플러그인은 `encoding_rs` 크레이트를 사용하여 다양한 인코딩 간 변환을 지원하며, 자동 인코딩 감지 기능도 포함되어 있습니다.

## 지원 형식

### 입력 형식
- **확장자**: `.txt`
- **MIME 타입**: `text/plain`
- **지원 인코딩**: UTF-8, EUC-KR, CP949 및 encoding_rs가 지원하는 모든 인코딩

### 출력 형식
- **확장자**: `.txt`
- **MIME 타입**: `text/plain`
- **지원 인코딩**: UTF-8, EUC-KR, CP949 및 encoding_rs가 지원하는 모든 인코딩

## 사용법

### 기본 사용

플러그인은 File Converter 애플리케이션에 자동으로 로드되며, GUI를 통해 사용할 수 있습니다:

1. 변환할 텍스트 파일을 선택합니다
2. 출력 형식으로 "Plain Text"를 선택합니다
3. 변환 옵션에서 대상 인코딩을 지정합니다
4. 변환을 실행합니다

### 변환 옵션

플러그인은 다음과 같은 커스텀 파라미터를 지원합니다:

#### `source_encoding` (선택사항)
- **설명**: 입력 파일의 인코딩을 명시적으로 지정합니다
- **기본값**: 자동 감지
- **예시**: `"UTF-8"`, `"EUC-KR"`, `"windows-949"`

#### `target_encoding` (선택사항)
- **설명**: 출력 파일의 인코딩을 지정합니다
- **기본값**: `UTF-8`
- **예시**: `"UTF-8"`, `"EUC-KR"`, `"windows-949"`

### 출력 파일 경로

출력 파일 경로를 지정하지 않으면, 플러그인은 자동으로 다음 형식으로 파일명을 생성합니다:

```
원본파일명_인코딩명.txt
```

**예시**:
- 입력: `document.txt`
- 대상 인코딩: `UTF-8`
- 출력: `document_utf-8.txt`

## 인코딩 자동 감지

플러그인은 입력 파일의 인코딩을 자동으로 감지합니다:

1. 먼저 UTF-8로 디코딩을 시도합니다
2. 실패하면 EUC-KR 등 일반적인 한국어 인코딩을 시도합니다
3. 모든 감지가 실패하면 UTF-8을 기본값으로 사용합니다

명시적으로 `source_encoding` 옵션을 지정하면 자동 감지를 건너뜁니다.

## 에러 처리

플러그인은 다음과 같은 상황에서 에러를 반환합니다:

- 입력 파일을 읽을 수 없는 경우
- 지정된 인코딩으로 디코딩/인코딩할 수 없는 경우
- 출력 파일이 이미 존재하고 `overwrite` 옵션이 `false`인 경우
- 출력 파일을 쓸 수 없는 경우

## 플러그인 개발 가이드

이 플러그인은 File Converter 애플리케이션의 플러그인 시스템을 사용하는 예제입니다. 새로운 플러그인을 개발하려면 다음 단계를 따르세요:

### 1. 프로젝트 구조 생성

```bash
cd plugins
cargo new --lib your-plugin-name
```

### 2. Cargo.toml 설정

```toml
[package]
name = "your-plugin-name"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
plugin-interface = { path = "../../plugin-interface" }
# 필요한 다른 의존성 추가
```

**중요**: `crate-type`에 `"cdylib"`를 포함해야 동적 로딩이 가능합니다.

### 3. Plugin 트레이트 구현

```rust
use plugin_interface::*;
use std::path::Path;
use std::error::Error;

pub struct YourPlugin;

impl Plugin for YourPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Your Plugin Name".to_string(),
            version: "0.1.0".to_string(),
            author: "Your Name".to_string(),
            description: "Plugin description".to_string(),
        }
    }

    fn supported_input_formats(&self) -> Vec<FileFormat> {
        vec![
            FileFormat {
                extension: "ext".to_string(),
                mime_type: "type/subtype".to_string(),
                description: "Format description".to_string(),
            },
        ]
    }

    fn supported_output_formats(&self) -> Vec<FileFormat> {
        // 출력 형식 정의
        vec![/* ... */]
    }

    fn can_convert(&self, from: &FileFormat, to: &FileFormat) -> bool {
        // 변환 가능 여부 확인 로직
        true
    }

    fn convert(
        &self,
        input_path: &Path,
        output_format: &FileFormat,
        options: &ConversionOptions,
    ) -> Result<ConversionResult, Box<dyn Error>> {
        // 변환 로직 구현
        Ok(ConversionResult {
            success: true,
            output_path: Some("output.ext".to_string()),
            message: "Conversion successful".to_string(),
            bytes_processed: 0,
        })
    }
}
```

### 4. 플러그인 생성자 함수 추가

```rust
#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(YourPlugin)
}
```

**중요**: `#[no_mangle]` 속성은 동적 로딩을 위해 필수입니다.

### 5. 플러그인 테스트

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = YourPlugin;
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "Your Plugin Name");
    }

    #[test]
    fn test_conversion() {
        // 변환 테스트 구현
    }
}
```

### 6. Workspace에 플러그인 추가

루트 `Cargo.toml`의 `[workspace]` 섹션에 플러그인을 추가합니다:

```toml
[workspace]
members = [
    # ... 기존 멤버들
    "plugins/your-plugin-name",
]
```

## 구현 세부사항

### 인코딩 감지 알고리즘

```rust
fn detect_encoding(bytes: &[u8]) -> &'static Encoding {
    // 1. UTF-8 검증
    if std::str::from_utf8(bytes).is_ok() {
        return encoding_rs::UTF_8;
    }

    // 2. 일반적인 인코딩 시도
    let encodings = [encoding_rs::EUC_KR];
    for encoding in &encodings {
        let (_, _, had_errors) = encoding.decode(bytes);
        if !had_errors {
            return encoding;
        }
    }

    // 3. 기본값 반환
    encoding_rs::UTF_8
}
```

### 출력 경로 생성

```rust
fn generate_output_path(input_path: &Path, encoding_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let file_stem = input_path.file_stem()
        .ok_or("Invalid input file path")?;
    let extension = input_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("txt");
    let parent = input_path.parent()
        .ok_or("Invalid input file path")?;
    
    let encoding_suffix = encoding_name.to_lowercase().replace(' ', "-");
    let output_filename = format!("{}_{}.{}", 
        file_stem.to_str().unwrap(), 
        encoding_suffix, 
        extension
    );
    
    Ok(parent.join(output_filename))
}
```

## 의존성

- `plugin-interface`: 플러그인 인터페이스 정의
- `encoding_rs`: 문자 인코딩 변환 라이브러리
- `anyhow`: 에러 처리 유틸리티

## 라이선스

이 플러그인은 File Converter 프로젝트의 일부입니다.

## 기여

버그 리포트나 기능 제안은 프로젝트의 이슈 트래커를 통해 제출해 주세요.

## 참고 자료

- [encoding_rs 문서](https://docs.rs/encoding_rs/)
- [Plugin Interface 문서](../../plugin-interface/README.md)
- [File Converter 프로젝트 문서](../../README.md)
