mod palette;
mod style;

use super::Palette;
use super::recipes::Avatar;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, SharedString, div, px};
pub(crate) fn avatar(
    id: impl Into<ElementId>,
    name: impl Into<SharedString>,
    initials: impl Into<SharedString>,
    p: Palette,
) -> Avatar {
    Avatar::new(id, name, initials, p)
        .size(px(52.))
        .border_2()
        .border_color(p.border)
        .bg(p.accent)
        .text_color(p.accent_foreground)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        div()
            .flex()
            .justify_center()
            .gap_2()
            .child(avatar("maya", "Maya Chen", "MC", p))
            .child(avatar("ada", "Ada Park", "AP", p))
            .child(avatar("lee", "Lee Kim", "LK", p))
            .into_any_element()
    })
}
