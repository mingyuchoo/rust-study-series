#![allow(clippy::new_without_default)]
use crate::base::{BaseTool,
                  ToolMetadata};
use std::{collections::HashMap,
          path::Path};

pub struct ReadFileTool {
    metadata: ToolMetadata,
}

impl ReadFileTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "read_file".into(),
                description: "파일 내용을 읽습니다".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "file_path": {"type": "string", "description": "읽을 파일 경로"},
                        "encoding": {"type": "string", "description": "파일 인코딩", "default": "utf-8"}
                    },
                    "required": ["file_path"]
                }),
                safety_level: "safe".into(),
                requires_approval: false,
            },
        }
    }
}

impl BaseTool for ReadFileTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let file_path = match params.get("file_path").and_then(|v| v.as_str()) {
            | Some(p) => p,
            | None => return err_map("file_path 파라미터가 필요합니다"),
        };

        let path = Path::new(file_path);
        if !path.exists() {
            return err_map(&format!("파일이 존재하지 않습니다: {}", file_path));
        }

        match std::fs::read_to_string(path) {
            | Ok(content) => {
                let size = content.len();
                let mut m = HashMap::new();
                m.insert("success".into(), serde_json::Value::Bool(true));
                m.insert("content".into(), serde_json::Value::String(content));
                m.insert("file_path".into(), serde_json::Value::String(path.to_string_lossy().into()));
                m.insert("size_bytes".into(), serde_json::json!(size));
                m
            },
            | Err(e) => err_map(&format!("파일 읽기 실패: {}", e)),
        }
    }
}

pub struct WriteFileTool {
    metadata: ToolMetadata,
}

impl WriteFileTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "write_file".into(),
                description: "파일에 내용을 씁니다".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "file_path": {"type": "string", "description": "쓸 파일 경로"},
                        "content": {"type": "string", "description": "파일 내용"},
                        "encoding": {"type": "string", "description": "파일 인코딩", "default": "utf-8"}
                    },
                    "required": ["file_path", "content"]
                }),
                safety_level: "caution".into(),
                requires_approval: true,
            },
        }
    }
}

impl BaseTool for WriteFileTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let file_path = match params.get("file_path").and_then(|v| v.as_str()) {
            | Some(p) => p,
            | None => return err_map("file_path 파라미터가 필요합니다"),
        };
        let content = match params.get("content").and_then(|v| v.as_str()) {
            | Some(c) => c,
            | None => return err_map("content 파라미터가 필요합니다"),
        };

        let path = Path::new(file_path);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        match std::fs::write(path, content) {
            | Ok(_) => {
                let mut m = HashMap::new();
                m.insert("success".into(), serde_json::Value::Bool(true));
                m.insert("file_path".into(), serde_json::Value::String(path.to_string_lossy().into()));
                m.insert("size_bytes".into(), serde_json::json!(content.len()));
                m
            },
            | Err(e) => err_map(&format!("파일 쓰기 실패: {}", e)),
        }
    }
}

pub struct ListDirectoryTool {
    metadata: ToolMetadata,
}

impl ListDirectoryTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "list_directory".into(),
                description: "디렉토리 내 파일/폴더 목록을 조회합니다".into(),
                parameters_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "directory_path": {"type": "string", "description": "조회할 디렉토리 경로"},
                        "recursive": {"type": "boolean", "description": "하위 디렉토리 포함 여부", "default": false}
                    },
                    "required": ["directory_path"]
                }),
                safety_level: "safe".into(),
                requires_approval: false,
            },
        }
    }
}

impl BaseTool for ListDirectoryTool {
    fn metadata(&self) -> &ToolMetadata { &self.metadata }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let dir_path = match params.get("directory_path").and_then(|v| v.as_str()) {
            | Some(p) => p,
            | None => return err_map("directory_path 파라미터가 필요합니다"),
        };
        let recursive = params.get("recursive").and_then(|v| v.as_bool()).unwrap_or(false);

