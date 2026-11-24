//! Compose GPUI styles in order. A state or caller can override the properties it owns.
mod palette;

use super::Palette;
use gpui::{prelude::*, *};

pub(crate) fn merge_styles(
    mut base: StyleRefinement,
    overrides: impl IntoIterator<Item = StyleRefinement>,
) -> StyleRefinement {
    for layer in overrides {
        base.refine(&layer);
    }
    base
}

struct StylePreview;

impl Render for StylePreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let p = palette::air();
        let mut surface = div()
            .px_5()
            .py_3()
            .rounded(p.radius)
            .bg(p.surface)
            .text_color(p.foreground)
            .shadow(palette::raised(p));
        let mut accent = div().bg(p.accent).text_color(p.accent_foreground);
        let combined = merge_styles(surface.style().clone(), [accent.style().clone()]);
        let mut sample = div();
        sample.style().refine(&combined);
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .items_center()
            .justify_center()
            .bg(p.canvas)
            .child(surface.child("Base style"))
            .child(sample.child("With accent override"))
    }
}

pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| StylePreview).into()
}
