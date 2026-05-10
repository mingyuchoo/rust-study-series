#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use anyhow::{Context,
             Result};
use minifb::{Key,
             MouseButton,
             MouseMode,
             Window,
             WindowOptions};
use nokhwa::{Buffer,
             Camera,
             pixel_format::RgbFormat,
             utils::{CameraIndex,
                     RequestedFormat,
                     RequestedFormatType}};
use std::{fs,
          io::Write,
          path::{Path,
                 PathBuf},
          process::{Child,
                    ChildStdin,
                    Command,
                    Stdio},
          time::{SystemTime,
                 UNIX_EPOCH}};
use webcam_core::rgb_to_minifb_buffer;

const TOOLBAR_HEIGHT: usize = 56;
const TOOLBAR_BG: u32 = 0x1f2937;
const BUTTON_BG: u32 = 0x374151;
const BUTTON_HOVER_BG: u32 = 0x4b5563;
const BUTTON_TEXT: u32 = 0xf9fafb;
const RECORDING_BG: u32 = 0xb91c1c;
const RECORD_IDLE_BG: u32 = 0x047857;
const EXIT_BG: u32 = 0x7f1d1d;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 첫 번째 카메라(인덱스 0) 열기
    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = Camera::new(index, requested).context("카메라를 열 수 없습니다")?;

    camera.open_stream().context("스트림 시작 실패")?;

    let first_frame = camera.frame().context("첫 프레임 캡처 실패")?;
    let resolution = first_frame.resolution();
    let mut width = resolution.width() as usize;
    let mut height = resolution.height() as usize;

    let frame_rate = camera.frame_rate().max(1);
    let mut window = create_window(width, height)?;
    let mut recorder: Option<Recorder> = None;
    let mut last_recording: Option<PathBuf> = None;
    let mut was_left_down = false;
    let mut should_exit = false;

    tracing::info!("웹캠 스트리밍 시작: {width}x{height}. ESC 키로 종료.");

    let mut pending_frame = Some(first_frame);
    while window.is_open() && !window.is_key_down(Key::Escape) && !should_exit {
        let frame = match pending_frame.take() {
            | Some(frame) => frame,
            | None => camera.frame().context("프레임 캡처 실패")?,
        };

        let resolution = frame.resolution();
        let frame_width = resolution.width() as usize;
        let frame_height = resolution.height() as usize;
        if frame_width != width || frame_height != height {
            if let Some(active_recorder) = recorder.take() {
                match active_recorder.stop() {
                    | Ok(path) => {
                        tracing::warn!("해상도가 변경되어 녹화를 중지했습니다: {}", path.display());
                        last_recording = Some(path);
                    },
                    | Err(error) => tracing::warn!("해상도 변경 중 녹화 종료 실패: {error}"),
                }
            }
            width = frame_width;
            height = frame_height;
            window = create_window(width, height)?;
            tracing::info!("웹캠 해상도 변경: {width}x{height}");
        }

        let decoded_frame = decode_frame(frame, width, height)?;
        if let Some(active_recorder) = recorder.as_mut()
            && let Err(error) = active_recorder.write_frame(&decoded_frame.rgb)
        {
            tracing::warn!("녹화 프레임 저장 실패: {error}");
            if let Some(active_recorder) = recorder.take() {
                match active_recorder.stop() {
                    | Ok(path) => last_recording = Some(path),
                    | Err(error) => tracing::warn!("녹화 종료 실패: {error}"),
                }
            }
        }

        let recording = recorder.is_some();
        let buttons = toolbar_buttons(recording);
        let hover = mouse_position(&window).and_then(|point| button_at(point, &buttons));
        let left_down = window.get_mouse_down(MouseButton::Left);
        if left_down
            && !was_left_down
            && let Some(action) = hover
        {
            match action {
                | UiAction::ToggleRecording =>
                    if let Some(active_recorder) = recorder.take() {
                        match active_recorder.stop() {
                            | Ok(path) => {
                                tracing::info!("녹화 저장 완료: {}", path.display());
                                last_recording = Some(path);
                            },
                            | Err(error) => tracing::warn!("녹화 중지 실패: {error}"),
                        }
                    } else {
                        match Recorder::start(width, height, frame_rate) {
                            | Ok(active_recorder) => {
                                tracing::info!("녹화 시작: {}", active_recorder.path().display());
                                recorder = Some(active_recorder);
                            },
                            | Err(error) => tracing::warn!("녹화 시작 실패: {error}"),
                        }
                    },
                | UiAction::PlayLastRecording => {
                    play_last_recording(last_recording.as_deref());
                },
                | UiAction::Exit => should_exit = true,
            }
        }
        was_left_down = left_down;

        let recording = recorder.is_some();
        let buffer = compose_frame(&decoded_frame.display, width, height, recording, hover);
        window
            .update_with_buffer(&buffer, width, height + TOOLBAR_HEIGHT)
            .context("화면 업데이트 실패")?;
    }

    if let Some(active_recorder) = recorder.take() {
        match active_recorder.stop() {
            | Ok(path) => tracing::info!("종료 전 녹화 저장 완료: {}", path.display()),
            | Err(error) => tracing::warn!("종료 전 녹화 저장 실패: {error}"),
        }
    }

    camera.stop_stream().ok();
    Ok(())
}

