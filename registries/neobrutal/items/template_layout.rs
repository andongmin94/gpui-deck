//! Application-owned typography, framing and button treatments shared by the templates.
#![allow(dead_code)] // Each independently installed page uses a subset of these treatments.
use super::super::Palette;
use super::{button::press_button_with_palette, palette::DEPTH};
use gpui::{
    Div, ElementId, FontWeight, InteractiveElement as _, IntoElement, ParentElement as _, Role,
    SharedString, StatefulInteractiveElement as _, Styled as _, div, px,
};
use gpui_deck::{Button, box_shadow};

pub(super) fn screen(id: &'static str, p: Palette, body: impl IntoElement) -> impl IntoElement {
    div()
        .id(id)
        .size_full()
        .overflow_y_scroll()
        .bg(p.canvas)
        .text_color(p.foreground)
        .child(div().w_full().flex().justify_center().p_4().child(body))
}

pub(super) fn column() -> Div {
    div()
        .w_full()
        .min_w_0()
        .max_w(px(820.))
        .flex()
        .flex_col()
        .gap_4()
}

pub(super) fn row() -> Div {
    div().flex().flex_wrap().items_center().gap_2()
}

pub(super) fn panel(p: Palette) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap_3()
        .p_4()
        .rounded(p.radius)
        .border_2()
        .border_color(p.border)
        .bg(p.surface)
        .shadow(vec![box_shadow(DEPTH, DEPTH, 0., 0., p.border)])
}

pub(super) fn banner(p: Palette, section: &str, title: impl Into<SharedString>) -> Div {
    panel(p)
        .bg(p.accent)
        .text_color(p.accent_foreground)
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .child(section.to_owned()),
        )
        .child(
            div()
                .id("page-title")
                .role(Role::Heading)
                .aria_level(1)
                .text_size(px(34.))
                .line_height(px(40.))
                .font_weight(FontWeight::BOLD)
                .child(title.into()),
        )
}

pub(super) fn heading(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(20.))
        .line_height(px(26.))
        .font_weight(FontWeight::BOLD)
        .child(text.into())
}

pub(super) fn action(
    p: Palette,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
) -> Button {
    press_button_with_palette(id, label, p)
}

pub(super) fn choice(
    p: Palette,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Button {
    framed(p, id, label, selected).aria_toggled(if selected {
        gpui::Toggled::True
    } else {
        gpui::Toggled::False
    })
}

pub(super) fn disclosure(
    p: Palette,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    expanded: bool,
) -> Button {
    framed(p, id, label, expanded).aria_expanded(expanded)
}

pub(super) fn framed(
    p: Palette,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Button {
    let background = if selected { p.accent } else { p.surface };
    let foreground = if selected {
        p.accent_foreground
    } else {
        p.foreground
    };
    let label = label.into();
    Button::new(id)
        .accessibility_label(label.clone())
        .selected(selected)
        .px_3()
        .py_2()
        .border_2()
        .rounded(p.radius)
        .border_color(p.border)
        .bg(background)
        .text_color(foreground)
        .text_sm()
        .font_weight(FontWeight::SEMIBOLD)
        .focus_visible(|s| s.underline())
        .hover(|s| s.underline())
        .child(label)
}
