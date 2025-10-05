#!/bin/bash

echo "Building integrated React + Rust application..."

# Navigate to frontend directory and build
echo "Building React frontend..."
cd "./frontend"
pnpm install
if [ $? -ne 0 ]; then
    echo "Failed to install frontend dependencies"
    exit 1
fi

pnpm run build:backend
if [ $? -ne 0 ]; then
    echo "Failed to build frontend"
    exit 1
fi

cd ../

# Navigate to backend directory and build
echo "Building Rust backend..."
cd "./backend"
cargo build
if [ $? -ne 0 ]; then
    echo "Failed to build backend"
    exit 1
fi

echo "Build completed successfully!"

# Ask user if they want to run the application
read -p "Do you want to start the application now? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Starting Rust application..."
    echo "Application will be available at:"
    echo "  - HTTPS: https://localhost:8000"
    echo "  - HTTP:  http://localhost:5173"
    echo ""
    echo "API endpoints:"
    echo "  - Products API: https://localhost:8000/api/products"
    echo "  - Swagger UI:   https://localhost:8000/swagger"
    echo ""
    echo "Press Ctrl+C to stop the application"
    echo "----------------------------------------"
    
    # Run the application
    cargo run
else
    echo "To start the application later, run:"
    echo "  cd backend"
    echo "  cargo run"
fi