use gpui::{
    App, AppContext as _, Styled as _, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};
use gpui_deck::Root;

use crate::screen::{Screen, TITLE, WINDOW_SIZE};

pub(crate) fn open(cx: &mut App) {
    let (width, height) = WINDOW_SIZE;
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(TITLE.into()),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::centered(size(px(width), px(height)), cx)),
            ..WindowOptions::default()
        },
        |window, cx| {
            let screen = cx.new(|cx| Screen::new(window, cx));
            cx.new(|cx| Root::new(screen, window, cx).font_family("Pretendard"))
        },
    )
    .expect("open the block window");
}
