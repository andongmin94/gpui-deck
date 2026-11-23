//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xd0fbea,
    main: 0x00d696,
    chart: [0x00d696, 0x7a83ff, 0xfacc00, 0xff4d50, 0x0099ff],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x0e201d,
    main: 0x00bd84,
    chart: [0x00bd84, 0x7a83ff, 0xe0b700, 0xff6669, 0x008ae5],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Emerald", LIGHT, DARK, cx)
}
