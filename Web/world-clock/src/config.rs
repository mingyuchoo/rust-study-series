// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2, FR-3, FR-4
// @trace file-type: impl
// =============================================================================

use std::path::{Path, PathBuf};

use chrono_tz::Tz;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// 도시 항목.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CityEntry {
    pub name: String,
    pub timezone: String,
}

/// 앱 설정 (저장된 도시 목록).
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub cities: Vec<CityEntry>,
}

impl Config {
    /// 설정 파일을 로드한다. 파일이 없으면 빈 설정을 반환한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-10, SPEC-001/TC-11
    /// @trace FR: PRD-001/FR-1, PRD-001/FR-4
    pub fn load(path: &Path) -> Result<Self, AppError> {
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(serde_json::from_str(&content)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(AppError::Config(e)),
        }
    }

    /// 설정을 JSON 파일로 저장한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-10
    /// @trace FR: PRD-001/FR-2, PRD-001/FR-3
    pub fn save(&self, path: &Path) -> Result<(), AppError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 도시를 추가한다. 중복 도시명이면 에러를 반환한다.
    /// 타임존 유효성을 먼저 검증한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-5, SPEC-001/TC-6, SPEC-001/TC-7
    /// @trace FR: PRD-001/FR-2
    pub fn add(&mut self, entry: CityEntry) -> Result<(), AppError> {
        // 타임존 유효성 검증
        entry
            .timezone
            .parse::<Tz>()
            .map_err(|_| AppError::UnknownTimezone(entry.timezone.clone()))?;

        // 중복 검사
        if self.cities.iter().any(|c| c.name == entry.name) {
            return Err(AppError::DuplicateCity(entry.name));
        }

        self.cities.push(entry);
        Ok(())
    }

    /// 도시를 삭제한다. 존재하지 않는 도시명이면 에러를 반환한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: SPEC-001/TC-8, SPEC-001/TC-9
    /// @trace FR: PRD-001/FR-3
    pub fn remove(&mut self, name: &str) -> Result<(), AppError> {
        let len_before = self.cities.len();
        self.cities.retain(|c| c.name != name);

        if self.cities.len() == len_before {
            return Err(AppError::CityNotFound(name.to_string()));
        }

        Ok(())
    }
}

/// OS별 기본 설정 파일 경로를 반환한다.
///
/// @trace SPEC: SPEC-001
/// @trace FR: PRD-001/FR-2, PRD-001/FR-3
pub fn default_config_path() -> PathBuf {
    ProjectDirs::from("", "", "world-clock")
        .map(|dirs| dirs.config_dir().join("config.json"))
        .unwrap_or_else(|| PathBuf::from("world-clock-config.json"))
}