fn create_window(width: usize, height: usize) -> Result<Window> {
    Window::new(
        &format!("웹캠 감지기 - {width}x{height}"),
        width,
        height + TOOLBAR_HEIGHT,
        WindowOptions::default(),
    )
    .context("윈도우 생성 실패")
}

struct DecodedFrame {
    rgb: Vec<u8>,
    display: Vec<u32>,
}

fn decode_frame(frame: Buffer, width: usize, height: usize) -> Result<DecodedFrame> {
    let rgb = frame.decode_image::<RgbFormat>().context("프레임 디코드 실패")?;
    let raw = rgb.into_raw();
    let display = rgb_to_minifb_buffer(&raw, width, height).context("프레임 버퍼 변환 실패")?;

    Ok(DecodedFrame {
        rgb: raw,
        display,
    })
}

struct Recorder {
    child: Child,
    stdin: Option<ChildStdin>,
    path: PathBuf,
}

impl Recorder {
    fn start(width: usize, height: usize, frame_rate: u32) -> Result<Self> {
        let output_dir = PathBuf::from("recordings");
        fs::create_dir_all(&output_dir).context("녹화 저장 디렉터리 생성 실패")?;

        let path = output_dir.join(recording_file_name()?);
        let size = format!("{width}x{height}");
        let frame_rate = frame_rate.to_string();
        let mut child = Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgb24",
                "-s",
                &size,
                "-r",
                &frame_rate,
                "-i",
                "pipe:0",
                "-an",
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-movflags",
                "+faststart",
            ])
            .arg(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("ffmpeg 실행 실패. ffmpeg가 설치되어 있고 PATH에 있는지 확인하세요")?;

        let stdin = child.stdin.take().context("ffmpeg stdin 연결 실패")?;

        Ok(Self {
            child,
            stdin: Some(stdin),
            path,
        })
    }

    fn path(&self) -> &Path { &self.path }

    fn write_frame(&mut self, rgb: &[u8]) -> Result<()> {
        let stdin = self.stdin.as_mut().context("녹화 프로세스가 이미 닫혔습니다")?;
        stdin.write_all(rgb).context("ffmpeg stdin 프레임 쓰기 실패")
    }

    fn stop(mut self) -> Result<PathBuf> {
        drop(self.stdin.take());
        let status = self.child.wait().context("ffmpeg 종료 대기 실패")?;
        if !status.success() {
            anyhow::bail!("ffmpeg가 실패 상태로 종료되었습니다: {status}");
        }

        Ok(self.path)
    }
}

fn recording_file_name() -> Result<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("시스템 시간이 UNIX_EPOCH보다 이전입니다")?
        .as_millis();

    Ok(format!("webcam-detector-{timestamp}.mp4"))
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rect {
    fn contains(self, point: Point) -> bool { point.x >= self.x && point.x < self.x + self.width && point.y >= self.y && point.y < self.y + self.height }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum UiAction {
    ToggleRecording,
    PlayLastRecording,
    Exit,
}

#[derive(Clone, Copy)]
struct Button {
    rect: Rect,
    label: &'static str,
    action: UiAction,
    color: u32,
}

fn toolbar_buttons(recording: bool) -> [Button; 3] {
    [
        Button {
            rect: Rect {
                x: 12,
                y: 10,
                width: 104,
                height: 36,
            },
            label: if recording { "STOP" } else { "REC" },
            action: UiAction::ToggleRecording,
            color: if recording { RECORDING_BG } else { RECORD_IDLE_BG },
        },
        Button {
            rect: Rect {
                x: 128,
                y: 10,
                width: 104,
                height: 36,
            },
            label: "PLAY",
            action: UiAction::PlayLastRecording,
            color: BUTTON_BG,
        },
        Button {
            rect: Rect {
                x: 244,
                y: 10,
                width: 84,
                height: 36,
            },
            label: "EXIT",
            action: UiAction::Exit,
            color: EXIT_BG,
        },
    ]
}

fn mouse_position(window: &Window) -> Option<Point> {
    window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| Point {
        x: x as usize,
        y: y as usize,
    })
}

fn button_at(point: Point, buttons: &[Button]) -> Option<UiAction> { buttons.iter().find(|button| button.rect.contains(point)).map(|button| button.action) }

