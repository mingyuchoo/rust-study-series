#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use anyhow::{Context,
             Result};
use minifb::{Key,
             KeyRepeat,
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
use webcam_core::{FaceDetection,
                  FaceDetector,
                  FaceRecognizer,
                  FaceRect,
                  FaceRegistry,
                  FaceTag,
                  HeuristicFaceDetector,
                  HeuristicFaceRecognizer,
                  rgb_to_minifb_buffer};

const TOOLBAR_HEIGHT: usize = 56;
const TOOLBAR_BG: u32 = 0x1f2937;
const BUTTON_BG: u32 = 0x374151;
const BUTTON_HOVER_BG: u32 = 0x4b5563;
const BUTTON_TEXT: u32 = 0xf9fafb;
const RECORDING_BG: u32 = 0xb91c1c;
const RECORD_IDLE_BG: u32 = 0x047857;
const EXIT_BG: u32 = 0x7f1d1d;
const FACE_BOX: u32 = 0x22c55e;
const FACE_LABEL_BG: u32 = 0x064e3b;
const FACE_LABEL_TEXT: u32 = 0xecfdf5;
const SELECTED_FACE_BOX: u32 = 0xfacc15;
const SELECTED_FACE_LABEL_BG: u32 = 0x713f12;
const FACE_SCAN_INTERVAL: u64 = 5;
const FACE_MATCH_THRESHOLD: f32 = 0.96;
const FORM_BG: u32 = 0x111827;
const FORM_FIELD_BG: u32 = 0x273548;
const FORM_ACTIVE_FIELD_BG: u32 = 0x1d4ed8;
const FORM_TEXT: u32 = 0xf9fafb;
const FORM_MUTED_TEXT: u32 = 0x9ca3af;
const RECORDINGS_DIR: &str = "recordings";
const RECORDING_FILE_PREFIX: &str = "webcam-detector-";
const MIN_WINDOW_WIDTH: usize = 760;
const DISPLAY_SCALE_STEPS: [usize; 7] = [50, 75, 100, 125, 150, 175, 200];

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
    let mut display_scale = DisplayScale::default();

    let frame_rate = camera.frame_rate().max(1);
    let mut window = create_window(width, height, display_scale)?;
    let mut recorder: Option<Recorder> = None;
    let mut last_recording = latest_recording_path();
    let mut was_left_down = false;
    let mut should_exit = false;
    let registry_path = PathBuf::from("face-registry/people.json");
    let mut face_registry = FaceRegistry::load(&registry_path).context("얼굴 등록 정보 로드 실패")?;
    face_registry.save(&registry_path).context("얼굴 등록 정보 초기화 실패")?;
    let mut face_detector = HeuristicFaceDetector::default();
    let face_recognizer = HeuristicFaceRecognizer::new(FACE_MATCH_THRESHOLD);
    let mut face_detections: Vec<FaceDetection> = Vec::new();
    let mut face_tags: Vec<FaceTag> = Vec::new();
    let mut selected_face_rect: Option<FaceRect> = None;
    let mut selected_face_manual = false;
    let mut drag_start: Option<Point> = None;
    let mut preview_origin = PreviewOrigin::default();
    let mut preview_drag: Option<PreviewDrag> = None;
    let mut frame_index = 0_u64;
    let mut registration_form: Option<RegistrationForm> = None;

    tracing::info!("웹캠 스트리밍 시작: {width}x{height}. ESC 키로 종료.");
    tracing::info!("얼굴 등록 정보: {}명", face_registry.people.len());

    let mut pending_frame = Some(first_frame);
    while window.is_open() && !should_exit {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No);
        if registration_form.is_none() && keys_pressed.contains(&Key::Escape) {
            should_exit = true;
        }

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
            window = create_window(width, height, display_scale)?;
            selected_face_rect = None;
            selected_face_manual = false;
            drag_start = None;
            preview_origin = PreviewOrigin::default();
            preview_drag = None;
            tracing::info!("웹캠 해상도 변경: {width}x{height}");
        }

        let decoded_frame = decode_frame(frame, width, height)?;
        if frame_index.is_multiple_of(FACE_SCAN_INTERVAL) {
            face_detections = face_detector.detect(&decoded_frame.rgb, width, height);
            face_tags = face_recognizer.recognize(&face_registry, &decoded_frame.rgb, width, height, &face_detections);
            if let Some(tracked_rect) = track_selected_face(selected_face_rect, &face_tags) {
                selected_face_rect = Some(tracked_rect);
                selected_face_manual = false;
            } else if !selected_face_manual {
                selected_face_rect = None;
            }
        }
        frame_index = frame_index.wrapping_add(1);

        if let Some(form) = registration_form.as_mut() {
            match form.handle_keys(&keys_pressed) {
                | RegistrationFormEvent::None => {},
                | RegistrationFormEvent::Cancel => {
                    tracing::info!("얼굴 등록 취소");
                    registration_form = None;
                },
                | RegistrationFormEvent::Submit => {
                    match complete_registration(&mut face_registry, &registry_path, form) {
                        | Ok(person_name) => {
                            tracing::info!("얼굴 등록 완료: {person_name}");
                            face_tags = face_recognizer.recognize(&face_registry, &decoded_frame.rgb, width, height, &face_detections);
                        },
                        | Err(error) => tracing::warn!("얼굴 등록 실패: {error}"),
                    }
                    registration_form = None;
                },
            }
        }

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
        let mouse = mouse_position(&window);
        let hover = mouse.and_then(|point| button_at(point, &buttons));
        let display_width = display_scale.scaled_dimension(width);
        let display_height = display_scale.scaled_dimension(height);
        let (window_width, window_height) = window.get_size();
        let window_width = window_width.max(1);
        let window_height = window_height.max(TOOLBAR_HEIGHT + 1);
        preview_origin.clamp(window_width, window_height, display_width, display_height);
        let left_down = window.get_mouse_down(MouseButton::Left);
        let right_down = window.get_mouse_down(MouseButton::Right);
        if registration_form.is_none() && right_down {
            if preview_drag.is_none()
                && let Some(point) = mouse
                && mouse_video_point(point, preview_origin, display_scale, width, height).is_some()
            {
                preview_drag = Some(PreviewDrag {
                    mouse_start: point,
                    origin_start: preview_origin,
                });
            }
            if let (Some(drag), Some(point)) = (preview_drag, mouse) {
                preview_origin = drag.preview_origin_at(point);
                preview_origin.clamp(window_width, window_height, display_width, display_height);
            }
        } else {
            preview_drag = None;
        }
        if registration_form.is_none() && left_down && !was_left_down {
            drag_start = None;
            if let Some(action) = hover {
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
                        if last_recording.as_ref().is_none_or(|path| !path.exists()) {
                            last_recording = latest_recording_path();
                        }
                        play_last_recording(last_recording.as_deref());
                    },
                    | UiAction::RegisterCurrentFace =>
                        if let Some(rect) = selected_face_rect {
                            registration_form = Some(start_registration_form(
                                &face_registry,
                                &face_recognizer,
                                &decoded_frame.rgb,
                                width,
                                height,
                                rect,
                            ));
                        } else {
                            tracing::warn!("등록할 얼굴 박스를 먼저 클릭해서 선택하세요.");
                        },
                    | UiAction::DeleteCurrentFace => match delete_current_face(&mut face_registry, &registry_path, &face_tags, selected_face_rect) {
                        | Ok(Some(person_name)) => {
                            tracing::info!("얼굴 등록 삭제 완료: {person_name}");
                            face_tags = face_recognizer.recognize(&face_registry, &decoded_frame.rgb, width, height, &face_detections);
                            selected_face_rect = track_selected_face(selected_face_rect, &face_tags);
                            selected_face_manual = selected_face_rect.is_some_and(|rect| face_tag_at_rect(rect, &face_tags).is_none());
                        },
                        | Ok(None) => tracing::warn!("삭제할 등록 얼굴이 현재 화면에 인식되지 않았습니다."),
                        | Err(error) => tracing::warn!("얼굴 등록 삭제 실패: {error}"),
                    },
                    | UiAction::DecreasePreviewSize => {
                        display_scale.decrease();
                        window = create_window(width, height, display_scale)?;
                        preview_origin = PreviewOrigin::default();
                        drag_start = None;
                        preview_drag = None;
                    },
                    | UiAction::ResetPreviewSize => {
                        display_scale.reset();
                        window = create_window(width, height, display_scale)?;
                        preview_origin = PreviewOrigin::default();
                        drag_start = None;
                        preview_drag = None;
                    },
                    | UiAction::IncreasePreviewSize => {
                        display_scale.increase();
                        window = create_window(width, height, display_scale)?;
                        preview_origin = PreviewOrigin::default();
                        drag_start = None;
                        preview_drag = None;
                    },
                    | UiAction::Exit => should_exit = true,
                }
            } else if let Some(point) = mouse {
                if let Some(rect) = face_tag_at(point, &face_tags, preview_origin, display_scale, width, height) {
                    selected_face_rect = Some(rect);
                    selected_face_manual = false;
                    tracing::info!("얼굴 박스 선택: x={} y={} w={} h={}", rect.x, rect.y, rect.width, rect.height);
                } else if mouse_video_point(point, preview_origin, display_scale, width, height).is_some() {
                    selected_face_rect = None;
                    selected_face_manual = false;
                    drag_start = Some(point);
                }
            }
        }
        if registration_form.is_none()
            && !left_down
            && was_left_down
            && let (Some(start), Some(end)) = (drag_start.take(), mouse)
            && let Some(rect) = face_rect_from_drag(start, end, preview_origin, display_scale, width, height)
        {
            selected_face_rect = Some(rect);
            selected_face_manual = true;
            tracing::info!("수동 얼굴 박스 선택: x={} y={} w={} h={}", rect.x, rect.y, rect.width, rect.height);
        }
        was_left_down = left_down;

        let recording = recorder.is_some();
        let drag_rect = if registration_form.is_none() && left_down {
            drag_start.and_then(|start| mouse.and_then(|end| face_rect_from_drag(start, end, preview_origin, display_scale, width, height)))
        } else {
            None
        };
        let scaled_display = scale_video_buffer(&decoded_frame.display, width, height, display_width, display_height);
        let buffer = compose_frame(
            &scaled_display,
            window_width,
            window_height,
            display_width,
            display_height,
            ComposeState {
                recording,
                hover,
                preview_origin,
                display_scale,
                face_tags: &face_tags,
                selected_face_rect,
                drag_rect,
                registration_form: registration_form.as_ref(),
            },
        );
        window.update_with_buffer(&buffer, window_width, window_height).context("화면 업데이트 실패")?;
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

