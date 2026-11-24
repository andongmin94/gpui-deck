//! Roving keyboard focus and single or multiple selection from ToggleGroupState.
mod control_style;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{ToggleGroup, ToggleGroupState, ToggleItem, ToggleOption};
pub(crate) fn toggle_group(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<ToggleGroupState>,
    p: Palette,
    cx: &App,
) -> ToggleGroup {
    let model = state.read(cx);
    ToggleGroup::new(id, state)
        .accessibility_label(label)
        .flex()
        .gap_2()
        .children(model.items().iter().enumerate().map(|(i, item)| {
            let selected = model.is_pressed(i);
            ToggleOption::new(state, i)
                .px_3()
                .h(px(38.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(palette::control_radius(p))
                .bg(if selected { p.field } else { p.surface })
                .shadow(if selected {
                    palette::inset(p)
                } else {
                    palette::raised(p)
                })
                .text_color(if selected {
                    palette::accent_ink(p)
                } else {
                    p.foreground
                })
                .font_weight(FontWeight::SEMIBOLD)
                .when(item.is_disabled(), |s| s.opacity(0.5))
                .focus_visible(|s| s.underline())
                .child(item.label().clone())
        }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        ToggleGroupState::new(
            [
                ToggleItem::new("bold", "Bold"),
                ToggleItem::new("italic", "Italic"),
                ToggleItem::new("underline", "Underline"),
            ],
            cx,
        )
        .default_values(["bold"])
    });
    control_style::sample(state, cx, |p, state, _, cx| {
        toggle_group("format", "Text formatting", state, p, cx)
    })
}
