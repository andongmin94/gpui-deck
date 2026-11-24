//! Month navigation and the semantic seven-column grid shared by Calendar and DatePicker.
use super::{Palette, control_style};
use gpui::{prelude::*, *};
use gpui_deck::{Button, Calendar, CalendarDay, CalendarState, Weekday};

pub(crate) fn calendar(
    id: impl Into<ElementId>,
    state: &Entity<CalendarState>,
    p: Palette,
    cx: &App,
) -> Div {
    let id = id.into();
    let model = state.read(cx);
    let month = model.month();
    let title = month.strftime("%B %Y").to_string();
    let previous = state.clone();
    let next = state.clone();
    let header = div()
        .flex()
        .items_center()
        .justify_between()
        .gap_2()
        .child(
            super::button::raised(p, Button::new((id.clone(), "previous")))
                .size(px(28.))
                .px_0()
                .accessibility_label("Previous month")
                .disabled(!model.can_previous_month())
                .child("‹")
                .on_click(move |_, window, cx| {
                    previous.update(cx, |s, cx| s.previous_month(window, cx))
                }),
        )
        .child(div().font_weight(FontWeight::SEMIBOLD).child(title.clone()))
        .child(
            super::button::raised(p, Button::new((id.clone(), "next")))
                .size(px(28.))
                .px_0()
                .accessibility_label("Next month")
                .disabled(!model.can_next_month())
                .child("›")
                .on_click(move |_, window, cx| next.update(cx, |s, cx| s.next_month(window, cx))),
        );
    let grid = Calendar::new(id, state)
        .accessibility_label(title)
        .flex()
        .flex_col()
        .child(
            div()
                .id("weekday-headings")
                .role(Role::Row)
                .aria_row_index(0)
                .flex()
                .children(
                    model
                        .weekdays()
                        .into_iter()
                        .enumerate()
                        .map(|(column, weekday)| {
                            let name = match weekday {
                                Weekday::Monday => "Mo",
                                Weekday::Tuesday => "Tu",
                                Weekday::Wednesday => "We",
                                Weekday::Thursday => "Th",
                                Weekday::Friday => "Fr",
                                Weekday::Saturday => "Sa",
                                Weekday::Sunday => "Su",
                            };
                            div()
                                .id(("weekday", column))
                                .role(Role::ColumnHeader)
                                .aria_column_index(column)
                                .w(px(32.))
                                .text_center()
                                .text_size(px(12.))
                                .child(name)
                        }),
                ),
        )
        .children(model.days().chunks(7).enumerate().map(|(row, week)| {
            div()
                .id(("week", row))
                .role(Role::Row)
                .aria_row_index(row + 1)
                .flex()
                .children(week.iter().map(|&date| {
                    let selected = model.is_selected(date);
                    let outside = month.month() != date.month();
                    CalendarDay::new(state, date)
                        .accessibility_label(date.strftime("%A, %B %-d, %Y").to_string())
                        .size(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_1()
                        .border_color(gpui::transparent_black())
                        .rounded(p.radius)
                        .when(outside, |s| s.text_color(p.muted))
                        .when(selected, |s| {
                            s.bg(p.accent)
                                .text_color(p.accent_foreground)
                                .border_color(p.border)
                        })
                        .hover(|s| s.border_color(p.border))
                        .focus_visible(|s| s.border_color(p.border).underline())
                        .styles(|s| s.disabled(|s| s.opacity(0.45)))
                        .child(date.day().to_string())
                }))
        }));
    control_style::panel(p)
        .p_3()
        .gap_3()
        .child(header)
        .child(grid)
}
