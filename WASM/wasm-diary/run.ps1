#Requires -Version 7.0
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RootDir   = $PSScriptRoot
$WasmDir   = Join-Path $RootDir "wasm-lib"
$ReactDir  = Join-Path $RootDir "react-app"

# 색상 출력
function Write-Info  { param([string]$Msg) Write-Host "[INFO]  $Msg" -ForegroundColor Blue }
function Write-Ok    { param([string]$Msg) Write-Host "[OK]    $Msg" -ForegroundColor Green }
function Write-Err   { param([string]$Msg) Write-Host "[ERR]   $Msg" -ForegroundColor Red }

function Test-Prerequisites {
    $missing = @()
    if (-not (Get-Command rustup  -ErrorAction SilentlyContinue)) { $missing += "rustup" }
    if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) { $missing += "wasm-pack (cargo install wasm-pack)" }
    if (-not (Get-Command bun     -ErrorAction SilentlyContinue)) { $missing += "bun" }

    if ($missing.Count -gt 0) {
        Write-Err "필수 도구가 설치되지 않았습니다:"
        foreach ($m in $missing) {
            Write-Err "  - $m"
        }
        exit 1
    }

    $installed = rustup target list --installed
    if ($installed -notcontains "wasm32-unknown-unknown") {
        Write-Info "wasm32-unknown-unknown 타겟 추가 중..."
        rustup target add wasm32-unknown-unknown
    }
}

function Invoke-WasmBuild {
    param([string]$Profile = "dev")

    Write-Info "WASM 빌드 시작 (profile: $Profile)..."

    Push-Location $WasmDir
    try {
        if ($Profile -eq "release") {
            wasm-pack build --target web --out-dir pkg --release
        } else {
            wasm-pack build --target web --out-dir pkg --dev
        }
    } finally {
        Pop-Location
    }

    Write-Ok "WASM 빌드 완료 → wasm-lib/pkg/"
}

function Invoke-ReactInstall {
    Write-Info "React 앱 의존성 설치 중..."

    Push-Location $ReactDir
    try {
        # bun install의 비정상 종료를 허용 (wasm-lib EPERM 무시)
        $prevPref = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        bun install 2>&1 | Out-Host
        $ErrorActionPreference = $prevPref
    } finally {
        Pop-Location
    }

    # bun의 Windows file: 프로토콜 EPERM 버그 workaround
    $wasmDest = Join-Path $ReactDir "node_modules" "wasm-lib"
    $wasmSrc  = Join-Path $WasmDir "pkg"
    $testFile = Join-Path $wasmDest "package.json"
    if (-not (Test-Path $testFile)) {
        Write-Info "wasm-lib 수동 복사 중 (bun EPERM workaround)..."
        if (Test-Path $wasmDest) { Remove-Item $wasmDest -Recurse -Force }
        Copy-Item $wasmSrc -Destination $wasmDest -Recurse
        Write-Ok "wasm-lib 복사 완료"
    }

    Write-Ok "의존성 설치 완료"
}

function Invoke-Dev {
    Test-Prerequisites
    Invoke-WasmBuild -Profile "dev"
    Invoke-ReactInstall

    Write-Info "개발 서버 시작..."
    Push-Location $ReactDir
    try {
        bun run dev
    } finally {
        Pop-Location
    }
}

function Invoke-Build {
    Test-Prerequisites
    Invoke-WasmBuild -Profile "release"
    Invoke-ReactInstall

    Write-Info "React 프로덕션 빌드 시작..."
    Push-Location $ReactDir
    try {
        bun run build
    } finally {
        Pop-Location
    }

    Write-Ok "프로덕션 빌드 완료 → react-app/dist/"
}

function Invoke-Clean {
    Write-Info "빌드 산출물 정리 중..."

    $targets = @(
        (Join-Path $WasmDir  "pkg"),
        (Join-Path $WasmDir  "target"),
        (Join-Path $ReactDir "node_modules"),
        (Join-Path $ReactDir "dist"),
        (Join-Path $ReactDir "node_modules" ".vite")
    )

    foreach ($path in $targets) {
        if (Test-Path $path) {
            Remove-Item $path -Recurse -Force
        }
    }

    Write-Ok "정리 완료"
}

function Show-Usage {
    Write-Host @"
사용법: .\run.ps1 <command>

Commands:
  dev     WASM 빌드(dev) + 의존성 설치 + Vite 개발 서버 실행
  build   WASM 빌드(release) + React 프로덕션 빌드
  clean   모든 빌드 산출물 제거 (pkg, target, node_modules, dist)
"@
}

# 메인 진입점
$command = if ($args.Count -gt 0) { $args[0] } else { "" }

switch ($command) {
    "dev"   { Invoke-Dev }
    "build" { Invoke-Build }
    "clean" { Invoke-Clean }
    default { Show-Usage; exit 1 }
}
