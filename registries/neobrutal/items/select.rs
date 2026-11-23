//! select in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Select, SelectItem, SelectOption, SelectState};
pub(crate) fn select(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<SelectState>,
    p: Palette,
    cx: &App,
) -> Select {
    let label = label.into();
    let m = state.read(cx);
    let text = m
        .selected_item()
        .map(|i| i.label().clone())
        .unwrap_or_else(|| label.clone());
    let options = m.items().iter().enumerate().map(|(i, item)| {
        SelectOption::new(state, i)
            .h(px(34.))
            .px_2()
            .flex()
            .items_center()
            .justify_between()
            .when(m.highlighted_index() == Some(i), |s| {
                s.bg(p.accent).text_color(p.accent_foreground)
            })
            .when(item.is_disabled(), |s| s.opacity(0.5))
            .child(item.label().clone())
            .when(m.selected_index() == Some(i), |s| s.child("✓"))
    });
    Select::new(id, state)
        .accessibility_label(label)
        .list_max_height(px(168.))
        .w(px(208.))
        .border_2()
        .border_color(p.border)
        .rounded(p.radius)
        .bg(p.field)
        .focus_visible(|s| s.underline())
        .trigger(
            div()
                .h(px(38.))
                .px_3()
                .flex()
                .items_center()
                .justify_between()
                .child(text)
                .child("▼"),
        )
        .content(control_style::panel(p).w(px(208.)).p_1().children(options))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        SelectState::new(
            [
                SelectItem::new("seoul", "Seoul"),
                SelectItem::new("tokyo", "Tokyo"),
                SelectItem::new("paris", "Paris"),
                SelectItem::new("london", "London"),
                SelectItem::new("new-york", "New York"),
                SelectItem::new("sydney", "Sydney"),
            ],
            cx,
        )
        .default_value("seoul")
    });
    control_style::sample(s, cx, |p, s, _, cx| select("city", "Pick a city", s, p, cx))
}
