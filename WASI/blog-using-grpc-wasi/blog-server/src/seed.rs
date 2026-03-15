use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SeedData {
    pub admin: AdminSeed,
}

#[derive(Debug, Deserialize)]
pub struct AdminSeed {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl SeedData {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("seed 파일을 읽을 수 없습니다: {}", path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("seed 파일 파싱 실패: {}", path.display()))
    }
}
