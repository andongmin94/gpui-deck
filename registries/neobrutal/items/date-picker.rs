//! date-picker in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
mod calendar_grid;
use gpui_deck::{CalendarState, Date, DatePicker, DatePickerState};
pub(crate) fn date_picker(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<DatePickerState>,
    p: Palette,
    cx: &App,
) -> DatePicker {
    let id = id.into();
    let label = label.into();
    let picker = state.read(cx);
    let date = picker
        .selection(cx)
        .first_date()
        .map(|date| date.strftime("%d %b %Y").to_string());
    let text = date.unwrap_or_else(|| label.to_string());
    DatePicker::new(id.clone(), state)
        .accessibility_label(label.clone())
        .offset(px(8.))
        .trigger(move |b| {
            button::raised(p, b)
                .accessibility_label(format!("{label}, {text}"))
                .child(text)
                .child(" ▼")
        })
        .content(calendar_grid::calendar(
            (id, "grid"),
            picker.calendar(),
            p,
            cx,
        ))
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    let cal = cx.new(|cx| CalendarState::new(Date::new(2026, 11, 24).unwrap(), cx));
    let picker = cx.new(|cx| DatePickerState::new(&cal, w, cx));
    control_style::sample(picker, cx, |p, s, _, cx| {
        date_picker("release", "Choose release date", s, p, cx)
    })
}
