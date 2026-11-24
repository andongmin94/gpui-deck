mod palette;
mod style;
use super::{Palette, recipes::Card};
use gpui::{prelude::*, *};
#[derive(Clone, Copy)]
#[allow(dead_code)] // The caller may choose a flat surface.
pub(crate) enum Surface {
    Raised,
    Flat,
    Inset,
}
pub(crate) fn card(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    surface: Surface,
    p: Palette,
) -> Card {
    let base = Card::new(id, title, p)
        .rounded(p.radius)
        .bg(p.surface)
        .border_1()
        .border_color(p.border);
    match surface {
        Surface::Raised => base.shadow(palette::raised(p)),
        Surface::Flat => base.shadow_none(),
        Surface::Inset => base.bg(p.field).shadow(palette::inset(p)),
    }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                card("market", "Market overview", Surface::Raised, p)
                    .description("See today’s watchlist movement at a glance.")
                    .header_action(div().text_sm().text_color(p.accent).child("Live"))
                    .content(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("₩72,400"),
                            )
                            .child("+2.41%"),
                    ),
            )
            .child(
                card("inset", "Inset surface", Surface::Inset, p)
                    .content("Use recessed depth for supporting information."),
            )
            .into_any_element()
    })
}
