use image::{GrayImage,
            ImageBuffer,
            Luma};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameBufferError {
    #[error("RGB frame size mismatch: actual {actual} bytes, expected {expected} bytes ({width}x{height} RGB)")]
    SizeMismatch {
        actual: usize,
        expected: usize,
        width: usize,
        height: usize,
    },
}

/// 두 프레임을 비교해 모션 점수(변화 픽셀 비율)를 반환한다.
pub fn detect_motion(prev: &GrayImage, curr: &GrayImage, threshold: u8) -> f32 {
    let total = (prev.width() * prev.height()) as f32;
    let changed = prev
        .pixels()
        .zip(curr.pixels())
        .filter(|(p, c)| {
            let diff = (p[0] as i16 - c[0] as i16).unsigned_abs() as u8;
            diff > threshold
        })
        .count() as f32;

    changed / total
}

/// RGB 이미지를 그레이스케일로 변환한다.
pub fn to_grayscale(rgb: &[u8], width: u32, height: u32) -> GrayImage {
    ImageBuffer::from_fn(width, height, |x, y| {
        let idx = ((y * width + x) * 3) as usize;
        let (r, g, b) = (rgb[idx] as f32, rgb[idx + 1] as f32, rgb[idx + 2] as f32);
        // ITU-R BT.601 가중치
        Luma([(0.299 * r + 0.587 * g + 0.114 * b) as u8])
    })
}

pub fn rgb_to_minifb_buffer(rgb: &[u8], width: usize, height: usize) -> Result<Vec<u32>, FrameBufferError> {
    let expected = width * height * 3;
    if rgb.len() != expected {
        return Err(FrameBufferError::SizeMismatch {
            actual: rgb.len(),
            expected,
            width,
            height,
        });
    }

    Ok(rgb
        .chunks_exact(3)
        .map(|px| {
            let (r, g, b) = (px[0] as u32, px[1] as u32, px[2] as u32);
            (r << 16) | (g << 8) | b
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{detect_motion,
                rgb_to_minifb_buffer,
                to_grayscale};
    use image::Luma;

    #[test]
    fn detects_changed_pixel_ratio() {
        let prev = image::GrayImage::from_pixel(2, 2, Luma([10]));
        let mut curr = image::GrayImage::from_pixel(2, 2, Luma([10]));
        curr.put_pixel(1, 1, Luma([40]));

        assert_eq!(detect_motion(&prev, &curr, 20), 0.25);
    }

    #[test]
    fn converts_rgb_to_grayscale() {
        let rgb = [255, 0, 0, 0, 255, 0, 0, 0, 255];
        let gray = to_grayscale(&rgb, 3, 1);

        assert_eq!(gray.get_pixel(0, 0)[0], 76);
        assert_eq!(gray.get_pixel(1, 0)[0], 149);
        assert_eq!(gray.get_pixel(2, 0)[0], 29);
    }

    #[test]
    fn converts_rgb_to_minifb_buffer() {
        let rgb = [255, 0, 0, 0, 255, 0, 0, 0, 255];
        let buffer = rgb_to_minifb_buffer(&rgb, 3, 1).expect("valid RGB frame");

        assert_eq!(buffer, vec![0xff0000, 0x00ff00, 0x0000ff]);
    }

    #[test]
    fn rejects_mismatched_rgb_buffer_size() {
        let err = rgb_to_minifb_buffer(&[255, 0, 0], 2, 1).expect_err("buffer should be too short");

        assert_eq!(err.to_string(), "RGB frame size mismatch: actual 3 bytes, expected 6 bytes (2x1 RGB)",);
    }
}
