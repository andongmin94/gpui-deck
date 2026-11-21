//! Disclosure, navigation, menu and overlay demos.

use gpui::{
    AnyView, App, AppContext as _, Context, Div, ElementId, Entity, FocusHandle, Focusable as _,
    FontWeight, InteractiveElement as _, IntoElement, ParentElement as _, Render, SharedString,
    Styled as _, Subscription, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_design::{
    Accordion, AccordionHeader, AccordionItem, AccordionPanel, AccordionState, AlertDialog,
    Breadcrumb, BreadcrumbItem, BreadcrumbList, Button, Collapsible, CollapsibleContent,
    CollapsibleState, CollapsibleTrigger, Command, CommandEvent, CommandGroup, CommandInput,
    CommandList, CommandOption, CommandState, ContextMenu, Dialog, DialogBackdrop, DialogClose,
    DialogDescription, DialogPopup, DialogState, DialogTitle, Drawer, DrawerState, DropdownMenu,
    HoverCard, HoverCardState, Input, InputAccessibility, InputState, Label, MenuEvent, MenuItem,
    MenuOption, MenuState, Menubar, MenubarItem, MenubarMenu, MenubarState, NavigationMenu,
    NavigationMenuEntry, NavigationMenuEvent, NavigationMenuItem, NavigationMenuLink,
    NavigationMenuState, Popover, PopoverState, SearchItem, Tab, TabItem, TabList, TabPanel,
    TabsState, Toast, ToastClose, Toaster, ToasterState, Tooltip, TooltipState,
};

use super::{data::style_editor, frame, outline, primary};
use crate::theme::Theme;

/// Demo names this module renders.
pub const NAMES: &[&str] = &[
    "tabs",
    "accordion",
    "collapsible",
    "tooltip",
    "popover",
    "hover-card",
    "dropdown-menu",
    "context-menu",
    "menubar",
    "navigation-menu",
    "breadcrumb",
    "command",
    "dialog",
    "alert-dialog",
    "drawer",
    "toast",
];

pub fn open(name: &str, theme: Theme, window: &mut Window, cx: &mut App) -> Option<AnyView> {
    Some(match name {
        "tabs" => tabs(theme, cx),
        "accordion" => accordion(theme, cx),
        "collapsible" => collapsible(theme, cx),
        "tooltip" => tooltip(theme, window, cx),
        "popover" => popover(theme, cx),
        "hover-card" => hover_card(theme, window, cx),
        "dropdown-menu" => dropdown_menu(theme, cx),
        "context-menu" => context_menu(theme, cx),
        "menubar" => menubar(theme, cx),
        "navigation-menu" => navigation_menu(theme, cx),
        "breadcrumb" => breadcrumb(theme, cx),
        "command" => command(theme, window, cx),
        "dialog" => dialog(theme, window, cx),
        "alert-dialog" => alert_dialog(theme, cx),
        "drawer" => drawer(theme, window, cx),
        "toast" => toast(theme, cx),
        _ => return None,
    })
}

// Shared presentation

/// A floating surface: menus, popovers and cards.
fn panel(theme: Theme) -> Div {
    div()
        .flex()
        .flex_col()
        .p(px(4.))
        .rounded(px(8.))
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .text_color(theme.foreground)
        .text_size(px(14.))
        .shadow_md()
}

fn caption(theme: Theme, text: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(13.))
        .text_color(theme.muted_foreground)
        .child(text.into())
}

fn menu_separator(theme: Theme) -> Div {
    div().h(px(1.)).mx(px(-4.)).my(px(4.)).bg(theme.border)
}

fn menu_row(theme: Theme, menu: &Entity<MenuState>, index: usize, cx: &App) -> MenuOption {
    let state = menu.read(cx);
    let item = &state.items()[index];
    MenuOption::new(menu, index)
        .h(px(32.))
        .px(px(8.))
        .flex()
        .items_center()
        .rounded(px(4.))
        .when(state.highlighted_index() == Some(index), |row| {
            row.bg(theme.accent)
        })
        .when(item.is_disabled(), |row| row.opacity(0.5))
        .child(item.label().clone())
}

fn menu_rows(
    theme: Theme,
    menu: &Entity<MenuState>,
    range: std::ops::Range<usize>,
    cx: &App,
) -> Vec<MenuOption> {
    range
        .map(|index| menu_row(theme, menu, index, cx))
        .collect()
}

fn label_of(menu: &Entity<MenuState>, value: &str, cx: &App) -> SharedString {
    menu.read(cx)
        .items()
        .iter()
        .find(|item| item.value().as_ref() == value)
        .map(|item| item.label().clone())
        .unwrap_or_default()
}

fn modal_backdrop() -> DialogBackdrop {
    DialogBackdrop::new()
        .size_full()
        .bg(gpui::black().opacity(0.5))
}

fn modal_popup(theme: Theme) -> DialogPopup {
    DialogPopup::new()
        .w(px(340.))
        .p(px(24.))
        .flex()
        .flex_col()
        .gap(px(16.))
        .rounded(px(10.))
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .text_color(theme.foreground)
        .text_size(px(14.))
        .shadow_lg()
}

// Tabs

struct TabsDemo {
    theme: Theme,
    tabs: Entity<TabsState>,
    _observe: Subscription,
}

