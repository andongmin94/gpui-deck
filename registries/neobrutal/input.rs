use gpui::{
    App, Context, Entity, Focusable as _, FontWeight, InteractiveElement as _, MouseButton,
    ParentElement as _, Role, Styled as _, Window, div, prelude::FluentBuilder as _, px, rgb,
    white,
};
use gpui_deck::{
    Field, Input, InputAccessibility, InputEditorStyle, InputState, box_shadow, input::InputMode,
};

use super::palette::{ACCENT, INK, PAPER, RADIUS};

/// Create this state once per field. Passwords stay masked and are omitted from accessibility values.
pub(crate) fn editor(
    window: &mut Window,
    cx: &mut Context<InputState>,
    placeholder: &'static str,
    role: Role,
) -> InputState {
    let secret = role == Role::PasswordInput;
    let mut state = InputState::new(window, cx)
        .placeholder(placeholder)
        .masked(secret);
    state.set_accessibility(InputAccessibility::new(Some(role)).with_value_exposed(!secret));
    state.set_editor_style(InputEditorStyle {
        foreground: rgb(INK).into(),
        muted_foreground: rgb(0x605f54).into(),
        selection: rgb(ACCENT).into(),
        caret: rgb(INK).into(),
        ..Default::default()
    });
    state
}

/// A labeled, editable field. The caller owns its value, requirement and validation.
pub(crate) fn field(
    id: &'static str,
    label: &'static str,
    state: &Entity<InputState>,
    error: Option<&'static str>,
    window: &Window,
    cx: &App,
) -> Field<InputMode> {
    let invalid = rgb(0x9f2021);
    let stroke = if error.is_some() { invalid } else { rgb(INK) };
    let focus = state.focus_handle(cx);
    let disabled = state.read(cx).presentation().is_disabled();
    let frame = div()
        .w_full()
        .min_w_0()
        .h(px(42.))
        .flex()
        .items_center()
        .px_3()
        .rounded(px(RADIUS))
        .border_2()
        .border_color(stroke)
        .bg(white())
        .text_size(px(15.))
        .when(focus.is_focused(window), |s| {
            s.shadow(vec![
                box_shadow(0., 0., 0., 4., stroke.into()),
                box_shadow(0., 0., 0., 2., white()),
            ])
        })
        .when(disabled, |s| s.bg(rgb(PAPER)).opacity(0.5))
        .when(!disabled, |s| {
            s.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                focus.focus(window, cx)
            })
        })
        .child(div().min_w_0().flex_1().child(Input::new(state)));
    Field::new(id, state, label)
        .error(error.map(Into::into))
        .styles(|s| {
            s.label(|s| s.text_size(px(14.)).font_weight(FontWeight::BOLD))
                .error(|s| s.text_size(px(13.)).text_color(invalid))
        })
        .flex()
        .flex_col()
        .gap(px(8.))
        .text_color(rgb(INK))
        .child(frame)
}
