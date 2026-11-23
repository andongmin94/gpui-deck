//! sheet in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
#[allow(dead_code)]
mod modal_parts;
use super::recipes::Sheet;
use gpui_deck::DialogState;
pub(crate) fn sheet(
    id: impl Into<ElementId>,
    state: &Entity<DialogState>,
    scroll: &ScrollHandle,
    title: impl Into<SharedString>,
    p: Palette,
) -> Sheet {
    Sheet::new(id, state, scroll, title, p)
        .border_2()
        .border_color(p.border)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let focus = cx.focus_handle().tab_stop(true);
    let scroll = ScrollHandle::new();
    let s = cx.new(|cx| DialogState::new(cx).with_initial_focus(&focus));
    control_style::sample(s, cx, move |p, s, _, _| {
        div()
            .child(modal_parts::open("shortcuts", "Keyboard shortcuts", s, p))
            .child(
                sheet("shortcuts-sheet", s, &scroll, "Keyboard shortcuts", p)
                    .w(px(320.))
                    .child("Tab — Next control")
                    .child("Shift + Tab — Previous control")
                    .child("Escape — Close panel")
                    .footer(modal_parts::close("Close", s, &focus, p)),
            )
    })
}
