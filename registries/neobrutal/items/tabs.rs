//! tabs in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
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
        .flex()
        .gap_1()
        .children(model.items().iter().enumerate().map(|(i, item)| {
            let active = model.selected_index() == Some(i);
            Tab::new(state, i)
                .px_3()
                .h(px(38.))
                .flex()
                .items_center()
                .rounded(p.radius)
                .border_2()
                .border_color(p.border)
                .font_weight(FontWeight::BOLD)
                .bg(if active { p.accent } else { p.surface })
                .text_color(if active {
                    p.accent_foreground
                } else {
                    p.foreground
                })
                .focus_visible(|s| s.underline())
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
