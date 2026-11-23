//! dialog in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
mod modal_parts;
use gpui_deck::{Dialog, DialogState};
pub(crate) fn dialog(
    state: &Entity<DialogState>,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    body: impl IntoElement,
    p: Palette,
) -> Dialog {
    let title = title.into();
    let description = description.into();
    Dialog::new(state)
        .accessibility_label(title.clone())
        .accessibility_description(description.clone())
        .backdrop(modal_parts::backdrop())
        .popup(modal_parts::popup(title, description, body, p))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let focus = cx.focus_handle().tab_stop(true);
    let state = cx.new(|cx| DialogState::new(cx).with_initial_focus(&focus));
    control_style::sample(state, cx, move |p, s, _, _| {
        div()
            .child(modal_parts::open("open", "Workspace details", s, p))
            .child(dialog(
                s,
                "Workspace details",
                "The Studio workspace has three members.",
                modal_parts::close("Done", s, &focus, p),
                p,
            ))
    })
}
