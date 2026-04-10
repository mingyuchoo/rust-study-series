# Desktop app icons

Tauri 2.x 번들링(`cargo tauri build`)은 이 디렉토리에서 플랫폼별 아이콘 파일을 요구합니다.
개발 실행(`cargo tauri dev`)에는 아이콘이 필수가 아니지만, 릴리즈 번들을 만들기 전에 아래 파일들을 추가하세요.

필요한 파일 (Tauri v2 기본):

- `icon.png` (512×512 PNG)
- `icon.ico` (Windows)
- `icon.icns` (macOS)
- `32x32.png`, `128x128.png`, `128x128@2x.png` (Linux)

생성 예시:

```bash
# 단일 PNG 로부터 전체 세트 자동 생성
cargo install tauri-icon
tauri-icon path/to/source.png desktop/icons
```

또는 Tauri CLI 의 `tauri icon` 명령을 사용할 수도 있습니다:

```bash
cargo tauri icon path/to/source.png
```

`tauri.conf.json` 의 `bundle.active` 는 현재 `false` 로 설정되어 있으므로 번들링을 시도하기 전까지
아이콘이 없어도 `cargo tauri dev` 는 정상 동작합니다.
