//! collapsible in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Collapsible, CollapsibleContent, CollapsibleState, CollapsibleTrigger};
pub(crate) fn collapsible(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    state: &Entity<CollapsibleState>,
    content: impl IntoElement,
    p: Palette,
    cx: &App,
) -> Collapsible {
    let title = title.into();
    let open = state.read(cx).is_open();
    Collapsible::new(id, state)
        .w_full()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            CollapsibleTrigger::new(state)
                .accessibility_label(title.clone())
                .flex()
                .items_center()
                .justify_between()
                .px_3()
                .h(px(40.))
                .border_2()
                .border_color(p.border)
                .rounded(p.radius)
                .bg(p.accent)
                .text_color(p.accent_foreground)
                .font_weight(FontWeight::BOLD)
                .focus_visible(|s| s.underline())
                .child(title)
                .child(if open { "−" } else { "+" }),
        )
        .child(CollapsibleContent::new(state).child(control_style::panel(p).p_3().child(content)))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(CollapsibleState::new);
    control_style::sample(s, cx, |p, s, _, cx| {
        div().w_full().max_w(px(300.)).child(collapsible(
            "details",
            "Release notes",
            s,
            "Keyboard navigation and focus styling.",
            p,
            cx,
        ))
    })
}
