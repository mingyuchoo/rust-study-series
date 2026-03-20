use super::models::GoldenSetFile;
use crate::agent_core::domain_config::DomainConfig;
use anyhow::{Context,
             Result};
use std::path::Path;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn new() -> Self { Self }

    pub fn load_domain_config(&self, filepath: &str) -> Result<DomainConfig> {
        let path = Path::new(filepath);
        let content = std::fs::read_to_string(path).with_context(|| format!("설정 파일 읽기 실패: {}", filepath))?;
        serde_yaml::from_str(&content).with_context(|| format!("YAML 파싱 실패: {}", filepath))
    }

    pub fn load_all_domains(&self, directory: &str) -> Result<Vec<DomainConfig>> {
        let dir = Path::new(directory);
        let mut configs = Vec::new();
        let mut entries: Vec<_> = std::fs::read_dir(dir)?
            .flatten()
            .filter(|e| e.path().extension().map(|x| x == "yaml").unwrap_or(false))
            .collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let config = self.load_domain_config(&entry.path().to_string_lossy())?;
            configs.push(config);
        }
        Ok(configs)
    }

    #[allow(dead_code)]
    pub fn load_golden_sets(&self, filepath: &str) -> Result<GoldenSetFile> {
        let content = std::fs::read_to_string(filepath).with_context(|| format!("골든셋 파일 읽기 실패: {}", filepath))?;
        serde_json::from_str(&content).with_context(|| format!("JSON 파싱 실패: {}", filepath))
    }

    #[allow(dead_code)]
    pub fn load_all_golden_sets(&self, directory: &str) -> Result<Vec<GoldenSetFile>> {
        let dir = Path::new(directory);
        let mut result = Vec::new();
        let mut entries: Vec<_> = std::fs::read_dir(dir)?
            .flatten()
            .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
            .collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let gs = self.load_golden_sets(&entry.path().to_string_lossy())?;
            result.push(gs);
        }
        Ok(result)
    }
}

impl Default for ScenarioLoader {
    fn default() -> Self { Self::new() }
}
