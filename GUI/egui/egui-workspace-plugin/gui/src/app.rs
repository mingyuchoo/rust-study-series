use eframe::egui;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use converter_core::{ConversionEngine, PluginRegistry};
use database::{HistoryManager, ConversionHistoryEntry, SettingsManager};
use plugin_interface::{FileFormat, PluginMetadata, ConversionOptions};

/// 애플리케이션의 메인 탭
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTab {
    Converter,
    History,
    Settings,
}

/// 변환 진행 상태
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionStatus {
    Idle,
    InProgress { current_file: String, progress: f32 },
    Completed { success_count: usize, total_count: usize },
}

/// 워커 스레드로 보내는 메시지
#[derive(Debug)]
pub enum WorkerMessage {
    StartConversion {
        files: Vec<String>,
        output_format: FileFormat,
        plugin_name: String,
        options: ConversionOptions,
    },
}

/// 워커 스레드에서 받는 메시지
#[derive(Debug, Clone)]
pub enum ProgressMessage {
    Started { total_files: usize },
    Progress { current_file: String, file_index: usize, total_files: usize },
    FileCompleted { file_path: String, success: bool, output_path: Option<String>, error: Option<String> },
    Completed { success_count: usize, total_count: usize },
}

/// 에러 다이얼로그 상태
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorDialog {
    pub title: String,
    pub message: String,
    pub details: Option<String>,
}

/// 파일 변환기 애플리케이션
pub struct FileConverterApp {
    // Core 시스템 연결
    plugin_registry: Arc<PluginRegistry>,
    conversion_engine: Arc<ConversionEngine>,
    
    // Database 연결
    history_manager: Option<Arc<HistoryManager>>,
    settings_manager: Option<Arc<SettingsManager>>,
    
    // 비동기 처리를 위한 채널
    worker_tx: Option<Sender<WorkerMessage>>,
    progress_rx: Option<Receiver<ProgressMessage>>,
    
    // UI 상태 - 파일 선택
    selected_files: Vec<String>,
    
    // UI 상태 - 변환 설정
    available_plugins: Vec<PluginMetadata>,
    selected_plugin: Option<String>,
    selected_output_format: Option<FileFormat>,
    available_output_formats: Vec<FileFormat>,
    detected_input_format: Option<FileFormat>,
    
    // UI 상태 - 변환 진행
    conversion_status: ConversionStatus,
    
    // UI 상태 - 탭 관리
    active_tab: AppTab,
    
    // UI 상태 - 이력
    history_entries: Vec<ConversionHistoryEntry>,
    selected_history_entry: Option<i64>, // 선택된 이력 항목 ID
    
    // UI 상태 - 설정
    default_output_dir: String,
    theme: String,
    language: String,
    
    // UI 상태 - 에러 다이얼로그
    error_dialog: Option<ErrorDialog>,
}

impl FileConverterApp {
    /// 새로운 애플리케이션 인스턴스 생성
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Plugin Registry 초기화
        let plugin_registry = Arc::new(PluginRegistry::new());
        
        // Conversion Engine 초기화
        let conversion_engine = Arc::new(ConversionEngine::new(Arc::clone(&plugin_registry)));
        
        // History Manager 초기화 (실패해도 계속 진행)
        let history_manager = HistoryManager::new("file_converter.db")
            .ok()
            .map(Arc::new);
        
        if history_manager.is_none() {
            log::warn!("Failed to initialize history manager, history features will be disabled");
        }
        
        // Settings Manager 초기화 (실패해도 계속 진행)
        let settings_manager = SettingsManager::new("file_converter.db")
            .ok()
            .map(Arc::new);
        
        if settings_manager.is_none() {
            log::warn!("Failed to initialize settings manager, settings features will be disabled");
        }
        
        // 설정 로드
        let mut default_output_dir = String::new();
        let mut theme = "System".to_string();
        let mut language = "ko".to_string();
        
        if let Some(ref settings_mgr) = settings_manager {
            if let Ok(Some(dir)) = settings_mgr.load_setting("default_output_dir") {
                default_output_dir = dir;
            }
            if let Ok(Some(t)) = settings_mgr.load_setting("theme") {
                theme = t;
            }
            if let Ok(Some(lang)) = settings_mgr.load_setting("language") {
                language = lang;
            }
        }
        
        // 워커 스레드 및 채널 설정
        let (worker_tx, worker_rx) = channel::<WorkerMessage>();
        let (progress_tx, progress_rx) = channel::<ProgressMessage>();
        
        // 워커 스레드 시작
        let engine_clone = Arc::clone(&conversion_engine);
        thread::spawn(move || {
            Self::worker_thread(worker_rx, progress_tx, engine_clone);
        });
        
