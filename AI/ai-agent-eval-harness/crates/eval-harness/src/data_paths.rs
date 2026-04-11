// =============================================================================
// @trace SPEC-015, SPEC-016, SPEC-017
// @trace PRD: PRD-015, PRD-016, PRD-017
// @trace FR: PRD-015/FR-1, PRD-015/FR-2, PRD-015/FR-3, PRD-015/FR-4,
// PRD-015/FR-5, PRD-015/FR-6 @trace FR: PRD-016/FR-1, PRD-016/FR-2,
// PRD-016/FR-3 @trace FR: PRD-017/FR-4
// @trace file-type: impl
// =============================================================================
//
// `eval_data/` 계열 디렉토리(시나리오/골든셋)의 경로 해석을 한 곳으로
// 중앙화한다.
//
// 우선순위 (높음 → 낮음):
//   1. CLI 인자  (--scenarios-dir / --golden-sets-dir)
//   2. 환경변수  (EVAL_HARNESS_SCENARIOS_DIR / EVAL_HARNESS_GOLDEN_SETS_DIR)
//   3. 설정 파일 (eval-harness.toml 의 [data] 섹션)
//   4. 내장 기본값 (eval_data/eval_scenarios, eval_data/golden_sets)
//
// 상대 경로 해석 기준:
//   - 설정 파일에서 온 값  → 설정 파일이 위치한 디렉토리 기준
//   - 환경변수 / CLI 인자  → 호출자의 CWD 기준 (PathBuf 그대로 보관)
//   - 내장 기본값          → `DataPaths::load`  는 CWD 기준 상대,
//     `DataPaths::resolve_for_root` 는 root 기준 join

use std::{collections::BTreeMap,
          path::{Path,
                 PathBuf}};

