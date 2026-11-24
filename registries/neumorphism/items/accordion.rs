//! A keyboard-accessible disclosure group using raised and selected surfaces.

#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Accordion, AccordionHeader, AccordionItem, AccordionPanel, AccordionState};
/// The state supplies stable labels/keys. The panel callback supplies editable application content.
pub(crate) fn accordion<E: IntoElement>(
    id: impl Into<ElementId>,
    state: &Entity<AccordionState>,
    p: Palette,
    cx: &App,
    body: impl Fn(usize) -> E,
) -> Accordion {
    let model = state.read(cx);
    Accordion::new(id, state)
        .w_full()
        .flex()
        .flex_col()
        .gap_2()
        .children(model.items().iter().enumerate().map(|(i, item)| {
            let open = model.is_open(i);
            control_style::panel(p)
                .child(
                    AccordionHeader::new(state, i)
                        .h(px(44.))
                        .px_3()
                        .flex()
                        .items_center()
                        .justify_between()
                        .rounded(p.radius)
                        .bg(if open { p.highlight } else { p.surface })
                        .text_color(if open { p.foreground } else { p.foreground })
                        .font_weight(FontWeight::SEMIBOLD)
                        .focus_visible(|s| s.underline().text_color(palette::accent_ink(p)))
                        .child(item.label().clone())
                        .child(if open { "−" } else { "+" }),
                )
                .child(
                    AccordionPanel::new(state, i)
                        .p_3()
                        .border_t_1()
                        .border_color(p.border)
                        .child(body(i)),
                )
        }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        AccordionState::new(
            [
                AccordionItem::new("source", "Where does the source go?"),
                AccordionItem::new("theme", "Can I change the colors?"),
            ],
            cx,
        )
        .default_open(["source"])
    });
    control_style::sample(state, cx, |p, s, _, cx| {
        div()
            .w_full()
            .max_w(px(360.))
            .child(accordion("faq", s, p, cx, |i| {
                [
                    "Into the UI module of your Rust application.",
                    "Choose an installed palette or edit the color tokens.",
                ][i]
            }))
    })
}
