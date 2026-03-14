.PHONY: all build build-component build-server build-client run-server run-client clean help start-db start-db-file stop-db

WASM_TARGET := wasm32-wasip2
WASM_OUTPUT := target/$(WASM_TARGET)/release/blog_component.wasm

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

all: build ## Build all components

setup: ## Install required targets and tools
	rustup target add $(WASM_TARGET)

start-db: ## Start SurrealDB (in-memory)
	docker run --rm --name surrealdb -p 8000:8000 \
		surrealdb/surrealdb:latest start \
		--username root --password root --bind 0.0.0.0:8000 memory

start-db-file: ## Start SurrealDB (file-based persistent storage)
	docker run --rm --name surrealdb -p 8000:8000 \
		-v $(CURDIR)/blog.db:/data/blog.db \
		surrealdb/surrealdb:latest start \
		--username root --password root --bind 0.0.0.0:8000 file:/data/blog.db

stop-db: ## Stop SurrealDB
	docker stop surrealdb

build-component: ## Build the WASI 0.2 component (wasm32-wasip2)
	cargo build --target $(WASM_TARGET) --release -p blog-component
	@echo "WASM component built: $(WASM_OUTPUT)"

build-server: ## Build the gRPC server
	cargo build --release -p blog-server

build-client: ## Build the gRPC client
	cargo build --release -p blog-client

build: build-component build-server build-client ## Build all targets

run-server: build-component ## Run the gRPC server (builds WASM component first)
	BLOG_WASM_PATH=$(WASM_OUTPUT) cargo run --release -p blog-server

run-client: ## Run the gRPC client
	cargo run --release -p blog-client

run-client-dev: ## Run the gRPC client (debug mode)
	cargo run -p blog-client

clean: ## Clean build artifacts
	cargo clean

check: ## Check all crates for errors
	cargo check -p blog-server
	cargo check -p blog-client
	cargo check --target $(WASM_TARGET) -p blog-component

fmt: ## Format all code
	cargo fmt --all

clippy: ## Run clippy on native crates
	cargo clippy -p blog-server -p blog-client -- -D warnings
