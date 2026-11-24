//! Group related native fields under a visible, accessible legend.
mod control_style;
mod input;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
pub(crate) fn fieldset(
    id: impl Into<ElementId>,
    legend: impl Into<SharedString>,
    p: Palette,
) -> Stateful<Div> {
    let legend = legend.into();
    control_style::panel(p)
        .id(id)
        .role(Role::Group)
        .aria_label(legend.clone())
        .p_5()
        .gap_3()
        .child(div().font_weight(FontWeight::SEMIBOLD).child(legend))
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let name = cx.new(|cx| {
        input::editor_with_palette(window, cx, "Full name", Role::TextInput, palette::air())
    });
    let email = cx.new(|cx| {
        input::editor_with_palette(
            window,
            cx,
            "studio@example.com",
            Role::EmailInput,
            palette::air(),
        )
    });
    control_style::sample(name, cx, move |p, name, window, cx| {
        fieldset("contact", "Contact details", p)
            .w_full()
            .max_w(px(330.))
            .child(input::field_with_palette(
                "name", "Name", name, None, window, cx, p,
            ))
            .child(input::field_with_palette(
                "email", "Email", &email, None, window, cx, p,
            ))
    })
}