        Self {
            plugin_registry,
            conversion_engine,
            history_manager,
            settings_manager,
            worker_tx: Some(worker_tx),
            progress_rx: Some(progress_rx),
            selected_files: Vec::new(),
            available_plugins: Vec::new(),
            selected_plugin: None,
            selected_output_format: None,
            available_output_formats: Vec::new(),
            detected_input_format: None,
            conversion_status: ConversionStatus::Idle,
            active_tab: AppTab::Converter,
            history_entries: Vec::new(),
            selected_history_entry: None,
            default_output_dir,
            theme,
            language,
            error_dialog: None,
        }
    }
    
    /// 플러그인 등록
    pub fn register_plugin(&mut self, plugin: Box<dyn plugin_interface::Plugin>) -> Result<(), String> {
        let result = self.plugin_registry.register_plugin(plugin);
        if result.is_ok() {
            self.refresh_plugins();
        }
        result
    }
    
    /// 플러그인 목록 새로고침
    pub fn refresh_plugins(&mut self) {
        self.available_plugins = self.plugin_registry.list_plugins();
    }
    
    /// 이력 목록 새로고침
    pub fn refresh_history(&mut self) {
        if let Some(ref history_manager) = self.history_manager {
            if let Ok(entries) = history_manager.get_recent_entries(100) {
                self.history_entries = entries;
            }
        }
    }
    
    /// 워커 스레드 함수 - 별도 스레드에서 변환 작업 수행
    fn worker_thread(
        worker_rx: Receiver<WorkerMessage>,
        progress_tx: Sender<ProgressMessage>,
        engine: Arc<ConversionEngine>,
    ) {
        log::info!("Worker thread started");
        
        // 워커 스레드는 메시지를 받을 때까지 대기
        while let Ok(message) = worker_rx.recv() {
            match message {
                WorkerMessage::StartConversion { files, output_format, plugin_name, options } => {
                    log::info!("Worker thread received conversion request for {} files", files.len());
                    
                    let total_files = files.len();
                    
                    // 변환 시작 알림
                    if progress_tx.send(ProgressMessage::Started { total_files }).is_err() {
                        log::error!("Failed to send Started message");
                        break;
                    }
                    
                    let mut success_count = 0;
                    
                    // 각 파일을 순차적으로 변환
                    for (idx, file_path) in files.iter().enumerate() {
                        let path = std::path::PathBuf::from(file_path);
                        let current_file = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("알 수 없음")
                            .to_string();
                        
                        // 진행 상태 업데이트 (파일 처리 시작)
                        if progress_tx.send(ProgressMessage::Progress {
                            current_file: current_file.clone(),
                            file_index: idx,
                            total_files,
                        }).is_err() {
                            log::error!("Failed to send Progress message");
                            break;
                        }
                        
                        // 변환 실행
                        match engine.convert_file(&path, &output_format, &plugin_name, &options) {
                            Ok(result) => {
                                let success = result.success;
                                if success {
                                    success_count += 1;
                                }
                                
                                // 파일 완료 알림
                                if progress_tx.send(ProgressMessage::FileCompleted {
                                    file_path: file_path.clone(),
                                    success,
                                    output_path: result.output_path,
                                    error: if success { None } else { Some(result.message) },
                                }).is_err() {
                                    log::error!("Failed to send FileCompleted message");
                                    break;
                                }
                                
                                // 파일 완료 후 진행률 업데이트 (다음 파일로 이동)
                                // idx + 1은 완료된 파일 수를 나타냄
                                if progress_tx.send(ProgressMessage::Progress {
                                    current_file: if idx + 1 < total_files {
                                        format!("다음 파일 준비 중...")
                                    } else {
                                        "모든 파일 처리 완료".to_string()
                                    },
                                    file_index: idx + 1,
                                    total_files,
                                }).is_err() {
                                    log::error!("Failed to send Progress update after file completion");
                                    break;
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to convert {:?}: {}", path, e);
                                
                                // 파일 실패 알림
                                if progress_tx.send(ProgressMessage::FileCompleted {
                                    file_path: file_path.clone(),
                                    success: false,
                                    output_path: None,
                                    error: Some(e.to_string()),
                                }).is_err() {
                                    log::error!("Failed to send FileCompleted message");
                                    break;
                                }
                                
                                // 실패한 경우에도 진행률 업데이트
                                if progress_tx.send(ProgressMessage::Progress {
                                    current_file: if idx + 1 < total_files {
                                        format!("다음 파일 준비 중...")
                                    } else {
                                        "처리 완료".to_string()
                                    },
                                    file_index: idx + 1,
                                    total_files,
                                }).is_err() {
                                    log::error!("Failed to send Progress update after file failure");
                                    break;
                                }
                            }
                        }
                    }
                    
                    // 변환 완료 알림
                    if progress_tx.send(ProgressMessage::Completed {
                        success_count,
                        total_count: total_files,
                    }).is_err() {
                        log::error!("Failed to send Completed message");
                        break;
                    }
                    
                    log::info!("Worker thread completed conversion: {}/{} files successful", success_count, total_files);
                }
            }
        }
        
        log::info!("Worker thread terminated");
    }
}

impl eframe::App for FileConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 워커 스레드로부터 진행 상태 메시지 처리
        self.process_progress_messages();
        
        // UI가 계속 업데이트되도록 요청 (변환 중일 때)
        if matches!(self.conversion_status, ConversionStatus::InProgress { .. }) {
            ctx.request_repaint();
        }
        
        // 테마 적용 (매 프레임마다 적용하여 일관성 유지)
        self.apply_theme(ctx);
        
        // 에러 다이얼로그 표시
        let mut close_dialog = false;
        if let Some(error) = &self.error_dialog {
            let error_clone = error.clone();
            egui::Window::new(&error_clone.title)
                .collapsible(false)
                .resizable(true)
                .default_width(400.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(&error_clone.message)
                            .color(egui::Color32::RED));
                        
                        if let Some(ref details) = error_clone.details {
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(5.0);
                            ui.label("상세 정보:");
                            ui.add_space(5.0);
                            
                            egui::ScrollArea::vertical()
                                .max_height(200.0)
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new(details)
                                        .small()
                                        .color(egui::Color32::GRAY));
                                });
                        }
                        
                        ui.add_space(10.0);
                        
                        if ui.button("확인").clicked() {
                            close_dialog = true;
                        }
                    });
                });
        }
        
        if close_dialog {
            self.error_dialog = None;
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // 상단 탭 메뉴
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, AppTab::Converter, "🔄 변환");
                ui.selectable_value(&mut self.active_tab, AppTab::History, "📋 이력");
                ui.selectable_value(&mut self.active_tab, AppTab::Settings, "⚙ 설정");
            });
            
            ui.separator();
            
            // 탭별 컨텐츠 영역
            match self.active_tab {
                AppTab::Converter => self.show_converter_tab(ui),
                AppTab::History => self.show_history_tab(ui),
                AppTab::Settings => self.show_settings_tab(ui),
            }
        });
    }
}

