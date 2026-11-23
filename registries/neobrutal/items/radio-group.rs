//! radio-group in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{RadioGroup, RadioGroupState, RadioItem, RadioOption};
pub(crate) fn radio_group(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<RadioGroupState>,
    p: Palette,
    cx: &App,
) -> RadioGroup {
    let model = state.read(cx);
    RadioGroup::new(id, state)
        .accessibility_label(label)
        .flex()
        .flex_col()
        .gap_2()
        .children(model.items().iter().enumerate().map(|(index, item)| {
            let checked = model.selected_index() == Some(index);
            RadioOption::new(state, index)
                .flex()
                .items_center()
                .gap_2()
                .focus_visible(|s| s.underline())
                .when(item.is_disabled(), |s| s.opacity(0.5))
                .child(
                    div()
                        .size(px(22.))
                        .rounded_full()
                        .border_2()
                        .border_color(p.border)
                        .bg(if checked { p.accent } else { p.field })
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(checked, |s| {
                            s.child(div().size(px(8.)).rounded_full().bg(p.accent_foreground))
                        }),
                )
                .child(item.label().clone())
        }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        RadioGroupState::new(
            [
                RadioItem::new("daily", "Daily digest"),
                RadioItem::new("weekly", "Weekly digest"),
                RadioItem::new("never", "Never"),
            ],
            cx,
        )
        .default_value("weekly")
    });
    control_style::sample(state, cx, |p, s, _, cx| {
        radio_group("schedule", "Notification frequency", s, p, cx)
    })
}
