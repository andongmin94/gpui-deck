//! Shared panel and heading styles for the billing team's screens.

use gpui::{
    Div, FontWeight, InteractiveElement as _, IntoElement, ParentElement as _, Role,
    StatefulInteractiveElement as _, Styled as _, div, px,
};

use super::super::Palette;

pub(super) fn heading(id: &'static str, title: &'static str, level: usize) -> impl IntoElement {
    div()
        .id(id)
        .role(Role::Heading)
        .aria_level(level)
        .aria_label(title)
        .font_weight(FontWeight::SEMIBOLD)
        .text_size(px(if level == 1 { 28. } else { 18. }))
        .child(title)
}

pub(super) fn panel(id: &'static str, title: &'static str, palette: Palette) -> Div {
    div()
        .min_w_0()
        .w_full()
        .flex()
        .flex_col()
        .gap(palette.gap)
        .p(palette.padding)
        .bg(palette.surface)
        .border_1()
        .border_color(palette.border)
        .rounded(palette.radius)
        .child(heading(id, title, 2))
}
