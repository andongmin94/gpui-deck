use std::borrow::Cow;

mod block_window;
mod screen;
// Installed recipe modules expose more controls than this screen uses.
#[allow(dead_code, unused_imports)]
mod ui;

fn main() {
    gpui_platform::application().run(|cx| {
        cx.text_system()
            .add_fonts(vec![
                Cow::Borrowed(include_bytes!("../assets/fonts/Pretendard-Regular.otf")),
                Cow::Borrowed(include_bytes!("../assets/fonts/Pretendard-Medium.otf")),
                Cow::Borrowed(include_bytes!("../assets/fonts/Pretendard-SemiBold.otf")),
            ])
            .expect("register the block's Pretendard fonts");
        gpui_deck::init(cx);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        block_window::open(cx);
        cx.activate(true);
    });
}
