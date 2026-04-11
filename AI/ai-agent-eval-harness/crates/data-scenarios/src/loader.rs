// =============================================================================
// @trace SPEC-017
// @trace PRD: PRD-017
// @trace FR: PRD-017/FR-3, PRD-017/FR-5, PRD-017/FR-6
// @trace file-type: impl
// =============================================================================

use crate::{models::GoldenSetFile,
            sqlite_store::SqliteStore};
use agent_models::domain_config::DomainConfig;
use anyhow::{Context,
             Result,
             anyhow};
use std::{path::{Path,
                 PathBuf},
          sync::{Arc,
                 OnceLock}};
use tokio::runtime::{Builder,
                     Runtime};

/// 런타임 + SqliteStore 를 묶은 내부 핸들.
struct LoaderInner {
    runtime: Runtime,
    store: SqliteStore,
}

impl LoaderInner {
    fn new_blocking(db_path: &Path, scen_dir: &Path, gold_dir: &Path) -> Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .thread_name("data-scenarios-loader")
            .build()
            .context("failed to build tokio runtime for ScenarioLoader")?;
        let store = runtime
            .block_on(async { SqliteStore::open_and_seed(db_path, scen_dir, gold_dir).await })
            .map(|(s, _)| s)
            .map_err(|e| anyhow!("SqliteStore open_and_seed failed: {e}"))?;
        Ok(Self {
            runtime,
            store,
        })
    }

    fn new_in_memory_blocking(scen_dir: &Path, gold_dir: &Path) -> Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .thread_name("data-scenarios-loader-mem")
            .build()?;
        // 인메모리 저장소: 파일 DB 대신 max_connections=1 SQLite ":memory:"
        let store = runtime.block_on(async {
            let s = SqliteStore::open_in_memory_for_loader().await.map_err(|e| anyhow!("{e}"))?;
            s.seed_from_fs(scen_dir, gold_dir).await.map_err(|e| anyhow!("{e}"))?;
            Ok::<SqliteStore, anyhow::Error>(s)
        })?;
        Ok(Self {
            runtime,
            store,
        })
    }
}

static INSTALLED: OnceLock<Arc<LoaderInner>> = OnceLock::new();
static FALLBACK: OnceLock<Arc<LoaderInner>> = OnceLock::new();

/// 기존 호출부와 호환되는 얇은 로더 래퍼.
///
/// @trace SPEC: SPEC-017
/// @trace FR: PRD-017/FR-3
pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn new() -> Self { Self }

    /// 진입점에서 기동 시 1회 호출. 파일 DB 기반 로더를 전역 설치.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-5
    pub fn install(db_path: &Path, scenarios_dir: &Path, golden_sets_dir: &Path) -> Result<()> {
        let inner = Arc::new(LoaderInner::new_blocking(db_path, scenarios_dir, golden_sets_dir)?);
        INSTALLED.set(inner).map_err(|_| anyhow!("ScenarioLoader already installed")).ok();
        Ok(())
    }

    fn resolve(seed_dir_hint: &str) -> Result<Arc<LoaderInner>> {
        if let Some(g) = INSTALLED.get() {
            return Ok(g.clone());
        }
        if let Some(f) = FALLBACK.get() {
            return Ok(f.clone());
        }
        let scen = PathBuf::from(seed_dir_hint);
        let gold = derive_golden_dir(&scen);
        let inner = Arc::new(LoaderInner::new_in_memory_blocking(&scen, &gold)?);
        let _ = FALLBACK.set(inner.clone());
        Ok(inner)
    }

    /// 호환 API. `directory` 인자는 전역 로더가 없을 때 seed 소스 힌트로만
    /// 사용.
    pub fn load_domain_config(&self, filepath: &str) -> Result<DomainConfig> {
        // 옛 API: 단일 YAML 파일 로드. 파일명에서 도메인 유추.
        let p = Path::new(filepath);
        let hint = p.parent().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
        let inner = Self::resolve(&hint)?;
        let stem = p.file_stem().and_then(|s| s.to_str()).ok_or_else(|| anyhow!("invalid filepath: {filepath}"))?;
        let all = inner
            .runtime
            .block_on(async { inner.store.load_all_domains().await })
            .map_err(|e| anyhow!("{e}"))?;
        all.into_iter()
            .find(|d| d.name == stem)
            .ok_or_else(|| anyhow!("domain '{stem}' not found in store"))
    }

    pub fn load_all_domains(&self, directory: &str) -> Result<Vec<DomainConfig>> {
        let inner = Self::resolve(directory)?;
        inner
            .runtime
            .block_on(async { inner.store.load_all_domains().await })
            .map_err(|e| anyhow!("{e}"))
    }

    /// 골든셋은 SQLite 에도 적재되지만, 웹/테스트 호환성을 위해 loader 공개
    /// API 는 디렉토리/파일 인자를 그대로 사용해 파일에서 직접 파싱한다.
    /// (시나리오 경로만 SQLite 경유.)
    pub fn load_golden_sets(&self, filepath: &str) -> Result<GoldenSetFile> {
        let content = std::fs::read_to_string(filepath).with_context(|| format!("골든셋 파일 읽기 실패: {}", filepath))?;
        serde_json::from_str(&content).with_context(|| format!("JSON 파싱 실패: {}", filepath))
    }

    pub fn load_all_golden_sets(&self, directory: &str) -> Result<Vec<GoldenSetFile>> {
        let dir = Path::new(directory);
        let mut result = Vec::new();
        if !dir.exists() {
            return Ok(result);
        }
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

fn derive_golden_dir(scen_dir: &Path) -> PathBuf {
    // `eval_data/eval_scenarios` → `eval_data/golden_sets` 유추.
    scen_dir
        .parent()
        .map(|p| p.join("golden_sets"))
        .unwrap_or_else(|| PathBuf::from("eval_data/golden_sets"))
}
