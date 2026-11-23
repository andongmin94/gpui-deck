mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, FontWeight, SharedString, div};
pub(crate) fn badge(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    filled: bool,
    p: Palette,
) -> impl IntoElement {
    let label = label.into();
    div()
        .id(id)
        .role(gpui::Role::Label)
        .aria_label(label.clone())
        .child(label)
        .px_2()
        .py_1()
        .rounded(p.radius)
        .border_2()
        .border_color(p.border)
        .bg(if filled { p.accent } else { p.surface })
        .text_color(if filled {
            p.accent_foreground
        } else {
            p.foreground
        })
        .font_weight(FontWeight::BOLD)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        div()
            .flex()
            .justify_center()
            .gap_2()
            .child(badge("draft", "Draft", false, p))
            .child(badge("live", "Published", true, p))
            .into_any_element()
    })
}
