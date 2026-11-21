//! Workspace surfaces, navigation colors and control sizes.

use gpui::{px, rems, rgb};

use super::super::{Appearance, Palette};

pub(super) fn workspace_palette() -> Palette {
    Palette {
        canvas: rgb(0xf1f2ee).into(),
        surface: rgb(0xffffff).into(),
        field: rgb(0xfafbf8).into(),
        foreground: rgb(0x222c27).into(),
        muted: rgb(0x536157).into(),
        border: rgb(0xced5cc).into(),
        accent: rgb(0x2a5845).into(),
        accent_foreground: rgb(0xffffff).into(),
        highlight: rgb(0xe2eee3).into(),
        radius: px(8.),
        font_size: rems(0.9375),
        control_height: rems(2.625),
        ..Appearance::Settings.palette()
    }
}

pub(super) fn sidebar_palette(base: Palette) -> Palette {
    Palette {
        surface: rgb(0x203a30).into(),
        foreground: rgb(0xf2f6ef).into(),
        muted: rgb(0xbccdbf).into(),
        border: rgb(0x647d6c).into(),
        highlight: rgb(0x375846).into(),
        accent: rgb(0xd2eba8).into(),
        ..base
    }
}
