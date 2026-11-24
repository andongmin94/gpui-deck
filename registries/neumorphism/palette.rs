//! Neumorphism UI's five source-owned presets. The light source and shadow depths
//! follow the upstream registry's theme.ts. The app owns these colors and controls.
#![allow(dead_code)] // An independently installed item uses only its own treatments.
use super::super::{Appearance, Palette};
use gpui::{BoxShadow, Div, Hsla, Styled as _, div, px, rgb, rgba};
use gpui_deck::box_shadow;

#[derive(Clone, Copy)]
struct Tokens {
    background: u32,
    surface: u32,
    low: u32,
    ink: u32,
    muted: u32,
    primary: u32,
    on_primary: u32,
    selected: u32,
    border: u32,
    error: u32,
    shade: u32,
    light: u32,
}
#[derive(Clone, Copy)]
pub(crate) struct Preset {
    pub(crate) name: &'static str,
    light: Tokens,
    dark: Tokens,
    radius: f32,
    control_radius: f32,
    dx: f32,
    dy: f32,
    blur: f32,
}
impl Preset {
    pub(crate) fn palette(self, dark: bool) -> Palette {
        let t = if dark { self.dark } else { self.light };
        Palette {
            canvas: rgb(t.background).into(),
            surface: rgb(t.surface).into(),
            field: rgb(t.low).into(),
            foreground: rgb(t.ink).into(),
            muted: rgb(t.muted).into(),
            accent: rgb(t.primary).into(),
            accent_foreground: rgb(t.on_primary).into(),
            border: rgba(t.border).into(),
            highlight: rgb(t.selected).into(),
            error: rgb(t.error).into(),
            radius: px(self.radius),
            ..Appearance::Settings.palette()
        }
    }
}
pub(crate) const AIR: Preset = Preset {
    name: "Air",
    light: Tokens {
        background: 0xebecf0,
        surface: 0xebecf0,
        low: 0xdfe1e7,
        ink: 0x25264b,
        muted: 0x586079,
        primary: 0x0768da,
        on_primary: 0xffffff,
        selected: 0xd9e1ee,
        border: 0x53637b2e,
        error: 0xb92e43,
        shade: 0x42485e38,
        light: 0xfffffff0,
    },
    dark: Tokens {
        background: 0x242731,
        surface: 0x242731,
        low: 0x1c1f28,
        ink: 0xe9f0fa,
        muted: 0xaab4c8,
        primary: 0x237bff,
        on_primary: 0x07111f,
        selected: 0x242e41,
        border: 0xffffff1a,
        error: 0xff7685,
        shade: 0x00000057,
        light: 0xffffff11,
    },
    radius: 12.,
    control_radius: 8.,
    dx: 6.,
    dy: 6.,
    blur: 14.,
};

pub(crate) const LAVENDER: Preset = Preset {
    name: "Lavender",
    light: Tokens {
        background: 0xebe9f4,
        surface: 0xebe9f4,
        low: 0xddd9e9,
        ink: 0x252036,
        muted: 0x625970,
        primary: 0x7557df,
        on_primary: 0xffffff,
        selected: 0xe2ddf2,
        border: 0x6f668038,
        error: 0xcf5368,
        shade: 0x2d204c29,
        light: 0xfffffff5,
    },
    dark: Tokens {
        background: 0x171520,
        surface: 0x1e1a2a,
        low: 0x131119,
        ink: 0xf1edff,
        muted: 0xaaa0bd,
        primary: 0xa98cff,
        on_primary: 0x171022,
        selected: 0x29233b,
        border: 0xffffff1a,
        error: 0xff7891,
        shade: 0x00000070,
        light: 0xffffff0b,
    },
    radius: 26.,
    control_radius: 999.,
    dx: 3.,
    dy: 3.,
    blur: 8.,
};

pub(crate) const SAGE: Preset = Preset {
    name: "Sage",
    light: Tokens {
        background: 0xe7eee9,
        surface: 0xe7eee9,
        low: 0xd8e3db,
        ink: 0x1a2b24,
        muted: 0x50665b,
        primary: 0x23856b,
        on_primary: 0xffffff,
        selected: 0xd7e6df,
        border: 0x5f746a38,
        error: 0xcf5962,
        shade: 0x1a382b26,
        light: 0xfffffff0,
    },
    dark: Tokens {
        background: 0x121b17,
        surface: 0x19251f,
        low: 0x101713,
        ink: 0xe9f5ee,
        muted: 0x91aa9d,
        primary: 0x62c8a7,
        on_primary: 0x071710,
        selected: 0x1f322a,
        border: 0xffffff1a,
        error: 0xff7b82,
        shade: 0x00000070,
        light: 0xffffff0a,
    },
    radius: 18.,
    control_radius: 8.,
    dx: -3.,
    dy: 3.,
    blur: 8.,
};

pub(crate) const CLAY: Preset = Preset {
    name: "Clay",
    light: Tokens {
        background: 0xefe8df,
        surface: 0xefe8df,
        low: 0xe2d7cb,
        ink: 0x35271f,
        muted: 0x6a584d,
        primary: 0xd46b4c,
        on_primary: 0x2b140d,
        selected: 0xedded3,
        border: 0x806e6238,
        error: 0xc84851,
        shade: 0x4c322129,
        light: 0xffffffe6,
    },
    dark: Tokens {
        background: 0x211814,
        surface: 0x2a1f19,
        low: 0x18120f,
        ink: 0xf8eee6,
        muted: 0xb8a093,
        primary: 0xf08b6b,
        on_primary: 0x21110b,
        selected: 0x3a2820,
        border: 0xffffff1a,
        error: 0xff7378,
        shade: 0x00000075,
        light: 0xffffff0a,
    },
    radius: 14.,
    control_radius: 6.,
    dx: 6.,
    dy: 6.,
    blur: 14.,
};