impl Render for TabsDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let state = self.tabs.read(cx);
        let selected = state.selected_index();
        let list = TabList::new("account-tabs", &self.tabs)
            .accessibility_label("Account settings")
            .flex()
            .p(px(3.))
            .rounded(px(8.))
            .bg(theme.muted)
            .children(state.items().iter().enumerate().map(|(index, item)| {
                let active = selected == Some(index);
                Tab::new(&self.tabs, index)
                    .flex_1()
                    .h(px(30.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(6.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(if active {
                        theme.primary
                    } else {
                        theme.muted_foreground
                    })
                    .when(active, |tab| tab.bg(theme.background).shadow_sm())
                    .focus_visible(|style| style.border_1().border_color(theme.ring))
                    .child(item.label().clone())
            }));
        let panels = [
            (
                "Account",
                "Make changes to your account here.",
                [("Name", "Ada Park"), ("Username", "@adapark")],
            ),
            (
                "Password",
                "Change your password here.",
                [("Last changed", "Nov 3, 2026"), ("Two-factor", "Enabled")],
            ),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (title, description, rows))| {
            TabPanel::new(&self.tabs, index)
                .flex()
                .flex_col()
                .gap(px(6.))
                .p(px(20.))
                .rounded(px(8.))
                .border_1()
                .border_color(theme.border)
                .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                .child(caption(theme, description))
                .children(rows.into_iter().map(|(label, value)| {
                    div()
                        .pt(px(8.))
                        .flex()
                        .justify_between()
                        .child(div().text_color(theme.muted_foreground).child(label))
                        .child(value)
                }))
        });
        frame(theme).child(
            div()
                .w(px(360.))
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(list)
                .children(panels),
        )
    }
}

pub fn tabs(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let tabs = cx.new(|cx| {
            TabsState::new(
                [
                    TabItem::new("account", "Account"),
                    TabItem::new("password", "Password"),
                ],
                cx,
            )
        });
        let observe = cx.observe(&tabs, |_, _, cx| cx.notify());
        TabsDemo {
            theme,
            tabs,
            _observe: observe,
        }
    })
    .into()
}

// Accordion

struct AccordionDemo {
    theme: Theme,
    state: Entity<AccordionState>,
    _observe: Subscription,
}

const ANSWERS: [&str; 3] = [
    "Yes. Focus, keyboard navigation and accessibility metadata live in the library.",
    "No. Your application owns presentation; this demo uses a neutral documentation style.",
    "No. Panels open and close immediately, so reduced motion needs no extra work.",
];

impl Render for AccordionDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let state = self.state.read(cx);
        frame(theme).child(
            Accordion::new("faq", &self.state)
                .w(px(360.))
                .flex()
                .flex_col()
                .children(state.items().iter().enumerate().map(|(index, item)| {
                    div()
                        .flex()
                        .flex_col()
                        .border_b_1()
                        .border_color(theme.border)
                        .child(
                            AccordionHeader::new(&self.state, index)
                                .w_full()
                                .h(px(48.))
                                .flex()
                                .items_center()
                                .justify_between()
                                .font_weight(FontWeight::MEDIUM)
                                .hover(|style| style.underline())
                                .focus_visible(|style| style.text_color(theme.primary).underline())
                                .child(item.label().clone())
                                .child(
                                    div()
                                        .text_color(theme.muted_foreground)
                                        .child(if state.is_open(index) { "−" } else { "+" }),
                                ),
                        )
                        .child(
                            AccordionPanel::new(&self.state, index)
                                .pb(px(16.))
                                .text_color(theme.muted_foreground)
                                .child(ANSWERS[index]),
                        )
                })),
        )
    }
}

pub fn accordion(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let state = cx.new(|cx| {
            AccordionState::new(
                [
                    AccordionItem::new("accessible", "Is it accessible?"),
                    AccordionItem::new("styled", "Is it styled?"),
                    AccordionItem::new("animated", "Is it animated?"),
                ],
                cx,
            )
            .default_open(["accessible"])
        });
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        AccordionDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

// Collapsible

struct CollapsibleDemo {
    theme: Theme,
    state: Entity<CollapsibleState>,
    _observe: Subscription,
}

fn repository(theme: Theme, name: &'static str) -> Div {
    div()
        .px(px(16.))
        .py(px(8.))
        .rounded(px(6.))
        .border_1()
        .border_color(theme.border)
        .child(name)
}

impl Render for CollapsibleDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let open = self.state.read(cx).is_open();
        frame(theme).child(
            Collapsible::new("starred", &self.state)
                .w(px(320.))
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .pl(px(16.))
                        .child(
                            div()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("@adapark starred 3 repositories"),
                        )
                        .child(
                            CollapsibleTrigger::new(&self.state)
                                .accessibility_label("Toggle repositories")
                                .size(px(32.))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(6.))
                                .hover(|style| style.bg(theme.accent))
                                .focus_visible(|style| style.border_1().border_color(theme.ring))
                                .child(if open { "−" } else { "+" }),
                        ),
                )
                .child(repository(theme, "@radix-ui/primitives"))
                .child(
                    CollapsibleContent::new(&self.state)
                        .accessibility_label("More repositories")
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(repository(theme, "@radix-ui/colors"))
                        .child(repository(theme, "@stitches/react")),
                ),
        )
    }
}

pub fn collapsible(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let state = cx.new(CollapsibleState::new);
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        CollapsibleDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

// Tooltip

struct TooltipDemo {
    theme: Theme,
    state: Entity<TooltipState>,
}

impl Render for TooltipDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme)
            .flex_col()
            .child(
                Tooltip::new("library-hint", &self.state, "Add to library")
                    .font_family(window.text_style().font_family)
                    .px(px(12.))
                    .py(px(6.))
                    .rounded(px(6.))
                    .bg(theme.primary)
                    .text_color(theme.primary_foreground)
                    .text_size(px(12.))
                    .trigger(|button| {
                        outline(theme, button)
                            .accessibility_label("Hover")
                            .child("Hover")
                    }),
            )
            .child(caption(
                theme,
                "Hover or Tab to the button; Escape hides the tooltip.",
            ))
    }
}

pub fn tooltip(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let focus = cx.focus_handle().tab_stop(true);
        let state = cx.new(|cx| TooltipState::new(&focus, window, cx));
        TooltipDemo { theme, state }
    })
    .into()
}

// Popover

