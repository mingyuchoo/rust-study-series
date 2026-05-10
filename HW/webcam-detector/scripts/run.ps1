[CmdletBinding()]
param(
    [ValidateSet("build", "test", "run", "release", "package", "ci", "all", "help")]
    [string]$Action = "all",

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoRunArgs
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "..")
Set-Location $ProjectRoot

$AppPackage = if ($env:APP_PACKAGE) { $env:APP_PACKAGE } else { "webcam-detector" }
if ($CargoRunArgs.Count -gt 0 -and $CargoRunArgs[0] -eq "--") {
    if ($CargoRunArgs.Count -eq 1) {
        $CargoRunArgs = @()
    } else {
        $CargoRunArgs = $CargoRunArgs[1..($CargoRunArgs.Count - 1)]
    }
}

function Show-Usage {
    Write-Host @"
Usage: scripts/run.ps1 [build|test|run|release|package|ci|all] [-- <cargo run args>]

Actions:
  build  Build the current codebase.
  test   Run all tests.
  run    Run the application.
  release Build an optimized desktop binary.
  package Copy the release binary into dist/.
  ci     Format, check, clippy, and test.
  all    Build, test, then run. This is the default.

Environment:
  APP_PACKAGE  Workspace package to run. Default: $AppPackage
"@
}

function Invoke-CargoBuild {
    cargo build --workspace --profile dev --all-features
}

function Invoke-CargoTest {
    cargo test --workspace --all-targets --all-features
}

function Invoke-CargoRun {
    cargo run -p $AppPackage -- @CargoRunArgs
}

function Invoke-CargoRelease {
    cargo build --release -p $AppPackage
}

function Invoke-CargoPackage {
    Invoke-CargoRelease

    New-Item -ItemType Directory -Force -Path "dist" | Out-Null
    $ExeName = if ($IsWindows -or $env:OS -eq "Windows_NT") { "$AppPackage.exe" } else { $AppPackage }
    Copy-Item -Force (Join-Path "target/release" $ExeName) (Join-Path "dist" $ExeName)
    Write-Host "Packaged desktop binary: dist/$ExeName"
}

function Invoke-CargoCi {
    cargo fmt --all
    cargo check --workspace --all-targets --all-features
    cargo clippy --workspace --all-targets --all-features
    Invoke-CargoTest
}

switch ($Action) {
    "build" {
        Invoke-CargoBuild
    }
    "test" {
        Invoke-CargoTest
    }
    "run" {
        Invoke-CargoRun
    }
    "release" {
        Invoke-CargoRelease
    }
    "package" {
        Invoke-CargoPackage
    }
    "ci" {
        Invoke-CargoCi
    }
    "all" {
        Invoke-CargoBuild
        Invoke-CargoTest
        Invoke-CargoRun
    }
    "help" {
        Show-Usage
    }
}