pub(crate) const GRAPHITE: Preset = Preset {
    name: "Graphite",
    light: Tokens {
        background: 0xe6e8eb,
        surface: 0xe6e8eb,
        low: 0xd5d9de,
        ink: 0x1d232b,
        muted: 0x55606b,
        primary: 0x3f536f,
        on_primary: 0xffffff,
        selected: 0xd9dce1,
        border: 0x626b7638,
        error: 0xc8545f,
        shade: 0x1f27322b,
        light: 0xfffffff0,
    },
    dark: Tokens {
        background: 0x111419,
        surface: 0x181c22,
        low: 0x0d1014,
        ink: 0xedf1f6,
        muted: 0x929ca9,
        primary: 0xb2c2d8,
        on_primary: 0x11161d,
        selected: 0x242931,
        border: 0xffffff1a,
        error: 0xff7885,
        shade: 0x0000007a,
        light: 0xffffff0b,
    },
    radius: 8.,
    control_radius: 6.,
    dx: 8.,
    dy: 8.,
    blur: 18.,
};

pub(crate) fn air() -> Palette {
    AIR.palette(false)
}

fn lighting(p: Palette) -> (Preset, Tokens) {
    for preset in [AIR, LAVENDER, SAGE, CLAY, GRAPHITE] {
        for t in [preset.light, preset.dark] {
            if p.surface == Hsla::from(rgb(t.surface)) {
                return (preset, t);
            }
        }
    }
    (
        AIR,
        if p.surface.l < 0.5 {
            AIR.dark
        } else {
            AIR.light
        },
    )
}

pub(crate) fn control_radius(p: Palette) -> gpui::Pixels {
    px(lighting(p).0.control_radius)
}

pub(crate) fn raised(p: Palette) -> Vec<BoxShadow> {
    let (s, t) = lighting(p);
    let dark = p.surface.l < 0.5;
    vec![
        box_shadow(
            -s.dx.signum(),
            -s.dy.signum(),
            0.,
            0.,
            if dark {
                p.foreground.opacity(0.10)
            } else {
                Hsla::from(rgba(t.light)).opacity(0.82)
            },
        ),
        box_shadow(
            s.dx,
            s.dy,
            s.blur,
            0.,
            Hsla::from(rgba(t.shade)).opacity(if dark { 1. } else { 0.92 }),
        ),
        box_shadow(
            -s.dx,
            -s.dy,
            s.blur,
            0.,
            Hsla::from(rgba(t.light)).opacity(if dark { 0.94 } else { 0.74 }),
        ),
    ]
}

pub(crate) fn inset(p: Palette) -> Vec<BoxShadow> {
    let (s, t) = lighting(p);
    let x = s.dx.signum() * 2.;
    let y = s.dy.signum() * 2.;
    vec![
        BoxShadow {
            inset: true,
            ..box_shadow(x, y, 5., 0., rgba(t.shade).into())
        },
        BoxShadow {
            inset: true,
            ..box_shadow(-x, -y, 5., 0., Hsla::from(rgba(t.light)).opacity(0.86))
        },
    ]
}

pub(crate) fn well(p: Palette) -> Div {
    div()
        .bg(p.field)
        .rounded(control_radius(p))
        .shadow(inset(p))
}

pub(crate) fn chart_colors(p: Palette) -> [Hsla; 5] {
    let dark = p.surface.l < 0.5;
    [
        p.accent,
        rgb(if dark { 0x40d6ad } else { 0x08775e }).into(),
        rgb(if dark { 0xbba7fc } else { 0x775ab9 }).into(),
        rgb(if dark { 0xf5bd79 } else { 0x99591c }).into(),
        p.error,
    ]
}

fn luminance(color: gpui::Rgba) -> f32 {
    fn linear(channel: f32) -> f32 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
}

/// Upstream's readable accent transitions toward the text color in 5% steps.
pub(crate) fn accent_ink(p: Palette) -> Hsla {
    if p.surface == Hsla::from(rgb(AIR.dark.surface))
        && p.accent == Hsla::from(rgb(AIR.dark.primary))
    {
        return rgb(0x68a8ff).into();
    }
    let background = luminance(p.surface.to_rgb());
    for step in 0..=20 {
        let mixed = p
            .accent
            .to_rgb()
            .blend(p.foreground.to_rgb().alpha(step as f32 / 20.));
        let round = |value: f32| (value * 255.).round() / 255.;
        let color = gpui::Rgba {
            r: round(mixed.r),
            g: round(mixed.g),
            b: round(mixed.b),
            a: 1.,
        };
        let light = luminance(color);
        if (background.max(light) + 0.05) / (background.min(light) + 0.05) >= 4.5 {
            return color.into();
        }
    }
    p.foreground
}
pub(crate) fn error_ink(p: Palette) -> Hsla {
    if p.surface.l < 0.5 {
        p.error
    } else {
        p.error
            .to_rgb()
            .blend(p.foreground.to_rgb().alpha(0.24))
            .into()
    }
}
