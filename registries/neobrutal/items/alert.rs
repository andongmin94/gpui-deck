mod palette;
mod style;

use super::Palette;
use super::recipes::Alert;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, SharedString};
use gpui_deck::box_shadow;

pub(crate) fn alert(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    message: impl Into<SharedString>,
    p: Palette,
) -> Alert {
    Alert::new(id, title, message, p)
        .border_2()
        .border_color(p.border)
        .rounded(p.radius)
        .bg(p.surface)
        .shadow(vec![box_shadow(4., 4., 0., 0., p.border)])
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        alert(
            "saved",
            "Changes saved",
            "Your workspace settings are up to date.",
            p,
        )
        .into_any_element()
    })
}