pub const DEFAULT_SCENARIOS_DIR: &str = "eval_data/eval_scenarios";
pub const DEFAULT_GOLDEN_SETS_DIR: &str = "eval_data/golden_sets";
pub const DEFAULT_DB_PATH: &str = "eval_data/eval_harness.db";
pub const DEFAULT_CONFIG_FILENAME: &str = "eval-harness.toml";
pub const ENV_SCENARIOS_DIR: &str = "EVAL_HARNESS_SCENARIOS_DIR";
pub const ENV_GOLDEN_SETS_DIR: &str = "EVAL_HARNESS_GOLDEN_SETS_DIR";
pub const ENV_DB_PATH: &str = "EVAL_HARNESS_DB_PATH";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPaths {
    pub scenarios_dir: PathBuf,
    pub golden_sets_dir: PathBuf,
    pub db_path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum DataPathsError {
    #[error("설정 파일 읽기 실패 ({path}): {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("설정 파일 TOML 파싱 실패 ({path}): {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("설정 파일 검증 실패 ({path}): {message}")]
    Invalid { path: PathBuf, message: String },
}

#[derive(Debug, serde::Deserialize, Default)]
struct ConfigFile {
    #[serde(default)]
    data: ConfigData,
    #[serde(default)]
    evaluation: ConfigEvaluation,
}

#[derive(Debug, serde::Deserialize, Default)]
struct ConfigData {
    scenarios_dir: Option<String>,
    golden_sets_dir: Option<String>,
    db_path: Option<String>,
}

#[derive(Debug, serde::Deserialize, Default)]
struct ConfigEvaluation {
    max_iterations: Option<u32>,
    early_stop_threshold: Option<u32>,
    domain_router_top_k: Option<usize>,
}

impl Default for DataPaths {
    /// CWD 기준 상대 경로의 내장 기본값.
    fn default() -> Self {
        Self {
            scenarios_dir: PathBuf::from(DEFAULT_SCENARIOS_DIR),
            golden_sets_dir: PathBuf::from(DEFAULT_GOLDEN_SETS_DIR),
            db_path: PathBuf::from(DEFAULT_DB_PATH),
        }
    }
}

impl DataPaths {
    /// `base` 디렉토리에서 `eval-harness.toml` 을 찾고, 환경변수 + 내장
    /// 기본값과 병합하여 반환한다. 설정 파일이 존재하지 않으면 ENV 와
    /// 기본값만 적용한다. 내장 기본값은 CWD 기준 상대 경로 그대로 둔다.
    pub fn load(base: &Path) -> Result<Self, DataPathsError> {
        let mut paths = Self::default();
        let cfg_path = base.join(DEFAULT_CONFIG_FILENAME);
        if cfg_path.is_file() {
            let parsed = read_config(&cfg_path)?;
            apply_config(&mut paths, &parsed, base);
        }
        paths.apply_env();
        Ok(paths)
    }

    /// desktop 진입점용. `root` 를 기본 검색 위치로 사용하며, 설정 파일도 root
    /// 에서 찾는다. 설정 파일이 없을 때의 내장 기본값도 root 기준으로 join
    /// 한다.
    pub fn resolve_for_root(root: &Path) -> Result<Self, DataPathsError> {
        let mut paths = Self {
            scenarios_dir: root.join(DEFAULT_SCENARIOS_DIR),
            golden_sets_dir: root.join(DEFAULT_GOLDEN_SETS_DIR),
            db_path: root.join(DEFAULT_DB_PATH),
        };
        let cfg_path = root.join(DEFAULT_CONFIG_FILENAME);
        if cfg_path.is_file() {
            let parsed = read_config(&cfg_path)?;
            apply_config(&mut paths, &parsed, root);
        }
        paths.apply_env();
        Ok(paths)
    }

    /// 환경변수 값이 있으면 self 의 필드를 덮어쓴다. 환경변수 값은 그대로
    /// `PathBuf` 로 사용한다(상대 경로면 호출자 CWD 기준).
    pub fn apply_env(&mut self) {
        let env_map: BTreeMap<&str, String> = [ENV_SCENARIOS_DIR, ENV_GOLDEN_SETS_DIR, ENV_DB_PATH]
            .into_iter()
            .filter_map(|k| std::env::var(k).ok().map(|v| (k, v)))
            .collect();
        self.apply_env_from(&env_map);
    }

    /// 단위 테스트에서 환경 격리를 위해 사용하는 주입형 변형.
    pub fn apply_env_from(&mut self, env: &BTreeMap<&str, String>) {
        if let Some(v) = env.get(ENV_SCENARIOS_DIR) {
            self.scenarios_dir = PathBuf::from(v);
        }
        if let Some(v) = env.get(ENV_GOLDEN_SETS_DIR) {
            self.golden_sets_dir = PathBuf::from(v);
        }
        if let Some(v) = env.get(ENV_DB_PATH) {
            self.db_path = PathBuf::from(v);
        }
    }

    /// CLI 인자가 `Some` 이면 해당 필드를 덮어쓴다.
    pub fn with_overrides(mut self, scenarios: Option<&str>, golden_sets: Option<&str>) -> Self {
        if let Some(v) = scenarios {
            self.scenarios_dir = PathBuf::from(v);
        }
        if let Some(v) = golden_sets {
            self.golden_sets_dir = PathBuf::from(v);
        }
        self
    }

    /// db_path 에 대한 CLI override.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-4
    pub fn with_db_override(mut self, db_path: Option<&str>) -> Self {
        if let Some(v) = db_path {
            self.db_path = PathBuf::from(v);
        }
        self
    }
}

/// `base` 디렉토리의 `eval-harness.toml` 에서 `[evaluation]` 섹션을 읽어
/// `EvaluationConfig` 로 overlay 한 값을 반환한다. 파일이 없으면 기본값을
/// 그대로 반환한다. 0 또는 음수로 해석될 수 있는 값은 검증 에러.
///
/// @trace SPEC: SPEC-016
/// @trace TC: SPEC-016/TC-1, SPEC-016/TC-2, SPEC-016/TC-3, SPEC-016/TC-4
/// @trace FR: PRD-016/FR-1, PRD-016/FR-2
pub fn load_evaluation_config(base: &Path) -> Result<agent_core::config::EvaluationConfig, DataPathsError> {
    let mut cfg = agent_core::config::EvaluationConfig::default();
    let cfg_path = base.join(DEFAULT_CONFIG_FILENAME);
    if !cfg_path.is_file() {
        return Ok(cfg);
    }
    let parsed = read_config(&cfg_path)?;
    if let Some(v) = parsed.evaluation.max_iterations {
        if v == 0 {
            return Err(DataPathsError::Invalid {
                path: cfg_path,
                message: "evaluation.max_iterations must be >= 1".into(),
            });
        }
        cfg.max_iterations = v;
    }
    if let Some(v) = parsed.evaluation.early_stop_threshold {
        if v == 0 {
            return Err(DataPathsError::Invalid {
                path: cfg_path,
                message: "evaluation.early_stop_threshold must be >= 1".into(),
            });
        }
        cfg.early_stop_threshold = v;
    }
    if let Some(v) = parsed.evaluation.domain_router_top_k {
        cfg.domain_router_top_k = v;
    }
    Ok(cfg)
}

fn read_config(path: &Path) -> Result<ConfigFile, DataPathsError> {
    let text = std::fs::read_to_string(path).map_err(|e| DataPathsError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    toml::from_str::<ConfigFile>(&text).map_err(|e| DataPathsError::Parse {
        path: path.to_path_buf(),
        source: e,
    })
}

fn apply_config(paths: &mut DataPaths, cfg: &ConfigFile, base: &Path) {
    if let Some(v) = cfg.data.scenarios_dir.as_deref() {
        paths.scenarios_dir = resolve_relative(base, v);
    }
    if let Some(v) = cfg.data.golden_sets_dir.as_deref() {
        paths.golden_sets_dir = resolve_relative(base, v);
    }
    if let Some(v) = cfg.data.db_path.as_deref() {
        paths.db_path = resolve_relative(base, v);
    }
}

/// 경로가 절대면 그대로, 상대면 `base` 와 결합한다.
fn resolve_relative(base: &Path, p: &str) -> PathBuf {
    let candidate = PathBuf::from(p);
    if candidate.is_absolute() { candidate } else { base.join(candidate) }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    // =============================================================================
    // @trace SPEC-015
    // @trace PRD: PRD-015
    // @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5, FR-6
    // @trace file-type: test
    // =============================================================================

    use super::*;
    use std::{collections::BTreeMap,
              fs};
    use tempfile::tempdir;

    fn write_cfg(dir: &Path, body: &str) { fs::write(dir.join(DEFAULT_CONFIG_FILENAME), body).unwrap(); }

    /// @trace TC: SPEC-015/TC-1
    /// @trace FR: PRD-015/FR-1
    /// @trace scenario: 두 키 모두 있는 TOML 로드
    #[test]
    fn test_tc_1_load_full_config() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [data]
                scenarios_dir   = "custom/scen"
                golden_sets_dir = "custom/gold"
            "#,
        );
        // 환경변수 격리: 본 테스트가 영향받지 않도록 unset
        std::env::remove_var(ENV_SCENARIOS_DIR);
        std::env::remove_var(ENV_GOLDEN_SETS_DIR);

        let paths = DataPaths::load(base.path()).unwrap();
        assert_eq!(paths.scenarios_dir, base.path().join("custom/scen"));
        assert_eq!(paths.golden_sets_dir, base.path().join("custom/gold"));
    }

    /// @trace TC: SPEC-015/TC-2
    /// @trace FR: PRD-015/FR-1, PRD-015/FR-5
    /// @trace scenario: 일부 키 누락 시 누락된 키만 기본값 fallback
    #[test]
    fn test_tc_2_partial_config_fallback() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [data]
                scenarios_dir = "only/scen"
            "#,
        );
        std::env::remove_var(ENV_SCENARIOS_DIR);
        std::env::remove_var(ENV_GOLDEN_SETS_DIR);

        let paths = DataPaths::load(base.path()).unwrap();
        assert_eq!(paths.scenarios_dir, base.path().join("only/scen"));
        assert_eq!(paths.golden_sets_dir, PathBuf::from(DEFAULT_GOLDEN_SETS_DIR));
    }

    /// @trace TC: SPEC-015/TC-3
    /// @trace FR: PRD-015/FR-2
    /// @trace scenario: 설정 파일의 상대 경로는 설정 파일 디렉토리 기준
    #[test]
    fn test_tc_3_relative_path_uses_config_dir() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [data]
                scenarios_dir = "rel/scen"
            "#,
        );
        std::env::remove_var(ENV_SCENARIOS_DIR);
        std::env::remove_var(ENV_GOLDEN_SETS_DIR);

        let paths = DataPaths::load(base.path()).unwrap();
        assert!(paths.scenarios_dir.starts_with(base.path()));
        assert!(paths.scenarios_dir.ends_with("rel/scen"));
    }

    /// @trace TC: SPEC-015/TC-4
    /// @trace FR: PRD-015/FR-2
    /// @trace scenario: 설정 파일의 절대 경로는 그대로 사용
    #[test]
    fn test_tc_4_absolute_path_kept_as_is() {
        let base = tempdir().unwrap();
        // OS 무관 절대 경로를 만들기 위해 다른 tempdir 사용
        let abs_dir = tempdir().unwrap();
        let abs_str = abs_dir.path().to_str().unwrap().replace('\\', "/");
        write_cfg(
            base.path(),
            &format!(
                r#"
                [data]
                scenarios_dir = "{}"
            "#,
                abs_str
            ),
        );
        std::env::remove_var(ENV_SCENARIOS_DIR);
        std::env::remove_var(ENV_GOLDEN_SETS_DIR);

        let paths = DataPaths::load(base.path()).unwrap();
        assert_eq!(paths.scenarios_dir, PathBuf::from(&abs_str));
    }

    /// @trace TC: SPEC-015/TC-5
    /// @trace FR: PRD-015/FR-3
    /// @trace scenario: 환경변수가 설정 파일을 override (apply_env_from 주입형)
    #[test]
    fn test_tc_5_env_overrides_config() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [data]
                scenarios_dir   = "from/cfg/scen"
                golden_sets_dir = "from/cfg/gold"
            "#,
        );

        // 격리: 직접 std::env 를 건드리지 않고 주입형 헬퍼로 검증한다.
        let mut paths = {
            // 임시로 ENV 를 비운 상태에서 load 후 apply_env_from 만 호출
            std::env::remove_var(ENV_SCENARIOS_DIR);
            std::env::remove_var(ENV_GOLDEN_SETS_DIR);
            DataPaths::load(base.path()).unwrap()
        };
        let mut env: BTreeMap<&str, String> = BTreeMap::new();
        env.insert(ENV_SCENARIOS_DIR, "/from/env/scen".to_string());
        env.insert(ENV_GOLDEN_SETS_DIR, "/from/env/gold".to_string());
        paths.apply_env_from(&env);

        assert_eq!(paths.scenarios_dir, PathBuf::from("/from/env/scen"));
        assert_eq!(paths.golden_sets_dir, PathBuf::from("/from/env/gold"));
    }

    /// @trace TC: SPEC-015/TC-6
    /// @trace FR: PRD-015/FR-4
    /// @trace scenario: CLI 인자가 환경변수와 설정 파일을 모두 override
    #[test]
    fn test_tc_6_cli_overrides_all() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [data]
                scenarios_dir   = "from/cfg/scen"
                golden_sets_dir = "from/cfg/gold"
            "#,
        );
        std::env::remove_var(ENV_SCENARIOS_DIR);
        std::env::remove_var(ENV_GOLDEN_SETS_DIR);
        let mut paths = DataPaths::load(base.path()).unwrap();
        let mut env: BTreeMap<&str, String> = BTreeMap::new();
        env.insert(ENV_SCENARIOS_DIR, "/env/scen".to_string());
        paths.apply_env_from(&env);

        let final_paths = paths.with_overrides(Some("cli/scen"), Some("cli/gold"));
        assert_eq!(final_paths.scenarios_dir, PathBuf::from("cli/scen"));
        assert_eq!(final_paths.golden_sets_dir, PathBuf::from("cli/gold"));
    }

    /// @trace TC: SPEC-015/TC-7
    /// @trace FR: PRD-015/FR-5
    /// @trace scenario: 설정 파일/ENV/CLI 모두 없으면 내장 기본값
    #[test]
    fn test_tc_7_builtin_defaults_when_nothing_set() {
        let base = tempdir().unwrap();
        std::env::remove_var(ENV_SCENARIOS_DIR);
        std::env::remove_var(ENV_GOLDEN_SETS_DIR);

        let paths = DataPaths::load(base.path()).unwrap();
        assert_eq!(paths.scenarios_dir, PathBuf::from(DEFAULT_SCENARIOS_DIR));
        assert_eq!(paths.golden_sets_dir, PathBuf::from(DEFAULT_GOLDEN_SETS_DIR));
    }

    /// @trace TC: SPEC-015/TC-8
    /// @trace FR: PRD-015/FR-6
    /// @trace scenario: resolve_for_root 가 워크스페이스 루트 기준으로 동작
    #[test]
    fn test_tc_8_resolve_for_root_joins_root() {
        let root = tempdir().unwrap();
        std::env::remove_var(ENV_SCENARIOS_DIR);
        std::env::remove_var(ENV_GOLDEN_SETS_DIR);

        let paths = DataPaths::resolve_for_root(root.path()).unwrap();
        assert_eq!(paths.scenarios_dir, root.path().join(DEFAULT_SCENARIOS_DIR));
        assert_eq!(paths.golden_sets_dir, root.path().join(DEFAULT_GOLDEN_SETS_DIR));
    }

    /// @trace TC: SPEC-016/TC-1
    /// @trace FR: PRD-016/FR-1
    /// @trace scenario: [evaluation] 두 키 모두 있는 TOML 로드
    #[test]
    fn test_spec016_tc_1_load_evaluation_full() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [evaluation]
                max_iterations       = 5
                early_stop_threshold = 4
            "#,
        );
        let cfg = load_evaluation_config(base.path()).unwrap();
        assert_eq!(cfg.max_iterations, 5);
        assert_eq!(cfg.early_stop_threshold, 4);
    }

    /// @trace TC: SPEC-016/TC-2
    /// @trace FR: PRD-016/FR-2
    /// @trace scenario: [evaluation] 섹션 없음 → 기본값
    #[test]
    fn test_spec016_tc_2_missing_section_defaults() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [data]
                scenarios_dir = "x/y"
            "#,
        );
        let cfg = load_evaluation_config(base.path()).unwrap();
        let default = agent_core::config::EvaluationConfig::default();
        assert_eq!(cfg.max_iterations, default.max_iterations);
        assert_eq!(cfg.early_stop_threshold, default.early_stop_threshold);
    }

    /// @trace TC: SPEC-016/TC-3
    /// @trace FR: PRD-016/FR-1, PRD-016/FR-2
    /// @trace scenario: 일부 키만 override
    #[test]
    fn test_spec016_tc_3_partial_key_override() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [evaluation]
                max_iterations = 7
            "#,
        );
        let cfg = load_evaluation_config(base.path()).unwrap();
        let default = agent_core::config::EvaluationConfig::default();
        assert_eq!(cfg.max_iterations, 7);
        assert_eq!(cfg.early_stop_threshold, default.early_stop_threshold);
    }

    /// @trace TC: SPEC-016/TC-4
    /// @trace FR: PRD-016/NFR-1
    /// @trace scenario: max_iterations=0 거부
    #[test]
    fn test_spec016_tc_4_zero_rejected() {
        let base = tempdir().unwrap();
        write_cfg(
            base.path(),
            r#"
                [evaluation]
                max_iterations = 0
            "#,
        );
        let err = load_evaluation_config(base.path()).unwrap_err();
        match err {
            | DataPathsError::Invalid {
                message, ..
            } => assert!(message.contains("max_iterations")),
            | other => panic!("expected Invalid, got: {other:?}"),
        }
    }

    /// @trace TC: SPEC-015/TC-9
    /// @trace FR: PRD-015/NFR-2
    /// @trace scenario: 잘못된 TOML 은 명확한 에러로 실패
    #[test]
    fn test_tc_9_invalid_toml_returns_parse_error() {
        let base = tempdir().unwrap();
        write_cfg(base.path(), "this is = = not toml");
        let err = DataPaths::load(base.path()).unwrap_err();
        match err {
            | DataPathsError::Parse {
                path, ..
            } => {
                assert_eq!(path, base.path().join(DEFAULT_CONFIG_FILENAME));
            },
            | other => panic!("expected Parse error, got: {other:?}"),
        }
    }
}
