.PHONY: help build run docker-up docker-down test clean fmt

help:
	@echo "Available commands:"
	@echo "  make build       - Build the project"
	@echo "  make run         - Run the web service locally"
	@echo "  make docker-up   - Start with Docker Compose"
	@echo "  make docker-down - Stop Docker containers"
	@echo "  make test        - Test the API endpoints"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make fmt         - Format code"

build:
	cargo build --release

run:
	cargo run -p main

docker-up:
	docker-compose -f docker/docker-compose.yaml up --build

docker-down:
	docker-compose -f docker/docker-compose.yaml down

test:
	@echo "Testing API endpoints..."
	@./test_api.sh

clean:
	cargo clean

fmt:
	cargo fmt --all
