//! The billing team's colors and spacing, shared by its chart and invoice register.

use gpui::{px, rems, rgb};

use super::super::{Appearance, Palette};

pub(super) fn billing_palette() -> Palette {
    Palette {
        canvas: rgb(0xf5f7fa).into(),
        surface: rgb(0xffffff).into(),
        field: rgb(0xf8fafc).into(),
        foreground: rgb(0x17233a).into(),
        muted: rgb(0x53647a).into(),
        border: rgb(0xb9c6d7).into(),
        accent: rgb(0x2858aa).into(),
        accent_foreground: rgb(0xffffff).into(),
        highlight: rgb(0xe6efff).into(),
        padding: rems(1.25),
        gap: rems(0.75),
        control_height: rems(2.),
        radius: px(8.),
        ..Appearance::Settings.palette()
    }
}
