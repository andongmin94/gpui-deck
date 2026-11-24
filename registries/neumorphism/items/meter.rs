mod palette;
mod style;
use super::Palette;
use gpui::{prelude::*, *};
/// A read-only measurement; the caller supplies the human-readable unit.
pub(crate) fn meter(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    value: f64,
    range: std::ops::RangeInclusive<f64>,
    unit: &str,
    p: Palette,
) -> impl IntoElement {
    let (min, max) = (*range.start(), *range.end());
    assert!(
        min.is_finite() && max.is_finite() && max > min && value.is_finite(),
        "Meter needs a finite increasing range and value"
    );
    let value = value.clamp(min, max);
    let fraction = ((value - min) / (max - min)) as f32;
    let label = label.into();
    let caption = format!("{value} {unit} / {max} {unit}");
    div()
        .id(id)
        .role(Role::Meter)
        .aria_label(label.clone())
        .aria_value(caption.clone())
        .aria_min_numeric_value(min)
        .aria_max_numeric_value(max)
        .aria_numeric_value(value)
        .w_full()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .justify_between()
                .gap_2()
                .font_weight(FontWeight::SEMIBOLD)
                .child(label)
                .child(format!("{:.0}%", 100. * fraction)),
        )
        .child(
            palette::well(p).rounded_full().h(px(12.)).w_full().child(
                div()
                    .h_full()
                    .rounded_full()
                    .bg(p.accent)
                    .w(relative(fraction)),
            ),
        )
        .child(div().text_sm().text_color(p.muted).child(caption))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        style::surface(p)
            .child(meter("storage", "Storage", 64., 0.0..=100.0, "GB", p))
            .into_any_element()
    })
}
