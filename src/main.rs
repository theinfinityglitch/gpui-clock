use chrono::Timelike;
use gpui::prelude::*;
use gpui_platform::application;

struct LayershellTest;

impl LayershellTest {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| {
            loop {
                let _ = this.update(cx, |_, cx| cx.notify());
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(500))
                    .await;
            }
        })
        .detach();

        Self
    }

    pub fn get_minute(&mut self, minute: u32) -> String {
        match minute {
            0..9 => format!("0{}", minute),
            _ => format!("{}", minute),
        }
    }
}

impl gpui::Render for LayershellTest {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let now = chrono::Local::now();

        gpui::div()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .size_full()
            .bg(gpui::rgba(0xffffff00))
            .text_color(gpui::rgb(0xffffff))
            .text_size(gpui::px(64.0))
            .child(format!("{}:{}", now.hour(), self.get_minute(now.minute())))
    }
}

fn main() {
    application().run(|cx: &mut gpui::App| {
        cx.open_window(
            gpui::WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds {
                    origin: gpui::point(gpui::px(0.0), gpui::px(0.0)),
                    size: gpui::size(gpui::px(1920.0), gpui::px(1200.0)),
                })),
                app_id: Some("gpui_layershell_test".to_string()),
                window_background: gpui::WindowBackgroundAppearance::Transparent,
                kind: gpui::WindowKind::LayerShell(gpui::layer_shell::LayerShellOptions {
                    namespace: "clock".to_string(),
                    layer: gpui::layer_shell::Layer::Background,
                    anchor: gpui::layer_shell::Anchor::all(),
                    margin: None,
                    keyboard_interactivity: gpui::layer_shell::KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(LayershellTest::new),
        )
        .unwrap();
    });
}
