//! Match the actual GPUI text, caret and selection to the application palette.
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{
    InputEditorStyle, box_shadow,
    input::{InputBaseState, InputModeKind},
};

pub(crate) fn style_editor<M: InputModeKind>(state: &mut InputBaseState<M>, p: Palette) {
    state.set_editor_style(InputEditorStyle {
        foreground: p.foreground,
        muted_foreground: p.muted,
        caret: p.foreground,
        selection: p.highlight,
        ..Default::default()
    });
}

pub(super) fn frame<M: InputModeKind>(
    state: &Entity<InputBaseState<M>>,
    invalid: bool,
    p: Palette,
    window: &Window,
    cx: &App,
) -> Div {
    let edge = if invalid { p.error } else { p.border };
    let focus = state.focus_handle(cx);
    let disabled = state.read(cx).presentation().is_disabled();
    div()
        .w_full()
        .p_2()
        .border_2()
        .border_color(edge)
        .rounded(p.radius)
        .bg(p.field)
        .when(disabled, |s| s.opacity(0.5))
        .when(focus.is_focused(window), |s| {
            s.shadow(vec![
                box_shadow(0., 0., 0., 4., p.foreground),
                box_shadow(0., 0., 0., 2., p.surface),
            ])
        })
        .when(!disabled, |s| {
            s.on_mouse_down(MouseButton::Left, move |_, w, cx| focus.focus(w, cx))
        })
}
