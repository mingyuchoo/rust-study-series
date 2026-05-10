use image::{GrayImage,
            ImageBuffer,
            Luma};
use serde::{Deserialize,
            Serialize};
use std::{cmp::Reverse,
          fs,
          path::Path};
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct FaceRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PersonMetadata {
    pub id: String,
    pub name: String,
    pub age: Option<u8>,
    pub gender: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FaceEmbedding {
    pub person_id: String,
    pub vector: Vec<f32>,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Serialize)]
pub struct FaceRegistry {
    pub people: Vec<PersonMetadata>,
    pub embeddings: Vec<FaceEmbedding>,
}

impl FaceRegistry {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, FaceRegistryError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), FaceRegistryError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn register_person(&mut self, name: String, age: Option<u8>, gender: Option<String>, embedding: Vec<f32>) -> PersonMetadata {
        let id = format!("person-{:04}", self.people.len() + 1);
        let person = PersonMetadata {
            id: id.clone(),
            name,
            age,
            gender,
        };

        self.people.push(person.clone());
        self.embeddings.push(FaceEmbedding {
            person_id: id,
            vector: embedding,
        });

        person
    }

    pub fn remove_person(&mut self, person_id: &str) -> Option<PersonMetadata> {
        let index = self.people.iter().position(|person| person.id == person_id)?;
        let person = self.people.remove(index);
        self.embeddings.retain(|embedding| embedding.person_id != person_id);
        Some(person)
    }

    pub fn match_embedding(&self, embedding: &[f32], threshold: f32) -> Option<FaceMatch> {
        self.embeddings
            .iter()
            .filter_map(|candidate| {
                let score = cosine_similarity(embedding, &candidate.vector);
                let person = self.people.iter().find(|person| person.id == candidate.person_id)?;
                Some(FaceMatch {
                    person: person.clone(),
                    score,
                })
            })
            .filter(|candidate| candidate.score >= threshold)
            .max_by(|a, b| a.score.total_cmp(&b.score))
    }
}

