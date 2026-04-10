// =============================================================================
// @trace SPEC-002
// @trace PRD: PRD-002
// @trace FR: FR-1, FR-2, FR-3, FR-4, FR-5
// @trace file-type: impl
// =============================================================================

pub mod api;
pub mod api_exec;
pub mod handlers;

use axum::{Router,
           routing::{get,
                     post}};
use std::{io,
          net::SocketAddr,
          path::PathBuf};

#[derive(Clone)]
pub struct AppState {
    pub scenarios_dir: PathBuf,
    pub reports_dir: PathBuf,
    pub golden_sets_dir: PathBuf,
    pub trajectories_dir: PathBuf,
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
    };
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        let app = build_router(state);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("eval-harness web client listening on http://{}", addr);
        axum::serve(listener, app).await
    })
}
