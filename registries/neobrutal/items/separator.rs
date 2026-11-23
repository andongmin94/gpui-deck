mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{Axis, ElementId, px};
use gpui_deck::Separator;
pub(crate) fn separator(id: impl Into<ElementId>, axis: Axis, p: Palette) -> Separator {
    let rule = Separator::new(id).axis(axis).bg(p.border);
    match axis {
        Axis::Horizontal => rule.h(px(2.)).w_full(),
        Axis::Vertical => rule.w(px(2.)).h_full(),
    }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        style::surface(p)
            .flex()
            .flex_col()
            .gap_3()
            .child("Project activity")
            .child(separator("divider", Axis::Horizontal, p))
            .child("Updated today")
            .into_any_element()
    })
}
