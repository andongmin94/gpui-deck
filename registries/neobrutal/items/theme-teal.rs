//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xccfaf2,
    main: 0x00d6bd,
    chart: [0x00d6bd, 0x0099ff, 0x7a83ff, 0xff4d50, 0xfacc00],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x0e2020,
    main: 0x00bda7,
    chart: [0x00bda7, 0x008ae5, 0x7a83ff, 0xff6669, 0xe0b700],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Teal", LIGHT, DARK, cx)
}
