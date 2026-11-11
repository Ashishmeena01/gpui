
use gpui::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds =
            Bounds::centered(None, size(px(1080.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("test-screen".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(Point {
                        x: px(1000.0),
                        y: px(100.0),
                    }),
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Counter {
                    count: 0,
                    renders: 0,
                })
            },
        )
        .unwrap();
    });
}

// Stateful component
struct Counter {
    count: i32,
    renders: i32,
}

impl Render for Counter {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {

        self.renders += 1;

        println!("Counter rendered {} times", self.renders);

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x0f172a))

            // Header
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child("My Counter")
                    .child(
                        div()
                            .id("hide")
                            .cursor_pointer()
                            .child("Open website")
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.open_url(
                                    "https://ashishmeena.vercel.app"
                                );
                            }))
                    )
            )

            // Counter value
            .child(
                div()
                    .text_3xl()
                    .child(format!("Count: {}", self.count))
            )

            // Increment
            .child(
                div()
                    .id("increment")
                    .cursor_pointer()
                    .bg(rgb(0x2563eb))
                    .text_color(rgb(0xffffff))
                    .p_2()
                    .child("Increment")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count += 1;
                        cx.notify();
                    }))
            )

            // Decrement
            .child(
                div()
                    .id("decrement")
                    .cursor_pointer()
                    .bg(rgb(0x475569))
                    .text_color(rgb(0xffffff))
                    .p_2()
                    .child("Decrement")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count -= 1;
                        cx.notify();
                    }))
            )

            // Reset
            .child(
                div()
                    .id("reset")
                    .cursor_pointer()
                    .bg(rgb(0x475569))
                    .text_color(rgb(0xffffff))
                    .p_2()
                    .child("Reset")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count = 0;
                        cx.notify();
                    }))
            )

            // Custom child component
            .child(Test {
                _renders: 10,
            })
    }
}

// Stateless child component
#[derive(IntoElement)]
struct Test {
    _renders: i32,
}

impl RenderOnce for Test {
    fn render(
        self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        div()
            .bg(rgb(0x000000))
            .text_3xl()
            .text_color(rgb(0xffffff))
            .child("This is test screen")
    }
}
