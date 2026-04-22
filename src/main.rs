use gpui::{
    App, Bounds, Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, div,
    layer_shell::{Anchor, KeyboardInteractivity, Layer, LayerShellOptions},
    point,
    prelude::*,
    px, rgb, rgba, size,
};
use gpui_platform::application;

use chrono::prelude::*;

struct GpuiClock;

impl GpuiClock {
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

    pub fn to_24(&mut self, number: u32) -> String {
        match number {
            0..9 => format!("0{}", number),
            _ => format!("{}", number),
        }
    }
}

impl Render for GpuiClock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let now = Local::now();

        div()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .size_full()
            .bg(rgba(0xffffff00))
            .text_color(rgb(0xffffff))
            .text_size(px(64.0))
            .child(format!(
                "{}:{}",
                self.to_24(now.hour()),
                self.to_24(now.minute())
            ))
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.0), px(0.0)),
                    size: size(px(1920.0), px(1200.0)),
                })),
                app_id: Some("gpui_clock".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "clock".to_string(),
                    layer: Layer::Background,
                    anchor: Anchor::all(),
                    margin: None,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(GpuiClock::new),
        )
        .unwrap();
    });
}
