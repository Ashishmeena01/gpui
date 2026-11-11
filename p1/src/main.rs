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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(1080.0))
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x0f172a))
            .child(format!("Count: {}", self.count))
            .child(
                div()
                    .id("increment")
                    .px_4()
                    .py_2()
                    .bg(rgb(0x2563eb))
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .child("Increment")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count += 1;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .id("decrement")
                    .px_4()
                    .py_2()
                    .bg(rgb(0x475569))
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .child("Decrement")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count -= 1;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .id("reset")
                    .px_4()
                    .py_2()
                    .bg(rgb(0xe2e8f0))
                    .text_color(rgb(0x0f172a))
                    .cursor_pointer()
                    .child("Reset"),
            )
    }
}
