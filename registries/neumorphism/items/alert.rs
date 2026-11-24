mod palette;
mod style;
use super::{Palette, recipes::Alert};
use gpui::{prelude::*, *};
#[derive(Clone, Copy)]
pub(crate) enum Variant {
    Default,
    Success,
    Destructive,
}
pub(crate) fn alert(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    message: impl Into<SharedString>,
    variant: Variant,
    p: Palette,
) -> Alert {
    let border = match variant {
        Variant::Default => p.border,
        Variant::Success => palette::chart_colors(p)[1].opacity(0.45),
        Variant::Destructive => p.error.opacity(0.48),
    };
    Alert::new(id, title, message, p)
        .urgent(true)
        .destructive(matches!(variant, Variant::Destructive))
        .border_1()
        .border_color(border)
        .rounded(p.radius)
        .bg(p.surface)
        .shadow(palette::inset(p))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(alert(
                "version",
                "New version available",
                "Review the release notes before updating.",
                Variant::Default,
                p,
            ))
            .child(alert(
                "synced",
                "Settings synced",
                "Your workspace settings are up to date.",
                Variant::Success,
                p,
            ))
            .child(alert(
                "tokens",
                "Missing tokens",
                "Complete the required theme values.",
                Variant::Destructive,
                p,
            ))
            .into_any_element()
    })
}
