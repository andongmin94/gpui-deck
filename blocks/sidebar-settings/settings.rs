//! A responsive settings workspace. Applied preferences live in this view.

mod palette;

use gpui::{
    App, AppContext as _, Context, Div, Entity, Focusable as _, FontWeight,
    InteractiveElement as _, IntoElement, ParentElement as _, Render, Role, SharedString,
    StatefulInteractiveElement as _, Styled as _, Subscription, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_deck::{
    Button, Field, Input, InputBase, InputEditorStyle, InputState, Switch, TabItem, TabsState,
};

use super::{
    Palette,
    recipes::{ButtonVariant, Sidebar},
};
use palette::{sidebar_palette, workspace_palette};

const NARROW: f32 = 700.;

#[derive(Clone, Copy, PartialEq)]
struct Preferences {
    compact: bool,
    mentions: bool,
    digest: bool,
}

#[derive(PartialEq)]
struct Workspace {
    name: SharedString,
    description: SharedString,
    preferences: Preferences,
}

pub(crate) struct SidebarSettings {
    sections: Entity<TabsState>,
    name: Entity<InputState>,
    description: Entity<InputState>,
    preferences: Preferences,
    applied: Workspace,
    _observations: [Subscription; 3],
}

impl SidebarSettings {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let applied = Workspace {
            name: "Fieldwork Studio".into(),
            description: "A shared space for the product design team.".into(),
            preferences: Preferences {
                compact: false,
                mentions: true,
                digest: false,
            },
        };
        let sections = cx.new(|cx| {
            TabsState::new(
                [
                    TabItem::new("general", "General"),
                    TabItem::new("appearance", "Appearance"),
                    TabItem::new("notifications", "Notifications"),
                ],
                cx,
            )
        });
        let p = workspace_palette();
        let mut make_input = |value: SharedString| {
            cx.new(|cx| {
                let mut input = InputState::new(window, cx).default_value(value);
                input.set_editor_style(InputEditorStyle {
                    foreground: p.foreground,
                    background: p.field,
                    caret: p.accent,
                    selection: p.highlight,
                    ..Default::default()
                });
                input
            })
        };
        let name = make_input(applied.name.clone());
        let description = make_input(applied.description.clone());
        let observations = [
            cx.observe(&sections, |_, _, cx| cx.notify()),
            cx.observe(&name, |_, _, cx| cx.notify()),
            cx.observe(&description, |_, _, cx| cx.notify()),
        ];
        Self {
            sections,
            name,
            description,
            preferences: applied.preferences,
            applied,
            _observations: observations,
        }
    }

    fn draft(&self, cx: &App) -> Workspace {
        Workspace {
            name: self.name.read(cx).value(),
            description: self.description.read(cx).value(),
            preferences: self.preferences,
        }
    }

    fn discard(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.name.update(cx, |input, cx| {
            input.set_value(self.applied.name.clone(), window, cx)
        });
        self.description.update(cx, |input, cx| {
            input.set_value(self.applied.description.clone(), window, cx)
        });
        self.preferences = self.applied.preferences;
        cx.notify();
    }

    fn preference_row(
        &mut self,
        label: &'static str,
        description: &'static str,
        access: fn(&mut Preferences) -> &mut bool,
        p: Palette,
        cx: &mut Context<Self>,
    ) -> Div {
        let checked = *access(&mut self.preferences);
        let view = cx.entity();
        let toggle = Switch::new(label)
            .accessibility_label(label)
            .checked(checked)
            .w(px(46.))
            .h(px(26.))
            .flex_shrink_0()
            .flex()
            .items_center()
            .p(px(3.))
            .rounded_full()
            .border_1()
            .border_color(p.muted)
            .bg(if checked { p.accent } else { p.muted })
            .when(checked, |track| track.justify_end())
            .focus_visible(|style| style.border_2().border_color(p.foreground))
            .child(div().size(px(18.)).rounded_full().bg(p.surface))
            .on_change(move |next, _, _, cx| {
                view.update(cx, |this, cx| {
                    *access(&mut this.preferences) = next;
                    cx.notify();
                })
            });
        div()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .gap_4()
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child(label))
                    .child(div().text_sm().text_color(p.muted).child(description)),
            )
            .child(toggle)
    }
}

fn text_field(
    id: &'static str,
    label: &'static str,
    input: &Entity<InputState>,
    required: bool,
    p: Palette,
    window: &Window,
    cx: &App,
) -> Field<gpui_deck::input::InputMode> {
    let editor = InputBase::new((gpui::ElementId::from(id), "editor"))
        .role(None::<Role>)
        .semantic_owner::<Input>(input.entity_id())
        .focused(input.focus_handle(cx).is_focused(window))
        .h(p.control_height)
        .w_full()
        .px_3()
        .rounded(p.radius)
        .border_1()
        .border_color(p.border)
        .bg(p.field)
        .styles(|styles| styles.focused(|style| style.border_color(p.accent)))
        .child(Input::new(input));
    Field::new(id, input, label)
        .required(required)
        .error(
            (required && input.read(cx).value().trim().is_empty())
                .then(|| "Enter a workspace name.".into()),
        )
        .styles(|style| {
            style
                .label(|label| label.font_weight(FontWeight::SEMIBOLD))
                .error(|error| error.text_color(p.error))
        })
        .w_full()
        .flex()
        .flex_col()
        .gap_2()
        .child(editor)
}

