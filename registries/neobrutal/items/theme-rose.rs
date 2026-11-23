//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xffe0e3,
    main: 0xff6678,
    chart: [0xff6678, 0x7a83ff, 0xfacc00, 0x00d696, 0x5294ff],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x36171e,
    main: 0xff4d61,
    chart: [0xff4d61, 0x7a83ff, 0xe0b700, 0x00bd84, 0x5294ff],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Rose", LIGHT, DARK, cx)
}
