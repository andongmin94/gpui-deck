//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xfbe5ff,
    main: 0xe96bff,
    chart: [0xe96bff, 0xfacc00, 0xff7a05, 0xff4d50, 0x7a83ff],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x361736,
    main: 0xc700eb,
    chart: [0xc700eb, 0xe0b700, 0xeb6d00, 0xff6669, 0x7a83ff],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Fuchsia", LIGHT, DARK, cx)
}
