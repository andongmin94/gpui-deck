//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xf4f5f7,
    main: 0x27282b,
    on_main: 0xf4f5f7,
    chart: [0x526e85, 0x687e62, 0x9a744c, 0x887196, 0x477c79],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x18191c,
    main: 0xe4e7ec,
    chart: [0x9db8cc, 0xa3b997, 0xd3b083, 0xbaa3c9, 0x8bbab5],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Mono", LIGHT, DARK, cx)
}
