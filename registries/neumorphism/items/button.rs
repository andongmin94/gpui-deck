//! A tactile action that rises at rest and presses into the surface on activation.
mod button;
mod control_style;
mod palette;
use super::Palette;
pub(crate) use button::{ButtonVariant, press_button_with_palette, style_button};
use gpui::{prelude::*, *};
use gpui_deck::Button;

pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let count = cx.new(|_| 0_u32);
    control_style::sample(count, cx, |p, count, _, cx| {
        let click = count.clone();
        let total = *count.read(cx);
        let label = if total == 0 {
            "Save changes".to_owned()
        } else {
            format!("Saved {total}")
        };
        style_button(Button::new("save"), ButtonVariant::Primary, p)
            .child(label.clone())
            .accessibility_label(label)
            .on_click(move |_, _, cx| {
                click.update(cx, |n, cx| {
                    *n += 1;
                    cx.notify();
                })
            })
    })
}
