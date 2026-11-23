mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, SharedString, div, px, relative};
use gpui_deck::Progress;
pub(crate) fn progress(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    percent: f32,
    p: Palette,
) -> Progress {
    let percent = percent.clamp(0., 100.);
    Progress::new(id)
        .accessibility_label(label)
        .value(percent)
        .w_full()
        .h(px(22.))
        .rounded(p.radius)
        .border_2()
        .border_color(p.border)
        .bg(p.surface)
        .overflow_hidden()
        .child(div().h_full().w(relative(percent / 100.)).bg(p.accent))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        style::surface(p)
            .flex()
            .flex_col()
            .gap_2()
            .child("Transfer · 68%")
            .child(progress("transfer", "File transfer", 68., p))
            .into_any_element()
    })
}
