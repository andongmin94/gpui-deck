//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xf2e6ff,
    main: 0xca7aff,
    chart: [0xca7aff, 0xfacc00, 0x00d696, 0xff7a05, 0x0099ff],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x3c1f47,
    main: 0xc061ff,
    chart: [0xd494ff, 0xe0b700, 0x00bd84, 0xeb6d00, 0x008ae5],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Purple", LIGHT, DARK, cx)
}
