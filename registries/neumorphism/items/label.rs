//! label in the application's Neumorphism palette.

#[allow(dead_code)]
mod control_style;
mod palette;

#[allow(dead_code)]
mod editor_style;
pub(crate) use editor_style::style_editor;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Input, InputAccessibility, InputState, Label};
pub(crate) fn label(
    id: impl Into<ElementId>,
    focus: &FocusHandle,
    text: impl Into<SharedString>,
    p: Palette,
) -> Label {
    Label::new(id, focus, text)
        .text_color(p.foreground)
        .font_weight(FontWeight::SEMIBOLD)
        .text_size(px(14.))
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        let mut s = InputState::new(window, cx).placeholder("name@company.com");
        style_editor(&mut s, palette::air());
        s.set_accessibility(
            InputAccessibility::new(Some(Role::TextInput)).with_label(Some("Email address".into())),
        );
        s
    });
    control_style::sample(s, cx, |p, s, window, cx| {
        div()
            .w(px(240.))
            .flex()
            .flex_col()
            .gap_2()
            .child(label("name-label", &s.focus_handle(cx), "Email address", p))
            .child(editor_style::frame(s, false, p, window, cx).child(Input::new(s)))
    })
}