impl Render for SidebarSettings {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = workspace_palette();
        let nav = sidebar_palette(p);
        let narrow = window.viewport_size().width < px(NARROW);
        let current = self.sections.read(cx).selected_item().unwrap();
        let title = current.label().clone();
        let section = current.value().clone();
        let draft = self.draft(cx);
        let dirty = draft != self.applied;
        let valid = !draft.name.trim().is_empty();
        let initials: String = self
            .applied
            .name
            .split_whitespace()
            .take(2)
            .filter_map(|word| word.chars().next())
            .collect();
        let identity = div()
            .min_w_0()
            .when(narrow, |identity| identity.flex_1())
            .flex()
            .items_center()
            .gap_3()
            .child(
                div()
                    .size(px(38.))
                    .flex_shrink_0()
                    .rounded(nav.radius)
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(nav.accent)
                    .text_color(nav.surface)
                    .font_weight(FontWeight::BOLD)
                    .child(initials),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(div().text_sm().text_color(nav.muted).child("Workspace"))
                    .child(
                        div()
                            .truncate()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(self.applied.name.clone()),
                    ),
            );
        let sidebar = Sidebar::new("settings-nav", &self.sections, "Browse settings", nav)
            .group("Settings", ["general", "appearance", "notifications"])
            .w_full()
            .sheet_below(px(NARROW));
        let rail = div()
            .bg(nav.surface)
            .text_color(nav.foreground)
            .p_5()
            .flex()
            .gap_5()
            .when(narrow, |el| el.w_full().items_center().justify_between())
            .when(!narrow, |el| el.w(px(230.)).flex_shrink_0().flex_col())
            .child(identity)
            .child(sidebar);
        let mut fields = div()
            .flex()
            .flex_col()
            .gap(px(if self.applied.preferences.compact {
                12.
            } else {
                24.
            }));
        let subtitle = match section.as_ref() {
            "appearance" => {
                fields = fields.child(self.preference_row(
                    "Compact spacing",
                    "Reduce the space between settings in this workspace.",
                    |prefs| &mut prefs.compact,
                    p,
                    cx,
                ));
                "Adjust the layout of your workspace."
            }
            "notifications" => {
                fields = fields
                    .child(self.preference_row(
                        "Mentions",
                        "Include direct mentions in your notification preferences.",
                        |prefs| &mut prefs.mentions,
                        p,
                        cx,
                    ))
                    .child(self.preference_row(
                        "Weekly digest",
                        "Include a weekly summary in your notification preferences.",
                        |prefs| &mut prefs.digest,
                        p,
                        cx,
                    ));
                "Choose the updates you want to receive."
            }
            _ => {
                fields = fields
                    .child(text_field(
                        "workspace-name",
                        "Workspace name",
                        &self.name,
                        true,
                        p,
                        window,
                        cx,
                    ))
                    .child(text_field(
                        "workspace-description",
                        "Description",
                        &self.description,
                        false,
                        p,
                        window,
                        cx,
                    ));
                "Manage the name and description of your workspace."
            }
        };
        let status = if !valid {
            "General requires a workspace name."
        } else if dirty {
            "Unapplied changes"
        } else {
            "Settings applied to this session"
        };
        let buttons = div()
            .flex()
            .gap_2()
            .child(
                ButtonVariant::Outline
                    .apply(Button::new("discard-settings"), p)
                    .disabled(!dirty)
                    .accessibility_label("Discard changes")
                    .child("Discard")
                    .on_click(cx.listener(|this, _, window, cx| this.discard(window, cx))),
            )
            .child(
                ButtonVariant::Primary
                    .apply(Button::new("apply-settings"), p)
                    .disabled(!dirty || !valid)
                    .accessibility_label("Apply changes")
                    .child("Apply changes")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.applied = this.draft(cx);
                        cx.notify();
                    })),
            );
        let form = div()
            .min_w_0()
            .flex_1()
            .bg(p.surface)
            .flex()
            .flex_col()
            .gap_6()
            .p(px(if narrow { 20. } else { 36. }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .id("settings-title")
                            .role(Role::Heading)
                            .aria_level(1)
                            .aria_label(title.clone())
                            .text_size(px(32.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(title),
                    )
                    .child(div().text_color(p.muted).child(subtitle)),
            )
            .child(div().h(px(1.)).bg(p.border))
            .child(fields)
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .pt_5()
                    .border_t_1()
                    .border_color(p.border)
                    .child(
                        div()
                            .id("settings-status")
                            .role(Role::Status)
                            .aria_label(status)
                            .text_sm()
                            .text_color(p.muted)
                            .child(status),
                    )
                    .child(buttons),
            );
        div()
            .id("sidebar-settings")
            .size_full()
            .overflow_y_scroll()
            .bg(p.canvas)
            .text_color(p.foreground)
            .text_size(p.font_size)
            .child(
                div()
                    .w_full()
                    .min_h_full()
                    .flex()
                    .items_start()
                    .justify_center()
                    .p(px(if narrow { 12. } else { 40. }))
                    .child(
                        div()
                            .w_full()
                            .max_w(px(1100.))
                            .min_h(px(if narrow { 0. } else { 500. }))
                            .flex()
                            .when(narrow, |el| el.flex_col())
                            .overflow_hidden()
                            .rounded_lg()
                            .border_1()
                            .border_color(p.border)
                            .bg(p.surface)
                            .child(rail)
                            .child(form),
                    ),
            )
    }
}
