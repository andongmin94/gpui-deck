//! Associate a label, description and validation feedback with an editable control.
mod control_style;
mod input;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Field, InputState, input::InputMode};
pub(crate) fn field(
    id: &'static str,
    label: &'static str,
    state: &Entity<InputState>,
    error: Option<&'static str>,
    window: &Window,
    cx: &App,
    p: Palette,
) -> Field<InputMode> {
    input::field_with_palette(id, label, state, error, window, cx, p)
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        input::editor_with_palette(
            window,
            cx,
            "studio@example.com",
            Role::EmailInput,
            palette::air(),
        )
    });
    control_style::sample(state, cx, |p, state, window, cx| {
        let value = state.read(cx).value();
        let error = (!value.is_empty() && !value.contains('@')).then_some("Include an @ sign.");
        div().w_full().max_w(px(300.)).child(
            field("email", "Email", state, error, window, cx, p)
                .required(true)
                .description("Used for workspace notifications."),
        )
    })
}
