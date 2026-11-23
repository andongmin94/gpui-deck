//! Use `palette(false)` for light and `palette(true)` for dark. The returned
//! palette can be passed directly to Neobrutal templates and Deck recipes.

mod palette;
mod swatch;
use super::Palette;
use palette::{Colors, YELLOW_DARK, YELLOW_LIGHT};

pub(crate) const LIGHT: Colors = Colors {
    paper: 0xffedd6,
    main: 0xff7a05,
    chart: [0xff7a05, 0x0099ff, 0xffbf00, 0x00d696, 0x7a83ff],
    ..YELLOW_LIGHT
};
pub(crate) const DARK: Colors = Colors {
    paper: 0x322215,
    main: 0xeb6d00,
    chart: [0xeb6d00, 0x008ae5, 0xe5ac00, 0x00bd84, 0x7a83ff],
    ..YELLOW_DARK
};

pub(crate) fn palette(dark: bool) -> Palette {
    (if dark { DARK } else { LIGHT }).palette()
}

pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show("Neobrutal Orange", LIGHT, DARK, cx)
}
