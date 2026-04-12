// =============================================================================
// eval-harness 기동 시 SQLite 저장소 초기화, 도메인 키워드 시드,
// PromptSet 시드를 수행하는 부트스트랩 모듈.
//
// main.rs 의 install_data_store() 에서 추출.
// =============================================================================

use crate::data_paths::DataPaths;

/// 기동 시 SQLite 저장소를 전역 설치하고 부트스트랩 시드를 수행한다.
/// 멱등 -- 이미 설치되었으면 이전 인스턴스를 재사용한다.
///
/// @trace SPEC: SPEC-017
/// @trace FR: PRD-017/FR-5
pub fn install_data_store(paths: &DataPaths) {
    use data_scenarios::loader::{ScenarioLoader,
                                 try_installed_store};
    if let Err(e) = ScenarioLoader::install(&paths.db_path) {
        eprintln!("[warn] SQLite 저장소 초기화 실패: {e} -- 인메모리 fallback 모드");
        return;
    }
    println!("[store] SQLite DB: {}", paths.db_path.display());

    if let Some(store) = try_installed_store() {
        seed_domain_keywords(&store);
        seed_prompt_sets();
    }
}

/// SPEC-022: 부트스트랩 도메인의 라우터 키워드를 DB 에 시드한다(멱등).
fn seed_domain_keywords(store: &std::sync::Arc<data_scenarios::sqlite_store::SqliteStore>) {
    let pairs = agent_core::domain_router::default_keywords_flat();
    let store_clone = store.clone();
    let result = run_blocking(async move { store_clone.seed_domain_keywords(&pairs).await });
    match result {
        | Ok(n) if n > 0 => println!("[store] 부트스트랩 키워드 {n}개 시드"),
        | Ok(_) => {},
        | Err(e) => eprintln!("[warn] 키워드 시드 실패: {e}"),
    }
}

/// SPEC-025: 각 도메인에 v1 bootstrap PromptSet 시드 (멱등).
///
/// @trace SPEC: SPEC-025
/// @trace FR: PRD-025/FR-2
fn seed_prompt_sets() {
    let Some(store) = data_scenarios::loader::try_installed_store() else {
        return;
    };
    let store_clone = store.clone();
    let bundle = data_scenarios::sqlite_store::BootstrapBundleRef {
        perceive_system: agent_core::llm_client::BOOTSTRAP_PERCEIVE_SYSTEM,
        perceive_user: agent_core::llm_client::BOOTSTRAP_PERCEIVE_USER,
        policy_system: agent_core::llm_client::BOOTSTRAP_POLICY_SYSTEM,
        policy_user: agent_core::llm_client::BOOTSTRAP_POLICY_USER,
    };
    let result = run_blocking(async move { store_clone.seed_bootstrap_prompt_sets(&bundle).await });
    match result {
        | Ok(n) if n > 0 => println!("[store] 부트스트랩 PromptSet {n}개 시드"),
        | Ok(_) => {},
        | Err(e) => eprintln!("[warn] PromptSet 시드 실패: {e}"),
    }
}

/// 동기 컨텍스트에서 async future 를 실행하는 헬퍼.
/// tokio 런타임이 이미 있으면 `block_in_place` 로, 없으면 임시 런타임을 만든다.
fn run_blocking<F: std::future::Future>(fut: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        | Ok(handle) => tokio::task::block_in_place(|| handle.block_on(fut)),
        | Err(_) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
            .block_on(fut),
    }
}
