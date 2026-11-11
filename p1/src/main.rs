
use std::task::Context;

use gpui::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1080.0), px(720.0)), cx);
        let test = Test{
            renders:0
        };


        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::new("test-screen")),
                    appears_transparent: true,
                    traffic_light_position: Some(Point {
                        x: px(1000.0),
                        y: px(100.0),
                    }),
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|_| {
                div
            }),
        )
        .unwrap();
    });
}

struct Test {
    renders: i32,
}

impl Render for Test {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0x0))
            .text_3xl()
            .flex()
            .text_color(rgb(0xf))
            .child("This is test screen")
    }
}

struct Counter {
    count: i32,
    renders: i32,
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("function rendered for {} time", self.renders);
        self.renders += 1;
        div()
            .child(
                div()
                    .flex()
                    .bg(rgb(0xf8fafc))
                    .h(px(50.0))
                    .w(px(1080.0))
                    .text_color(rgb(0x0f172a))
                    .child(
                        div()
                            .id("hide")
                            .h_full()
                            .w_1_2()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x2563eb))
                            .text_color(rgb(0x000000))
                            .cursor_pointer()
                            .child("hide")
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.open_url("https://ashishmeena.vercel.app");
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .id("x")
                            .px_4()
                            .h_full()
                            .w_1_2()
                            .py_2()
                            .bg(rgb(0x2563eb))
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .child("x")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count += 1;
                                cx.quit();
                            })),
                    ),
            )
            .child(
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
                            .bg(rgb(0x475569))
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .child("reset")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count = 0;
                                cx.notify();
                            })),
                    ),
            )
    }
}
