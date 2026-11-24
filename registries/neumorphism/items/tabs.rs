//! A tab list and associated panels backed by TabsState.

#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Tab, TabItem, TabList, TabPanel, TabsState};
pub(crate) fn tabs<E: IntoElement>(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<TabsState>,
    p: Palette,
    cx: &App,
    content: impl Fn(usize) -> E,
) -> Div {
    let model = state.read(cx);
    let list = TabList::new(id, state)
        .accessibility_label(label)
        .p_1()
        .rounded(palette::control_radius(p))
        .bg(p.field)
        .shadow(palette::inset(p))
        .flex()
        .gap_1()
        .children(model.items().iter().enumerate().map(|(i, item)| {
            let active = model.selected_index() == Some(i);
            Tab::new(state, i)
                .px_3()
                .h(px(38.))
                .flex()
                .items_center()
                .rounded(palette::control_radius(p))
                .when(active, |s| s.shadow(palette::raised(p)))
                .border_1()
                .border_color(transparent_black())
                .font_weight(FontWeight::SEMIBOLD)
                .bg(if active { p.surface } else { p.field })
                .text_color(if active {
                    palette::accent_ink(p)
                } else {
                    p.foreground
                })
                .focus_visible(|s| s.underline().text_color(palette::accent_ink(p)))
                .child(item.label().clone())
        }));
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap_2()
        .child(list)
        .children((0..model.items().len()).map(|i| {
            TabPanel::new(state, i).child(control_style::panel(p).p_4().child(content(i)))
        }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        TabsState::new(
            [
                TabItem::new("overview", "Overview"),
                TabItem::new("activity", "Activity"),
            ],
            cx,
        )
    });
    control_style::sample(s, cx, |p, s, _, cx| {
        div()
            .w_full()
            .max_w(px(320.))
            .child(tabs("project", "Project", s, p, cx, |i| {
                ["3 projects in your workspace.", "Updated the style guide."][i]
            }))
    })
}