struct PopoverDemo {
    theme: Theme,
    state: Entity<PopoverState>,
    done: FocusHandle,
}

impl Render for PopoverDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let rows = [
            ("Width", "100%"),
            ("Max. width", "300px"),
            ("Height", "25px"),
        ];
        frame(theme).items_start().pt(px(48.)).child(
            Popover::new("dimensions", &self.state)
                .accessibility_label("Dimensions")
                .offset(px(8.))
                .trigger(|button| {
                    outline(theme, button)
                        .accessibility_label("Open popover")
                        .child("Open popover")
                })
                .content(
                    panel(theme)
                        .w(px(280.))
                        .p(px(16.))
                        .gap(px(12.))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(div().font_weight(FontWeight::SEMIBOLD).child("Dimensions"))
                                .child(caption(theme, "Set the dimensions for the layer.")),
                        )
                        .children(rows.into_iter().map(|(label, value)| {
                            div()
                                .flex()
                                .justify_between()
                                .child(div().text_color(theme.muted_foreground).child(label))
                                .child(value)
                        }))
                        .child(
                            outline(theme, Button::new("close-dimensions"))
                                .track_focus(&self.done)
                                .h(px(32.))
                                .accessibility_label("Done")
                                .child("Done")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.state.update(cx, |state, cx| state.close(window, cx));
                                })),
                        ),
                ),
        )
    }
}

pub fn popover(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let done = cx.focus_handle();
        let state = cx.new(|cx| PopoverState::new(cx).with_initial_focus(&done));
        PopoverDemo { theme, state, done }
    })
    .into()
}

// Hover card

struct HoverCardDemo {
    theme: Theme,
    state: Entity<HoverCardState>,
}

impl Render for HoverCardDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme).items_start().pt(px(64.)).child(
            HoverCard::new("profile-card", &self.state)
                .accessibility_label("@gpui-design profile")
                .offset(px(8.))
                .trigger(|button| {
                    button
                        .accessibility_label("@gpui-design")
                        .h(px(32.))
                        .px(px(8.))
                        .flex()
                        .items_center()
                        .rounded(px(6.))
                        .font_weight(FontWeight::MEDIUM)
                        .hover(|style| style.underline())
                        .focus_visible(|style| style.underline().bg(theme.accent))
                        .child("@gpui-design")
                })
                .content(
                    panel(theme)
                        .w(px(300.))
                        .p(px(16.))
                        .flex_row()
                        .gap(px(16.))
                        .child(
                            div()
                                .size(px(40.))
                                .flex_shrink_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .bg(theme.muted)
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("GD"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(div().font_weight(FontWeight::SEMIBOLD).child("@gpui-design"))
                                .child("Headless components for GPUI, with behavior and focus in the library.")
                                .child(caption(theme, "Joined November 2026")),
                        ),
                ),
        )
    }
}

pub fn hover_card(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let focus = cx.focus_handle().tab_stop(true);
        let state = cx.new(|cx| HoverCardState::new(&focus, window, cx));
        HoverCardDemo { theme, state }
    })
    .into()
}

// Dropdown menu

struct DropdownDemo {
    theme: Theme,
    menu: Entity<MenuState>,
    last: SharedString,
    _subscriptions: [Subscription; 2],
}

impl Render for DropdownDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme)
            .flex_col()
            .justify_start()
            .pt(px(32.))
            .child(
                DropdownMenu::new("account-menu", &self.menu)
                    .accessibility_label("My account")
                    .offset(px(6.))
                    .trigger(|button| {
                        outline(theme, button)
                            .accessibility_label("Open account menu")
                            .child("Open")
                    })
                    .content(
                        panel(theme)
                            .w(px(224.))
                            .child(
                                div()
                                    .px(px(8.))
                                    .py(px(6.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("My Account"),
                            )
                            .child(menu_separator(theme))
                            .children(menu_rows(theme, &self.menu, 0..4, cx))
                            .child(menu_separator(theme))
                            .children(menu_rows(theme, &self.menu, 4..6, cx)),
                    ),
            )
            .child(caption(theme, self.last.clone()))
    }
}

pub fn dropdown_menu(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let menu = cx.new(|cx| {
            MenuState::new(
                [
                    MenuItem::new("profile", "Profile"),
                    MenuItem::new("billing", "Billing"),
                    MenuItem::new("settings", "Settings"),
                    MenuItem::new("shortcuts", "Keyboard shortcuts"),
                    MenuItem::new("api", "API").disabled(true),
                    MenuItem::new("logout", "Log out"),
                ],
                cx,
            )
        });
        let subscriptions = [
            cx.observe(&menu, |_, _, cx| cx.notify()),
            cx.subscribe(&menu, |this: &mut DropdownDemo, menu, event, cx| {
                let MenuEvent::Action(value) = event;
                this.last = format!("Selected: {}", label_of(&menu, value, cx)).into();
                cx.notify();
            }),
        ];
        DropdownDemo {
            theme,
            menu,
            last: "Choose an item from the menu.".into(),
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Context menu

struct ContextMenuDemo {
    theme: Theme,
    menu: Entity<MenuState>,
    bookmarks: bool,
    last: SharedString,
    _subscriptions: [Subscription; 2],
}

impl Render for ContextMenuDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme)
            .flex_col()
            .child(
                ContextMenu::new("page-area", &self.menu)
                    .target_label("Page area")
                    .accessibility_label("Page actions")
                    .w(px(300.))
                    .h(px(150.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(8.))
                    .border_1()
                    .border_dashed()
                    .border_color(theme.border)
                    .focus_visible(|style| style.border_color(theme.ring))
                    .target(
                        div()
                            .text_color(theme.muted_foreground)
                            .child("Right click here"),
                    )
                    .content(
                        panel(theme)
                            .w(px(224.))
                            .children(menu_rows(theme, &self.menu, 0..3, cx))
                            .child(menu_separator(theme))
                            .child(
                                menu_row(theme, &self.menu, 3, cx)
                                    .checked(self.bookmarks)
                                    .justify_between()
                                    .child(if self.bookmarks { "✓" } else { "" }),
                            )
                            .child(menu_row(theme, &self.menu, 4, cx)),
                    ),
            )
            .child(caption(theme, self.last.clone()))
    }
}

