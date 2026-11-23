//! navigation-menu in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{
    NavigationMenu, NavigationMenuEntry, NavigationMenuEvent, NavigationMenuItem,
    NavigationMenuLink, NavigationMenuState,
};
pub(crate) fn navigation_menu<E: IntoElement>(
    label: impl Into<SharedString>,
    state: &Entity<NavigationMenuState>,
    current: &str,
    p: Palette,
    cx: &App,
    content: impl Fn(usize) -> E,
) -> NavigationMenu {
    let m = state.read(cx);
    NavigationMenu::new(state)
        .accessibility_label(label)
        .flex()
        .gap_2()
        .children(m.items().iter().enumerate().map(|(i, item)| {
            let selected = item.value().as_ref() == current;
            let active = m.open_index(cx) == Some(i);
            let label = item.label().clone();
            NavigationMenuEntry::new(state, i)
                .offset(px(6.))
                .current(selected)
                .trigger(move |b| {
                    button::raised(p, b)
                        .when(!active && !selected, |s| {
                            s.bg(p.surface).text_color(p.foreground)
                        })
                        .child(label)
                })
                .when(item.has_popup(), |s| {
                    s.content(control_style::panel(p).p_2().child(content(i)))
                })
        }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let first = cx.focus_handle();
    let s = cx.new(|cx| {
        NavigationMenuState::new(
            [
                NavigationMenuItem::new("guides", "Guides ▼")
                    .with_popup()
                    .initial_focus(&first),
                NavigationMenuItem::new("about", "About"),
            ],
            cx,
        )
    });
    let page = cx.new(|_| SharedString::from("Start"));
    let route = page.clone();
    cx.subscribe(&s, move |_, event: &NavigationMenuEvent, cx| {
        let NavigationMenuEvent::Navigate(value) = event;
        route.update(cx, |current, cx| {
            *current = value.clone();
            cx.notify();
        });
    })
    .detach();
    control_style::sample(s, cx, move |p, s, _, cx| {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(navigation_menu(
                "Documentation",
                s,
                page.read(cx).as_ref(),
                p,
                cx,
                |_| {
                    div().flex().flex_col().children(
                        ["Installation", "Customization"]
                            .into_iter()
                            .enumerate()
                            .map(|(i, text)| {
                                NavigationMenuLink::new(text, s, text)
                                    .button(|b| {
                                        b.p_2()
                                            .when(i == 0, |b| b.track_focus(&first))
                                            .focus_visible(|s| s.underline())
                                    })
                                    .child(text)
                            }),
                    )
                },
            ))
            .child(format!("Current page: {}", page.read(cx)))
    })
}
