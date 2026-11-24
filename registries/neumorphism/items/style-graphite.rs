//! Application palette and interactive light/dark samples.
mod palette;
mod swatch;
use super::Palette;

pub(crate) fn palette(dark: bool) -> Palette {
    palette::GRAPHITE.palette(dark)
}
pub(crate) fn preview(_: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    swatch::show(palette::GRAPHITE, cx)
}
