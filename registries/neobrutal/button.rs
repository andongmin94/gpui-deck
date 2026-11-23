use gpui::{
    ElementId, FontWeight, InteractiveElement as _, ParentElement as _, SharedString,
    StatefulInteractiveElement as _, Styled as _, px,
};
use gpui_deck::{Button, box_shadow};

use super::{super::Palette, palette::yellow};

/// A raised action. Attach a click handler or disable it through Deck's Button API.
pub(crate) fn press_button(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
    press_button_with_palette(id, label, yellow())
}

pub(crate) fn press_button_with_palette(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: Palette,
) -> Button {
    let label = label.into();
    raised(palette, Button::new(id).accessibility_label(label.clone())).child(label)
}

/// Apply the same press and focus treatment to a trigger owned by a headless control.
pub(crate) fn raised(palette: Palette, button: Button) -> Button {
    let ink = palette.border;
    button
        .relative()
        .h(px(42.))
        .px_4()
        .rounded(palette.radius)
        .border_2()
        .border_color(ink)
        .bg(palette.accent)
        .text_color(palette.accent_foreground)
        .text_size(px(15.))
        .font_weight(FontWeight::BOLD)
        .cursor_pointer()
        .shadow(vec![box_shadow(4., 4., 0., 0., ink)])
        .focus_visible(|s| {
            s.bg(palette.canvas)
                .text_color(palette.foreground)
                .underline()
        })
        .hover(|s| {
            s.top(px(2.))
                .left(px(2.))
                .shadow(vec![box_shadow(2., 2., 0., 0., ink)])
        })
        .active(|s| s.top(px(4.)).left(px(4.)).shadow_none())
        .styles(|s| s.disabled(|s| s.opacity(0.5).cursor_default()))
}
