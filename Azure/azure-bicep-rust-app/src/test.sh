#!/bin/bash

echo "Testing ASP.NET Core application..."

# Check if the backend directory exists
if [ ! -d "backend" ]; then
    echo "Backend directory not found: backend"
    echo "Please run build.sh first to build the application"
    exit 1
fi

# Navigate to backend directory
cd backend

# Check if the application is built
if [ ! -f "bin/Debug/net9.0/backend.dll" ]; then
    echo "Application not built. Building now..."
    cargo build
    if [ $? -ne 0 ]; then
        echo "Failed to build application"
        exit 1
    fi
fi

echo "Starting ASP.NET Core application for testing..."
echo "Application will be available at:"
echo "  - HTTPS: https://localhost:8000"
echo "  - HTTP:  http://localhost:5173"
echo ""
echo "API endpoints:"
echo "  - Health check: https://localhost:8000/api/api/health"
echo "  - Sample data:  https://localhost:8000/api/api/data"
echo ""
echo "Press Ctrl+C to stop the application"
echo "----------------------------------------"

# Run the application
cargo run