pub fn context_menu(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let menu = cx.new(|cx| {
            MenuState::new(
                [
                    MenuItem::new("back", "Back"),
                    MenuItem::new("forward", "Forward").disabled(true),
                    MenuItem::new("reload", "Reload"),
                    MenuItem::new("bookmarks", "Show bookmarks bar"),
                    MenuItem::new("save", "Save page as…"),
                ],
                cx,
            )
        });
        let subscriptions = [
            cx.observe(&menu, |_, _, cx| cx.notify()),
            cx.subscribe(&menu, |this: &mut ContextMenuDemo, menu, event, cx| {
                let MenuEvent::Action(value) = event;
                if value.as_ref() == "bookmarks" {
                    this.bookmarks = !this.bookmarks;
                }
                this.last = format!("Last action: {}", label_of(&menu, value, cx)).into();
                cx.notify();
            }),
        ];
        ContextMenuDemo {
            theme,
            menu,
            bookmarks: true,
            last: "Or focus the area and press Shift+F10.".into(),
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Menubar

struct MenubarDemo {
    theme: Theme,
    bar: Entity<MenubarState>,
    menus: [Entity<MenuState>; 3],
    bookmarks: bool,
    last: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl Render for MenubarDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let open = self.bar.read(cx).open_index(cx);
        let [file, edit, view] = &self.menus;
        let contents = [
            panel(theme)
                .w(px(208.))
                .children(menu_rows(theme, file, 0..2, cx))
                .child(menu_separator(theme))
                .children(menu_rows(theme, file, 2..4, cx)),
            panel(theme)
                .w(px(208.))
                .children(menu_rows(theme, edit, 0..2, cx))
                .child(menu_separator(theme))
                .children(menu_rows(theme, edit, 2..5, cx)),
            panel(theme)
                .w(px(208.))
                .child(
                    menu_row(theme, view, 0, cx)
                        .checked(self.bookmarks)
                        .justify_between()
                        .child(if self.bookmarks { "✓" } else { "" }),
                )
                .child(menu_separator(theme))
                .children(menu_rows(theme, view, 1..3, cx)),
        ];
        let labels = ["File", "Edit", "View"];
        frame(theme)
            .flex_col()
            .justify_start()
            .pt(px(32.))
            .child(
                Menubar::new(&self.bar)
                    .accessibility_label("Application menu")
                    .flex()
                    .items_center()
                    .gap(px(4.))
                    .p(px(4.))
                    .rounded(px(8.))
                    .border_1()
                    .border_color(theme.border)
                    .children(contents.into_iter().enumerate().map(|(index, content)| {
                        MenubarMenu::new(&self.bar, index)
                            .offset(px(8.))
                            .trigger(|button| {
                                button
                                    .h(px(28.))
                                    .px(px(12.))
                                    .flex()
                                    .items_center()
                                    .rounded(px(4.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .when(open == Some(index), |button| button.bg(theme.accent))
                                    .hover(|style| style.bg(theme.accent))
                                    .focus_visible(|style| style.bg(theme.accent))
                                    .child(labels[index])
                            })
                            .content(content)
                    })),
            )
            .child(caption(theme, self.last.clone()))
    }
}

pub fn menubar(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let menu = |items: Vec<MenuItem>, cx: &mut Context<MenubarDemo>| {
            cx.new(|cx| MenuState::new(items, cx))
        };
        let file = menu(
            vec![
                MenuItem::new("new-tab", "New Tab"),
                MenuItem::new("new-window", "New Window"),
                MenuItem::new("share", "Share").disabled(true),
                MenuItem::new("print", "Print…"),
            ],
            cx,
        );
        let edit = menu(
            vec![
                MenuItem::new("undo", "Undo"),
                MenuItem::new("redo", "Redo"),
                MenuItem::new("cut", "Cut"),
                MenuItem::new("copy", "Copy"),
                MenuItem::new("paste", "Paste"),
            ],
            cx,
        );
        let view = menu(
            vec![
                MenuItem::new("bookmarks", "Always show bookmarks bar"),
                MenuItem::new("reload", "Reload"),
                MenuItem::new("fullscreen", "Toggle full screen"),
            ],
            cx,
        );
        let bar = cx.new(|cx| {
            MenubarState::new(
                [
                    MenubarItem::new("File", &file),
                    MenubarItem::new("Edit", &edit),
                    MenubarItem::new("View", &view),
                ],
                cx,
            )
        });
        let mut subscriptions = vec![cx.observe(&bar, |_, _, cx| cx.notify())];
        for menu in [&file, &edit, &view] {
            subscriptions.push(cx.observe(menu, |_, _, cx| cx.notify()));
            subscriptions.push(
                cx.subscribe(menu, |this: &mut MenubarDemo, menu, event, cx| {
                    let MenuEvent::Action(value) = event;
                    if value.as_ref() == "bookmarks" {
                        this.bookmarks = !this.bookmarks;
                    }
                    this.last = format!("Last command: {}", label_of(&menu, value, cx)).into();
                    cx.notify();
                }),
            );
        }
        MenubarDemo {
            theme,
            bar,
            menus: [file, edit, view],
            bookmarks: true,
            last: "Open a menu with a click or the Down arrow.".into(),
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Navigation menu

struct NavigationDemo {
    theme: Theme,
    navigation: Entity<NavigationMenuState>,
    first_links: [FocusHandle; 2],
    page: SharedString,
    _subscriptions: [Subscription; 2],
}

const GUIDES: [(&str, &str, &str); 3] = [
    (
        "introduction",
        "Introduction",
        "Headless components for GPUI.",
    ),
    (
        "installation",
        "Installation",
        "Add the crate and initialize it.",
    ),
    (
        "theming",
        "Theming",
        "Style each part from your application.",
    ),
];

const COMPONENTS: [(&str, &str, &str); 3] = [
    ("dialog", "Dialog", "A modal window over the page."),
    ("tooltip", "Tooltip", "A label shown on hover or focus."),
    ("tabs", "Tabs", "Layered sections shown one at a time."),
];

impl NavigationDemo {
    fn links(
        &self,
        theme: Theme,
        links: [(&'static str, &'static str, &'static str); 3],
        first: &FocusHandle,
    ) -> Div {
        panel(theme)
            .w(px(280.))
            .gap(px(2.))
            .children(
                links
                    .into_iter()
                    .enumerate()
                    .map(|(index, (value, title, description))| {
                        let current = self.page.as_ref() == value;
                        NavigationMenuLink::new(
                            ElementId::Name(value.into()),
                            &self.navigation,
                            value,
                        )
                        .current(current)
                        .button(|button| {
                            button
                                .accessibility_label(title)
                                .when(index == 0, |button| button.track_focus(first))
                                .w_full()
                                .p(px(8.))
                                .flex()
                                .flex_col()
                                .gap(px(2.))
                                .rounded(px(6.))
                                .when(current, |button| button.bg(theme.accent))
                                .hover(|style| style.bg(theme.accent))
                                .focus_visible(|style| style.bg(theme.accent))
                        })
                        .child(div().font_weight(FontWeight::MEDIUM).child(title))
                        .child(caption(theme, description))
                    }),
            )
    }
}

fn nav_trigger(theme: Theme, button: Button, label: &'static str, active: bool) -> Button {
    button
        .h(px(36.))
        .px(px(14.))
        .flex()
        .items_center()
        .rounded(px(6.))
        .font_weight(FontWeight::MEDIUM)
        .when(active, |button| button.bg(theme.accent))
        .hover(|style| style.bg(theme.accent))
        .focus_visible(|style| style.bg(theme.accent))
        .child(label)
}

impl Render for NavigationDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let open = self.navigation.read(cx).open_index(cx);
        let title = GUIDES
            .iter()
            .chain(COMPONENTS.iter())
            .find(|(value, ..)| *value == self.page.as_ref())
            .map_or("Documentation", |(_, title, _)| *title);
        frame(theme)
            .flex_col()
            .justify_start()
            .pt(px(32.))
            .child(
                NavigationMenu::new(&self.navigation)
                    .accessibility_label("Documentation")
                    .flex()
                    .gap(px(4.))
                    .child(
                        NavigationMenuEntry::new(&self.navigation, 0)
                            .offset(px(6.))
                            .trigger(|button| {
                                nav_trigger(theme, button, "Getting started", open == Some(0))
                            })
                            .content(self.links(theme, GUIDES, &self.first_links[0])),
                    )
                    .child(
                        NavigationMenuEntry::new(&self.navigation, 1)
                            .offset(px(6.))
                            .trigger(|button| {
                                nav_trigger(theme, button, "Components", open == Some(1))
                            })
                            .content(self.links(theme, COMPONENTS, &self.first_links[1])),
                    )
                    .child(
                        NavigationMenuEntry::new(&self.navigation, 2)
                            .current(self.page.as_ref() == "docs")
                            .trigger(|button| {
                                nav_trigger(theme, button, "Docs", self.page.as_ref() == "docs")
                            }),
                    ),
            )
            .child(caption(theme, format!("Current page: {title}")))
    }
}

pub fn navigation_menu(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let first_links = [cx.focus_handle(), cx.focus_handle()];
        let navigation = cx.new(|cx| {
            NavigationMenuState::new(
                [
                    NavigationMenuItem::new("getting-started", "Getting started")
                        .with_popup()
                        .initial_focus(&first_links[0]),
                    NavigationMenuItem::new("components", "Components")
                        .with_popup()
                        .initial_focus(&first_links[1]),
                    NavigationMenuItem::new("docs", "Docs"),
                ],
                cx,
            )
        });
        let subscriptions = [
            cx.observe(&navigation, |_, _, cx| cx.notify()),
            cx.subscribe(&navigation, |this: &mut NavigationDemo, _, event, cx| {
                let NavigationMenuEvent::Navigate(value) = event;
                this.page = value.clone();
                cx.notify();
            }),
        ];
        NavigationDemo {
            theme,
            navigation,
            first_links,
            page: "introduction".into(),
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Breadcrumb

struct BreadcrumbDemo {
    theme: Theme,
    hidden: Entity<MenuState>,
    status: SharedString,
    _subscriptions: [Subscription; 2],
}

impl BreadcrumbDemo {
    fn link(&self, id: &'static str, label: &'static str, cx: &mut Context<Self>) -> Button {
        let theme = self.theme.resolve(cx);
        Button::new(id)
            .accessibility_label(label)
            .rounded(px(4.))
            .hover(|style| style.text_color(theme.primary))
            .focus_visible(|style| style.text_color(theme.primary).underline())
            .child(label)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.status = format!("Navigated to {label}").into();
                cx.notify();
            }))
    }
}

impl Render for BreadcrumbDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let separator = || div().text_color(theme.muted_foreground).child("›");
        let home = self.link("crumb-home", "Home", cx);
        let components = self.link("crumb-components", "Components", cx);
        frame(theme)
            .flex_col()
            .child(
                Breadcrumb::new("docs-breadcrumb")
                    .accessibility_label("Breadcrumb")
                    .child(
                        BreadcrumbList::new("docs-breadcrumb-list")
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .text_color(theme.muted_foreground)
                            .child(BreadcrumbItem::new("home").child(home))
                            .child(separator())
                            .child(
                                BreadcrumbItem::new("hidden").child(
                                    DropdownMenu::new("hidden-pages", &self.hidden)
                                        .accessibility_label("Hidden pages")
                                        .offset(px(6.))
                                        .trigger(|button| {
                                            button
                                                .accessibility_label("Show hidden pages")
                                                .size(px(24.))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded(px(4.))
                                                .hover(|style| style.bg(theme.accent))
                                                .focus_visible(|style| style.bg(theme.accent))
                                                .child("…")
                                        })
                                        .content(panel(theme).w(px(160.)).children(menu_rows(
                                            theme,
                                            &self.hidden,
                                            0..3,
                                            cx,
                                        ))),
                                ),
                            )
                            .child(separator())
                            .child(BreadcrumbItem::new("components").child(components))
                            .child(separator())
                            .child(
                                BreadcrumbItem::new("current")
                                    .current(true)
                                    .text_color(theme.foreground)
                                    .child("Breadcrumb"),
                            ),
                    ),
            )
            .child(caption(theme, self.status.clone()))
    }
}

pub fn breadcrumb(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let hidden = cx.new(|cx| {
            MenuState::new(
                [
                    MenuItem::new("documentation", "Documentation"),
                    MenuItem::new("themes", "Themes"),
                    MenuItem::new("github", "GitHub"),
                ],
                cx,
            )
        });
        let subscriptions = [
            cx.observe(&hidden, |_, _, cx| cx.notify()),
            cx.subscribe(&hidden, |this: &mut BreadcrumbDemo, menu, event, cx| {
                let MenuEvent::Action(value) = event;
                this.status = format!("Navigated to {}", label_of(&menu, value, cx)).into();
                cx.notify();
            }),
        ];
        BreadcrumbDemo {
            theme,
            hidden,
            status: "Choose a page or open the hidden pages.".into(),
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Command

struct CommandDemo {
    theme: Theme,
    commands: Entity<CommandState>,
    last: SharedString,
    _subscriptions: [Subscription; 2],
}

impl Render for CommandDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let state = self.commands.read(cx);
        let groups = state.visible_groups().into_iter().map(|group| {
            let options = state
                .visible_group_indices(Some(&group))
                .into_iter()
                .map(|index| {
                    let item = &state.items()[index];
                    CommandOption::new(&self.commands, index)
                        .h(px(32.))
                        .px(px(8.))
                        .flex()
                        .items_center()
                        .rounded(px(4.))
                        .when(state.highlighted_index() == Some(index), |option| {
                            option.bg(theme.accent)
                        })
                        .when(item.is_disabled(), |option| option.opacity(0.5))
                        .child(item.label().clone())
                });
            CommandGroup::new(ElementId::Name(group.clone()), &self.commands, group)
                .heading(|heading| {
                    heading
                        .px(px(8.))
                        .py(px(6.))
                        .text_size(px(12.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.muted_foreground)
                })
                .children(options)
        });
        frame(theme)
            .flex_col()
            .child(
                Command::new("palette", &self.commands)
                    .accessibility_label("Command palette")
                    .w(px(340.))
                    .flex()
                    .flex_col()
                    .rounded(px(8.))
                    .border_1()
                    .border_color(theme.border)
                    .shadow_md()
                    .overflow_hidden()
                    .child(
                        div()
                            .h(px(44.))
                            .px(px(12.))
                            .flex()
                            .items_center()
                            .border_b_1()
                            .border_color(theme.border)
                            .child(
                                CommandInput::new("palette-query", &self.commands)
                                    .accessibility_label("Search commands")
                                    .placeholder("Type a command or search…")
                                    .flex_1(),
                            ),
                    )
                    .child(
                        CommandList::new("palette-results", &self.commands)
                            .accessibility_label("Commands")
                            .max_h(px(220.))
                            .p(px(4.))
                            .flex()
                            .flex_col()
                            .children(groups),
                    )
                    .when(state.visible_indices().is_empty(), |command| {
                        command.child(
                            div()
                                .py(px(24.))
                                .flex()
                                .justify_center()
                                .text_color(theme.muted_foreground)
                                .child("No results found."),
                        )
                    }),
            )
            .child(caption(theme, self.last.clone()))
    }
}

pub fn command(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let commands = cx.new(|cx| {
            CommandState::new(
                [
                    SearchItem::new("calendar", "Calendar").group("Suggestions"),
                    SearchItem::new("emoji", "Search emoji")
                        .keywords(["smile"])
                        .group("Suggestions"),
                    SearchItem::new("calculator", "Calculator")
                        .disabled(true)
                        .group("Suggestions"),
                    SearchItem::new("profile", "Profile").group("Settings"),
                    SearchItem::new("billing", "Billing").group("Settings"),
                    SearchItem::new("settings", "Settings")
                        .keywords(["preferences"])
                        .group("Settings"),
                ],
                window,
                cx,
            )
        });
        commands
            .read(cx)
            .input()
            .clone()
            .update(cx, |input, cx| style_editor(input, theme, cx));
        let subscriptions = [
            cx.observe(&commands, |_, _, cx| cx.notify()),
            cx.subscribe(&commands, |this: &mut CommandDemo, commands, event, cx| {
                let CommandEvent::Action(value) = event;
                let label = commands
                    .read(cx)
                    .items()
                    .iter()
                    .find(|item| item.value() == value)
                    .map(|item| item.label().clone())
                    .unwrap_or_default();
                this.last = format!("Ran: {label}").into();
                cx.notify();
            }),
        ];
        CommandDemo {
            theme,
            commands,
            last: "Type to filter; ↑ ↓ to choose, Enter to run.".into(),
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Dialog

struct DialogDemo {
    theme: Theme,
    dialog: Entity<DialogState>,
    name: Entity<InputState>,
    username: Entity<InputState>,
    saved: Option<SharedString>,
    _observe: Subscription,
}

impl DialogDemo {
    fn field(
        &self,
        id: &'static str,
        label: &'static str,
        input: &Entity<InputState>,
        window: &Window,
        cx: &App,
    ) -> Div {
        let theme = self.theme.resolve(cx);
        let focus = input.focus_handle(cx);
        div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(Label::new(id, &focus, label).font_weight(FontWeight::MEDIUM))
            .child(
                div()
                    .h(px(36.))
                    .px(px(12.))
                    .flex()
                    .items_center()
                    .rounded(px(6.))
                    .border_1()
                    .border_color(if focus.is_focused(window) {
                        theme.ring
                    } else {
                        theme.border
                    })
                    .child(Input::new(input)),
            )
    }
}

impl Render for DialogDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let description = "Make changes to your profile here. Click save when you're done.";
        frame(theme)
            .flex_col()
            .child(
                outline(theme, Button::new("open-profile"))
                    .accessibility_label("Edit profile")
                    .child("Edit profile")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.dialog.update(cx, |state, cx| state.open(window, cx));
                    })),
            )
            .when_some(self.saved.clone(), |frame, name| {
                frame.child(caption(theme, format!("Saved profile for {name}.")))
            })
            .child(
                Dialog::new(&self.dialog)
                    .accessibility_label("Edit profile")
                    .accessibility_description(description)
                    .backdrop(modal_backdrop())
                    .popup(
                        modal_popup(theme)
                            .child(
                                DialogTitle::new()
                                    .text_size(px(18.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Edit profile"),
                            )
                            .child(
                                DialogDescription::new()
                                    .text_color(theme.muted_foreground)
                                    .child(description),
                            )
                            .child(self.field("name-label", "Name", &self.name, window, cx))
                            .child(self.field(
                                "username-label",
                                "Username",
                                &self.username,
                                window,
                                cx,
                            ))
                            .child(
                                div()
                                    .flex()
                                    .justify_end()
                                    .gap(px(8.))
                                    .child(DialogClose::new(&self.dialog).trigger(|button| {
                                        outline(theme, button)
                                            .accessibility_label("Cancel")
                                            .child("Cancel")
                                    }))
                                    .child(
                                        primary(theme, Button::new("save-profile"))
                                            .accessibility_label("Save changes")
                                            .child("Save changes")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.saved = Some(this.name.read(cx).value());
                                                this.dialog.update(cx, |state, cx| {
                                                    state.close(window, cx)
                                                });
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    ),
            )
    }
}

pub fn dialog(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let mut input = |label: &'static str, value: &'static str, cx: &mut Context<DialogDemo>| {
            cx.new(|cx| {
                let mut state = InputState::new(window, cx).default_value(value);
                style_editor(&mut state, theme, cx);
                state.set_accessibility(
                    InputAccessibility::new(Some(gpui::Role::TextInput))
                        .with_label(Some(label.into())),
                );
                state
            })
        };
        let name = input("Name", "Ada Park", cx);
        let username = input("Username", "@adapark", cx);
        let focus = name.focus_handle(cx);
        let dialog = cx.new(|cx| DialogState::new(cx).with_initial_focus(&focus));
        let observe = cx.observe(&dialog, |_, _, cx| cx.notify());
        DialogDemo {
            theme,
            dialog,
            name,
            username,
            saved: None,
            _observe: observe,
        }
    })
    .into()
}

// Alert dialog

struct AlertDialogDemo {
    theme: Theme,
    dialog: Entity<DialogState>,
    cancel: FocusHandle,
    deleted: bool,
    _observe: Subscription,
}

impl Render for AlertDialogDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let title = "Are you absolutely sure?";
        let description = "This action cannot be undone. This permanently deletes your account and removes your data from our servers.";
        frame(theme)
            .flex_col()
            .child(
                outline(theme, Button::new("delete-account"))
                    .accessibility_label("Delete account")
                    .child("Delete account")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.dialog.update(cx, |state, cx| state.open(window, cx));
                    })),
            )
            .when(self.deleted, |frame| {
                frame.child(caption(
                    theme,
                    "Account deleted. Nothing was sent anywhere.",
                ))
            })
            .child(
                AlertDialog::new(&self.dialog)
                    .accessibility_label(title)
                    .accessibility_description(description)
                    .backdrop(modal_backdrop())
                    .popup(
                        modal_popup(theme)
                            .gap(px(8.))
                            .child(
                                DialogTitle::new()
                                    .text_size(px(18.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(title),
                            )
                            .child(
                                DialogDescription::new()
                                    .text_color(theme.muted_foreground)
                                    .child(description),
                            )
                            .child(
                                div()
                                    .pt(px(8.))
                                    .flex()
                                    .justify_end()
                                    .gap(px(8.))
                                    .child(DialogClose::new(&self.dialog).trigger(|button| {
                                        outline(theme, button)
                                            .track_focus(&self.cancel)
                                            .accessibility_label("Cancel")
                                            .child("Cancel")
                                    }))
                                    .child(
                                        primary(theme, Button::new("confirm-delete"))
                                            .accessibility_label("Continue")
                                            .child("Continue")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.deleted = true;
                                                this.dialog.update(cx, |state, cx| {
                                                    state.close(window, cx)
                                                });
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    ),
            )
    }
}

pub fn alert_dialog(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let cancel = cx.focus_handle();
        let dialog = cx.new(|cx| DialogState::new(cx).with_initial_focus(&cancel));
        let observe = cx.observe(&dialog, |_, _, cx| cx.notify());
        AlertDialogDemo {
            theme,
            dialog,
            cancel,
            deleted: false,
            _observe: observe,
        }
    })
    .into()
}

