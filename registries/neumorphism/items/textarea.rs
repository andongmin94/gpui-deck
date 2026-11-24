//! A recessed multiline editor with label and validation feedback.

#[allow(dead_code)]
mod control_style;
mod palette;

#[allow(dead_code)]
mod editor_style;
pub(crate) use editor_style::style_editor;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Field, Textarea, TextareaState, input::TextareaMode};
pub(crate) fn textarea(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<TextareaState>,
    p: Palette,
    window: &Window,
    cx: &App,
) -> Field<TextareaMode> {
    Field::new(id, state, label)
        .styles(|s| s.label(|s| s.font_weight(FontWeight::SEMIBOLD)))
        .flex()
        .flex_col()
        .gap_2()
        .child(
            editor_style::frame(state, false, p, window, cx)
                .min_h(px(112.))
                .p_3()
                .child(Textarea::new(state)),
        )
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        let mut input =
            TextareaState::new(window, cx).placeholder("Write a short project description…");
        style_editor(&mut input, palette::air());
        input
    });
    control_style::sample(s, cx, |p, s, window, cx| {
        div()
            .w_full()
            .max_w(px(320.))
            .child(textarea("notes", "Project notes", s, p, window, cx))
    })
}
