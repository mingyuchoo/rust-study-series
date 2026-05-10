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

## Recording

The `REC` button records the live webcam stream to MP4 files under `recordings/`.
The app uses `ffmpeg` for encoding, so install `ffmpeg` and make sure it is available on `PATH`.

The `PLAY` button opens the last completed recording with the operating system's default video player. When the app starts again, it scans `recordings/` and uses the newest `webcam-detector-*.mp4` file as the last recording.

## Preview Size

Use `ZOOM-` and `ZOOM+` to change the webcam preview size from 50% to 200%.
Use `100%` to reset the preview to the camera's native frame size.
Recording and face recognition keep using the original camera frame, so changing the preview size only affects the on-screen display.
The preview window is resizable, and you can move the webcam image inside it by dragging the image with the right mouse button.

## Face Tags

The preview scans incoming frames for face-like regions and draws an `UNKNOWN` tag over each candidate.
Click the face box you want to register first. The selected box is highlighted in yellow.
If the detector misses the face, drag over the face in the preview to create a manual yellow box.
Click `ADD` to register the selected candidate. A small form opens for `NAME`, `AGE`, and `GENDER`.
Use `TAB` to move between fields, `ENTER` to save, and `ESC` to cancel.
When a future candidate matches the local embedding, the overlay changes from `UNKNOWN` to that registered person's name, age, gender, and match confidence when the metadata is available.
Click `DEL` while a registered person is selected or recognized to remove that person and their local embeddings.
The local face registry is stored at `face-registry/people.json`.

This is still a lightweight local recognition pipeline. The current detector and embedding are heuristic so the app can run without model files. The next stage should replace them with an ONNX face detector and ArcFace-style embedding model for reliable identity matching.
The code is already split behind `FaceDetector` and `FaceRecognizer` traits so the heuristic implementation can be replaced without changing the UI loop.

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
