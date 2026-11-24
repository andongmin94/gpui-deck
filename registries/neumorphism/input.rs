use gpui::{
    App, Context, Entity, Focusable as _, FontWeight, InteractiveElement as _, MouseButton,
    ParentElement as _, Role, Styled as _, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_deck::{Field, Input, InputAccessibility, InputEditorStyle, InputState, input::InputMode};

use super::{super::Palette, palette::air};

/// Create this state once per field. Passwords stay masked and are omitted from accessibility values.
pub(crate) fn editor(
    window: &mut Window,
    cx: &mut Context<InputState>,
    placeholder: &'static str,
    role: Role,
) -> InputState {
    editor_with_palette(window, cx, placeholder, role, air())
}

pub(crate) fn editor_with_palette(
    window: &mut Window,
    cx: &mut Context<InputState>,
    placeholder: &'static str,
    role: Role,
    palette: Palette,
) -> InputState {
    let secret = role == Role::PasswordInput;
    let mut state = InputState::new(window, cx)
        .placeholder(placeholder)
        .masked(secret);
    state.set_accessibility(InputAccessibility::new(Some(role)).with_value_exposed(!secret));
    state.set_editor_style(InputEditorStyle {
        foreground: palette.foreground,
        muted_foreground: palette.muted,
        selection: palette.highlight,
        caret: palette.foreground,
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
    field_with_palette(id, label, state, error, window, cx, air())
}

pub(crate) fn field_with_palette(
    id: &'static str,
    label: &'static str,
    state: &Entity<InputState>,
    error: Option<&'static str>,
    window: &Window,
    cx: &App,
    palette: Palette,
) -> Field<InputMode> {
    let invalid = super::palette::error_ink(palette);
    let stroke = if error.is_some() {
        invalid
    } else {
        palette.border
    };
    let focus = state.focus_handle(cx);
    let disabled = state.read(cx).presentation().is_disabled();
    let frame = super::palette::well(palette)
        .w_full()
        .min_w_0()
        .h(px(42.))
        .flex()
        .items_center()
        .px_3()
        .rounded(palette.radius)
        .border_1()
        .border_color(stroke)
        .bg(palette.field)
        .text_size(px(15.))
        .when(focus.is_focused(window), |s| {
            s.border_color(if error.is_some() {
                palette.error
            } else {
                palette.accent
            })
        })
        .when(disabled, |s| s.bg(palette.canvas).opacity(0.5))
        .when(!disabled, |s| {
            s.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                focus.focus(window, cx)
            })
        })
        .child(div().min_w_0().flex_1().child(Input::new(state)));
    Field::new(id, state, label)
        .error(error.map(Into::into))
        .styles(|s| {
            s.label(|s| s.text_size(px(14.)).font_weight(FontWeight::SEMIBOLD))
                .error(|s| s.text_size(px(13.)).text_color(invalid))
        })
        .flex()
        .flex_col()
        .gap(px(8.))
        .text_color(palette.foreground)
        .child(frame)
}
