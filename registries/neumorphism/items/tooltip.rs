//! A raised shortcut trigger with a delayed, dismissible hint.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Tooltip, TooltipState};
pub(crate) fn tooltip(
    id: impl Into<ElementId>,
    state: &Entity<TooltipState>,
    label: impl Into<SharedString>,
    hint: impl Into<SharedString>,
    p: Palette,
    window: &Window,
) -> Tooltip {
    let label = label.into();
    Tooltip::new(id, state, hint)
        .font_family(window.text_style().font_family)
        .shadow(palette::raised(p))
        .border_1()
        .border_color(p.border)
        .rounded(p.radius)
        .px_3()
        .py_1()
        .bg(p.surface)
        .text_color(p.foreground)
        .trigger(move |b| {
            button::raised(p, b)
                .accessibility_label(label.clone())
                .child(label)
        })
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    let focus = cx.focus_handle().tab_stop(true);
    let s = cx.new(|cx| TooltipState::new(&focus, w, cx));
    control_style::sample(s, cx, |p, s, window, _| {
        tooltip(
            "shortcut",
            s,
            "Shortcut",
            "Save this file with Ctrl+S",
            p,
            window,
        )
    })
}
