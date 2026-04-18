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
            .items_center()
            .size_full()
            .bg(gpui::rgb(0x282828))
            .text_color(gpui::rgb(0xe0e0e0))
            .child(format!("{}:{}:{}", now.hour(), now.minute(), now.second()))
    }
}

fn main() {
    application().run(|cx: &mut gpui::App| {
        cx.open_window(
            gpui::WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds {
                    origin: gpui::point(gpui::px(0.0), gpui::px(0.0)),
                    size: gpui::size(gpui::px(1920.0), gpui::px(30.0)),
                })),
                app_id: Some("gpui_layershell_test".to_string()),
                window_background: gpui::WindowBackgroundAppearance::Transparent,
                kind: gpui::WindowKind::LayerShell(gpui::layer_shell::LayerShellOptions {
                    namespace: "bar".to_string(),
                    layer: gpui::layer_shell::Layer::Top,
                    anchor: gpui::layer_shell::Anchor::RIGHT
                        | gpui::layer_shell::Anchor::LEFT
                        | gpui::layer_shell::Anchor::TOP,
                    exclusive_zone: Some(gpui::px(30.0)),
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
