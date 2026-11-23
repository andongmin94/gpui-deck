//! alert-dialog in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
mod modal_parts;
use gpui_deck::{AlertDialog, Button, DialogState};
pub(crate) fn alert_dialog(
    state: &Entity<DialogState>,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    actions: impl IntoElement,
    p: Palette,
) -> AlertDialog {
    let title = title.into();
    let description = description.into();
    AlertDialog::new(state)
        .accessibility_label(title.clone())
        .accessibility_description(description.clone())
        .backdrop(modal_parts::backdrop())
        .popup(modal_parts::popup(title, description, actions, p))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let cancel = cx.focus_handle().tab_stop(true);
    let s = cx.new(|cx| DialogState::new(cx).with_initial_focus(&cancel));
    let archived = cx.new(|_| false);
    control_style::sample(s, cx, move |p, s, _, cx| {
        let model = s.clone();
        let change = archived.clone();
        let was_archived = *archived.read(cx);
        let actions = div()
            .flex()
            .justify_end()
            .gap_3()
            .child(modal_parts::close("Cancel", s, &cancel, p))
            .child(
                button::raised(p, Button::new("confirm"))
                    .accessibility_label("Archive")
                    .child("Archive")
                    .on_click(move |_, w, cx| {
                        change.update(cx, |s, cx| {
                            *s = true;
                            cx.notify();
                        });
                        model.update(cx, |s, cx| s.close(w, cx));
                    }),
            );
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(modal_parts::open("archive", "Archive project", s, p).disabled(was_archived))
            .when(was_archived, |s| s.child("Project archived"))
            .child(alert_dialog(
                s,
                "Archive project?",
                "The project will be removed from this list.",
                actions,
                p,
            ))
    })
}