#[derive(Debug, Error)]
pub enum FaceRegistryError {
    #[error("face registry I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("face registry JSON failed: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaceDetection {
    pub rect: FaceRect,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaceTag {
    pub rect: FaceRect,
    pub label: String,
    pub confidence: f32,
    pub person_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaceMatch {
    pub person: PersonMetadata,
    pub score: f32,
}

pub trait FaceDetector {
    fn detect(&mut self, rgb: &[u8], width: usize, height: usize) -> Vec<FaceDetection>;
}

pub trait FaceRecognizer {
    fn embed(&self, rgb: &[u8], width: usize, height: usize, rect: FaceRect) -> Vec<f32>;

    fn recognize(&self, registry: &FaceRegistry, rgb: &[u8], width: usize, height: usize, detections: &[FaceDetection]) -> Vec<FaceTag>;
}

#[derive(Debug)]
pub struct HeuristicFaceDetector {
    min_area_ratio: f32,
    scan_step: usize,
}

impl Default for HeuristicFaceDetector {
    fn default() -> Self {
        Self {
            min_area_ratio: 0.015,
            scan_step: 4,
        }
    }
}

impl FaceDetector for HeuristicFaceDetector {
    fn detect(&mut self, rgb: &[u8], width: usize, height: usize) -> Vec<FaceDetection> {
        detect_skin_region_faces(rgb, width, height, self.scan_step, self.min_area_ratio)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HeuristicFaceRecognizer {
    threshold: f32,
}

impl HeuristicFaceRecognizer {
    #[must_use]
    pub fn new(threshold: f32) -> Self {
        Self {
            threshold,
        }
    }
}

impl FaceRecognizer for HeuristicFaceRecognizer {
    fn embed(&self, rgb: &[u8], width: usize, height: usize, rect: FaceRect) -> Vec<f32> { embed_face(rgb, width, height, rect) }

    fn recognize(&self, registry: &FaceRegistry, rgb: &[u8], width: usize, height: usize, detections: &[FaceDetection]) -> Vec<FaceTag> {
        recognize_face_tags(registry, rgb, width, height, detections, self.threshold)
    }
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

pub fn unknown_face_tags(detections: &[FaceDetection]) -> Vec<FaceTag> {
    detections
        .iter()
        .map(|detection| FaceTag {
            rect: detection.rect,
            label: "UNKNOWN".to_string(),
            confidence: detection.confidence,
            person_id: None,
        })
        .collect()
}

pub fn recognize_face_tags(registry: &FaceRegistry, rgb: &[u8], width: usize, height: usize, detections: &[FaceDetection], threshold: f32) -> Vec<FaceTag> {
    detections
        .iter()
        .map(|detection| {
            let embedding = embed_face(rgb, width, height, detection.rect);
            match registry.match_embedding(&embedding, threshold) {
                | Some(face_match) => FaceTag {
                    rect: detection.rect,
                    label: face_tag_label(&face_match),
                    confidence: face_match.score,
                    person_id: Some(face_match.person.id),
                },
                | None => FaceTag {
                    rect: detection.rect,
                    label: "UNKNOWN".to_string(),
                    confidence: detection.confidence,
                    person_id: None,
                },
            }
        })
        .collect()
}

fn face_tag_label(face_match: &FaceMatch) -> String {
    let mut parts = vec![face_match.person.name.to_uppercase()];
    if let Some(age) = face_match.person.age {
        parts.push(age.to_string());
    }
    if let Some(gender) = face_match.person.gender.as_deref().filter(|gender| !gender.trim().is_empty()) {
        parts.push(gender.trim().to_uppercase());
    }
    parts.push(format!("{}%", (face_match.score * 100.0).round() as u8));
    parts.join(" ")
}

pub fn embed_face(rgb: &[u8], width: usize, height: usize, rect: FaceRect) -> Vec<f32> {
    let mut bins = vec![0.0_f32; 12];
    let x_end = (rect.x + rect.width).min(width);
    let y_end = (rect.y + rect.height).min(height);
    let mut count = 0.0_f32;

    for y in rect.y .. y_end {
        for x in rect.x .. x_end {
            let idx = (y * width + x) * 3;
            if idx + 2 >= rgb.len() {
                continue;
            }

            let r = rgb[idx] as usize;
            let g = rgb[idx + 1] as usize;
            let b = rgb[idx + 2] as usize;
            bins[(r * 4 / 256).min(3)] += 1.0;
            bins[4 + (g * 4 / 256).min(3)] += 1.0;
            bins[8 + (b * 4 / 256).min(3)] += 1.0;
            count += 1.0;
        }
    }

    if count > 0.0 {
        for value in &mut bins {
            *value /= count;
        }
    }
    normalize_vector(&mut bins);
    bins
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot = 0.0_f32;
    let mut a_norm = 0.0_f32;
    let mut b_norm = 0.0_f32;
    for (a_value, b_value) in a.iter().zip(b) {
        dot += a_value * b_value;
        a_norm += a_value * a_value;
        b_norm += b_value * b_value;
    }

    if a_norm == 0.0 || b_norm == 0.0 {
        return 0.0;
    }

    dot / (a_norm.sqrt() * b_norm.sqrt())
}

fn normalize_vector(vector: &mut [f32]) {
    let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in vector {
            *value /= norm;
        }
    }
}

fn detect_skin_region_faces(rgb: &[u8], width: usize, height: usize, scan_step: usize, min_area_ratio: f32) -> Vec<FaceDetection> {
    if width == 0 || height == 0 || scan_step == 0 || rgb.len() < width * height * 3 {
        return Vec::new();
    }

    let sample_width = width.div_ceil(scan_step);
    let sample_height = height.div_ceil(scan_step);
    let mut mask = vec![false; sample_width * sample_height];

    for sy in 0 .. sample_height {
        for sx in 0 .. sample_width {
            let x = (sx * scan_step).min(width - 1);
            let y = (sy * scan_step).min(height - 1);
            let idx = (y * width + x) * 3;
            mask[sy * sample_width + sx] = is_skin_like(rgb[idx], rgb[idx + 1], rgb[idx + 2]);
        }
    }

    let mut visited = vec![false; mask.len()];
    let mut detections = Vec::new();
    let min_area = (width * height) as f32 * min_area_ratio;
    let max_area = (width * height) as f32 * 0.45;
    let min_side = ((width.min(height) as f32) * 0.08).max(16.0);
    let min_density = 0.35;

    for sy in 0 .. sample_height {
        for sx in 0 .. sample_width {
            let start = sy * sample_width + sx;
            if visited[start] || !mask[start] {
                continue;
            }

            let component = flood_fill_component(&mask, &mut visited, sample_width, sample_height, sx, sy);
            let rect = FaceRect {
                x: component.min_x * scan_step,
                y: component.min_y * scan_step,
                width: ((component.max_x - component.min_x + 1) * scan_step).min(width - component.min_x * scan_step),
                height: ((component.max_y - component.min_y + 1) * scan_step).min(height - component.min_y * scan_step),
            };

            let area = (rect.width * rect.height) as f32;
            let aspect = rect.width as f32 / rect.height.max(1) as f32;
            let density = component.count as f32 / ((component.max_x - component.min_x + 1) * (component.max_y - component.min_y + 1)) as f32;
            if area >= min_area
                && area <= max_area
                && rect.width as f32 >= min_side
                && rect.height as f32 >= min_side
                && density >= min_density
                && (0.55 ..= 1.6).contains(&aspect)
            {
                detections.push(FaceDetection {
                    rect,
                    confidence: density.clamp(0.0, 1.0),
                });
            }
        }
    }

    detections.sort_by_key(|detection| Reverse(detection.rect.width * detection.rect.height));
    detections.truncate(3);
    detections
}

fn is_skin_like(r: u8, g: u8, b: u8) -> bool {
    let r_i = r as i16;
    let g_i = g as i16;
    let b_i = b as i16;
    let max = r_i.max(g_i).max(b_i);
    let min = r_i.min(g_i).min(b_i);
    let rgb_skin = r_i > 95 && g_i > 40 && b_i > 20 && max - min > 15 && (r_i - g_i).abs() > 15 && r_i > g_i && r_i > b_i;

    let r = r as f32;
    let g = g as f32;
    let b = b as f32;
    let cb = 128.0 - 0.168_736 * r - 0.331_264 * g + 0.5 * b;
    let cr = 128.0 + 0.5 * r - 0.418_688 * g - 0.081_312 * b;
    let ycbcr_skin = (77.0 ..= 127.0).contains(&cb) && (133.0 ..= 173.0).contains(&cr);

    rgb_skin && ycbcr_skin
}

struct ComponentBounds {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    count: usize,
}

fn flood_fill_component(mask: &[bool], visited: &mut [bool], width: usize, height: usize, start_x: usize, start_y: usize) -> ComponentBounds {
    let mut stack = vec![(start_x, start_y)];
    let mut bounds = ComponentBounds {
        min_x: start_x,
        min_y: start_y,
        max_x: start_x,
        max_y: start_y,
        count: 0,
    };

    while let Some((x, y)) = stack.pop() {
        let idx = y * width + x;
        if visited[idx] || !mask[idx] {
            continue;
        }
        visited[idx] = true;
        bounds.min_x = bounds.min_x.min(x);
        bounds.min_y = bounds.min_y.min(y);
        bounds.max_x = bounds.max_x.max(x);
        bounds.max_y = bounds.max_y.max(y);
        bounds.count += 1;

        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < width {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < height {
            stack.push((x, y + 1));
        }
    }

    bounds
}

#[cfg(test)]
mod tests {
    use super::{FaceDetection,
                FaceDetector,
                FaceRecognizer,
                FaceRect,
                FaceRegistry,
                HeuristicFaceDetector,
                HeuristicFaceRecognizer,
                detect_motion,
                embed_face,
                recognize_face_tags,
                rgb_to_minifb_buffer,
                to_grayscale,
                unknown_face_tags};
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

    #[test]
    fn creates_unknown_tags_from_detections() {
        let detections = [FaceDetection {
            rect: FaceRect {
                x: 10,
                y: 20,
                width: 30,
                height: 40,
            },
            confidence: 0.7,
        }];

        let tags = unknown_face_tags(&detections);

        assert_eq!(tags[0].label, "UNKNOWN");
        assert_eq!(tags[0].rect, detections[0].rect);
        assert_eq!(tags[0].person_id, None);
    }

    #[test]
    fn loads_missing_registry_as_empty() {
        let path = std::env::temp_dir().join("webcam-detector-missing-registry.json");
        let _ = std::fs::remove_file(&path);

        let registry = FaceRegistry::load(path).expect("missing registry should be empty");

        assert!(registry.people.is_empty());
        assert!(registry.embeddings.is_empty());
    }

    #[test]
    fn heuristic_detector_finds_skin_like_region() {
        let width = 80;
        let height = 80;
        let mut rgb = vec![0_u8; width * height * 3];
        for y in 20 .. 56 {
            for x in 24 .. 52 {
                let idx = (y * width + x) * 3;
                rgb[idx] = 210;
                rgb[idx + 1] = 145;
                rgb[idx + 2] = 110;
            }
        }

        let mut detector = HeuristicFaceDetector::default();
        let detections = detector.detect(&rgb, width, height);

        assert!(!detections.is_empty());
    }

    #[test]
    fn heuristic_detector_rejects_tiny_skin_like_region() {
        let width = 80;
        let height = 80;
        let mut rgb = vec![0_u8; width * height * 3];
        for y in 20 .. 28 {
            for x in 24 .. 32 {
                let idx = (y * width + x) * 3;
                rgb[idx] = 210;
                rgb[idx + 1] = 145;
                rgb[idx + 2] = 110;
            }
        }

        let mut detector = HeuristicFaceDetector::default();
        let detections = detector.detect(&rgb, width, height);

        assert!(detections.is_empty());
    }

    #[test]
    fn heuristic_detector_rejects_full_frame_skin_like_region() {
        let width = 80;
        let height = 80;
        let rgb = [210_u8, 145, 110].repeat(width * height);

        let mut detector = HeuristicFaceDetector::default();
        let detections = detector.detect(&rgb, width, height);

        assert!(detections.is_empty());
    }

    #[test]
    fn registry_matches_registered_embedding() {
        let mut registry = FaceRegistry::default();
        let embedding = vec![1.0, 0.0, 0.0];
        registry.register_person("PERSON 1".to_string(), None, None, embedding.clone());

        let face_match = registry.match_embedding(&embedding, 0.9).expect("registered embedding should match");

        assert_eq!(face_match.person.name, "PERSON 1");
        assert!(face_match.score >= 0.99);
    }

    #[test]
    fn registry_removes_person_and_embeddings() {
        let mut registry = FaceRegistry::default();
        let person = registry.register_person("PERSON 1".to_string(), None, None, vec![1.0, 0.0, 0.0]);

        let removed = registry.remove_person(&person.id).expect("registered person should be removed");

        assert_eq!(removed.name, "PERSON 1");
        assert!(registry.people.is_empty());
        assert!(registry.embeddings.is_empty());
    }

    #[test]
    fn recognizes_registered_face_tag() {
        let width = 4;
        let height = 4;
        let rgb = [200_u8, 140, 100].repeat(width * height);
        let rect = FaceRect {
            x: 0,
            y: 0,
            width,
            height,
        };
        let mut registry = FaceRegistry::default();
        registry.register_person(
            "PERSON 1".to_string(),
            Some(31),
            Some("MALE".to_string()),
            embed_face(&rgb, width, height, rect),
        );

        let tags = recognize_face_tags(
            &registry,
            &rgb,
            width,
            height,
            &[FaceDetection {
                rect,
                confidence: 1.0,
            }],
            0.9,
        );

        assert!(tags[0].label.starts_with("PERSON 1 31 MALE"));
        assert_eq!(tags[0].person_id.as_deref(), Some("person-0001"));
    }

    #[test]
    fn heuristic_recognizer_matches_registered_face_tag() {
        let width = 4;
        let height = 4;
        let rgb = [200_u8, 140, 100].repeat(width * height);
        let rect = FaceRect {
            x: 0,
            y: 0,
            width,
            height,
        };
        let recognizer = HeuristicFaceRecognizer::new(0.9);
        let mut registry = FaceRegistry::default();
        registry.register_person("PERSON 1".to_string(), None, None, recognizer.embed(&rgb, width, height, rect));

        let tags = recognizer.recognize(
            &registry,
            &rgb,
            width,
            height,
            &[FaceDetection {
                rect,
                confidence: 1.0,
            }],
        );

        assert!(tags[0].label.starts_with("PERSON 1"));
    }
}
