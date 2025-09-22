#!/usr/bin/env pwsh

Write-Host "Building integrated React + Rust application..." -ForegroundColor Green

# Navigate to frontend directory and build
Write-Host "Building React frontend..." -ForegroundColor Yellow
Set-Location "/home/mgch/github/mingyuchoo/rust-study-series/Azure/azure-bicep-rust-app/src/frontend"
pnpm install
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to install frontend dependencies" -ForegroundColor Red
    exit 1
}

pnpm run build:backend
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build frontend" -ForegroundColor Red
    exit 1
}

# Navigate to backend directory and build
Write-Host "Building Rust backend..." -ForegroundColor Yellow
Set-Location "/home/mgch/github/mingyuchoo/rust-study-series/Azure/azure-bicep-rust-app/src/backend"
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build backend" -ForegroundColor Red
    exit 1
}

Write-Host "Build completed successfully!" -ForegroundColor Green

# Ask user if they want to run the application
$response = Read-Host "Do you want to start the application now? (y/n)"
if ($response -match "^[Yy]") {
    Write-Host "Starting Rust application..." -ForegroundColor Green
    Write-Host "Application will be available at:" -ForegroundColor Cyan
    Write-Host "  - HTTPS: https://localhost:7000" -ForegroundColor White
    Write-Host "  - HTTP:  http://localhost:5000" -ForegroundColor White
    Write-Host ""
    Write-Host "API endpoints:" -ForegroundColor Cyan
    Write-Host "  - Products API: https://localhost:7000/api/products" -ForegroundColor White
    Write-Host "  - Swagger UI:   https://localhost:7000/swagger" -ForegroundColor White
    Write-Host ""
    Write-Host "Press Ctrl+C to stop the application" -ForegroundColor Yellow
    Write-Host "----------------------------------------" -ForegroundColor Gray
    
    # Run the application
    cargo run
} else {
    Write-Host "To start the application later, run:" -ForegroundColor Cyan
    Write-Host "  Set-Location backend" -ForegroundColor White
    Write-Host "  cargo run" -ForegroundColor White
}