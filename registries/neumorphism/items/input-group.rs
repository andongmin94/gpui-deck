//! Attach leading or trailing content to an editable field.

#[allow(dead_code)]
mod control_style;
mod palette;

#[allow(dead_code)]
mod editor_style;
pub(crate) use editor_style::style_editor;

use super::Palette;
use super::recipes::InputGroup;
use gpui::{prelude::*, *};
use gpui_deck::{Input, InputAccessibility, InputState};
pub(crate) fn input_group(
    id: impl Into<ElementId>,
    state: &Entity<InputState>,
    p: Palette,
) -> InputGroup {
    InputGroup::new(id, Input::new(state), p)
        .rounded(palette::control_radius(p))
        .border_1()
        .border_color(p.border)
        .shadow(palette::inset(p))
        .bg(p.field)
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        let mut s = InputState::new(window, cx).placeholder("workspace");
        style_editor(&mut s, palette::air());
        s.set_accessibility(
            InputAccessibility::new(Some(Role::TextInput))
                .with_label(Some("Workspace handle".into())),
        );
        s
    });
    control_style::sample(state, cx, |p, s, window, cx| {
        input_group("handle", s, p)
            .focused(s.focus_handle(cx).is_focused(window))
            .w(px(260.))
            .leading("@")
    })
}
