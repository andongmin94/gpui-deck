//! An anchored menu with keyboard selection, disabled items and nested submenus.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
mod menu_surface;
use gpui_deck::{DropdownMenu, MenuItem, MenuState};
pub(crate) fn dropdown_menu(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<MenuState>,
    p: Palette,
    cx: &App,
) -> DropdownMenu {
    let label = label.into();
    DropdownMenu::new(id, state)
        .accessibility_label(label.clone())
        .offset(px(8.))
        .trigger(move |button| {
            button::raised(p, button)
                .accessibility_label(label.clone())
                .child(label)
        })
        .content(menu_surface::surface(state, p, cx))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        MenuState::new(
            [
                MenuItem::new("duplicate", "Duplicate"),
                MenuItem::new("archive", "Archive"),
                MenuItem::new("export", "Export"),
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
            .child(dropdown_menu("actions", "Actions ▼", s, p, cx))
            .child(selected.read(cx).clone())
    })
}
