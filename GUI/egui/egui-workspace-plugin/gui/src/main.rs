use eframe::egui;

mod app;
use app::FileConverterApp;

fn main() -> Result<(), eframe::Error> {
    // 로깅 초기화
    env_logger::init();
    
    // eframe 옵션 설정
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("파일 변환기"),
        ..Default::default()
    };
    
    // 애플리케이션 실행
    eframe::run_native(
        "파일 변환기",
        options,
        Box::new(|cc| {
            // egui 스타일 설정
            cc.egui_ctx.set_visuals(egui::Visuals::default());
            
            let mut app = FileConverterApp::new(cc);
            
            // 텍스트 변환 플러그인 등록
            let text_plugin = text_converter::TextConverterPlugin::new();
            if let Err(e) = app.register_plugin(Box::new(text_plugin)) {
                log::error!("Failed to register text converter plugin: {}", e);
            } else {
                log::info!("Text converter plugin registered successfully");
            }
            
            Ok(Box::new(app))
        }),
    )
}
