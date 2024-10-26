rustup target add wasm32-unknown-unknown

try {
    Copy-Item .\target\wasm32-unknown-unknown\release\deps\rust.wasm ..\node\  -Force -ErrorAction Stop
    Write-Output "File coyping was successful."
} catch {
    Write-Output "File copy failed."
}
