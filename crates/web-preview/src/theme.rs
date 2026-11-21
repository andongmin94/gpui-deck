//! Documentation preview surfaces and accent colors in light and dark modes.

use crate::{Appearance, Palette};
use gpui::{App, Global, Hsla, rgb};

/// Colors matching the site's neutral theme so previews sit naturally in it.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    appearance: Appearance,
    pub background: Hsla,
    pub foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub border: Hsla,
    pub control_border: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub accent: Hsla,
    pub ring: Hsla,
    pub destructive: Hsla,
}

impl Global for Theme {}

impl Theme {
    pub fn light() -> Self {
        Self {
            appearance: Appearance::Settings,
            background: rgb(0xffffff).into(),
            foreground: rgb(0x0a0a0a).into(),
            muted: rgb(0xf5f5f5).into(),
            muted_foreground: rgb(0x737373).into(),
            border: rgb(0xe5e5e5).into(),
            control_border: rgb(0xa3a3a3).into(),
            primary: rgb(0x404040).into(),
            primary_foreground: rgb(0xfafafa).into(),
            accent: rgb(0xf5f5f5).into(),
            ring: rgb(0x404040).into(),
            destructive: rgb(0xdc2626).into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            appearance: Appearance::Compact,
            background: rgb(0x0a0a0a).into(),
            foreground: rgb(0xfafafa).into(),
            muted: rgb(0x262626).into(),
            muted_foreground: rgb(0xa3a3a3).into(),
            border: rgb(0x2e2e2e).into(),
            control_border: rgb(0x737373).into(),
            primary: rgb(0xd4d4d4).into(),
            primary_foreground: rgb(0x171717).into(),
            accent: rgb(0x262626).into(),
            ring: rgb(0xd4d4d4).into(),
            destructive: rgb(0xf87171).into(),
        }
    }

    pub(crate) fn resolve(self, cx: &App) -> Self {
        cx.try_global::<Self>().copied().unwrap_or(self)
    }

    pub fn from_site(dark: bool, name: Option<&str>) -> Self {
        let mut theme = if dark { Self::dark() } else { Self::light() };
        let (light, dark) = match name {
            Some("blue") => (0x1d4ed8, 0x93c5fd),
            Some("green") => (0x15765d, 0x75d7b7),
            Some("violet") => (0x6d28d9, 0xc4b5fd),
            Some("rose") => (0xbe123c, 0xfda4af),
            _ => return theme,
        };
        theme.primary = rgb(match theme.appearance {
            Appearance::Settings => light,
            Appearance::Compact => dark,
        })
        .into();
        theme.ring = theme.primary;
        theme.accent = theme.background.blend(theme.primary.opacity(0.12));
        theme
    }

    pub(crate) fn recipe_palette(self) -> Palette {
        Palette {
            canvas: self.muted,
            surface: self.background,
            field: self.background,
            foreground: self.foreground,
            muted: self.muted_foreground,
            border: self.border,
            accent: self.primary,
            accent_foreground: self.primary_foreground,
            highlight: self.accent,
            error: self.destructive,
            #[cfg(target_family = "wasm")]
            code_font_family: crate::app_fonts::FAMILY,
            ..self.appearance.palette()
        }
    }
}
