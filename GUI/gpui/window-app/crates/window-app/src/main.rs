use gpui::*;

struct WindowApp {
    text: SharedString,
}

impl Render for WindowApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().bg(rgb(0x2e7d32)).size_full().justify_center().items_center().child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(div().text_xl().text_color(rgb(0xeceff4)).child(self.text.clone()))
                .child(div().text_sm().text_color(rgb(0xd8dee9)).child("윈도우 매니저 테두리가 표시됩니다.")),
        )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // 윈도우 옵션 설정
        let window_options = WindowOptions {
            // 윈도우 타이틀
            titlebar: Some(TitlebarOptions {
                title: Some("GPUI 데스크탑 앱".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            // 윈도우 크기
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point::new(px(100.0), px(100.0)),
                size: Size {
                    width: px(800.0),
                    height: px(600.0),
                },
            })),
            // 윈도우 매니저 데코레이션 활성화
            window_decorations: Some(WindowDecorations::Server),
            // 포커스
            focus: true,
            // 표시 여부
            show: true,
            // 기타 옵션
            kind: WindowKind::Normal,
            is_movable: true,
            is_minimizable: true,
            is_resizable: true,
            display_id: None,
            window_background: WindowBackgroundAppearance::Opaque,
            window_min_size: Some(Size {
                width: Pixels::from(10.0),
                height: Pixels::from(10.0),
            }),
            app_id: Some("com.example.gpui-app".into()),
            tabbing_identifier: Some("com.example.gpui-app".into()),
        };
        // 윈도우 생성
        cx.open_window(window_options, |_, cx| {
            cx.new(|_cx| WindowApp {
                text: "안녕하세요, GPUI!".into(),
            })
        })
        .unwrap();
    });
}
