use gpui::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1080.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Counter { count: 0 }),
        )
        .unwrap();
    });
}

struct Counter {
    count: i32,
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(1080.0))
            .bg(rgb(0xf8fafc))
            .child(format!("Count: {}", self.count))
    }
}
