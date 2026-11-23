//! input-otp in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{InputOtp, InputOtpState};
pub(crate) fn input_otp(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<InputOtpState>,
    p: Palette,
) -> InputOtp {
    InputOtp::new(id, state)
        .accessibility_label(label)
        .flex()
        .gap_1()
        .slot(move |slot, _, _| {
            let active = slot.active || slot.selected;
            div()
                .size(px(36.))
                .rounded(p.radius)
                .border_2()
                .border_color(p.border)
                .bg(if active { p.accent } else { p.field })
                .text_color(if active {
                    p.accent_foreground
                } else {
                    p.foreground
                })
                .flex()
                .items_center()
                .justify_center()
                .font_weight(FontWeight::BOLD)
                .child(match slot.character {
                    Some(ch) => ch.to_string(),
                    None if slot.active => "│".into(),
                    None => String::new(),
                })
        })
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| InputOtpState::new(6, window, cx));
    control_style::sample(state, cx, |p, s, _, _| {
        input_otp("code", "Six-digit verification code", s, p)
    })
}
