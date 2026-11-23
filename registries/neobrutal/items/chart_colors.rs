//! ChartRecipe uses each series color for both the plot and its legend text.
use gpui::{Hsla, rgb};

use super::Palette;

fn luminance(color: Hsla) -> f32 {
    let color = color.to_rgb();
    let linear = |c: f32| {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
}

/// Keep the hue as close to the palette as the legend's 4.5:1 contrast permits.
fn series_ink(color: Hsla, p: Palette) -> Hsla {
    let background = luminance(p.surface);
    for step in 0..=10 {
        let candidate = color.blend(p.foreground.opacity(step as f32 / 10.));
        let ink = luminance(candidate);
        if (ink.max(background) + 0.05) / (ink.min(background) + 0.05) >= 4.5 {
            return candidate;
        }
    }
    p.foreground
}

/// The accent and secondary ink colors remain distinguishable on the chosen surface.
pub(super) fn series_colors(p: Palette) -> [Hsla; 5] {
    [
        p.accent,
        p.foreground,
        rgb(0x7a83ff).into(),
        rgb(0x00a773).into(),
        rgb(0xff4d50).into(),
    ]
    .map(|color| series_ink(color, p))
}
