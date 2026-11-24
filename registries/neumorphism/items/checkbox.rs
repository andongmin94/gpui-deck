//! A labeled checkbox with checked, unchecked and indeterminate states.

#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Checkbox, CheckboxState};
/// Attach on_change to store the next state in your application.
pub(crate) fn checkbox(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    value: CheckboxState,
    p: Palette,
) -> Checkbox {
    let label = label.into();
    let mark = match value {
        CheckboxState::Unchecked => "",
        CheckboxState::Checked => "✓",
        CheckboxState::Indeterminate => "−",
    };
    Checkbox::new(id)
        .state(value)
        .accessibility_label(label.clone())
        .flex()
        .items_center()
        .gap_2()
        .focus_visible(|s| s.underline().text_color(palette::accent_ink(p)))
        .child(
            palette::well(p)
                .size(px(22.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(6.))
                .border_1()
                .border_color(p.border)
                .bg(p.surface)
                .text_color(palette::accent_ink(p))
                .font_weight(FontWeight::SEMIBOLD)
                .child(mark),
        )
        .child(label)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|_| CheckboxState::Unchecked);
    control_style::sample(state, cx, |p, state, _, cx| {
        let model = state.clone();
        checkbox("comments", "Allow comments", *state.read(cx), p).on_change(
            move |value, _, _, cx| {
                model.update(cx, |model, cx| {
                    *model = value;
                    cx.notify();
                })
            },
        )
    })
}
