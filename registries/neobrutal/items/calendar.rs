//! calendar in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
mod calendar_grid;
pub(crate) use calendar_grid::calendar;
use gpui_deck::{CalendarSelection, CalendarState, Date};
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let date = Date::new(2026, 11, 24).unwrap();
    let state = cx.new(|cx| {
        CalendarState::new(date, cx).default_selection(CalendarSelection::Single(Some(date)))
    });
    control_style::sample(state, cx, |p, s, _, cx| calendar("schedule", s, p, cx))
}
