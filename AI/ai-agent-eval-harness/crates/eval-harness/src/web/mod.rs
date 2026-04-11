// =============================================================================
// @trace SPEC-002
// @trace PRD: PRD-002
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5
// @trace file-type: impl
// =============================================================================

pub mod api;
pub mod api_crud;
pub mod api_exec;
pub mod handlers;

use axum::{Router,
           routing::{get,
                     post,
                     put}};
use data_scenarios::sqlite_store::SqliteStore;
use std::{io,
          net::SocketAddr,
          path::PathBuf,
          sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub scenarios_dir: PathBuf,
    pub reports_dir: PathBuf,
    pub golden_sets_dir: PathBuf,
    pub trajectories_dir: PathBuf,
    /// CRUD 라우트에서 사용하는 SQLite 저장소. 기동 시 주입된다.
    /// `None` 이면 CRUD 핸들러가 500 을 반환한다 (Option 으로 기존
    /// 호출부 호환성 유지).
    ///
    /// @trace SPEC: SPEC-019
    /// @trace FR: PRD-019/FR-7
    pub store: Option<Arc<SqliteStore>>,
}

/// axum 라우터 빌드. 테스트에서도 재사용 가능.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-1
/// @trace FR: PRD-002/FR-1
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .route("/help", get(handlers::help))
        .route("/api/scenarios", get(handlers::list_scenarios))
        .route("/api/scenarios/:domain/:id", get(api::scenario_detail))
        .route("/api/reports", get(handlers::list_reports))
        .route("/api/reports/:name", get(handlers::get_report))
        .route("/api/agents", get(api::list_agents))
        .route("/api/tools", get(api::list_tools))
        .route("/api/golden-sets", get(api::list_golden_sets))
        .route("/api/list", get(api::list_all))
        .route("/api/run", post(api::run_eval_scenario))
        .route("/api/compare", post(api::compare_reports))
        .route("/api/scenarios/:domain/:id/run", post(api_exec::run_scenario))
        .route("/api/agents/:name/execute", post(api_exec::agent_execute))
        .route("/api/tools/:name/invoke", post(api_exec::tool_invoke))
        .route("/api/tools/:name/simulate-fault", post(api_exec::fault_sim))
        .route("/api/golden-sets/:domain/:scenario_id", get(api_exec::golden_entry))
        .route("/api/score", post(api_exec::score))
        .route("/api/trajectories", get(api_exec::list_trajectories))
        .route("/api/trajectories/:name", get(api_exec::get_trajectory))
        // -------- SPEC-019: CRUD 라우트 --------
        .route("/api/eval-scenarios/:domain", post(api_crud::create_scenario))
        .route("/api/eval-scenarios/:domain/:id", put(api_crud::update_scenario_handler).delete(api_crud::delete_scenario_handler))
        .route("/api/golden-sets/:domain", post(api_crud::create_golden_entry))
        .route("/api/golden-sets/:domain/:scenario_id", put(api_crud::update_golden_entry_handler).delete(api_crud::delete_golden_entry_handler))
        .with_state(state)
}

/// 블로킹 진입점. 내부에서 tokio 런타임을 띄운다.
///
/// @trace SPEC: SPEC-002
/// @trace TC: SPEC-002/TC-1
/// @trace FR: PRD-002/FR-1
pub fn run_server(addr: SocketAddr, scenarios_dir: PathBuf, reports_dir: PathBuf, golden_sets_dir: PathBuf, trajectories_dir: PathBuf) -> io::Result<()> {
    let state = AppState {
        scenarios_dir,
        reports_dir,
        golden_sets_dir,
        trajectories_dir,
        store: None,
    };
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        let app = build_router(state);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("eval-harness web client listening on http://{}", addr);
        axum::serve(listener, app).await
    })
}
