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
            // 윈도우 타이틀바 설정
            // appears_transparent를 false로 설정하여 네이티브 타이틀바가 표시되도록 함
            titlebar: Some(TitlebarOptions {
                title: Some("GPUI 데스크탑 앱".into()),
                appears_transparent: false, // 투명하지 않은 타이틀바 (네이티브 스타일)
                traffic_light_position: None,
            }),
            // 윈도우 크기 및 위치
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point::new(px(100.0), px(100.0)),
                size: Size {
                    width: px(800.0),
                    height: px(600.0),
                },
            })),
            // 윈도우 매니저 데코레이션 활성화
            // WindowDecorations::Server는 OS의 네이티브 윈도우 데코레이션(테두리, 타이틀바)을 사용
            // WindowDecorations::Client는 커스텀 데코레이션을 사용 (테두리/타이틀바 없음)
            window_decorations: Some(WindowDecorations::Server),
            // 윈도우 동작 설정
            focus: true,              // 윈도우 생성 시 포커스
            show: true,               // 윈도우 즉시 표시
            kind: WindowKind::Normal, // 일반 윈도우 타입
            is_movable: true,         // 윈도우 이동 가능
            is_minimizable: true,     // 최소화 버튼 활성화
            is_resizable: true,       // 크기 조절 가능
            // 윈도우 외관
            window_background: WindowBackgroundAppearance::Opaque, // 불투명 배경
            window_min_size: Some(Size {
                width: Pixels::from(400.0),  // 최소 너비
                height: Pixels::from(300.0), // 최소 높이
            }),
            // 기타 옵션
            display_id: None,
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
