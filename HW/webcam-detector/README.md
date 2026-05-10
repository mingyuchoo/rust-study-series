# Webcam Detector

Cross-platform desktop webcam viewer built with Rust.

## Platforms

- Linux
- macOS
- Windows

## Layout

```text
apps/webcam-detector   Desktop application
crates/webcam-core     Shared image and frame-buffer logic
```

## Prerequisites

Install a stable Rust toolchain first.

Linux also needs desktop and camera development libraries. On Ubuntu:

```bash
sudo apt-get update
sudo apt-get install -y clang libclang-dev pkg-config libx11-dev libxrandr-dev libxi-dev libxcursor-dev libgl1-mesa-dev libv4l-dev
```

macOS and Windows normally only need Rust and camera permission for the terminal or built binary.

## Run

Linux/macOS:

```bash
./scripts/run.sh run
```

Windows PowerShell:

```powershell
./scripts/run.ps1 run
```

## Build And Test

Linux/macOS:

```bash
./scripts/run.sh build
./scripts/run.sh test
./scripts/run.sh package
```

Windows PowerShell:

```powershell
./scripts/run.ps1 build
./scripts/run.ps1 test
./scripts/run.ps1 package
```

The `package` command writes the optimized desktop binary to `dist/`.

## Installer Bundles

Linux:

```bash
./scripts/release.sh deb
./scripts/release.sh rpm
./scripts/release.sh linux
```

macOS:

```bash
./scripts/release.sh dmg
```

Windows PowerShell:

```powershell
./scripts/release.ps1 msi
```

Installer outputs are written to `dist/`.

Packaging tool requirements:

- `.deb`: `ar`, `tar`, `gzip`
- `.rpm`: `rpmbuild`
- `.dmg`: macOS `hdiutil`
- `.msi`: WiX Toolset v4 `wix` or WiX Toolset v3 `candle` and `light`