impl FileConverterApp {
    /// 변환 탭 UI 표시
    fn show_converter_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("파일 변환");
        
        ui.add_space(10.0);
        
        // 파일 선택 영역
        ui.group(|ui| {
            ui.heading("📁 파일 선택");
            
            ui.horizontal(|ui| {
                if ui.button("📄 파일 선택...").clicked() {
                    self.open_file_dialog(false);
                }
                
                if ui.button("📂 여러 파일 선택...").clicked() {
                    self.open_file_dialog(true);
                }
                
                if !self.selected_files.is_empty() && ui.button("🗑 모두 지우기").clicked() {
                    self.selected_files.clear();
                    self.selected_output_format = None;
                }
            });
            
            ui.add_space(5.0);
            
            // 선택된 파일 목록 표시
            if self.selected_files.is_empty() {
                ui.label("선택된 파일이 없습니다.");
            } else {
                ui.label(format!("선택된 파일: {}개", self.selected_files.len()));
                
                ui.add_space(5.0);
                
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        let mut to_remove = None;
                        
                        for (idx, file) in self.selected_files.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}.", idx + 1));
                                ui.label(file);
                                
                                if ui.small_button("❌").clicked() {
                                    to_remove = Some(idx);
                                }
                            });
                        }
                        
                        if let Some(idx) = to_remove {
                            self.selected_files.remove(idx);
                            if self.selected_files.is_empty() {
                                self.selected_output_format = None;
                            }
                        }
                    });
            }
        });
        
        ui.add_space(10.0);
        
        // 출력 형식 선택 영역
        if !self.selected_files.is_empty() {
            ui.group(|ui| {
                ui.heading("🎯 출력 형식 선택");
                
                // 현재 파일 형식 표시
                if let Some(ref input_format) = self.detected_input_format {
                    ui.horizontal(|ui| {
                        ui.label("현재 형식:");
                        ui.label(egui::RichText::new(&input_format.extension)
                            .strong()
                            .color(egui::Color32::from_rgb(100, 150, 255)));
                        ui.label(format!("({})", input_format.description));
                    });
                } else {
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "⚠ 파일 형식을 감지할 수 없습니다."
                    );
                }
                
                ui.add_space(5.0);
                
                // 출력 형식 드롭다운
                ui.horizontal(|ui| {
                    ui.label("변환할 형식:");
                    
                    let selected_text = self.selected_output_format
                        .as_ref()
                        .map(|f| format!("{} ({})", f.extension, f.description))
                        .unwrap_or_else(|| "선택하세요".to_string());
                    
                    let formats = self.available_output_formats.clone();
                    let mut format_changed = false;
                    let mut new_format = None;
                    
                    egui::ComboBox::from_id_source("output_format_combo")
                        .selected_text(selected_text)
                        .width(250.0)
                        .show_ui(ui, |ui| {
                            if formats.is_empty() {
                                ui.label("사용 가능한 형식이 없습니다.");
                            } else {
                                for format in &formats {
                                    let label = format!("{} ({})", format.extension, format.description);
                                    let is_selected = self.selected_output_format
                                        .as_ref()
                                        .map(|f| f.extension == format.extension)
                                        .unwrap_or(false);
                                    
                                    if ui.selectable_label(is_selected, label).clicked() {
                                        new_format = Some(format.clone());
                                        format_changed = true;
                                    }
                                }
                            }
                        });
                    
                    if format_changed {
                        self.selected_output_format = new_format;
                        self.update_selected_plugin();
                    }
                });
                
                // 선택된 플러그인 표시
                if let Some(ref plugin_name) = self.selected_plugin {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("사용할 플러그인:");
                        ui.label(egui::RichText::new(plugin_name)
                            .strong()
                            .color(egui::Color32::from_rgb(100, 200, 100)));
                    });
                }
            });
            
            ui.add_space(10.0);
        }
        
        // 변환 실행 버튼
        if !self.selected_files.is_empty() 
            && self.selected_output_format.is_some() 
            && self.selected_plugin.is_some() {
            
            ui.group(|ui| {
                ui.heading("🚀 변환 실행");
                
                let can_convert = matches!(self.conversion_status, ConversionStatus::Idle);
                
                ui.add_enabled_ui(can_convert, |ui| {
                    if ui.button("▶ 변환 시작").clicked() {
                        self.start_conversion();
                    }
                });
                
                if !can_convert {
                    ui.label("변환이 진행 중입니다...");
                }
            });
            
            ui.add_space(10.0);
        }
        
        // 변환 진행 상태 표시
        ui.group(|ui| {
            ui.heading("📊 상태");
            
            ui.label(format!("사용 가능한 플러그인: {}개", self.available_plugins.len()));
            
            ui.add_space(5.0);
            
            match &self.conversion_status {
                ConversionStatus::Idle => {
                    ui.label("⏸ 대기 중");
                }
                ConversionStatus::InProgress { current_file, progress } => {
                    ui.label(egui::RichText::new("⏳ 변환 중...")
                        .strong()
                        .color(egui::Color32::from_rgb(100, 150, 255)));
                    
                    ui.add_space(5.0);
                    
                    ui.label(format!("현재 파일: {}", current_file));
                    
                    ui.add_space(5.0);
                    
                    // 진행률 바 - 백분율과 애니메이션 표시
                    let progress_bar = egui::ProgressBar::new(*progress)
                        .show_percentage()
                        .animate(true);
                    ui.add(progress_bar);
                    
                    ui.add_space(3.0);
                    
                    // 진행률 텍스트 표시
                    ui.label(egui::RichText::new(format!("진행률: {:.1}%", progress * 100.0))
                        .small()
                        .color(egui::Color32::GRAY));
                }
                ConversionStatus::Completed { success_count, total_count } => {
                    let (icon, text, color) = if *success_count == *total_count {
                        ("✅", format!("완료: {}/{} 파일 성공", success_count, total_count), egui::Color32::from_rgb(100, 200, 100))
                    } else if *success_count > 0 {
                        ("⚠", format!("일부 완료: {}/{} 파일 성공", success_count, total_count), egui::Color32::from_rgb(255, 200, 100))
                    } else {
                        ("❌", format!("실패: 0/{} 파일 성공", total_count), egui::Color32::from_rgb(255, 100, 100))
                    };
                    
                    ui.label(egui::RichText::new(format!("{} {}", icon, text))
                        .strong()
                        .color(color));
                    
                    if ui.button("🔄 새로 시작").clicked() {
                        self.conversion_status = ConversionStatus::Idle;
                    }
                }
            }
        });
    }
    
    /// 워커 스레드로부터 진행 상태 메시지 처리
    /// 
    /// 이 함수는 워커 스레드가 보낸 진행 상태 메시지를 수신하고 처리합니다.
    /// 진행률은 완료된 파일 수 / 전체 파일 수로 계산되며,
    /// UI의 진행률 바와 상태 텍스트를 업데이트합니다.
    /// 
    /// # 진행률 계산
    /// - 파일 처리 시작: file_index / total_files (0%, 33%, 66% 등)
    /// - 파일 처리 완료: (file_index + 1) / total_files (33%, 66%, 100% 등)
    /// - 모든 파일 완료: 100%
    fn process_progress_messages(&mut self) {
        // 메시지를 먼저 수집한 후 처리 (borrow checker 문제 해결)
        let mut messages = Vec::new();
        
        if let Some(ref progress_rx) = self.progress_rx {
            // 모든 대기 중인 메시지 수집
            while let Ok(message) = progress_rx.try_recv() {
                messages.push(message);
            }
        }
        
        // 수집된 메시지 처리
        for message in messages {
            match message {
                ProgressMessage::Started { total_files } => {
                    log::info!("Conversion started: {} files", total_files);
                    self.conversion_status = ConversionStatus::InProgress {
                        current_file: "준비 중...".to_string(),
                        progress: 0.0,
                    };
                }
                ProgressMessage::Progress { current_file, file_index, total_files } => {
                    // 진행률 계산: 현재 파일 인덱스 / 전체 파일 수
                    // file_index는 0부터 시작하므로 현재 처리 중인 파일의 진행률을 표시
                    // 완료된 경우 (file_index == total_files) 100% 표시
                    let progress = if file_index >= total_files {
                        1.0
                    } else {
                        (file_index as f32) / (total_files as f32)
                    };
                    
                    log::debug!("Progress: {}/{} ({:.1}%) - {}", 
                        file_index.min(total_files), total_files, progress * 100.0, current_file);
                    
                    let display_index = file_index.min(total_files);
                    self.conversion_status = ConversionStatus::InProgress {
                        current_file: format!("[{}/{}] {}", display_index, total_files, current_file),
                        progress,
                    };
                }
                ProgressMessage::FileCompleted { file_path, success, output_path, error } => {
                    log::info!("File completed: {} - success: {}", file_path, success);
                    
                    // 이력에 저장
                    if let Some(ref output_format) = self.selected_output_format {
                        if let Some(ref plugin_name) = self.selected_plugin {
                            self.save_to_history(
                                file_path,
                                output_path,
                                output_format,
                                plugin_name,
                                success,
                                error,
                            );
                        }
                    }
                }
                ProgressMessage::Completed { success_count, total_count } => {
                    log::info!("Conversion completed: {}/{} files successful", success_count, total_count);
                    self.conversion_status = ConversionStatus::Completed {
                        success_count,
                        total_count,
                    };
                    
                    // 일부 실패한 경우 경고 표시
                    if success_count < total_count && success_count > 0 {
                        self.show_error(
                            "일부 파일 변환 실패",
                            &format!("{}/{}개 파일이 성공적으로 변환되었습니다.", success_count, total_count),
                            Some("일부 파일 변환에 실패했습니다. 이력 탭에서 자세한 내용을 확인하세요.".to_string()),
                        );
                    } else if success_count == 0 {
                        self.show_error(
                            "변환 실패",
                            "모든 파일 변환에 실패했습니다.",
                            Some("이력 탭에서 자세한 오류 내용을 확인하세요.".to_string()),
                        );
                    }
                }
            }
        }
    }
    
    /// 변환 시작 - 워커 스레드로 작업 전달
    fn start_conversion(&mut self) {
        // 필요한 정보가 모두 있는지 확인
        let output_format = match &self.selected_output_format {
            Some(f) => f.clone(),
            None => {
                self.show_error(
                    "변환 실패",
                    "출력 형식이 선택되지 않았습니다.",
                    Some("변환할 형식을 선택한 후 다시 시도해주세요.".to_string()),
                );
                return;
            }
        };
        
        let plugin_name = match &self.selected_plugin {
            Some(p) => p.clone(),
            None => {
                self.show_error(
                    "변환 실패",
                    "플러그인이 선택되지 않았습니다.",
                    Some("변환을 수행할 플러그인을 찾을 수 없습니다.".to_string()),
                );
                return;
            }
        };
        
        if self.selected_files.is_empty() {
            self.show_error(
                "변환 실패",
                "변환할 파일이 선택되지 않았습니다.",
                Some("파일을 선택한 후 다시 시도해주세요.".to_string()),
            );
            return;
        }
        
        // 변환 옵션 설정
        let options = ConversionOptions {
            output_path: if !self.default_output_dir.is_empty() {
                Some(self.default_output_dir.clone())
            } else {
                None
            },
            overwrite: false,
            quality: None,
            custom_params: std::collections::HashMap::new(),
        };
        
        // 워커 스레드로 변환 작업 전달
        if let Some(ref worker_tx) = self.worker_tx {
            let message = WorkerMessage::StartConversion {
                files: self.selected_files.clone(),
                output_format,
                plugin_name,
                options,
            };
            
            if let Err(e) = worker_tx.send(message) {
                log::error!("Failed to send conversion request to worker thread: {}", e);
                self.show_error(
                    "변환 실패",
                    "변환 작업을 시작할 수 없습니다.",
                    Some(format!("워커 스레드 통신 오류: {}", e)),
                );
                return;
            }
            
            log::info!("Conversion request sent to worker thread");
            
            // 초기 상태 설정
            self.conversion_status = ConversionStatus::InProgress {
                current_file: "시작 중...".to_string(),
                progress: 0.0,
            };
        } else {
            self.show_error(
                "변환 실패",
                "워커 스레드가 초기화되지 않았습니다.",
                Some("애플리케이션을 다시 시작해주세요.".to_string()),
            );
        }
    }
    
    /// 변환 이력 저장
    fn save_to_history(
        &self,
        input_file: String,
        output_file: Option<String>,
        output_format: &FileFormat,
        plugin_name: &str,
        success: bool,
        error_message: Option<String>,
    ) {
        if let Some(ref history_manager) = self.history_manager {
            let entry = ConversionHistoryEntry {
                id: 0, // Will be set by database
                timestamp: chrono::Utc::now(),
                input_file,
                output_file,
                input_format: self.detected_input_format
                    .as_ref()
                    .map(|f| f.extension.clone())
                    .unwrap_or_else(|| "unknown".to_string()),
                output_format: output_format.extension.clone(),
                plugin_name: plugin_name.to_string(),
                status: if success { "success".to_string() } else { "failed".to_string() },
                error_message,
                bytes_processed: 0,
                duration_ms: 0,
            };
            
            if let Err(e) = history_manager.add_entry(&entry) {
                log::error!("Failed to save history entry: {}", e);
            }
        }
    }
    
    /// 에러 다이얼로그 표시
    fn show_error(&mut self, title: &str, message: &str, details: Option<String>) {
        self.error_dialog = Some(ErrorDialog {
            title: title.to_string(),
            message: message.to_string(),
            details,
        });
    }
    
    /// 파일 선택 다이얼로그 열기
    fn open_file_dialog(&mut self, multiple: bool) {
        let files = if multiple {
            // 다중 파일 선택
            rfd::FileDialog::new()
                .set_title("변환할 파일 선택")
                .pick_files()
        } else {
            // 단일 파일 선택
            rfd::FileDialog::new()
                .set_title("변환할 파일 선택")
                .pick_file()
                .map(|f| vec![f])
        };
        
        if let Some(paths) = files {
            for path in paths {
                let path_str = path.to_string_lossy().to_string();
                if !self.selected_files.contains(&path_str) {
                    self.selected_files.push(path_str);
                }
            }
            
            // 파일이 선택되면 입력 형식 감지 및 출력 형식 목록 업데이트
            if !self.selected_files.is_empty() {
                self.detect_input_format_and_update_outputs();
            }
        }
    }
    
    /// 입력 파일 형식 감지 및 사용 가능한 출력 형식 업데이트
    fn detect_input_format_and_update_outputs(&mut self) {
        if self.selected_files.is_empty() {
            self.detected_input_format = None;
            self.available_output_formats.clear();
            self.selected_output_format = None;
            self.selected_plugin = None;
            return;
        }
        
        // 첫 번째 파일을 기준으로 형식 감지
        let first_file = &self.selected_files[0];
        let path = std::path::Path::new(first_file);
        
        // 입력 형식 감지
        match self.conversion_engine.get_available_formats(path) {
            Ok(formats) => {
                self.available_output_formats = formats;
                
                // 입력 형식도 감지
                if let Ok(input_format) = self.detect_input_format(path) {
                    self.detected_input_format = Some(input_format);
                } else {
                    self.detected_input_format = None;
                }
                
                // 출력 형식 초기화
                self.selected_output_format = None;
                self.selected_plugin = None;
            }
            Err(e) => {
                log::error!("Failed to get available formats: {}", e);
                self.detected_input_format = None;
                self.available_output_formats.clear();
                self.selected_output_format = None;
                self.selected_plugin = None;
                
                // 사용자에게 에러 표시
                self.show_error(
                    "파일 형식 감지 실패",
                    "선택한 파일의 형식을 감지할 수 없습니다.",
                    Some(e.to_string()),
                );
            }
        }
    }
    
    /// 입력 파일 형식 감지
    fn detect_input_format(&self, path: &std::path::Path) -> Result<FileFormat, String> {
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| "파일 확장자를 감지할 수 없습니다.".to_string())?;
        
        // 모든 플러그인에서 이 확장자를 지원하는지 확인
        for plugin_meta in &self.available_plugins {
            if let Some(plugin) = self.plugin_registry.get_plugin(&plugin_meta.name) {
                for format in plugin.supported_input_formats() {
                    if format.extension == extension {
                        return Ok(format);
                    }
                }
            }
        }
        
        Err(format!("지원하지 않는 파일 형식: {}", extension))
    }
    
    /// 선택된 출력 형식에 맞는 플러그인 선택
    fn update_selected_plugin(&mut self) {
        if let (Some(ref input_format), Some(ref output_format)) = 
            (&self.detected_input_format, &self.selected_output_format) {
            
            // 변환을 지원하는 플러그인 찾기
            let plugins = self.plugin_registry.find_plugins_for_conversion(input_format, output_format);
            
            if !plugins.is_empty() {
                self.selected_plugin = Some(plugins[0].clone());
            } else {
                self.selected_plugin = None;
            }
        }
    }
    
    /// 이력 탭 UI 표시
    fn show_history_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("📋 변환 이력");
        
        if self.history_manager.is_none() {
            ui.colored_label(
                egui::Color32::YELLOW,
                "⚠ 데이터베이스 연결 실패: 이력 기능을 사용할 수 없습니다."
            );
            return;
        }
        
        ui.add_space(10.0);
        
        // 상단 컨트롤 영역
        ui.horizontal(|ui| {
            // 이력 새로고침 버튼
            if ui.button("🔄 새로고침").clicked() {
                self.refresh_history();
            }
            
            ui.separator();
            
            // 이력 개수 표시
            ui.label(format!("총 {}개의 이력 항목", self.history_entries.len()));
        });
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // 이력 목록이 비어있는 경우
        if self.history_entries.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(egui::RichText::new("📭 변환 이력이 없습니다.")
                    .size(16.0)
                    .color(egui::Color32::GRAY));
                ui.add_space(10.0);
                ui.label("파일을 변환하면 이곳에 이력이 표시됩니다.");
            });
            return;
        }
        
        // 두 개의 패널로 분할: 왼쪽은 목록, 오른쪽은 상세 정보
        ui.columns(2, |columns| {
            // 왼쪽 패널: 이력 목록
            columns[0].vertical(|ui| {
                ui.heading("목록");
                ui.add_space(5.0);
                
                // 스크롤 가능한 이력 목록
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for (idx, entry) in self.history_entries.iter().enumerate() {
                            let is_selected = self.selected_history_entry == Some(entry.id);
                            
                            // 각 이력 항목을 선택 가능한 그룹으로 표시
                            let response = ui.group(|ui| {
                                ui.set_min_width(ui.available_width());
                                
                                // 선택된 항목 강조
                                if is_selected {
                                    ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                                }
                                
                                // 상태 아이콘과 타임스탬프
                                ui.horizontal(|ui| {
                                    // 상태 아이콘
                                    let (icon, color) = if entry.status == "success" {
                                        ("✅", egui::Color32::from_rgb(100, 200, 100))
                                    } else {
                                        ("❌", egui::Color32::from_rgb(255, 100, 100))
                                    };
                                    
                                    ui.label(egui::RichText::new(icon).size(14.0).color(color));
                                    
                                    // 타임스탬프
                                    let timestamp_str = entry.timestamp
                                        .format("%m-%d %H:%M")
                                        .to_string();
                                    ui.label(egui::RichText::new(timestamp_str)
                                        .color(egui::Color32::GRAY)
                                        .size(11.0));
                                });
                                
                                ui.add_space(3.0);
                                
                                // 입력 파일명 (짧게)
                                let input_filename = std::path::Path::new(&entry.input_file)
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(&entry.input_file);
                                
                                let short_filename = if input_filename.len() > 25 {
                                    format!("{}...", &input_filename[..22])
                                } else {
                                    input_filename.to_string()
                                };
                                
                                ui.label(egui::RichText::new(short_filename).size(12.0));
                                
                                // 변환 정보 (간단히)
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(&entry.input_format)
                                        .small()
                                        .color(egui::Color32::from_rgb(100, 150, 255)));
                                    ui.label(egui::RichText::new("→").small());
                                    ui.label(egui::RichText::new(&entry.output_format)
                                        .small()
                                        .color(egui::Color32::from_rgb(100, 200, 100)));
                                });
                            });
                            
                            // 클릭 시 선택
                            if response.response.interact(egui::Sense::click()).clicked() {
                                self.selected_history_entry = Some(entry.id);
                            }
                            
                            // 선택된 항목 배경색 변경
                            if is_selected {
                                let rect = response.response.rect;
                                ui.painter().rect_filled(
                                    rect,
                                    3.0,
                                    egui::Color32::from_rgba_premultiplied(100, 150, 255, 30),
                                );
                            }
                            
                            // 항목 사이 간격
                            if idx < self.history_entries.len() - 1 {
                                ui.add_space(5.0);
                            }
                        }
                    });
            });
            
            // 오른쪽 패널: 상세 정보
            columns[1].vertical(|ui| {
                ui.heading("상세 정보");
                ui.add_space(5.0);
                
                // 선택된 항목이 있는 경우 상세 정보 표시
                if let Some(selected_id) = self.selected_history_entry {
                    if let Some(entry) = self.history_entries.iter().find(|e| e.id == selected_id) {
                        self.show_history_detail(ui, entry);
                    } else {
                        ui.vertical_centered(|ui| {
                            ui.add_space(50.0);
                            ui.label(egui::RichText::new("선택한 항목을 찾을 수 없습니다.")
                                .color(egui::Color32::GRAY));
                        });
                    }
                } else {
                    // 선택된 항목이 없는 경우
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(egui::RichText::new("👈 왼쪽에서 항목을 선택하세요")
                            .size(14.0)
                            .color(egui::Color32::GRAY));
                    });
                }
            });
        });
    }
    
    /// 이력 상세 정보 표시
    fn show_history_detail(&self, ui: &mut egui::Ui, entry: &ConversionHistoryEntry) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // 상태 헤더
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());
                    
                    let (icon, status_text, status_color) = if entry.status == "success" {
                        ("✅", "변환 성공", egui::Color32::from_rgb(100, 200, 100))
                    } else {
                        ("❌", "변환 실패", egui::Color32::from_rgb(255, 100, 100))
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(icon).size(20.0).color(status_color));
                        ui.label(egui::RichText::new(status_text)
                            .size(16.0)
                            .strong()
                            .color(status_color));
                    });
                });
                
                ui.add_space(10.0);
                
                // 기본 정보
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(egui::RichText::new("📋 기본 정보").strong());
                    ui.add_space(5.0);
                    
                    // ID
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("ID:").strong());
                        ui.label(format!("#{}", entry.id));
                    });
                    
                    // 타임스탬프
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("시간:").strong());
                        ui.label(entry.timestamp.format("%Y년 %m월 %d일 %H:%M:%S").to_string());
                    });
                    
                    // 플러그인
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("플러그인:").strong());
                        ui.label(&entry.plugin_name);
                    });
                });
                
                ui.add_space(10.0);
                
                // 파일 정보
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(egui::RichText::new("📁 파일 정보").strong());
                    ui.add_space(5.0);
                    
                    // 입력 파일
                    ui.label(egui::RichText::new("입력 파일:").strong());
                    ui.indent("input_file", |ui| {
                        ui.label(&entry.input_file);
                        ui.horizontal(|ui| {
                            ui.label("형식:");
                            ui.label(egui::RichText::new(&entry.input_format)
                                .color(egui::Color32::from_rgb(100, 150, 255)));
                        });
                    });
                    
                    ui.add_space(5.0);
                    
                    // 출력 파일
                    ui.label(egui::RichText::new("출력 파일:").strong());
                    ui.indent("output_file", |ui| {
                        if let Some(ref output_file) = entry.output_file {
                            ui.label(output_file);
                        } else {
                            ui.label(egui::RichText::new("(생성되지 않음)")
                                .color(egui::Color32::GRAY)
                                .italics());
                        }
                        ui.horizontal(|ui| {
                            ui.label("형식:");
                            ui.label(egui::RichText::new(&entry.output_format)
                                .color(egui::Color32::from_rgb(100, 200, 100)));
                        });
                    });
                });
                
                ui.add_space(10.0);
                
                // 처리 정보
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(egui::RichText::new("⚙ 처리 정보").strong());
                    ui.add_space(5.0);
                    
                    // 처리된 바이트
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("처리된 데이터:").strong());
                        let size_text = if entry.bytes_processed > 1024 * 1024 {
                            format!("{:.2} MB", entry.bytes_processed as f64 / (1024.0 * 1024.0))
                        } else if entry.bytes_processed > 1024 {
                            format!("{:.2} KB", entry.bytes_processed as f64 / 1024.0)
                        } else {
                            format!("{} bytes", entry.bytes_processed)
                        };
                        ui.label(size_text);
                    });
                    
                    // 소요 시간
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("소요 시간:").strong());
                        let duration_text = if entry.duration_ms > 1000 {
                            format!("{:.2}초", entry.duration_ms as f64 / 1000.0)
                        } else {
                            format!("{}ms", entry.duration_ms)
                        };
                        ui.label(duration_text);
                    });
                });
                
                // 에러 메시지 (실패한 경우)
                if let Some(ref error_msg) = entry.error_message {
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("⚠").color(egui::Color32::YELLOW));
                            ui.label(egui::RichText::new("에러 메시지").strong().color(egui::Color32::from_rgb(255, 150, 100)));
                        });
                        ui.add_space(5.0);
                        
                        // 에러 메시지를 스크롤 가능한 영역에 표시
                        egui::ScrollArea::vertical()
                            .max_height(150.0)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(error_msg)
                                    .color(egui::Color32::from_rgb(255, 100, 100))
                                    .monospace());
                            });
                    });
                }
            });
    }
    
    /// 설정 탭 UI 표시
    fn show_settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙ 설정");
        
        if self.settings_manager.is_none() {
            ui.colored_label(
                egui::Color32::YELLOW,
                "⚠ 데이터베이스 연결 실패: 설정 기능을 사용할 수 없습니다."
            );
            return;
        }
        
        ui.add_space(10.0);
        
        // 설정 변경 플래그
        let mut settings_changed = false;
        
        // 기본 출력 디렉토리 설정
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label(egui::RichText::new("📁 기본 출력 디렉토리").strong().size(14.0));
            ui.add_space(5.0);
            
            ui.label("변환된 파일이 저장될 기본 디렉토리를 설정합니다.");
            ui.label(egui::RichText::new("(비어있으면 원본 파일과 같은 위치에 저장됩니다)")
                .small()
                .color(egui::Color32::GRAY));
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(&mut self.default_output_dir);
                if response.changed() {
                    settings_changed = true;
                }
                
                if ui.button("📂 찾아보기...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("기본 출력 디렉토리 선택")
                        .pick_folder()
                    {
                        self.default_output_dir = path.to_string_lossy().to_string();
                        settings_changed = true;
                    }
                }
                
                if !self.default_output_dir.is_empty() && ui.button("🗑 지우기").clicked() {
                    self.default_output_dir.clear();
                    settings_changed = true;
                }
            });
            
            // 현재 설정 표시
            if !self.default_output_dir.is_empty() {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("현재 설정:").small());
                    ui.label(egui::RichText::new(&self.default_output_dir)
                        .small()
                        .color(egui::Color32::from_rgb(100, 150, 255)));
                });
            }
        });
        
        ui.add_space(10.0);
        
        // 테마 설정
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label(egui::RichText::new("🎨 테마").strong().size(14.0));
            ui.add_space(5.0);
            
            ui.label("애플리케이션의 색상 테마를 선택합니다.");
            
            ui.add_space(5.0);
            
            let old_theme = self.theme.clone();
            
            ui.horizontal(|ui| {
                if ui.selectable_label(self.theme == "Light", "☀ Light").clicked() {
                    self.theme = "Light".to_string();
                }
                if ui.selectable_label(self.theme == "Dark", "🌙 Dark").clicked() {
                    self.theme = "Dark".to_string();
                }
                if ui.selectable_label(self.theme == "System", "💻 System").clicked() {
                    self.theme = "System".to_string();
                }
            });
            
            if old_theme != self.theme {
                settings_changed = true;
                // 테마 즉시 적용
                self.apply_theme(ui.ctx());
            }
            
            ui.add_space(5.0);
            ui.label(egui::RichText::new(format!("현재 테마: {}", self.theme))
                .small()
                .color(egui::Color32::GRAY));
        });
        
        ui.add_space(10.0);
        
        // 언어 설정
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label(egui::RichText::new("🌐 언어").strong().size(14.0));
            ui.add_space(5.0);
            
            ui.label("애플리케이션의 표시 언어를 선택합니다.");
            ui.label(egui::RichText::new("(현재 버전에서는 한국어만 지원됩니다)")
                .small()
                .color(egui::Color32::GRAY));
            
            ui.add_space(5.0);
            
            let old_language = self.language.clone();
            
            egui::ComboBox::from_id_source("language_combo")
                .selected_text(match self.language.as_str() {
                    "ko" => "🇰🇷 한국어",
                    "en" => "🇺🇸 English",
                    "ja" => "🇯🇵 日本語",
                    _ => "Unknown",
                })
                .width(200.0)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(self.language == "ko", "🇰🇷 한국어").clicked() {
                        self.language = "ko".to_string();
                    }
                    if ui.selectable_label(self.language == "en", "🇺🇸 English").clicked() {
                        self.language = "en".to_string();
                    }
                    if ui.selectable_label(self.language == "ja", "🇯🇵 日本語").clicked() {
                        self.language = "ja".to_string();
                    }
                });
            
            if old_language != self.language {
                settings_changed = true;
            }
        });
        
        ui.add_space(20.0);
        
        // 설정 저장 버튼
        if settings_changed {
            ui.separator();
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚠ 설정이 변경되었습니다.")
                    .color(egui::Color32::from_rgb(255, 200, 100)));
                
                if ui.button("💾 저장").clicked() {
                    self.save_settings();
                }
            });
        }
        
        ui.add_space(20.0);
        
        // 정보 섹션
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label(egui::RichText::new("ℹ 정보").strong().size(14.0));
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("애플리케이션 버전:");
                ui.label(egui::RichText::new("0.1.0").strong());
            });
            
            ui.horizontal(|ui| {
                ui.label("등록된 플러그인:");
                ui.label(egui::RichText::new(format!("{}개", self.available_plugins.len())).strong());
            });
        });
    }
    
    /// 설정 저장
    fn save_settings(&self) {
        if let Some(ref settings_manager) = self.settings_manager {
            // 기본 출력 디렉토리 저장
            if let Err(e) = settings_manager.save_setting("default_output_dir", &self.default_output_dir) {
                log::error!("Failed to save default_output_dir: {}", e);
            }
            
            // 테마 저장
            if let Err(e) = settings_manager.save_setting("theme", &self.theme) {
                log::error!("Failed to save theme: {}", e);
            }
            
            // 언어 저장
            if let Err(e) = settings_manager.save_setting("language", &self.language) {
                log::error!("Failed to save language: {}", e);
            }
            
            log::info!("Settings saved successfully");
        }
    }
    
    /// 테마 적용
    fn apply_theme(&self, ctx: &egui::Context) {
        match self.theme.as_str() {
            "Light" => {
                ctx.set_visuals(egui::Visuals::light());
            }
            "Dark" => {
                ctx.set_visuals(egui::Visuals::dark());
            }
            "System" => {
                // System 테마는 기본값 사용
                // 실제로는 OS 설정을 감지해야 하지만, 여기서는 Dark를 기본으로 사용
                ctx.set_visuals(egui::Visuals::dark());
            }
            _ => {
                ctx.set_visuals(egui::Visuals::dark());
            }
        }
    }
}