// Drawer

struct DrawerDemo {
    theme: Theme,
    drawer: Entity<DrawerState>,
    submit: FocusHandle,
    goal: i32,
    saved: Option<i32>,
    _observe: Subscription,
}

impl DrawerDemo {
    fn step(
        &self,
        id: &'static str,
        label: &'static str,
        text: &'static str,
        delta: i32,
        cx: &mut Context<Self>,
    ) -> Button {
        let next = self.goal + delta;
        outline(self.theme.resolve(cx), Button::new(id))
            .accessibility_label(label)
            .size(px(32.))
            .px(px(0.))
            .rounded_full()
            .disabled(!(200..=500).contains(&next))
            .child(text)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.goal += delta;
                cx.notify();
            }))
    }
}

impl Render for DrawerDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let dialog = self.drawer.read(cx).dialog().clone();
        frame(theme)
            .flex_col()
            .child(
                outline(theme, Button::new("open-drawer"))
                    .accessibility_label("Open drawer")
                    .child("Open drawer")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.drawer.update(cx, |state, cx| state.open(window, cx));
                    })),
            )
            .when_some(self.saved, |frame, goal| {
                frame.child(caption(
                    theme,
                    format!("Daily goal set to {goal} calories."),
                ))
            })
            .child(
                Drawer::new("goal-drawer", &self.drawer)
                    .accessibility_label("Move goal")
                    .accessibility_description("Set your daily activity goal.")
                    .max_w(px(480.))
                    .bg(theme.background)
                    .text_color(theme.foreground)
                    .text_size(px(14.))
                    .border_1()
                    .border_color(theme.border)
                    .rounded_t(px(10.))
                    .backdrop(modal_backdrop())
                    .handle(
                        div()
                            .h(px(24.))
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().w(px(80.)).h(px(6.)).rounded_full().bg(theme.muted)),
                    )
                    .child(
                        div()
                            .px(px(24.))
                            .pb(px(20.))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(2.))
                                    .child(
                                        div()
                                            .text_size(px(18.))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Move goal"),
                                    )
                                    .child(caption(theme, "Set your daily activity goal.")),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(24.))
                                    .child(self.step(
                                        "goal-decrease",
                                        "Decrease goal",
                                        "−",
                                        -10,
                                        cx,
                                    ))
                                    .child(
                                        div()
                                            .w(px(120.))
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .child(
                                                div()
                                                    .text_size(px(40.))
                                                    .font_weight(FontWeight::BOLD)
                                                    .child(self.goal.to_string()),
                                            )
                                            .child(caption(theme, "calories/day")),
                                    )
                                    .child(self.step(
                                        "goal-increase",
                                        "Increase goal",
                                        "+",
                                        10,
                                        cx,
                                    )),
                            )
                            .child(
                                div()
                                    .w(px(280.))
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(
                                        primary(theme, Button::new("submit-goal"))
                                            .track_focus(&self.submit)
                                            .accessibility_label("Submit")
                                            .child("Submit")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.saved = Some(this.goal);
                                                this.drawer.update(cx, |state, cx| {
                                                    state.close(window, cx)
                                                });
                                                cx.notify();
                                            })),
                                    )
                                    .child(DialogClose::new(&dialog).trigger(|button| {
                                        outline(theme, button)
                                            .accessibility_label("Cancel")
                                            .child("Cancel")
                                    })),
                            ),
                    ),
            )
    }
}

