//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xdcebfe,
    main: 0x5294ff,
    chart: [0x5294ff, 0xff4d50, 0xfacc00, 0x05e17a, 0x7a83ff],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x20294b,
    main: 0x5294ff,
    chart: [0x5294ff, 0xff6669, 0xe0b700, 0x04c86d, 0x7a83ff],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Blue", LIGHT, DARK, cx)
}
