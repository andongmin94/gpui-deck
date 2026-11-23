//! Neobrutal menu rows. Nested items retain their own menu state and keyboard navigation.
use super::{Palette, control_style};
use gpui::{prelude::*, *};
use gpui_deck::{MenuEvent, MenuOption, MenuState, MenuSubmenu};

fn row(
    option: MenuOption,
    label: SharedString,
    highlight: bool,
    disabled: bool,
    p: Palette,
) -> MenuOption {
    option
        .h(px(34.))
        .px_2()
        .rounded(px(3.))
        .flex()
        .items_center()
        .justify_between()
        .when(highlight, |s| {
            s.bg(p.accent).text_color(p.accent_foreground)
        })
        .when(disabled, |s| s.opacity(0.5))
        .child(label)
}

pub(super) fn surface(menu: &Entity<MenuState>, p: Palette, cx: &App) -> Div {
    let model = menu.read(cx);
    control_style::panel(p)
        .w(px(204.))
        .p_1()
        .children(model.items().iter().enumerate().map(|(i, item)| {
            let label = item.label().clone();
            let current = model.highlighted_index() == Some(i);
            let disabled = item.is_disabled();
            if let Some(sub) = item.submenu_state() {
                MenuSubmenu::new(menu, i)
                    .trigger(|option| row(option, label, current, disabled, p).child("›"))
                    .content(surface(sub, p, cx))
                    .into_any_element()
            } else {
                row(MenuOption::new(menu, i), label, current, disabled, p).into_any_element()
            }
        }))
}

pub(super) fn selection(menus: &[Entity<MenuState>], cx: &mut App) -> Entity<SharedString> {
    let last = cx.new(|_| SharedString::from("Choose an action"));
    for menu in menus {
        let output = last.clone();
        cx.subscribe(menu, move |menu, event: &MenuEvent, cx| {
            let MenuEvent::Action(key) = event;
            let caption = menu
                .read(cx)
                .items()
                .iter()
                .find(|i| i.value() == key)
                .map_or_else(|| key.clone(), |i| i.label().clone());
            output.update(cx, |last, cx| {
                *last = caption;
                cx.notify();
            });
        })
        .detach();
    }
    last
}