fn create_window(width: usize, height: usize, display_scale: DisplayScale) -> Result<Window> {
    let display_width = display_scale.scaled_dimension(width);
    let display_height = display_scale.scaled_dimension(height);
    Window::new(
        &format!("웹캠 감지기 - {width}x{height} @ {}%", display_scale.percent()),
        window_width(display_width),
        display_height + TOOLBAR_HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
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

fn window_width(display_width: usize) -> usize { display_width.max(MIN_WINDOW_WIDTH) }

fn scale_video_buffer(video: &[u32], source_width: usize, source_height: usize, target_width: usize, target_height: usize) -> Vec<u32> {
    if source_width == target_width && source_height == target_height {
        return video.to_vec();
    }

    let mut scaled = vec![0; target_width * target_height];
    for y in 0 .. target_height {
        let source_y = y * source_height / target_height;
        let source_row = source_y * source_width;
        let target_row = y * target_width;
        for x in 0 .. target_width {
            let source_x = x * source_width / target_width;
            scaled[target_row + x] = video[source_row + source_x];
        }
    }

    scaled
}

struct Recorder {
    child: Child,
    stdin: Option<ChildStdin>,
    path: PathBuf,
}

impl Recorder {
    fn start(width: usize, height: usize, frame_rate: u32) -> Result<Self> {
        let output_dir = PathBuf::from(RECORDINGS_DIR);
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

    Ok(format!("{RECORDING_FILE_PREFIX}{timestamp}.mp4"))
}

fn latest_recording_path() -> Option<PathBuf> {
    fs::read_dir(RECORDINGS_DIR)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let file_name = path.file_name()?.to_str()?;
            if !file_name.starts_with(RECORDING_FILE_PREFIX) || path.extension().and_then(|extension| extension.to_str()) != Some("mp4") {
                return None;
            }

            let modified = entry.metadata().ok()?.modified().ok()?;
            Some((modified, path))
        })
        .max_by_key(|(modified, _)| *modified)
        .map(|(_, path)| path)
}

fn start_registration_form(
    registry: &FaceRegistry,
    recognizer: &impl FaceRecognizer,
    rgb: &[u8],
    width: usize,
    height: usize,
    rect: FaceRect,
) -> RegistrationForm {
    let embedding = recognizer.embed(rgb, width, height, rect);
    RegistrationForm {
        embedding,
        name: format!("PERSON {}", registry.people.len() + 1),
        age: String::new(),
        gender: String::new(),
        field: RegistrationField::Name,
    }
}

fn complete_registration(registry: &mut FaceRegistry, registry_path: &Path, form: &RegistrationForm) -> Result<String> {
    let name = form.name.trim();
    if name.is_empty() {
        anyhow::bail!("이름을 입력해야 합니다");
    }

    let age = if form.age.trim().is_empty() {
        None
    } else {
        Some(form.age.trim().parse::<u8>().context("나이는 0-255 사이 숫자여야 합니다")?)
    };
    let gender = if form.gender.trim().is_empty() {
        None
    } else {
        Some(form.gender.trim().to_string())
    };

    let person = registry.register_person(name.to_string(), age, gender, form.embedding.clone());
    registry.save(registry_path).context("얼굴 등록 정보 저장 실패")?;

    Ok(person.name)
}

fn delete_current_face(
    registry: &mut FaceRegistry,
    registry_path: &Path,
    face_tags: &[FaceTag],
    selected_face_rect: Option<FaceRect>,
) -> Result<Option<String>> {
    let selected_person_id = selected_face_rect.and_then(|selected| face_tags.iter().find(|tag| tag.rect == selected).and_then(|tag| tag.person_id.clone()));

    let largest_person_id = face_tags
        .iter()
        .filter_map(|tag| tag.person_id.as_ref().map(|person_id| (person_id, tag.rect.width * tag.rect.height)))
        .max_by_key(|(_, area)| *area)
        .map(|(person_id, _)| person_id.clone());

    let Some(person_id) = selected_person_id.or(largest_person_id) else {
        return Ok(None);
    };

    let person = registry.remove_person(&person_id);
    if person.is_some() {
        registry.save(registry_path).context("얼굴 등록 정보 저장 실패")?;
    }

    Ok(person.map(|person| person.name))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RegistrationField {
    Name,
    Age,
    Gender,
}

struct RegistrationForm {
    embedding: Vec<f32>,
    name: String,
    age: String,
    gender: String,
    field: RegistrationField,
}

enum RegistrationFormEvent {
    None,
    Cancel,
    Submit,
}

impl RegistrationForm {
    fn handle_keys(&mut self, keys: &[Key]) -> RegistrationFormEvent {
        for key in keys {
            match key {
                | Key::Escape => return RegistrationFormEvent::Cancel,
                | Key::Enter | Key::NumPadEnter => return RegistrationFormEvent::Submit,
                | Key::Tab => self.next_field(),
                | Key::Backspace => {
                    self.active_value_mut().pop();
                },
                | _ =>
                    if let Some(ch) = key_to_form_char(*key, self.field) {
                        self.push_char(ch);
                    },
            }
        }

        RegistrationFormEvent::None
    }

    fn next_field(&mut self) {
        self.field = match self.field {
            | RegistrationField::Name => RegistrationField::Age,
            | RegistrationField::Age => RegistrationField::Gender,
            | RegistrationField::Gender => RegistrationField::Name,
        };
    }

    fn active_value_mut(&mut self) -> &mut String {
        match self.field {
            | RegistrationField::Name => &mut self.name,
            | RegistrationField::Age => &mut self.age,
            | RegistrationField::Gender => &mut self.gender,
        }
    }

    fn push_char(&mut self, ch: char) {
        let max_len = match self.field {
            | RegistrationField::Name => 18,
            | RegistrationField::Age => 3,
            | RegistrationField::Gender => 10,
        };
        let value = self.active_value_mut();
        if value.len() < max_len {
            value.push(ch);
        }
    }
}

fn key_to_form_char(key: Key, field: RegistrationField) -> Option<char> {
    match field {
        | RegistrationField::Age => digit_key(key),
        | RegistrationField::Name | RegistrationField::Gender => alpha_numeric_key(key).or_else(|| if key == Key::Space { Some(' ') } else { None }),
    }
}

fn digit_key(key: Key) -> Option<char> {
    match key {
        | Key::Key0 | Key::NumPad0 => Some('0'),
        | Key::Key1 | Key::NumPad1 => Some('1'),
        | Key::Key2 | Key::NumPad2 => Some('2'),
        | Key::Key3 | Key::NumPad3 => Some('3'),
        | Key::Key4 | Key::NumPad4 => Some('4'),
        | Key::Key5 | Key::NumPad5 => Some('5'),
        | Key::Key6 | Key::NumPad6 => Some('6'),
        | Key::Key7 | Key::NumPad7 => Some('7'),
        | Key::Key8 | Key::NumPad8 => Some('8'),
        | Key::Key9 | Key::NumPad9 => Some('9'),
        | _ => None,
    }
}

fn alpha_numeric_key(key: Key) -> Option<char> {
    match key {
        | Key::A => Some('A'),
        | Key::B => Some('B'),
        | Key::C => Some('C'),
        | Key::D => Some('D'),
        | Key::E => Some('E'),
        | Key::F => Some('F'),
        | Key::G => Some('G'),
        | Key::H => Some('H'),
        | Key::I => Some('I'),
        | Key::J => Some('J'),
        | Key::K => Some('K'),
        | Key::L => Some('L'),
        | Key::M => Some('M'),
        | Key::N => Some('N'),
        | Key::O => Some('O'),
        | Key::P => Some('P'),
        | Key::Q => Some('Q'),
        | Key::R => Some('R'),
        | Key::S => Some('S'),
        | Key::T => Some('T'),
        | Key::U => Some('U'),
        | Key::V => Some('V'),
        | Key::W => Some('W'),
        | Key::X => Some('X'),
        | Key::Y => Some('Y'),
        | Key::Z => Some('Z'),
        | _ => digit_key(key),
    }
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

#[derive(Clone, Copy)]
struct PreviewOrigin {
    x: isize,
    y: isize,
}

impl Default for PreviewOrigin {
    fn default() -> Self {
        Self {
            x: 0,
            y: TOOLBAR_HEIGHT as isize,
        }
    }
}

impl PreviewOrigin {
    fn clamp(&mut self, window_width: usize, window_height: usize, display_width: usize, display_height: usize) {
        let max_x = window_width.saturating_sub(display_width) as isize;
        let min_y = TOOLBAR_HEIGHT as isize;
        let max_y = window_height.saturating_sub(display_height) as isize;
        self.x = self.x.clamp(0, max_x);
        self.y = self.y.clamp(min_y, max_y.max(min_y));
    }
}

#[derive(Clone, Copy)]
struct PreviewDrag {
    mouse_start: Point,
    origin_start: PreviewOrigin,
}

impl PreviewDrag {
    fn preview_origin_at(self, point: Point) -> PreviewOrigin {
        PreviewOrigin {
            x: self.origin_start.x + point.x as isize - self.mouse_start.x as isize,
            y: self.origin_start.y + point.y as isize - self.mouse_start.y as isize,
        }
    }
}

#[derive(Clone, Copy)]
struct DisplayScale {
    index: usize,
}

impl Default for DisplayScale {
    fn default() -> Self {
        let index = DISPLAY_SCALE_STEPS.iter().position(|percent| *percent == 100).unwrap_or(0);
        Self {
            index,
        }
    }
}

impl DisplayScale {
    fn percent(self) -> usize { DISPLAY_SCALE_STEPS[self.index] }

    fn scaled_dimension(self, dimension: usize) -> usize { (dimension * self.percent()).div_ceil(100).max(1) }

    fn decrease(&mut self) { self.index = self.index.saturating_sub(1); }

    fn increase(&mut self) {
        if self.index + 1 < DISPLAY_SCALE_STEPS.len() {
            self.index += 1;
        }
    }

    fn reset(&mut self) { *self = Self::default(); }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum UiAction {
    ToggleRecording,
    PlayLastRecording,
    RegisterCurrentFace,
    DeleteCurrentFace,
    DecreasePreviewSize,
    ResetPreviewSize,
    IncreasePreviewSize,
    Exit,
}

#[derive(Clone, Copy)]
struct Button {
    rect: Rect,
    label: &'static str,
    action: UiAction,
    color: u32,
}

fn toolbar_buttons(recording: bool) -> [Button; 8] {
    [
        Button {
            rect: Rect {
                x: 12,
                y: 10,
                width: 74,
                height: 36,
            },
            label: if recording { "STOP" } else { "REC" },
            action: UiAction::ToggleRecording,
            color: if recording { RECORDING_BG } else { RECORD_IDLE_BG },
        },
        Button {
            rect: Rect {
                x: 98,
                y: 10,
                width: 74,
                height: 36,
            },
            label: "PLAY",
            action: UiAction::PlayLastRecording,
            color: BUTTON_BG,
        },
        Button {
            rect: Rect {
                x: 184,
                y: 10,
                width: 64,
                height: 36,
            },
            label: "ADD",
            action: UiAction::RegisterCurrentFace,
            color: BUTTON_BG,
        },
        Button {
            rect: Rect {
                x: 260,
                y: 10,
                width: 64,
                height: 36,
            },
            label: "DEL",
            action: UiAction::DeleteCurrentFace,
            color: BUTTON_BG,
        },
        Button {
            rect: Rect {
                x: 336,
                y: 10,
                width: 64,
                height: 36,
            },
            label: "ZOOM-",
            action: UiAction::DecreasePreviewSize,
            color: BUTTON_BG,
        },
        Button {
            rect: Rect {
                x: 412,
                y: 10,
                width: 64,
                height: 36,
            },
            label: "100%",
            action: UiAction::ResetPreviewSize,
            color: BUTTON_BG,
        },
        Button {
            rect: Rect {
                x: 488,
                y: 10,
                width: 64,
                height: 36,
            },
            label: "ZOOM+",
            action: UiAction::IncreasePreviewSize,
            color: BUTTON_BG,
        },
        Button {
            rect: Rect {
                x: 564,
                y: 10,
                width: 64,
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

fn mouse_video_point(point: Point, preview_origin: PreviewOrigin, display_scale: DisplayScale, video_width: usize, video_height: usize) -> Option<Point> {
    let display_width = display_scale.scaled_dimension(video_width);
    let display_height = display_scale.scaled_dimension(video_height);
    let display_x = point.x as isize - preview_origin.x;
    let display_y = point.y as isize - preview_origin.y;
    if display_x < 0 || display_y < 0 || video_width == 0 || video_height == 0 {
        return None;
    }

    let display_x = display_x as usize;
    let display_y = display_y as usize;
    if display_x >= display_width || display_y >= display_height {
        return None;
    }

    Some(Point {
        x: (display_x * video_width / display_width).min(video_width - 1),
        y: (display_y * video_height / display_height).min(video_height - 1),
    })
}

fn face_tag_at(
    point: Point,
    face_tags: &[FaceTag],
    preview_origin: PreviewOrigin,
    display_scale: DisplayScale,
    video_width: usize,
    video_height: usize,
) -> Option<FaceRect> {
    let video_point = mouse_video_point(point, preview_origin, display_scale, video_width, video_height)?;

    face_tags
        .iter()
        .filter(|tag| is_known_face_tag(tag))
        .filter(|tag| face_rect_contains(tag.rect, video_point))
        .min_by_key(|tag| tag.rect.width * tag.rect.height)
        .map(|tag| tag.rect)
}

fn face_tag_at_rect(rect: FaceRect, face_tags: &[FaceTag]) -> Option<FaceRect> {
    face_tags.iter().find(|tag| is_known_face_tag(tag) && tag.rect == rect).map(|tag| tag.rect)
}

fn is_known_face_tag(tag: &FaceTag) -> bool { tag.person_id.is_some() }

fn face_rect_contains(rect: FaceRect, point: Point) -> bool {
    point.x >= rect.x && point.x < rect.x + rect.width && point.y >= rect.y && point.y < rect.y + rect.height
}

fn face_rect_from_drag(
    start: Point,
    end: Point,
    preview_origin: PreviewOrigin,
    display_scale: DisplayScale,
    video_width: usize,
    video_height: usize,
) -> Option<FaceRect> {
    let start = mouse_video_point(start, preview_origin, display_scale, video_width, video_height)?;
    let end = mouse_video_point(end, preview_origin, display_scale, video_width, video_height)?;
    let min_x = start.x.min(end.x);
    let min_y = start.y.min(end.y);
    let max_x = start.x.max(end.x);
    let max_y = start.y.max(end.y);
    let width = max_x.saturating_sub(min_x) + 1;
    let height = max_y.saturating_sub(min_y) + 1;
    if width < 16 || height < 16 {
        return None;
    }

    Some(FaceRect {
        x: min_x,
        y: min_y,
        width,
        height,
    })
}

fn track_selected_face(selected: Option<FaceRect>, face_tags: &[FaceTag]) -> Option<FaceRect> {
    let selected = selected?;

    face_tags
        .iter()
        .filter(|tag| is_known_face_tag(tag))
        .filter_map(|tag| {
            let score = face_rect_overlap_score(selected, tag.rect);
            (score > 0.0).then_some((score, tag.rect))
        })
        .max_by(|(left_score, _), (right_score, _)| left_score.total_cmp(right_score))
        .map(|(_, rect)| rect)
}

fn face_rect_overlap_score(left: FaceRect, right: FaceRect) -> f32 {
    let x1 = left.x.max(right.x);
    let y1 = left.y.max(right.y);
    let x2 = (left.x + left.width).min(right.x + right.width);
    let y2 = (left.y + left.height).min(right.y + right.height);
    if x2 <= x1 || y2 <= y1 {
        return 0.0;
    }

    let intersection = ((x2 - x1) * (y2 - y1)) as f32;
    let left_area = (left.width * left.height) as f32;
    let right_area = (right.width * right.height) as f32;
    intersection / (left_area + right_area - intersection)
}

struct ComposeState<'a> {
    recording: bool,
    hover: Option<UiAction>,
    preview_origin: PreviewOrigin,
    display_scale: DisplayScale,
    face_tags: &'a [FaceTag],
    selected_face_rect: Option<FaceRect>,
    drag_rect: Option<FaceRect>,
    registration_form: Option<&'a RegistrationForm>,
}

fn compose_frame(video: &[u32], window_width: usize, window_height: usize, display_width: usize, display_height: usize, state: ComposeState<'_>) -> Vec<u32> {
    let mut output = vec![0; window_width * window_height];

    draw_toolbar(&mut output, window_width, state.recording, state.hover, state.display_scale);
    draw_video(&mut output, window_width, video, display_width, display_height, state.preview_origin);
    draw_face_tags(
        &mut output,
        window_width,
        state.preview_origin,
        state.display_scale,
        state.face_tags,
        state.selected_face_rect,
    );
    draw_manual_face_rect(
        &mut output,
        window_width,
        state.preview_origin,
        state.display_scale,
        state.face_tags,
        state.selected_face_rect,
    );
    if let Some(drag_rect) = state.drag_rect {
        draw_face_rect(
            buffer_rect(drag_rect, state.preview_origin, state.display_scale),
            &mut output,
            window_width,
            SELECTED_FACE_BOX,
            2,
        );
    }
    if let Some(form) = state.registration_form {
        draw_registration_form(&mut output, window_width, window_height, form);
    }

    output
}

fn draw_video(buffer: &mut [u32], width: usize, video: &[u32], display_width: usize, display_height: usize, preview_origin: PreviewOrigin) {
    let height = buffer.len() / width;
    for row in 0 .. display_height {
        let target_y = preview_origin.y + row as isize;
        if target_y < TOOLBAR_HEIGHT as isize || target_y >= height as isize {
            continue;
        }

        let source_start = row * display_width;
        let mut source_x = 0;
        let mut target_x = preview_origin.x;
        let mut copy_width = display_width;
        if target_x < 0 {
            source_x = (-target_x) as usize;
            copy_width = copy_width.saturating_sub(source_x);
            target_x = 0;
        }
        if target_x as usize + copy_width > width {
            copy_width = width.saturating_sub(target_x as usize);
        }
        if copy_width == 0 {
            continue;
        }

        let target_start = target_y as usize * width + target_x as usize;
        let source_start = source_start + source_x;
        buffer[target_start .. target_start + copy_width].copy_from_slice(&video[source_start .. source_start + copy_width]);
    }
}

fn draw_registration_form(buffer: &mut [u32], width: usize, total_height: usize, form: &RegistrationForm) {
    let form_width = width.saturating_sub(40).clamp(260, 460);
    let form_height = 150;
    let x = width.saturating_sub(form_width) / 2;
    let y = total_height.saturating_sub(form_height) / 2;
    let rect = Rect {
        x,
        y,
        width: form_width,
        height: form_height,
    };

    fill_rect(buffer, width, rect, FORM_BG);
    draw_rect_border_thick(buffer, width, rect, 0x60a5fa, 2);
    draw_text(buffer, width, x + 14, y + 16, "REGISTER FACE", FORM_TEXT);

    draw_form_field(buffer, width, x + 14, y + 42, "NAME", &form.name, form.field == RegistrationField::Name);
    draw_form_field(buffer, width, x + 14, y + 70, "AGE", &form.age, form.field == RegistrationField::Age);
    draw_form_field(buffer, width, x + 14, y + 98, "GENDER", &form.gender, form.field == RegistrationField::Gender);
    draw_text(buffer, width, x + 14, y + 128, "TAB NEXT ENTER SAVE ESC CANCEL", FORM_MUTED_TEXT);
}

fn draw_form_field(buffer: &mut [u32], width: usize, x: usize, y: usize, label: &str, value: &str, active: bool) {
    draw_text(buffer, width, x, y + 7, label, FORM_TEXT);
    let field_rect = Rect {
        x: x + 72,
        y,
        width: 220,
        height: 22,
    };
    fill_rect(buffer, width, field_rect, if active { FORM_ACTIVE_FIELD_BG } else { FORM_FIELD_BG });
    draw_rect_border(buffer, width, field_rect, 0x030712);
    draw_text(buffer, width, field_rect.x + 6, field_rect.y + 8, value, FORM_TEXT);
}

fn draw_face_tags(
    buffer: &mut [u32],
    width: usize,
    preview_origin: PreviewOrigin,
    display_scale: DisplayScale,
    face_tags: &[FaceTag],
    selected_face_rect: Option<FaceRect>,
) {
    for tag in face_tags {
        if !is_known_face_tag(tag) {
            continue;
        }

        let selected = selected_face_rect == Some(tag.rect);
        let rect = buffer_rect(tag.rect, preview_origin, display_scale);
        draw_face_rect(
            rect,
            buffer,
            width,
            if selected { SELECTED_FACE_BOX } else { FACE_BOX },
            if selected { 3 } else { 2 },
        );

        let label_width = tag.label.len() * 6 + 8;
        let label_rect = Rect {
            x: rect.x,
            y: rect.y.saturating_sub(18),
            width: label_width,
            height: 16,
        };
        fill_rect(buffer, width, label_rect, if selected { SELECTED_FACE_LABEL_BG } else { FACE_LABEL_BG });
        draw_text(buffer, width, label_rect.x + 4, label_rect.y + 5, &tag.label, FACE_LABEL_TEXT);
    }
}

fn draw_manual_face_rect(
    buffer: &mut [u32],
    width: usize,
    preview_origin: PreviewOrigin,
    display_scale: DisplayScale,
    face_tags: &[FaceTag],
    selected_face_rect: Option<FaceRect>,
) {
    let Some(rect) = selected_face_rect else {
        return;
    };
    if face_tag_at_rect(rect, face_tags).is_some() {
        return;
    }

    let rect = buffer_rect(rect, preview_origin, display_scale);
    draw_face_rect(rect, buffer, width, SELECTED_FACE_BOX, 3);
    let label_rect = Rect {
        x: rect.x,
        y: rect.y.saturating_sub(18),
        width: 80,
        height: 16,
    };
    fill_rect(buffer, width, label_rect, SELECTED_FACE_LABEL_BG);
    draw_text(buffer, width, label_rect.x + 4, label_rect.y + 5, "MANUAL", FACE_LABEL_TEXT);
}

fn buffer_rect(rect: FaceRect, preview_origin: PreviewOrigin, display_scale: DisplayScale) -> Rect {
    let percent = display_scale.percent();
    Rect {
        x: (preview_origin.x + (rect.x * percent / 100) as isize).max(0) as usize,
        y: (preview_origin.y + (rect.y * percent / 100) as isize).max(TOOLBAR_HEIGHT as isize) as usize,
        width: (rect.width * percent).div_ceil(100).max(1),
        height: (rect.height * percent).div_ceil(100).max(1),
    }
}

fn draw_face_rect(rect: Rect, buffer: &mut [u32], width: usize, color: u32, thickness: usize) { draw_rect_border_thick(buffer, width, rect, color, thickness); }

fn draw_toolbar(buffer: &mut [u32], width: usize, recording: bool, hover: Option<UiAction>, display_scale: DisplayScale) {
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
        draw_text(buffer, width, 642, 22, "RECORDING", 0xfca5a5);
        fill_circle(buffer, width, 630, 28, 5, 0xef4444);
    }
    draw_text(buffer, width, 706, 22, &format!("{}%", display_scale.percent()), 0xd1d5db);
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

fn draw_rect_border_thick(buffer: &mut [u32], width: usize, rect: Rect, color: u32, thickness: usize) {
    for offset in 0 .. thickness {
        if rect.width <= offset * 2 || rect.height <= offset * 2 {
            return;
        }

        draw_rect_border(
            buffer,
            width,
            Rect {
                x: rect.x + offset,
                y: rect.y + offset,
                width: rect.width - offset * 2,
                height: rect.height - offset * 2,
            },
            color,
        );
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
        | 'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        | 'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
        | 'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        | 'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        | 'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        | 'G' => [0b01111, 0b10000, 0b10000, 0b10011, 0b10001, 0b10001, 0b01111],
        | 'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        | 'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        | 'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100],
        | 'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        | 'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        | 'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        | 'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        | 'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        | 'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        | 'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        | 'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        | 'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        | 'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        | 'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        | 'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        | 'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
        | 'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        | 'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        | 'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        | '0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        | '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        | '2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        | '3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
        | '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        | '5' => [0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110],
        | '6' => [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        | '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        | '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        | '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
        | '%' => [0b11001, 0b11010, 0b00010, 0b00100, 0b01000, 0b01011, 0b10011],
        | '+' => [0, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0],
        | '-' => [0, 0, 0, 0b11111, 0, 0, 0],
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
