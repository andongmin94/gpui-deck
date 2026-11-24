//! A dismissible drawer with a drag handle and focus-managed actions.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Button, DialogClose, Drawer, DrawerState};
pub(crate) fn drawer(
    id: impl Into<ElementId>,
    state: &Entity<DrawerState>,
    title: impl Into<SharedString>,
    body: impl IntoElement,
    p: Palette,
) -> Drawer {
    let title = title.into();
    Drawer::new(id, state)
        .accessibility_label(title.clone())
        .shadow(palette::raised(p))
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .text_color(p.foreground)
        .backdrop(div().size_full().bg(black().opacity(0.34)))
        .handle(
            div()
                .w_full()
                .h(px(26.))
                .flex()
                .items_center()
                .justify_center()
                .child(div().w(px(54.)).h(px(5.)).rounded_full().bg(p.border)),
        )
        .child(
            div()
                .p_4()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_size(px(20.))
                        .child(title),
                )
                .child(body),
        )
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    let focus = cx.focus_handle().tab_stop(true);
    let dialog = cx.new(|cx| gpui_deck::DialogState::new(cx).with_initial_focus(&focus));
    let s = cx.new(|cx| DrawerState::with_dialog(dialog.clone(), w, cx));
    control_style::sample(s, cx, move |p, s, _, _| {
        let model = s.clone();
        div()
            .child(
                button::raised(p, Button::new("open"))
                    .accessibility_label("Show options")
                    .child("Show options")
                    .on_click(move |_, w, cx| model.update(cx, |s, cx| s.open(w, cx))),
            )
            .child(drawer(
                "options",
                s,
                "Publish options",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child("The workspace draft is ready to share.")
                    .child(DialogClose::new(&dialog).trigger(|b| {
                        button::raised(p, b)
                            .track_focus(&focus)
                            .accessibility_label("Done")
                            .child("Done")
                    })),
                p,
            ))
    })
}
