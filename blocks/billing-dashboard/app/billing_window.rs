use gpui::{
    App, AppContext as _, Styled as _, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};
use gpui_deck::Root;

use crate::ui::billing_dashboard::BillingDashboard as Screen;

pub(crate) fn open(cx: &mut App) {
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Billing dashboard".into()),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::centered(size(px(1176.), px(806.)), cx)),
            ..WindowOptions::default()
        },
        |window, cx| {
            let screen = cx.new(|cx| Screen::new(window, cx));
            cx.new(|cx| Root::new(screen, window, cx).font_family("Pretendard"))
        },
    )
    .expect("open the billing dashboard");
}
