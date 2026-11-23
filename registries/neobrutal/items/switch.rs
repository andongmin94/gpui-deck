//! switch in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::Switch;
pub(crate) fn switch(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    checked: bool,
    p: Palette,
) -> Switch {
    let label = label.into();
    Switch::new(id)
        .accessibility_label(label.clone())
        .checked(checked)
        .flex()
        .items_center()
        .gap_2()
        .focus_visible(|s| s.underline())
        .child(
            div()
                .w(px(44.))
                .h(px(26.))
                .p(px(2.))
                .flex()
                .items_center()
                .rounded_full()
                .border_2()
                .border_color(p.border)
                .bg(if checked { p.accent } else { p.field })
                .when(checked, |s| s.justify_end())
                .child(
                    div()
                        .size(px(18.))
                        .rounded_full()
                        .border_2()
                        .border_color(p.border)
                        .bg(p.surface),
                ),
        )
        .child(label)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|_| true);
    control_style::sample(state, cx, |p, state, _, cx| {
        let model = state.clone();
        switch("public", "Public workspace", *state.read(cx), p).on_change(move |v, _, _, cx| {
            model.update(cx, |value, cx| {
                *value = v;
                cx.notify();
            })
        })
    })
}
