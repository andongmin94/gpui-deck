use std::borrow::Cow;

use gpui::{AppContext as _, TitlebarOptions, WindowBounds, WindowOptions, px, size};
use gpui_deck::Root;
use ui::preset;

mod gallery;
#[allow(dead_code, unused_imports)] // The installed sources expose more controls than this gallery uses.
mod ui;

fn main() {
    gpui_platform::application().run(|cx| {
        cx.text_system()
            .add_fonts(vec![
                Cow::Borrowed(include_bytes!("ui/preset/assets/Pretendard-Regular.otf")),
                Cow::Borrowed(include_bytes!("fonts/Pretendard-Medium.otf")),
                Cow::Borrowed(include_bytes!("fonts/Pretendard-SemiBold.otf")),
            ])
            .expect("register the gallery's Pretendard fonts");
        gpui_deck::init(cx);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(env!("CARGO_PKG_NAME").into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::centered(size(px(1176.), px(806.)), cx)),
                ..WindowOptions::default()
            },
            |window, cx| {
                let gallery = gallery::open(CREATE_DARK_MODE, window, cx);
                cx.new(|cx| Root::new(gallery, window, cx))
            },
        )
        .expect("open the GPUI Deck gallery");
        cx.activate(true);
    });
}
