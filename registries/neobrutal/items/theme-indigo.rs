//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xe0e2ff,
    main: 0x7a83ff,
    chart: [0x7a83ff, 0xfacc00, 0xff4d50, 0x00d696, 0x0099ff],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x221f47,
    main: 0x7a83ff,
    chart: [0x7a83ff, 0xe0b700, 0xff6669, 0x00bd84, 0x008ae5],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Indigo", LIGHT, DARK, cx)
}
