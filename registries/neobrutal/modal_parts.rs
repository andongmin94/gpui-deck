use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{
    Button, DialogBackdrop, DialogClose, DialogDescription, DialogPopup, DialogState, DialogTitle,
    box_shadow,
};

pub(super) fn open(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    dialog: &Entity<DialogState>,
    p: Palette,
) -> Button {
    let label = label.into();
    let model = dialog.clone();
    super::button::raised(p, Button::new(id))
        .accessibility_label(label.clone())
        .child(label)
        .on_click(move |_, window, cx| model.update(cx, |s, cx| s.open(window, cx)))
}
pub(super) fn close(
    label: impl Into<SharedString>,
    dialog: &Entity<DialogState>,
    focus: &FocusHandle,
    p: Palette,
) -> DialogClose {
    let label = label.into();
    DialogClose::new(dialog).trigger(|b| {
        super::button::raised(p, b)
            .track_focus(focus)
            .accessibility_label(label.clone())
            .child(label)
    })
}
pub(super) fn backdrop() -> DialogBackdrop {
    DialogBackdrop::new().size_full().bg(black().opacity(0.55))
}
pub(super) fn popup(
    title: SharedString,
    description: SharedString,
    body: impl IntoElement,
    p: Palette,
) -> DialogPopup {
    DialogPopup::new()
        .w_full()
        .max_w(px(350.))
        .p_5()
        .flex()
        .flex_col()
        .gap_3()
        .rounded(p.radius)
        .border_2()
        .border_color(p.border)
        .bg(p.surface)
        .text_color(p.foreground)
        .shadow(vec![box_shadow(5., 5., 0., 0., p.border)])
        .child(
            DialogTitle::new()
                .text_size(px(20.))
                .font_weight(FontWeight::BOLD)
                .child(title),
        )
        .child(DialogDescription::new().child(description))
        .child(body)
}
