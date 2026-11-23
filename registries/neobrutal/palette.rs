//! Source-owned Neobrutal colors. For an installed theme, pass its `palette(dark)`
//! to any template or recipe accepting the app's shared `ui::Palette` type.
//! Colors are 24-bit sRGB equivalents of the upstream CSS tokens.

use super::super::{Appearance, Palette};
use gpui::{px, rgb};

pub(crate) const RADIUS: f32 = 5.;
pub(crate) const DEPTH: f32 = 4.;

#[derive(Clone, Copy)]
pub(crate) struct Colors {
    /// Page, card, popover and sidebar background.
    pub(crate) paper: u32,
    pub(crate) ink: u32,
    /// Primary controls and selected items.
    pub(crate) main: u32,
    pub(crate) on_main: u32,
    pub(crate) secondary: u32,
    pub(crate) on_secondary: u32,
    /// Secondary background for raised panels and fields.
    pub(crate) surface: u32,
    pub(crate) border: u32,
    pub(crate) ring: u32,
    pub(crate) destructive: u32,
    pub(crate) error: u32,
    pub(crate) chart: [u32; 5],
}

impl Colors {
    pub(crate) fn palette(self) -> Palette {
        Palette {
            canvas: rgb(self.paper).into(),
            surface: rgb(self.surface).into(),
            field: rgb(self.surface).into(),
            foreground: rgb(self.ink).into(),
            muted: rgb(self.ink).opacity(0.72).into(),
            border: rgb(self.border).into(),
            accent: rgb(self.main).into(),
            accent_foreground: rgb(self.on_main).into(),
            highlight: rgb(self.main).opacity(0.24).into(),
            error: rgb(self.error).into(),
            radius: px(RADIUS),
            ..Appearance::Settings.palette()
        }
    }
}

pub(crate) const YELLOW_LIGHT: Colors = Colors {
    paper: 0xfdf7c4,
    ink: 0x000000,
    main: 0xfacc00,
    on_main: 0x000000,
    secondary: 0x1f1f1f,
    on_secondary: 0xffffff,
    surface: 0xffffff,
    border: 0x000000,
    ring: 0x000000,
    destructive: 0xef4444,
    error: 0x9f0712,
    chart: [0xfacc00, 0x7a83ff, 0xff4d50, 0x00d696, 0x0099ff],
};

pub(crate) const YELLOW_DARK: Colors = Colors {
    paper: 0x322c15,
    ink: 0xe6e6e6,
    main: 0xe0b700,
    on_main: 0x000000,
    secondary: 0xffffff,
    on_secondary: 0x000000,
    surface: 0x1f1f1f,
    border: 0x000000,
    ring: 0xffffff,
    destructive: 0xef4444,
    error: 0xffa2a2,
    chart: [0xe0b700, 0x7a83ff, 0xff6669, 0x00bd84, 0x008ae5],
};

pub(crate) const INK: u32 = YELLOW_LIGHT.ink;
pub(crate) const ACCENT: u32 = YELLOW_LIGHT.main;
pub(crate) const PAPER: u32 = YELLOW_LIGHT.paper;

pub(crate) fn yellow() -> Palette {
    YELLOW_LIGHT.palette()
}
