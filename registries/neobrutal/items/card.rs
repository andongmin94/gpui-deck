mod palette;
mod style;

use super::Palette;
use super::recipes::Card;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, SharedString};
use gpui_deck::box_shadow;
pub(crate) fn card(id: impl Into<ElementId>, title: impl Into<SharedString>, p: Palette) -> Card {
    Card::new(id, title, p)
        .border_2()
        .rounded(p.radius)
        .border_color(p.border)
        .shadow(vec![box_shadow(4., 4., 0., 0., p.border)])
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        card("field-notes", "Field notes", p)
            .description("Studio journal")
            .content("A space for ideas, work in progress and decisions from the design team.")
            .into_any_element()
    })
}
