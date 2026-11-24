//! A pressable two-state action. Pass the resulting value to your application state.
mod control_style;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::Toggle;
pub(crate) fn toggle(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    pressed: bool,
    p: Palette,
) -> Toggle {
    let label = label.into();
    Toggle::new(id)
        .accessibility_label(label.clone())
        .pressed(pressed)
        .h(px(40.))
        .px_4()
        .flex()
        .items_center()
        .justify_center()
        .rounded(palette::control_radius(p))
        .bg(if pressed { p.field } else { p.surface })
        .shadow(if pressed {
            palette::inset(p)
        } else {
            palette::raised(p)
        })
        .text_color(if pressed {
            palette::accent_ink(p)
        } else {
            p.foreground
        })
        .font_weight(FontWeight::SEMIBOLD)
        .cursor_pointer()
        .focus_visible(|s| s.underline())
        .child(label)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|_| false);
    control_style::sample(state, cx, |p, state, _, cx| {
        let owner = state.clone();
        toggle("bold", "Bold", *state.read(cx), p).on_change(move |next, _, _, cx| {
            owner.update(cx, |v, cx| {
                *v = next;
                cx.notify();
            })
        })
    })
}
