mod auth;
mod db;
mod seed;
mod service;

use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::info;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub mod proto {
    tonic::include_proto!("blog");
}

use db::Database;
use proto::blog_service_server::BlogServiceServer;
use service::BlogServiceImpl;

wasmtime::component::bindgen!({
    world: "blog-world",
    path: "wit",
});

// WASI host state
struct WasiState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for WasiState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

// Shared WASM runtime context
pub struct WasmRuntime {
    engine: Engine,
    component: Component,
    linker: Linker<WasiState>,
}

impl WasmRuntime {
    fn new(wasm_path: &PathBuf) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, wasm_path)?;

        let mut linker: Linker<WasiState> = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        Ok(Self {
            engine,
            component,
            linker,
        })
    }

    fn new_store(&self) -> Store<WasiState> {
        let state = WasiState {
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            table: ResourceTable::new(),
        };
        Store::new(&self.engine, state)
    }

    pub fn call_validate_title(&self, title: &str) -> Result<String> {
        let mut store = self.new_store();
        let world = BlogWorld::instantiate(&mut store, &self.component, &self.linker)?;
        let result = world
            .component_blog_blogger()
            .call_validate_title(&mut store, title)?;
        Ok(result)
    }

    pub fn call_validate_content(&self, content: &str) -> Result<String> {
        let mut store = self.new_store();
        let world = BlogWorld::instantiate(&mut store, &self.component, &self.linker)?;
        let result = world
            .component_blog_blogger()
            .call_validate_content(&mut store, content)?;
        Ok(result)
    }

    pub fn call_validate_comment(&self, content: &str) -> Result<String> {
        let mut store = self.new_store();
        let world = BlogWorld::instantiate(&mut store, &self.component, &self.linker)?;
        let result = world
            .component_blog_blogger()
            .call_validate_comment(&mut store, content)?;
        Ok(result)
    }

    pub fn call_validate_role(&self, role: &str) -> Result<String> {
        let mut store = self.new_store();
        let world = BlogWorld::instantiate(&mut store, &self.component, &self.linker)?;
        let result = world
            .component_blog_blogger()
            .call_validate_role(&mut store, role)?;
        Ok(result)
    }

    pub fn call_validate_visibility(&self, visibility: &str) -> Result<String> {
        let mut store = self.new_store();
        let world = BlogWorld::instantiate(&mut store, &self.component, &self.linker)?;
        let result = world
            .component_blog_blogger()
            .call_validate_visibility(&mut store, visibility)?;
        Ok(result)
    }

    pub fn call_get_version(&self) -> Result<String> {
        let mut store = self.new_store();
        let world = BlogWorld::instantiate(&mut store, &self.component, &self.linker)?;
        let result = world
            .component_blog_blogger()
            .call_get_version(&mut store)?;
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("blog_server=info".parse()?),
        )
        .init();

    // WASM component 로드
    let wasm_path = PathBuf::from(
        std::env::var("BLOG_WASM_PATH")
            .unwrap_or_else(|_| "../target/wasm32-wasip2/release/blog_component.wasm".into()),
    );
    info!("Loading WASM component from: {:?}", wasm_path);
    let wasm_runtime = Arc::new(WasmRuntime::new(&wasm_path)?);
    info!("WASM component loaded successfully");

    // SurrealDB 연결
    let db_addr = std::env::var("SURREALDB_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".into());
    let db_user = std::env::var("SURREALDB_USER").unwrap_or_else(|_| "root".into());
    let db_pass = std::env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".into());

    info!("Connecting to SurrealDB at {}", db_addr);
    let database = Arc::new(Database::new(&db_addr, &db_user, &db_pass).await?);
    database.init_schema().await?;
    info!("SurrealDB connected and schema initialized");

    // seed.json에서 기본 관리자 로드
    let seed_path = PathBuf::from(
        std::env::var("SEED_PATH").unwrap_or_else(|_| "blog-server/data/seed.json".into()),
    );
    let seed_data = seed::SeedData::load(&seed_path)?;
    let admin_hash = auth::hash_password(&seed_data.admin.password)?;

    if database
        .seed_admin(
            &seed_data.admin.username,
            &seed_data.admin.email,
            &admin_hash,
        )
        .await?
    {
        info!(
            "Default admin created: {} ({})",
            seed_data.admin.username, seed_data.admin.email
        );
    }

    // 샘플 데이터 시딩
    let seeded = database
        .seed_sample_data(&seed_data.users, &seed_data.posts)
        .await?;
    if seeded > 0 {
        info!("Sample data seeded: {} posts with comments", seeded);
    }

    // gRPC 서버 시작
    let addr = "0.0.0.0:50051".parse()?;
    let service = BlogServiceImpl::new(database, wasm_runtime);

    info!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(BlogServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
