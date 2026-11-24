//! A recessed text input. The caller owns its value and label.
mod control_style;
mod input;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
pub(crate) use input::{editor_with_palette, field_with_palette};
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let value = cx.new(|cx| {
        editor_with_palette(
            window,
            cx,
            "Studio workspace",
            Role::TextInput,
            palette::air(),
        )
    });
    control_style::sample(value, cx, |p, value, window, cx| {
        div().w_full().max_w(px(300.)).child(field_with_palette(
            "workspace",
            "Workspace",
            value,
            None,
            window,
            cx,
            p,
        ))
    })
}
