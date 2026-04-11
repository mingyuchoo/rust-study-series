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
use std::{path::Path,
          sync::{Arc,
                 OnceLock}};
use tokio::runtime::{Builder,
                     Runtime};

/// 런타임 + SqliteStore 를 묶은 내부 핸들.
struct LoaderInner {
    runtime: Runtime,
    store: Arc<SqliteStore>,
}

/// SPEC-021: 전역 install 된 SqliteStore 가 있으면 빌려온다. 없으면 None.
/// 외부 크레이트(예: reporting::SqliteLogger) 가 dual-write 를 위해 사용한다.
///
/// @trace SPEC: SPEC-021
/// @trace FR: PRD-021/FR-3
pub fn try_installed_store() -> Option<Arc<SqliteStore>> { INSTALLED.get().map(|inner| inner.store.clone()) }

impl LoaderInner {
    fn new_blocking(db_path: &Path) -> Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .thread_name("data-scenarios-loader")
            .build()
            .context("failed to build tokio runtime for ScenarioLoader")?;
        let store = runtime
            .block_on(async { SqliteStore::open_and_seed(db_path).await })
            .map(|(s, _)| Arc::new(s))
            .map_err(|e| anyhow!("SqliteStore open_and_seed failed: {e}"))?;
        Ok(Self {
            runtime,
            store,
        })
    }

    fn new_in_memory_blocking() -> Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .thread_name("data-scenarios-loader-mem")
            .build()?;
        // 인메모리 저장소: 파일 DB 대신 max_connections=1 SQLite ":memory:"
        let store = runtime.block_on(async {
            let s = SqliteStore::open_in_memory_for_loader().await.map_err(|e| anyhow!("{e}"))?;
            s.seed_from_embedded().await.map_err(|e| anyhow!("{e}"))?;
            Ok::<SqliteStore, anyhow::Error>(s)
        })?;
        Ok(Self {
            runtime,
            store: Arc::new(store),
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

    /// 진입점에서 기동 시 1회 호출. 파일 DB 기반 로더를 전역 설치. 시드 원본은
    /// 크레이트에 내장된 기본값(`seed_embedded`)을 사용한다.
    ///
    /// @trace SPEC: SPEC-017
    /// @trace FR: PRD-017/FR-5
    pub fn install(db_path: &Path) -> Result<()> {
        let inner = Arc::new(LoaderInner::new_blocking(db_path)?);
        INSTALLED.set(inner).map_err(|_| anyhow!("ScenarioLoader already installed")).ok();
        Ok(())
    }

    fn resolve() -> Result<Arc<LoaderInner>> {
        if let Some(g) = INSTALLED.get() {
            return Ok(g.clone());
        }
        if let Some(f) = FALLBACK.get() {
            return Ok(f.clone());
        }
        let inner = Arc::new(LoaderInner::new_in_memory_blocking()?);
        let _ = FALLBACK.set(inner.clone());
        Ok(inner)
    }

    /// 호환 API: 단일 도메인 설정 로드. `filepath` 의 파일 stem 이 곧 도메인
    /// 이름이며, 실제 데이터는 SqliteStore 에서 가져온다.
    pub fn load_domain_config(&self, filepath: &str) -> Result<DomainConfig> {
        let p = Path::new(filepath);
        let stem = p.file_stem().and_then(|s| s.to_str()).ok_or_else(|| anyhow!("invalid filepath: {filepath}"))?;
        let inner = Self::resolve()?;
        let all = inner
            .runtime
            .block_on(async { inner.store.load_all_domains().await })
            .map_err(|e| anyhow!("{e}"))?;
        all.into_iter()
            .find(|d| d.name == stem)
            .ok_or_else(|| anyhow!("domain '{stem}' not found in store"))
    }

    /// 호환 API: `_directory` 인자는 더 이상 사용되지 않는다 (SqliteStore
    /// 조회).
    pub fn load_all_domains(&self, _directory: &str) -> Result<Vec<DomainConfig>> {
        let inner = Self::resolve()?;
        inner
            .runtime
            .block_on(async { inner.store.load_all_domains().await })
            .map_err(|e| anyhow!("{e}"))
    }

    /// 골든셋 로드. `filepath` 의 파일 stem 이 도메인 이름이며, 실제 데이터는
    /// SqliteStore 에서 가져온다.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-7
    pub fn load_golden_sets(&self, filepath: &str) -> Result<GoldenSetFile> {
        let p = Path::new(filepath);
        let stem = p.file_stem().and_then(|s| s.to_str()).ok_or_else(|| anyhow!("invalid filepath: {filepath}"))?;
        let inner = Self::resolve()?;
        inner
            .runtime
            .block_on(async { inner.store.load_golden_sets_by_domain(stem).await })
            .map_err(|e| anyhow!("{e}"))
    }

    /// 호환 API: `_directory` 인자는 더 이상 사용되지 않는다.
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-7
    pub fn load_all_golden_sets(&self, _directory: &str) -> Result<Vec<GoldenSetFile>> {
        let inner = Self::resolve()?;
        inner
            .runtime
            .block_on(async { inner.store.load_all_golden_sets().await })
            .map_err(|e| anyhow!("{e}"))
    }
}

impl Default for ScenarioLoader {
    fn default() -> Self { Self::new() }
}
