use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, instrument};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub mod proto {
    tonic::include_proto!("greeting");
}

use proto::greeting_service_server::{GreetingService, GreetingServiceServer};
use proto::{GreetRequest, GreetResponse, VersionRequest, VersionResponse};

wasmtime::component::bindgen!({
    world: "greeting-world",
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

// Shared WASM runtime context (Engine + Component + Linker are thread-safe)
struct WasmRuntime {
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

    fn call_greet(&self, name: &str) -> Result<String> {
        let state = WasiState {
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            table: ResourceTable::new(),
        };
        let mut store = Store::new(&self.engine, state);
        let world =
            GreetingWorld::instantiate(&mut store, &self.component, &self.linker)?;

        let result = world
            .component_greeting_greeter()
            .call_greet(&mut store, name)?;
        Ok(result)
    }

    fn call_get_version(&self) -> Result<String> {
        let state = WasiState {
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            table: ResourceTable::new(),
        };
        let mut store = Store::new(&self.engine, state);
        let world =
            GreetingWorld::instantiate(&mut store, &self.component, &self.linker)?;

        let result = world
            .component_greeting_greeter()
            .call_get_version(&mut store)?;
        Ok(result)
    }
}

// gRPC service implementation
pub struct GreetingServiceImpl {
    runtime: Arc<WasmRuntime>,
}

impl GreetingServiceImpl {
    fn new(runtime: Arc<WasmRuntime>) -> Self {
        Self { runtime }
    }
}

#[tonic::async_trait]
impl GreetingService for GreetingServiceImpl {
    #[instrument(skip(self))]
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        let name = request.into_inner().name;
        info!("Received greet request for: {}", name);

        let runtime = self.runtime.clone();
        let name_clone = name.clone();

        // Run blocking WASM call in thread pool
        let message = tokio::task::spawn_blocking(move || runtime.call_greet(&name_clone))
            .await
            .map_err(|e| Status::internal(format!("Task join error: {}", e)))?
            .map_err(|e| Status::internal(format!("WASM error: {}", e)))?;

        info!("Responding with: {}", message);
        Ok(Response::new(GreetResponse { message }))
    }

    #[instrument(skip(self))]
    async fn get_version(
        &self,
        _request: Request<VersionRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        info!("Received get_version request");

        let runtime = self.runtime.clone();
        let version = tokio::task::spawn_blocking(move || runtime.call_get_version())
            .await
            .map_err(|e| Status::internal(format!("Task join error: {}", e)))?
            .map_err(|e| Status::internal(format!("WASM error: {}", e)))?;

        Ok(Response::new(VersionResponse { version }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("greeting_server=info".parse()?),
        )
        .init();

    let wasm_path = PathBuf::from(
        std::env::var("GREETING_WASM_PATH")
            .unwrap_or_else(|_| "../target/wasm32-wasip2/release/greeting_component.wasm".into()),
    );

    info!("Loading WASM component from: {:?}", wasm_path);
    let runtime = Arc::new(WasmRuntime::new(&wasm_path)?);
    info!("WASM component loaded successfully");

    let addr = "0.0.0.0:50051".parse()?;
    let service = GreetingServiceImpl::new(runtime);

    info!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(GreetingServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
