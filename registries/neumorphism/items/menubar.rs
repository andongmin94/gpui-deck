//! Application command menus with shared menu parts and roving keyboard focus.

#[allow(dead_code)]
mod control_style;
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
mod menu_surface;
use gpui_deck::{MenuItem, MenuState, Menubar, MenubarItem, MenubarMenu, MenubarState};
pub(crate) fn menubar(
    label: impl Into<SharedString>,
    state: &Entity<MenubarState>,
    p: Palette,
    cx: &App,
) -> Menubar {
    let m = state.read(cx);
    Menubar::new(state)
        .accessibility_label(label)
        .flex()
        .gap_1()
        .p_1()
        .bg(p.surface)
        .rounded(p.radius)
        .shadow(palette::raised(p))
        .border_1()
        .border_color(p.border)
        .children(m.items().iter().enumerate().map(|(i, item)| {
            let label = item.label().clone();
            let active = m.open_index(cx) == Some(i);
            MenubarMenu::new(state, i)
                .offset(px(7.))
                .trigger(move |b| {
                    b.h(px(34.))
                        .px_3()
                        .rounded(p.radius)
                        .font_weight(FontWeight::SEMIBOLD)
                        .when(active, |b| b.bg(p.accent).text_color(p.accent_foreground))
                        .hover(|s| s.bg(p.highlight).text_color(p.foreground))
                        .focus_visible(|s| s.underline().text_color(palette::accent_ink(p)))
                        .child(label)
                })
                .content(menu_surface::surface(item.menu(), p, cx))
        }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let file = cx.new(|cx| {
        MenuState::new(
            [
                MenuItem::new("new", "New document"),
                MenuItem::new("open", "Open recent"),
            ],
            cx,
        )
    });
    let edit = cx.new(|cx| {
        MenuState::new(
            [MenuItem::new("undo", "Undo"), MenuItem::new("redo", "Redo")],
            cx,
        )
    });
    let s = cx.new(|cx| {
        MenubarState::new(
            [
                MenubarItem::new("File", &file),
                MenubarItem::new("Edit", &edit),
            ],
            cx,
        )
    });
    let selected = menu_surface::selection(&[file, edit], cx);
    control_style::sample(s, cx, move |p, s, _, cx| {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(menubar("Editor menus", s, p, cx))
            .child(selected.read(cx).clone())
    })
}
