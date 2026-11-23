//! popover in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Popover, PopoverState};
pub(crate) fn popover(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<PopoverState>,
    content: impl IntoElement,
    p: Palette,
) -> Popover {
    let label = label.into();
    Popover::new(id, state)
        .accessibility_label(label.clone())
        .offset(px(8.))
        .trigger(move |button| {
            button::raised(p, button)
                .accessibility_label(label.clone())
                .child(label)
        })
        .content(control_style::panel(p).p_3().child(content))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(PopoverState::new);
    control_style::sample(s, cx, |p, s, _, _| {
        popover(
            "metadata",
            "Document details",
            s,
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child("Style guide")
                .child("Edited yesterday · 4 pages"),
            p,
        )
    })
}
