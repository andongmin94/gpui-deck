//! A pointer-positioned command menu with keyboard selection.

#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
mod menu_surface;
use gpui_deck::{ContextMenu, MenuItem, MenuState};
pub(crate) fn context_menu(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<MenuState>,
    target: impl IntoElement,
    p: Palette,
    cx: &App,
) -> ContextMenu {
    let label = label.into();
    ContextMenu::new(id, state)
        .target_label(label.clone())
        .accessibility_label(label)
        .target(target)
        .focus_visible(|s| s.underline().text_color(palette::accent_ink(p)))
        .content(menu_surface::surface(state, p, cx))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        MenuState::new(
            [
                MenuItem::new("copy", "Copy link"),
                MenuItem::new("pin", "Pin to workspace"),
            ],
            cx,
        )
    });
    let selected = menu_surface::selection(std::slice::from_ref(&s), cx);
    control_style::sample(s, cx, move |p, s, _, cx| {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(context_menu(
                "tile",
                "Project actions",
                s,
                control_style::panel(p)
                    .p_5()
                    .child("Right click or press Shift+F10"),
                p,
                cx,
            ))
            .child(selected.read(cx).clone())
    })
}
