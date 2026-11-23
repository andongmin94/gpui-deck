//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xcdf6fe,
    main: 0x00c8f0,
    chart: [0x00c8f0, 0xff7a05, 0x7a83ff, 0xff4d50, 0xfacc00],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x152a32,
    main: 0x009dbd,
    chart: [0x009dbd, 0xeb6d00, 0x7a83ff, 0xff6669, 0xe0b700],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Cyan", LIGHT, DARK, cx)
}
