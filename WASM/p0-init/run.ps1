$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Output "==> Building Rust WASM..."
Set-Location "$ScriptDir\Rust"
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
Copy-Item "target\wasm32-unknown-unknown\release\deps\*.wasm" "..\Node\rust.wasm" -Force
Write-Output "==> WASM file copied to Node\rust.wasm"

Write-Output "==> Running Node.js..."
Set-Location "$ScriptDir\Node"
npm run start
