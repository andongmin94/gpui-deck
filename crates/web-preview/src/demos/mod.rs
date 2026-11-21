//! One small, self-contained demo per documented component.
//!
//! Each demo is an ordinary GPUI view styled like the documentation site. The
//! site embeds `?c=<name>`; unknown names render a short notice instead.

use gpui::{
    AnyView, App, AppContext as _, Context, Div, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, Styled as _, Window, div, px,
};
use gpui_design::Button;

use crate::theme::Theme;

mod controls;
mod data;
mod invoice_workspace;
mod overlays;
mod recipe_controls;
mod recipe_display;
mod recipe_views;

/// Demo names defined in this module; the other modules list their own.
const CONTROL_NAMES: &[&str] = &[
    "invoice-workspace",
    "button",
    "toggle",
    "checkbox",
    "radio-group",
    "slider",
    "progress",
    "separator",
];

/// Every demo name the site may embed.
pub fn names() -> impl Iterator<Item = &'static str> {
    CONTROL_NAMES
        .iter()
        .chain(overlays::NAMES)
        .chain(data::NAMES)
        .chain(["card", "data-table"].iter())
        .chain(recipe_controls::NAMES)
        .chain(recipe_display::NAMES)
        .copied()
}

pub fn open(name: &str, theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let control = match name {
        "invoice-workspace" => Some(invoice_workspace::open(theme, window, cx)),
        "button" => Some(controls::button(theme, cx)),
        "toggle" => Some(controls::toggle(theme, cx)),
        "checkbox" => Some(controls::checkbox(theme, cx)),
        "radio-group" => Some(controls::radio_group(theme, cx)),
        "slider" => Some(controls::slider(theme, cx)),
        "progress" => Some(controls::progress(theme, cx)),
        "separator" => Some(controls::separator(theme, cx)),
        "card" => Some(recipe_views::card(theme, cx)),
        "data-table" => Some(recipe_views::data_table(theme, window, cx)),
        _ => None,
    };
    if let Some(view) = control
        .or_else(|| overlays::open(name, theme, window, cx))
        .or_else(|| data::open(name, theme, window, cx))
        .or_else(|| recipe_controls::open(name, theme, window, cx))
        .or_else(|| recipe_display::open(name, theme, window, cx))
    {
        return view;
    }
    cx.new(|_| Missing {
        theme,
        name: name.to_owned().into(),
    })
    .into()
}

/// The preview surface: centered content on the site background.
pub(crate) fn frame(theme: Theme) -> Div {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .gap(px(16.))
        .p(px(24.))
        .bg(theme.background)
        .text_color(theme.foreground)
        .text_size(px(14.))
}

/// A Button in the site's primary style.
pub(crate) fn primary(theme: Theme, button: Button) -> Button {
    button
        .h(px(36.))
        .px(px(16.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(6.))
        .bg(theme.primary)
        .text_color(theme.primary_foreground)
        .hover(|style| style.opacity(0.9))
        .focus_visible(|style| style.border_2().border_color(theme.ring))
        .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
}

/// A Button with a border and transparent background.
pub(crate) fn outline(theme: Theme, button: Button) -> Button {
    button
        .h(px(36.))
        .px(px(16.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(6.))
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .text_color(theme.foreground)
        .hover(|style| style.bg(theme.accent))
        .focus_visible(|style| style.border_color(theme.ring))
        .styles(|styles| styles.disabled(|style| style.opacity(0.5)))
}

struct Missing {
    theme: Theme,
    name: SharedString,
}

impl Render for Missing {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        frame(self.theme)
            .text_color(self.theme.muted_foreground)
            .child(format!("No preview named “{}”.", self.name))
    }
}