fn compose_frame(video: &[u32], width: usize, height: usize, recording: bool, hover: Option<UiAction>) -> Vec<u32> {
    let mut output = vec![0; width * (height + TOOLBAR_HEIGHT)];

    draw_toolbar(&mut output, width, recording, hover);
    for row in 0 .. height {
        let source_start = row * width;
        let target_start = (row + TOOLBAR_HEIGHT) * width;
        output[target_start .. target_start + width].copy_from_slice(&video[source_start .. source_start + width]);
    }

    output
}

fn draw_toolbar(buffer: &mut [u32], width: usize, recording: bool, hover: Option<UiAction>) {
    fill_rect(
        buffer,
        width,
        Rect {
            x: 0,
            y: 0,
            width,
            height: TOOLBAR_HEIGHT,
        },
        TOOLBAR_BG,
    );

    for button in toolbar_buttons(recording) {
        let color = if hover == Some(button.action) { BUTTON_HOVER_BG } else { button.color };
        draw_button(buffer, width, button, color);
    }

    if recording {
        draw_text(buffer, width, 348, 22, "RECORDING", 0xfca5a5);
        fill_circle(buffer, width, 336, 28, 5, 0xef4444);
    }
}

fn draw_button(buffer: &mut [u32], width: usize, button: Button, color: u32) {
    fill_rect(buffer, width, button.rect, color);
    draw_rect_border(buffer, width, button.rect, 0x111827);

    let text_width = button.label.len() * 6;
    let text_x = button.rect.x + button.rect.width.saturating_sub(text_width) / 2;
    let text_y = button.rect.y + 14;
    draw_text(buffer, width, text_x, text_y, button.label, BUTTON_TEXT);
}

fn fill_rect(buffer: &mut [u32], width: usize, rect: Rect, color: u32) {
    let height = buffer.len() / width;
    let max_x = (rect.x + rect.width).min(width);
    let max_y = (rect.y + rect.height).min(height);

    for y in rect.y .. max_y {
        let row = y * width;
        for x in rect.x .. max_x {
            buffer[row + x] = color;
        }
    }
}

fn draw_rect_border(buffer: &mut [u32], width: usize, rect: Rect, color: u32) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }

    let right = rect.x + rect.width - 1;
    let bottom = rect.y + rect.height - 1;
    for x in rect.x ..= right {
        set_pixel(buffer, width, x, rect.y, color);
        set_pixel(buffer, width, x, bottom, color);
    }
    for y in rect.y ..= bottom {
        set_pixel(buffer, width, rect.x, y, color);
        set_pixel(buffer, width, right, y, color);
    }
}

fn fill_circle(buffer: &mut [u32], width: usize, cx: usize, cy: usize, radius: usize, color: u32) {
    let radius_squared = (radius * radius) as isize;
    let r = radius as isize;
    for dy in -r ..= r {
        for dx in -r ..= r {
            if dx * dx + dy * dy <= radius_squared {
                let x = cx as isize + dx;
                let y = cy as isize + dy;
                if x >= 0 && y >= 0 {
                    set_pixel(buffer, width, x as usize, y as usize, color);
                }
            }
        }
    }
}

fn draw_text(buffer: &mut [u32], width: usize, x: usize, y: usize, text: &str, color: u32) {
    let mut cursor = x;
    for ch in text.chars() {
        draw_char(buffer, width, cursor, y, ch, color);
        cursor += 6;
    }
}

fn draw_char(buffer: &mut [u32], width: usize, x: usize, y: usize, ch: char, color: u32) {
    let glyph = glyph(ch);
    for (row, bits) in glyph.iter().enumerate() {
        for col in 0 .. 5 {
            if bits & (1 << (4 - col)) != 0 {
                set_pixel(buffer, width, x + col, y + row, color);
            }
        }
    }
}

fn glyph(ch: char) -> [u8; 7] {
    match ch {
        | 'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        | 'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
        | 'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        | 'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        | 'G' => [0b01111, 0b10000, 0b10000, 0b10011, 0b10001, 0b10001, 0b01111],
        | 'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        | 'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        | 'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        | 'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        | 'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        | 'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        | 'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        | 'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        | 'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        | 'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        | ' ' => [0, 0, 0, 0, 0, 0, 0],
        | _ => [0b11111, 0b10001, 0b00010, 0b00100, 0b00100, 0, 0b00100],
    }
}

fn set_pixel(buffer: &mut [u32], width: usize, x: usize, y: usize, color: u32) {
    let height = buffer.len() / width;
    if x < width && y < height {
        buffer[y * width + x] = color;
    }
}

fn play_last_recording(path: Option<&Path>) {
    let Some(path) = path else {
        tracing::warn!("재생할 마지막 녹화 파일이 없습니다.");
        return;
    };

    if let Err(error) = open_with_default_player(path) {
        tracing::warn!("녹화 파일 재생 실패: {error}");
    }
}

fn open_with_default_player(path: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").arg("/C").arg("start").arg("").arg(path).spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn()?;
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg(path).spawn()?;
    }

    Ok(())
}
