#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use anyhow::{Context,
             Result};
use minifb::{Key,
             Window,
             WindowOptions};
use nokhwa::{Buffer,
             Camera,
             pixel_format::RgbFormat,
             utils::{CameraIndex,
                     RequestedFormat,
                     RequestedFormatType}};
use webcam_core::rgb_to_minifb_buffer;

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

    let mut window = create_window(width, height)?;

    tracing::info!("웹캠 스트리밍 시작: {width}x{height}. ESC 키로 종료.");

    let mut pending_frame = Some(first_frame);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = match pending_frame.take() {
            | Some(frame) => frame,
            | None => camera.frame().context("프레임 캡처 실패")?,
        };

        let resolution = frame.resolution();
        let frame_width = resolution.width() as usize;
        let frame_height = resolution.height() as usize;
        if frame_width != width || frame_height != height {
            width = frame_width;
            height = frame_height;
            window = create_window(width, height)?;
            tracing::info!("웹캠 해상도 변경: {width}x{height}");
        }

        let buffer = decode_frame(frame, width, height)?;
        window.update_with_buffer(&buffer, width, height).context("화면 업데이트 실패")?;
    }

    camera.stop_stream().ok();
    Ok(())
}

fn create_window(width: usize, height: usize) -> Result<Window> {
    Window::new(&format!("웹캠 감지기 - {width}x{height}"), width, height, WindowOptions::default()).context("윈도우 생성 실패")
}

fn decode_frame(frame: Buffer, width: usize, height: usize) -> Result<Vec<u32>> {
    let rgb = frame.decode_image::<RgbFormat>().context("프레임 디코드 실패")?;
    let raw = rgb.as_raw();
    rgb_to_minifb_buffer(raw, width, height).context("프레임 버퍼 변환 실패")
}
