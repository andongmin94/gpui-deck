//! Typography and surfaces shared by the independently installable pages.
#![allow(dead_code)]
use super::super::Palette;
use super::{button, palette};
use gpui::{prelude::*, *};
use gpui_deck::Button;

pub(super) fn screen(id: &'static str, p: Palette, body: impl IntoElement) -> impl IntoElement {
    div()
        .id(id)
        .size_full()
        .overflow_y_scroll()
        .bg(p.canvas)
        .text_color(p.foreground)
        .child(div().w_full().flex().justify_center().p_5().child(body))
}
pub(super) fn column() -> Div {
    div()
        .w_full()
        .max_w(px(850.))
        .min_w_0()
        .flex()
        .flex_col()
        .gap_5()
}
pub(super) fn row() -> Div {
    div().w_full().flex().flex_wrap().items_center().gap_2()
}
pub(super) fn panel(p: Palette) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap_3()
        .p_5()
        .bg(p.surface)
        .rounded(p.radius)
        .shadow(palette::raised(p))
}
pub(super) fn title(text: impl Into<SharedString>) -> impl IntoElement {
    div()
        .id("page-title")
        .role(Role::Heading)
        .aria_level(1)
        .min_w_0()
        .font_weight(FontWeight::SEMIBOLD)
        .text_size(px(31.))
        .line_height(px(38.))
        .child(text.into())
}
pub(super) fn heading(text: impl Into<SharedString>) -> Div {
    div()
        .font_weight(FontWeight::SEMIBOLD)
        .text_size(px(19.))
        .line_height(px(25.))
        .child(text.into())
}
pub(super) fn note(p: Palette, text: impl Into<SharedString>) -> Div {
    div().text_sm().text_color(p.muted).child(text.into())
}
pub(super) fn action(
    p: Palette,
    id: impl Into<ElementId>,
    text: impl Into<SharedString>,
) -> Button {
    button::press_button_with_palette(id, text, p)
}
pub(super) fn choice(
    p: Palette,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Button {
    let label = label.into();
    button::raised(p, Button::new(id))
        .accessibility_label(label.clone())
        .aria_toggled(if selected {
            Toggled::True
        } else {
            Toggled::False
        })
        .when(selected, |b| b.bg(p.highlight).shadow(palette::inset(p)))
        .child(label)
}

pub(super) fn banner(p: Palette, section: &str, text: impl Into<SharedString>) -> Div {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap_2()
        .child(note(p, section.to_owned()))
        .child(title(text))
}
pub(super) fn disclosure(
    p: Palette,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    expanded: bool,
) -> Button {
    framed(p, id, label, false).aria_expanded(expanded)
}
pub(super) fn framed(
    p: Palette,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Button {
    let label = label.into();
    button::raised(p, Button::new(id))
        .accessibility_label(label.clone())
        .when(selected, |b| b.bg(p.highlight).shadow(palette::inset(p)))
        .child(label)
}

pub(super) fn primary(
    p: Palette,
    id: impl Into<ElementId>,
    text: impl Into<SharedString>,
) -> Button {
    let text = text.into();
    button::style_button(
        Button::new(id).accessibility_label(text.clone()),
        button::ButtonVariant::Primary,
        p,
    )
    .child(text)
}
