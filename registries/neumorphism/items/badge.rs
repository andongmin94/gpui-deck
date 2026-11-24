mod palette;
mod style;
use super::Palette;
use gpui::{prelude::*, *};
#[derive(Clone, Copy)]
pub(crate) enum Variant {
    Default,
    Primary,
    Soft,
    Outline,
    Destructive,
}
pub(crate) fn badge(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    variant: Variant,
    p: Palette,
) -> impl IntoElement {
    let label = label.into();
    let (surface, ink) = match variant {
        Variant::Default | Variant::Outline => (p.surface, p.foreground),
        Variant::Primary => (p.accent, p.accent_foreground),
        Variant::Soft => (p.highlight, p.foreground),
        Variant::Destructive => (p.surface, p.error),
    };
    div()
        .id(id)
        .role(Role::Label)
        .aria_label(label.clone())
        .child(label)
        .px_3()
        .py_1()
        .rounded_full()
        .bg(surface)
        .text_color(ink)
        .text_size(px(12.))
        .font_weight(FontWeight::SEMIBOLD)
        .when(
            matches!(variant, Variant::Default | Variant::Primary),
            |s| s.shadow(palette::raised(p)),
        )
        .when(
            matches!(variant, Variant::Outline | Variant::Destructive),
            |s| s.border_1().border_color(ink.opacity(0.5)),
        )
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        div()
            .flex()
            .flex_wrap()
            .justify_center()
            .gap_3()
            .children(
                [
                    ("Default", Variant::Default),
                    ("Primary", Variant::Primary),
                    ("Soft", Variant::Soft),
                    ("Outline", Variant::Outline),
                    ("Destructive", Variant::Destructive),
                ]
                .map(|(label, variant)| badge(label, label, variant, p)),
            )
            .into_any_element()
    })
}
