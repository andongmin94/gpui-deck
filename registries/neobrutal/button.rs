use gpui::{
    ElementId, FontWeight, InteractiveElement as _, ParentElement as _, SharedString,
    StatefulInteractiveElement as _, Styled as _, px, rgb,
};
use gpui_deck::{Button, box_shadow};

use super::palette::{ACCENT, INK, PAPER, RADIUS};

/// A raised action. Attach a click handler or disable it through Deck's Button API.
pub(crate) fn press_button(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
    let label = label.into();
    let ink = rgb(INK);
    Button::new(id)
        .accessibility_label(label.clone())
        .relative()
        .h(px(42.))
        .px_4()
        .rounded(px(RADIUS))
        .border_2()
        .border_color(ink)
        .bg(rgb(ACCENT))
        .text_color(ink)
        .text_size(px(15.))
        .font_weight(FontWeight::BOLD)
        .cursor_pointer()
        .shadow(vec![box_shadow(4., 4., 0., 0., ink.into())])
        .focus_visible(|s| s.bg(rgb(PAPER)).underline())
        .hover(|s| {
            s.top(px(2.))
                .left(px(2.))
                .shadow(vec![box_shadow(2., 2., 0., 0., ink.into())])
        })
        .active(|s| s.top(px(4.)).left(px(4.)).shadow_none())
        .styles(|s| s.disabled(|s| s.opacity(0.5).cursor_default()))
        .child(label)
}
