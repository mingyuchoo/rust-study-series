.PHONY: all build build-component build-server build-client run-server run-client clean help

WASM_TARGET := wasm32-wasip2
WASM_OUTPUT := target/$(WASM_TARGET)/release/greeting_component.wasm

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

all: build ## Build all components

setup: ## Install required targets and tools
	rustup target add $(WASM_TARGET)

build-component: ## Build the WASI 0.2 component (wasm32-wasip2)
	cargo build --target $(WASM_TARGET) --release -p greeting-component
	@echo "WASM component built: $(WASM_OUTPUT)"

build-server: ## Build the gRPC server
	cargo build --release -p greeting-server

build-client: ## Build the gRPC client
	cargo build --release -p greeting-client

build: build-component build-server build-client ## Build all targets

run-server: build-component ## Run the gRPC server (builds WASM component first)
	GREETING_WASM_PATH=$(WASM_OUTPUT) cargo run --release -p greeting-server

run-client: ## Run the gRPC client
	cargo run --release -p greeting-client

run-client-dev: ## Run the gRPC client (debug mode)
	cargo run -p greeting-client

clean: ## Clean build artifacts
	cargo clean

check: ## Check all crates for errors
	cargo check -p greeting-server
	cargo check -p greeting-client
	cargo check --target $(WASM_TARGET) -p greeting-component

fmt: ## Format all code
	cargo fmt --all

clippy: ## Run clippy on native crates
	cargo clippy -p greeting-server -p greeting-client -- -D warnings
