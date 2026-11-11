use gpui::*;

const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 720.0;
const INITIAL_COUNT: i32 = 0;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(
            None,
            size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)),
            cx,
        );
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Counter::new()),
        )
        .unwrap();
    });
}

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Self {
            count: INITIAL_COUNT,
        }
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(WINDOW_WIDTH))
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x0f172a))
            .child(div().child("Counter"))
            .child(div().child("Current value"))
            .child(div().child(format!("Count: {}", self.count)))
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
                    .child("Reset")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count = 0;
                        cx.notify();
                    })),
            )
    }
}
