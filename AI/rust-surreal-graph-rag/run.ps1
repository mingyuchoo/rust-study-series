$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

# ── Backend 빌드 및 실행 ──
Write-Host "[run.ps1] Backend 빌드 중..." -ForegroundColor Cyan
Push-Location "$ScriptDir\backend"
cargo build --profile dev
Pop-Location

Write-Host "[run.ps1] Backend 실행 중... (http://localhost:4000)" -ForegroundColor Cyan
$env:RUST_LOG = "lib_api=debug,actix_web=info"
$backend = Start-Process -NoNewWindow -PassThru -FilePath "cargo" `
    -ArgumentList "run", "-p", "bin-main" `
    -WorkingDirectory "$ScriptDir\backend"

# ── Frontend 설치 및 실행 ──
Write-Host "[run.ps1] Frontend 의존성 설치 중..." -ForegroundColor Cyan
Push-Location "$ScriptDir\frontend"
bun install
Pop-Location

Write-Host "[run.ps1] Frontend 실행 중... (http://localhost:5173)" -ForegroundColor Cyan
$frontend = Start-Process -NoNewWindow -PassThru -FilePath "bun" `
    -ArgumentList "dev" `
    -WorkingDirectory "$ScriptDir\frontend"

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Backend  : http://localhost:4000"      -ForegroundColor Green
Write-Host "  Frontend : http://localhost:5173"      -ForegroundColor Green
Write-Host "  Swagger  : http://localhost:4000/swagger-ui/" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "  종료하려면 Ctrl+C 를 누르세요."        -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

try {
    while (-not $backend.HasExited -and -not $frontend.HasExited) {
        Start-Sleep -Milliseconds 500
    }
}
finally {
    Write-Host "[run.ps1] 프로세스를 종료합니다..." -ForegroundColor Cyan

    if (-not $backend.HasExited) {
        Stop-Process -Id $backend.Id -Force -ErrorAction SilentlyContinue
    }
    if (-not $frontend.HasExited) {
        Stop-Process -Id $frontend.Id -Force -ErrorAction SilentlyContinue
    }

    Write-Host "[run.ps1] 모든 프로세스가 종료되었습니다." -ForegroundColor Cyan
}
