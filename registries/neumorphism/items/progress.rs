mod palette;
mod style;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::Progress;

/// Progress is expressed as a percentage. A missing percentage leaves the operation indeterminate.
pub(crate) fn progress(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    percent: Option<f32>,
    p: Palette,
) -> Progress {
    let track = Progress::new(id)
        .accessibility_label(label)
        .w_full()
        .h(px(12.))
        .rounded_full()
        .border_1()
        .border_color(p.border)
        .bg(p.field)
        .shadow(palette::inset(p))
        .overflow_hidden();
    let fill = div().h_full().rounded_full().bg(p.accent);
    match percent {
        Some(percent) => track
            .value(percent)
            .child(fill.w(relative(percent.clamp(0., 100.) / 100.))),
        None => track
            .indeterminate(true)
            .child(fill.w(relative(0.28)).opacity(0.45)),
    }
}

pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        style::surface(p)
            .flex()
            .flex_col()
            .gap_4()
            .children(
                [
                    ("build", "Registry build", "82%", Some(82.)),
                    ("tokens", "Theme tokens", "12 / 16", Some(75.)),
                    ("waiting", "Waiting for data", "—", None),
                ]
                .map(|(id, label, reading, percent)| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .child(label)
                                .child(div().font_weight(FontWeight::SEMIBOLD).child(reading)),
                        )
                        .child(progress(id, label, percent, p))
                }),
            )
            .into_any_element()
    })
}
