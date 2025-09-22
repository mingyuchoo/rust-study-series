#!/usr/bin/env pwsh

Write-Host "Testing ASP.NET Core application..." -ForegroundColor Green

# Check if the backend directory exists
if (-not (Test-Path "backend")) {
    Write-Host "Backend directory not found: backend" -ForegroundColor Red
    Write-Host "Please run build.ps1 first to build the application" -ForegroundColor Yellow
    exit 1
}

# Navigate to backend directory
Set-Location backend

# Check if the application is built
if (-not (Test-Path "bin/Debug/net9.0/backend.dll")) {
    Write-Host "Application not built. Building now..." -ForegroundColor Yellow
    cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to build application" -ForegroundColor Red
        exit 1
    }
}

Write-Host "Starting ASP.NET Core application for testing..." -ForegroundColor Green
Write-Host "Application will be available at:" -ForegroundColor Cyan
Write-Host "  - HTTPS: https://localhost:7000" -ForegroundColor White
Write-Host "  - HTTP:  http://localhost:5000" -ForegroundColor White
Write-Host ""
Write-Host "API endpoints:" -ForegroundColor Cyan
Write-Host "  - Health check: https://localhost:7000/api/api/health" -ForegroundColor White
Write-Host "  - Sample data:  https://localhost:7000/api/api/data" -ForegroundColor White
Write-Host ""
Write-Host "Press Ctrl+C to stop the application" -ForegroundColor Yellow
Write-Host "----------------------------------------" -ForegroundColor Gray

# Run the application
cargo run