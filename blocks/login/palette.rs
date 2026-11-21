use gpui::{px, rems, rgb};

use super::super::{Appearance, Palette};

pub(super) fn login_palette() -> Palette {
    Palette {
        canvas: rgb(0xf3f2f8).into(),
        surface: rgb(0xffffff).into(),
        field: rgb(0xffffff).into(),
        foreground: rgb(0x211f2c).into(),
        muted: rgb(0x645e70).into(),
        border: rgb(0xd7d3e1).into(),
        accent: rgb(0x55418b).into(),
        accent_foreground: rgb(0xffffff).into(),
        highlight: rgb(0xebe6f6).into(),
        control_height: rems(2.75),
        font_size: rems(0.9375),
        radius: px(8.),
        ..Appearance::Settings.palette()
    }
}
