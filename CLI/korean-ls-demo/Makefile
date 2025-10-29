.PHONY: all build-server build-extension install clean test

all: build-server build-extension

# Rust Language Server 빌드
build-server:
	@echo "Building Rust Language Server..."
	cd rust-server && cargo build --release

# VSCode Extension 빌드
build-extension:
	@echo "Building VSCode Extension..."
	cd vscode-extension && npm install && npm run compile

# 전체 설치 (의존성 포함)
install:
	@echo "Installing dependencies..."
	cd rust-server && cargo fetch
	cd vscode-extension && npm install

# 정리
clean:
	@echo "Cleaning build artifacts..."
	cd rust-server && cargo clean
	cd vscode-extension && rm -rf node_modules out

# 테스트
test:
	@echo "Running tests..."
	cd rust-server && cargo test

# 개발 모드 (watch)
dev-extension:
	cd vscode-extension && npm run watch

# 서버 실행
run-server:
	cd rust-server && cargo run --release
