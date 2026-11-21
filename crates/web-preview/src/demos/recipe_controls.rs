//! Interactive previews of the source recipes installed by the CLI.

use gpui::{
    AnyView, App, AppContext as _, Context, Entity, FocusHandle, Focusable as _, IntoElement,
    ParentElement as _, Render, ScrollHandle, SharedString, Styled as _, Subscription, Window, div,
    px,
};
use gpui_design::{
    Button, CollapsibleState, DialogClose, DialogState, Input, InputState, TabItem, TabsState,
};

use super::{data::text_input, frame, outline};
use crate::{
    recipes::{CodeBlock, InputGroup, Pagination, Sheet, Sidebar, Trace},
    theme::Theme,
};

pub(super) const NAMES: &[&str] = &[
    "sheet",
    "sidebar",
    "trace",
    "input-group",
    "code-block",
    "pagination",
];

pub(super) fn open(name: &str, theme: Theme, window: &mut Window, cx: &mut App) -> Option<AnyView> {
    Some(match name {
        "sheet" => cx
            .new(|cx| {
                let close = cx.focus_handle().tab_stop(true);
                let dialog = cx.new(|cx| DialogState::new(cx).with_initial_focus(&close));
                let observe = cx.observe(&dialog, |_, _, cx| cx.notify());
                SheetDemo {
                    theme,
                    dialog,
                    close,
                    scroll: ScrollHandle::new(),
                    _observe: observe,
                }
            })
            .into(),
        "sidebar" => cx
            .new(|cx| {
                let sections = cx.new(|cx| {
                    TabsState::new(
                        [
                            TabItem::new("overview", "Overview"),
                            TabItem::new("members", "Members"),
                            TabItem::new("projects", "Projects"),
                        ],
                        cx,
                    )
                });
                let rail = cx.new(|cx| CollapsibleState::new(cx).default_open(true));
                let observers = [
                    cx.observe(&sections, |_, _, cx| cx.notify()),
                    cx.observe(&rail, |_, _, cx| cx.notify()),
                ];
                SidebarDemo {
                    theme,
                    sections,
                    rail,
                    _observers: observers,
                }
            })
            .into(),
        "trace" => cx
            .new(|cx| {
                let disclosure = cx.new(CollapsibleState::new);
                let observe = cx.observe(&disclosure, |_, _, cx| cx.notify());
                TraceDemo {
                    theme,
                    disclosure,
                    _observe: observe,
                }
            })
            .into(),
        "input-group" => cx
            .new(|cx| {
                let input = text_input(theme, "Username", "Username", window, cx);
                let observe = cx.observe(&input, |_, _, cx| cx.notify());
                InputGroupDemo {
                    theme,
                    input,
                    _observe: observe,
                }
            })
            .into(),
        "code-block" => cx
            .new(|cx| CodeBlockDemo {
                theme,
                source: include_str!("../../../base/examples/starter.rs")
                    .replace("\r\n", "\n")
                    .into(),
                scroll: ScrollHandle::new(),
                focus: cx.focus_handle().tab_stop(true),
            })
            .into(),
        "pagination" => cx.new(|_| PaginationDemo { theme, page: 0 }).into(),
        _ => return None,
    })
}

struct SheetDemo {
    theme: Theme,
    dialog: Entity<DialogState>,
    close: FocusHandle,
    scroll: ScrollHandle,
    _observe: Subscription,
}

impl Render for SheetDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let shortcuts = Sheet::new(
            "shortcuts",
            &self.dialog,
            &self.scroll,
            "Keyboard shortcuts",
            theme.recipe_palette(),
        )
        .w(px(300.))
        .child("Tab — Move to the next control")
        .child("Shift + Tab — Move to the previous control")
        .child("Esc — Close the panel")
        .footer(DialogClose::new(&self.dialog).trigger(|button| {
            outline(theme, button)
                .track_focus(&self.close)
                .accessibility_label("Close shortcuts")
                .child("Close")
        }));
        frame(theme)
            .child(
                outline(theme, Button::new("open-shortcuts"))
                    .accessibility_label("Keyboard shortcuts")
                    .child("Keyboard shortcuts")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.dialog.update(cx, |dialog, cx| dialog.open(window, cx));
                    })),
            )
            .child(shortcuts)
    }
}

struct SidebarDemo {
    theme: Theme,
    sections: Entity<TabsState>,
    rail: Entity<CollapsibleState>,
    _observers: [Subscription; 2],
}

impl Render for SidebarDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let state = self.sections.read(cx);
        let section = state.selected_item().unwrap();
        let content = match section.value().as_ref() {
            "members" => "Ada Park · Owner",
            "projects" => "Design system · In progress",
            _ => "1 member · 1 project",
        };
        frame(theme)
            .child(
                Sidebar::new(
                    "workspace",
                    &self.sections,
                    "Workspace",
                    theme.recipe_palette(),
                )
                .group("Workspace", ["overview", "members", "projects"])
                .sheet_below(px(640.))
                .rail(&self.rail),
            )
            .child(
                div()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(section.label().clone())
                    .child(div().text_color(theme.muted_foreground).child(content)),
            )
    }
}

struct TraceDemo {
    theme: Theme,
    disclosure: Entity<CollapsibleState>,
    _observe: Subscription,
}

impl Render for TraceDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme).child(
            Trace::new(
                "index-trace",
                &self.disclosure,
                "Indexed 3 documents",
                "Indexed files",
                "README.md\nCargo.toml\nLICENSE-APACHE",
                theme.recipe_palette(),
            )
            .max_w(px(400.)),
        )
    }
}

struct InputGroupDemo {
    theme: Theme,
    input: Entity<InputState>,
    _observe: Subscription,
}

impl Render for InputGroupDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let clear = outline(theme, Button::new("clear-username"))
            .accessibility_label("Clear username")
            .child("Clear")
            .disabled(self.input.read(cx).value().is_empty())
            .on_click(cx.listener(|this, _, window, cx| {
                this.input.update(cx, |input, cx| {
                    input.set_value("", window, cx);
                    input.focus(window, cx);
                });
            }));
        frame(theme).child(
            InputGroup::new("username", Input::new(&self.input), theme.recipe_palette())
                .max_w(px(360.))
                .focused(self.input.focus_handle(cx).is_focused(window))
                .leading("@")
                .trailing(clear),
        )
    }
}

struct CodeBlockDemo {
    theme: Theme,
    source: SharedString,
    scroll: ScrollHandle,
    focus: FocusHandle,
}

impl Render for CodeBlockDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme).child(
            CodeBlock::new(
                "starter-source",
                self.source.clone(),
                "starter.rs",
                &self.scroll,
                &self.focus,
                theme.recipe_palette(),
            )
            .max_w(px(600.)),
        )
    }
}

struct PaginationDemo {
    theme: Theme,
    page: usize,
}

impl Render for PaginationDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let files = [
            ["accordion.rs", "alert_dialog.rs", "breadcrumb.rs"],
            ["button.rs", "calendar.rs", "carousel.rs"],
            ["chart.rs", "checkbox.rs", "collapsible.rs"],
        ];
        let owner = cx.entity();
        frame(theme)
            .flex_col()
            .child(div().flex().flex_col().gap_1().children(files[self.page]))
            .child(
                Pagination::new(
                    "files-pages",
                    "Source file pages",
                    self.page,
                    files.len(),
                    theme.recipe_palette(),
                )
                .on_change(move |page, _, cx| {
                    owner.update(cx, |this, cx| {
                        this.page = page;
                        cx.notify();
                    });
                }),
            )
    }
}