        let path = Path::new(dir_path);
        if !path.exists() {
            return err_map(&format!("디렉토리가 존재하지 않습니다: {}", dir_path));
        }
        if !path.is_dir() {
            return err_map(&format!("디렉토리가 아닙니다: {}", dir_path));
        }

        let mut files = Vec::new();
        let mut directories = Vec::new();

        let mut collect = |p: &Path| {
            if let Ok(entries) = std::fs::read_dir(p) {
                for entry in entries.flatten() {
                    let ep = entry.path();
                    let name = ep.file_name().unwrap_or_default().to_string_lossy().into_owned();
                    if ep.is_file() {
                        files.push(name);
                    } else if ep.is_dir() {
                        directories.push(name);
                    }
                }
            }
        };

        if recursive {
            fn walk(p: &Path, files: &mut Vec<String>, dirs: &mut Vec<String>, base: &Path) {
                if let Ok(entries) = std::fs::read_dir(p) {
                    for entry in entries.flatten() {
                        let ep = entry.path();
                        let rel = ep.strip_prefix(base).unwrap_or(&ep).to_string_lossy().into_owned();
                        if ep.is_file() {
                            files.push(rel);
                        } else if ep.is_dir() {
                            dirs.push(rel.clone());
                            walk(&ep, files, dirs, base);
                        }
                    }
                }
            }
            walk(path, &mut files, &mut directories, path);
        } else {
            collect(path);
        }

        files.sort();
        directories.sort();
        let total = files.len() + directories.len();

        let mut m = HashMap::new();
        m.insert("success".into(), serde_json::Value::Bool(true));
        m.insert("directory_path".into(), serde_json::Value::String(path.to_string_lossy().into()));
        m.insert("files".into(), serde_json::json!(files));
        m.insert("directories".into(), serde_json::json!(directories));
        m.insert("total_items".into(), serde_json::json!(total));
        m
    }
}

fn err_map(msg: &str) -> HashMap<String, serde_json::Value> {
    let mut m = HashMap::new();
    m.insert("success".into(), serde_json::Value::Bool(false));
    m.insert("error".into(), serde_json::Value::String(msg.to_string()));
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_params(pairs: &[(&str, serde_json::Value)]) -> HashMap<String, serde_json::Value> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
    }

    #[test]
    fn read_file_success() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "hello world").unwrap();
        let tool = ReadFileTool::new();
        let params = make_params(&[("file_path", serde_json::json!(f.path().to_str().unwrap()))]);
        let result = tool.execute(&params);
        assert_eq!(result["success"], serde_json::Value::Bool(true));
        assert_eq!(result["content"], serde_json::json!("hello world"));
    }

    #[test]
    fn read_file_not_found() {
        let tool = ReadFileTool::new();
        let params = make_params(&[("file_path", serde_json::json!("/nonexistent/path/file.txt"))]);
        let result = tool.execute(&params);
        assert_eq!(result["success"], serde_json::Value::Bool(false));
    }

    #[test]
    fn write_file_success() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.txt");
        let tool = WriteFileTool::new();
        let params = make_params(&[
            ("file_path", serde_json::json!(path.to_str().unwrap())),
            ("content", serde_json::json!("테스트 내용")),
        ]);
        let result = tool.execute(&params);
        assert_eq!(result["success"], serde_json::Value::Bool(true));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "테스트 내용");
    }

    #[test]
    fn list_directory_success() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.txt"), "").unwrap();
        std::fs::write(dir.path().join("b.txt"), "").unwrap();
        let tool = ListDirectoryTool::new();
        let params = make_params(&[("directory_path", serde_json::json!(dir.path().to_str().unwrap()))]);
        let result = tool.execute(&params);
        assert_eq!(result["success"], serde_json::Value::Bool(true));
        let files = result["files"].as_array().unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn validate_parameters_missing_required() {
        let tool = ReadFileTool::new();
        let empty: HashMap<String, serde_json::Value> = HashMap::new();
        assert!(!tool.validate_parameters(&empty));
        let with_param = make_params(&[("file_path", serde_json::json!("x"))]);
        assert!(tool.validate_parameters(&with_param));
    }
}
