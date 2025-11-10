use gpui::*;
use gpui_component::{Root, TitleBar};

struct WindowApp {
    text: SharedString,
}

impl Render for WindowApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x282828)) // Gruvbox bg0
            .child(
                // GPUI 커스텀 타이틀바 추가
                TitleBar::new()
                    .on_close_window(move |_, window: &mut Window, _cx| {
                        window.remove_window(); // 닫기 버튼 이벤트
                    })
            )
            .child(
                // 콘텐츠 영역
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap_4()
                    .bg(rgb(0x3c3836)) // Gruvbox bg1
                    .p_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xebdbb2)) // Gruvbox fg1
                            .font_family("NanumGothic")
                            .child("GPUI 데스크탑 앱")
                    )
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xfbf1c7)) // Gruvbox fg0 (brighter)
                            .font_family("NanumGothic")
                            .child(self.text.clone())
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xd5c4a1)) // Gruvbox fg2 (dimmer)
                            .font_family("NanumGothic")
                            .child("윈도우 매니저 테두리가 표시됩니다.")
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Load Korean fonts for proper rendering on Linux
        // Try common Korean fonts available on Fedora
        let font_paths = vec![
            "/usr/share/fonts/google-noto-sans-mono-cjk-vf-fonts/NotoSansMonoCJK-VF.ttc",
            "/usr/share/fonts/google-noto-serif-cjk-vf-fonts/NotoSerifCJK-VF.ttc",
            "/usr/share/fonts/naver-nanum-gothic-fonts/NanumGothic.ttf",
            "/usr/share/fonts/naver-nanum-barun-gothic-fonts/NanumBarunGothic.ttf",
        ];
        
        for font_path in font_paths {
            if std::path::Path::new(font_path).exists() {
                if let Ok(font_data) = std::fs::read(font_path) {
                    cx.text_system()
                        .add_fonts(vec![font_data.into()])
                        .ok();
                    break;
                }
            }
        }
        
        // Initialize gpui-component before using any components
        gpui_component::init(cx);
        
        let window_options = WindowOptions {
            titlebar: None, // 네이티브 타이틀바는 사용하지 않음
            window_decorations: Some(WindowDecorations::Client), // 커스텀 데코레이션 활성화
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point::new(px(100.0), px(100.0)),
                size: Size {
                    width: px(800.0),
                    height: px(600.0),
                },
            })),
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
            is_minimizable: true,
            is_resizable: true,
            window_background: WindowBackgroundAppearance::Opaque,
            window_min_size: Some(Size {
                width: Pixels::from(400.0),
                height: Pixels::from(300.0),
            }),
            display_id: None,
            app_id: Some("com.example.gpui-app".into()),
            tabbing_identifier: Some("com.example.gpui-app".into()),
            ..Default::default()
        };
        
        cx.open_window(window_options, |window, cx| {
            let view = cx.new(|_cx| WindowApp {
                text: "안녕하세요, GPUI!".into(),
            });
            // Wrap the view in Root component (required for gpui-component)
            cx.new(|cx| Root::new(view.into(), window, cx))
        })
        .unwrap();
    });
}
