//! A supplemental preview of a link destination.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{HoverCard, HoverCardState};
pub(crate) fn hover_card(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<HoverCardState>,
    content: impl IntoElement,
    p: Palette,
) -> HoverCard {
    let label = label.into();
    HoverCard::new(id, state)
        .accessibility_label(label.clone())
        .offset(px(8.))
        .trigger(move |button| {
            button::raised(p, button)
                .accessibility_label(label.clone())
                .child(label)
        })
        .content(control_style::panel(p).p_4().child(content))
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    let focus = cx.focus_handle().tab_stop(true);
    let s = cx.new(|cx| HoverCardState::new(&focus, w, cx));
    control_style::sample(s, cx, |p, s, _, _| {
        hover_card(
            "studio",
            "@studio",
            s,
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().font_weight(FontWeight::SEMIBOLD).child("Studio"))
                .child("Independent design team"),
            p,
        )
    })
}
