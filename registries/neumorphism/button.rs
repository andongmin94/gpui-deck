use super::{super::Palette, palette};
use gpui::{prelude::*, *};
use gpui_deck::Button;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Soft,
    Ghost,
    Destructive,
}

/// Apply the same raised, pressed and focused treatment to a headless trigger.
pub(crate) fn raised(p: Palette, button: Button) -> Button {
    style_button(button, ButtonVariant::Default, p)
}

pub(crate) fn style_button(button: Button, variant: ButtonVariant, p: Palette) -> Button {
    let elevated = matches!(
        variant,
        ButtonVariant::Default | ButtonVariant::Primary | ButtonVariant::Soft
    );
    let ink = match variant {
        ButtonVariant::Primary => palette::accent_ink(p),
        ButtonVariant::Destructive => palette::error_ink(p),
        _ => p.foreground,
    };
    button
        .h(px(40.))
        .px_4()
        .flex()
        .items_center()
        .justify_center()
        .rounded(palette::control_radius(p))
        .border_1()
        .border_color(transparent_black())
        .text_size(px(14.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(ink)
        .cursor_pointer()
        .when(elevated, |s| {
            s.bg(if variant == ButtonVariant::Soft {
                p.field
            } else {
                p.surface
            })
            .shadow(palette::raised(p))
        })
        .hover(|s| s.bg(p.surface).shadow(palette::raised(p)))
        .active(|s| s.bg(p.field).shadow(palette::inset(p)))
        .focus_visible(|s| s.border_color(p.accent).underline())
        .styles(|s| s.disabled(|s| s.opacity(0.5).shadow_none().cursor_default()))
}

pub(crate) fn press_button_with_palette(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    p: Palette,
) -> Button {
    let label = label.into();
    raised(p, Button::new(id).accessibility_label(label.clone())).child(label)
}