pub fn drawer(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let submit = cx.focus_handle();
        let dialog = cx.new(|cx| DialogState::new(cx).with_initial_focus(&submit));
        let drawer = cx.new(|cx| DrawerState::with_dialog(dialog, window, cx));
        let observe = cx.observe(&drawer, |_, _, cx| cx.notify());
        DrawerDemo {
            theme,
            drawer,
            submit,
            goal: 350,
            saved: None,
            _observe: observe,
        }
    })
    .into()
}

// Toast

type Notice = (SharedString, SharedString);

struct ToastDemo {
    theme: Theme,
    toasts: Entity<ToasterState<Notice>>,
    _observe: Subscription,
}

impl Render for ToastDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme)
            .relative()
            .child(
                outline(theme, Button::new("show-toast"))
                    .accessibility_label("Show toast")
                    .child("Show toast")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toasts.update(cx, |toasts, cx| {
                            toasts.push(
                                (
                                    "Event has been created".into(),
                                    "Friday, November 20, 2026 at 9:00 AM".into(),
                                ),
                                window,
                                cx,
                            );
                        });
                    })),
            )
            .child(
                Toaster::new(
                    "notifications",
                    &self.toasts,
                    move |key, notice: &Notice, state| {
                        let (title, description) = notice;
                        Toast::new(key, state)
                            .accessibility_label(title.clone())
                            .w(px(320.))
                            .flex()
                            .items_center()
                            .gap(px(12.))
                            .p(px(16.))
                            .rounded(px(8.))
                            .border_1()
                            .border_color(theme.border)
                            .bg(theme.background)
                            .shadow_lg()
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.))
                                    .child(
                                        div()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child(title.clone()),
                                    )
                                    .child(caption(theme, description.clone())),
                            )
                            .child(
                                ToastClose::new((ElementId::from(key), "dismiss"), state).trigger(
                                    |button| {
                                        outline(theme, button)
                                            .h(px(32.))
                                            .px(px(12.))
                                            .accessibility_label("Dismiss")
                                            .child("Dismiss")
                                    },
                                ),
                            )
                    },
                )
                .accessibility_label("Notifications")
                .absolute()
                .bottom(px(16.))
                .right(px(16.))
                .gap(px(8.)),
            )
    }
}

pub fn toast(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let toasts = cx.new(|_| ToasterState::new());
        let observe = cx.observe(&toasts, |_, _, cx| cx.notify());
        ToastDemo {
            theme,
            toasts,
            _observe: observe,
        }
    })
    .into()
}
