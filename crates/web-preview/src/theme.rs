//! The neutral palette of the documentation site, in light and dark.

use gpui::{Hsla, rgb};

/// Colors matching the site's neutral theme so previews sit naturally in it.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub background: Hsla,
    pub foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub border: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub accent: Hsla,
    pub ring: Hsla,
    pub destructive: Hsla,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            background: rgb(0xffffff).into(),
            foreground: rgb(0x0a0a0a).into(),
            muted: rgb(0xf5f5f5).into(),
            muted_foreground: rgb(0x737373).into(),
            border: rgb(0xe5e5e5).into(),
            primary: rgb(0x171717).into(),
            primary_foreground: rgb(0xfafafa).into(),
            accent: rgb(0xf5f5f5).into(),
            ring: rgb(0xa3a3a3).into(),
            destructive: rgb(0xdc2626).into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: rgb(0x0a0a0a).into(),
            foreground: rgb(0xfafafa).into(),
            muted: rgb(0x262626).into(),
            muted_foreground: rgb(0xa3a3a3).into(),
            border: rgb(0x2e2e2e).into(),
            primary: rgb(0xe5e5e5).into(),
            primary_foreground: rgb(0x171717).into(),
            accent: rgb(0x262626).into(),
            ring: rgb(0x737373).into(),
            destructive: rgb(0xf87171).into(),
        }
    }
}
