//! input-group in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
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
        .rounded(p.radius)
        .border_2()
        .border_color(p.border)
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        let mut s = InputState::new(window, cx).placeholder("workspace");
        style_editor(&mut s, palette::yellow());
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
