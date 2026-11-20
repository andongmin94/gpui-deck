//! Application-owned navigation composed from TabsState, Button and Collapsible.

use gpui::{
    AnyElement, AnyView, App, AppContext as _, Context, Div, ElementId, Entity, FontWeight,
    InteractiveElement as _, IntoElement, ParentElement as _, Render, RenderOnce, Role,
    SharedString, Stateful, StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, rems,
};
use gpui_design::{
    Button, Collapsible, CollapsibleContent, CollapsibleState, CollapsibleTrigger, ObservedElement,
    TestSupportExt as _,
};

use std::rc::Rc;

use super::super::Palette;

/// A navigation sidebar whose selection is owned by an existing `TabsState`.
///
/// Unlike `TabList`, this recipe uses ordinary independent Buttons:
/// Tab and Shift-Tab traverse enabled entries, Enter/Space activate the
/// focused entry, and arrow keys do not change the selected section.
///
/// `Button::selected` controls presentation; `Button::current` separately exposes
/// the current page to assistive technology. The navigation landmark and its
/// Buttons retain their native roles, without tab selection semantics.
///
/// The caller must retain the supplied `TabsState` and render only the
/// content corresponding to its selected item. Each Button uses that state's
/// `tab_focus_handle(index)` for explicit focus restoration. Do not reuse this
/// state with `TabList`, `Tab`, or `TabPanel`: TabList installs automatic focus
/// selection, while this Sidebar activates only on Enter/Space or a click.
///
/// Optional named groups and whole-sidebar/group disclosures compose the existing
/// `CollapsibleState`. The caller retains and observes those states. Closing a
/// disclosure removes its navigation Buttons and returns contained focus to its
/// trigger without changing the current section. Ungrouped entries render first;
/// each group lists existing, unique item values in its desired display order.
/// Expand a closed disclosure before programmatically focusing an entry in it.
#[derive(IntoElement)]
pub(crate) struct Sidebar {
    base: ObservedElement<Stateful<Div>>,
    id: ElementId,
    state: Entity<gpui_design::TabsState>,
    label: SharedString,
    palette: Palette,
    collapse: Option<Entity<CollapsibleState>>,
    groups: Vec<SidebarGroup>,
    rail: Option<Entity<CollapsibleState>>,
    icons: Vec<(SharedString, SidebarIcon)>,
}

type SidebarIcon = Rc<dyn Fn() -> AnyElement>;

struct SidebarGroup {
    label: SharedString,
    values: Vec<SharedString>,
    collapse: Option<Entity<CollapsibleState>>,
}

impl Sidebar {
    pub(crate) fn new(
        id: impl Into<ElementId>,
        state: &Entity<gpui_design::TabsState>,
        label: impl Into<SharedString>,
        palette: Palette,
    ) -> Self {
        let id = id.into();
        Self {
            base: div()
                .id(id.clone())
                .test_support()
                .w(rems(10.))
                .min_w_0()
                .flex()
                .flex_col()
                .items_stretch()
                .gap_1()
                .text_size(palette.font_size)
                .text_color(palette.foreground),
            id,
            state: state.clone(),
            label: label.into(),
            palette,
            collapse: None,
            groups: Vec::new(),
            rail: None,
            icons: Vec::new(),
        }
    }

    /// Keeps a native disclosure trigger mounted when navigation is hidden.
    #[allow(dead_code)] // Optional in consumers that use only flat navigation.
    pub(crate) fn collapsible(mut self, state: &Entity<CollapsibleState>) -> Self {
        self.collapse = Some(state.clone());
        self
    }

    pub(crate) fn group(
        mut self,
        label: impl Into<SharedString>,
        values: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.groups.push(SidebarGroup {
            label: label.into(),
            values: values.into_iter().map(Into::into).collect(),
            collapse: None,
        });
        self
    }

    /// Lets the sidebar collapse to an icon rail, like shadcn's `collapsible="icon"`.
    /// The open state shows labels; the closed state keeps every entry mounted
    /// as an icon-only Button with the same accessible name, current-page
    /// metadata and focus, plus a hover tooltip naming it. A trigger after the
    /// entries toggles the state. Use either this or [`Self::collapsible`].
    #[allow(dead_code)] // Optional in consumers without an icon rail.
    pub(crate) fn rail(mut self, state: &Entity<CollapsibleState>) -> Self {
        self.rail = Some(state.clone());
        self
    }

    /// Supplies a decorative icon for the entry with `value`, drawn before its
    /// label and alone in the rail. Entries without one show their initial.
    #[allow(dead_code)] // Optional in consumers without icons.
    pub(crate) fn icon(
        mut self,
        value: impl Into<SharedString>,
        icon: impl Fn() -> AnyElement + 'static,
    ) -> Self {
        self.icons.push((value.into(), Rc::new(icon)));
        self
    }

    #[allow(dead_code)] // Optional in consumers that use only static groups.
    pub(crate) fn collapsible_group(
        mut self,
        label: impl Into<SharedString>,
        values: impl IntoIterator<Item = impl Into<SharedString>>,
        state: &Entity<CollapsibleState>,
    ) -> Self {
        self.groups.push(SidebarGroup {
            label: label.into(),
            values: values.into_iter().map(Into::into).collect(),
            collapse: Some(state.clone()),
        });
        self
    }
}

impl Styled for Sidebar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        assert!(
            self.rail.is_none() || self.collapse.is_none(),
            "Use either a Sidebar rail or a whole-sidebar disclosure"
        );
        let model = self.state.read(cx);
        let selected = model.selected_index();
        let items = model.items().to_vec();
        let palette = self.palette;
        let mut grouped_values = Vec::new();
        for value in self.groups.iter().flat_map(|group| &group.values) {
            assert!(
                !grouped_values.contains(value),
                "Sidebar group item values must be unique"
            );
            grouped_values.push(value.clone());
        }

        let collapsed = self
            .rail
            .as_ref()
            .is_some_and(|rail| !rail.read(cx).is_open());
        let show_icons = self.rail.is_some() || !self.icons.is_empty();
        let icon = |value: &SharedString, label: &SharedString| -> AnyElement {
            match self
                .icons
                .iter()
                .find(|(icon_value, _)| icon_value == value)
            {
                Some((_, build)) => build(),
                None => div()
                    .size(rems(1.5))
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(palette.radius)
                    .border_1()
                    .border_color(palette.border)
                    .text_xs()
                    .child(label.chars().next().map(String::from).unwrap_or_default())
                    .into_any_element(),
            }
        };
        let button = |index: usize| {
            let item = &items[index];
            let value = item.value().clone();
            let label = item.label().clone();
            let disabled = item.is_disabled();
            let is_selected = selected == Some(index);
            let state = self.state.clone();

            Button::new((self.id.clone(), index.to_string()))
                .accessibility_label(label.clone())
                .track_focus(model.tab_focus_handle(index))
                .selected(is_selected)
                .current(is_selected)
                .disabled(disabled)
                .w_full()
                .min_w_0()
                .justify_start()
                .items_start()
                .px_2()
                .py_2()
                .rounded(palette.radius)
                .text_color(palette.foreground)
                .when(is_selected, |button| button.bg(palette.highlight))
                .when(disabled, |button| button.opacity(0.45))
                .focus_visible(|style| style.border_1().border_color(palette.accent))
                .when(show_icons, |button| {
                    button.items_center().gap_2().child(icon(&value, &label))
                })
                .when(collapsed, |button| {
                    let tip = label.clone();
                    button.justify_center().tooltip(move |_, cx| {
                        let label = tip.clone();
                        AnyView::from(cx.new(|_| RailTooltip { label, palette }))
                    })
                })
                .when(!collapsed, |button| {
                    button.child(div().min_w_0().whitespace_normal().child(label))
                })
                .on_click(move |_, window, cx| {
                    state.update(cx, |state, cx| {
                        state.select_value(value.as_ref(), window, cx);
                    });
                })
        };

        let mut content = div().w_full().min_w_0().flex().flex_col().gap_1().children(
            items
                .iter()
                .enumerate()
                .filter(|(_, item)| !grouped_values.contains(item.value()))
                .map(|(index, _)| button(index)),
        );
        for (index, group) in self.groups.iter().enumerate() {
            let id = (self.id.clone(), format!("group-{index}"));
            let buttons = group.values.iter().map(|value| {
                let index = items
                    .iter()
                    .position(|item| item.value() == value)
                    .expect("Sidebar groups must reference existing item values");
                button(index)
            });
            let group_content: AnyElement =
                if let Some(state) = group.collapse.as_ref().filter(|_| !collapsed) {
                    Collapsible::new(id, state)
                        .w_full()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(disclosure_trigger(state, group.label.clone(), palette, cx))
                        .child(
                            CollapsibleContent::new(state)
                                .accessibility_label(group.label.clone())
                                .w_full()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .children(buttons),
                        )
                        .into_any_element()
                } else {
                    div()
                        .id(id)
                        .test_support()
                        .role(Role::Group)
                        .aria_label(group.label.clone())
                        .w_full()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .when(!collapsed, |group_frame| {
                            group_frame.child(
                                div()
                                    .id((self.id.clone(), format!("group-{index}-heading")))
                                    .test_support()
                                    .role(Role::Heading)
                                    .aria_level(3)
                                    .aria_label(group.label.clone())
                                    .min_w_0()
                                    .px_2()
                                    .py_1()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(palette.muted)
                                    .child(group.label.clone()),
                            )
                        })
                        .children(buttons)
                        .into_any_element()
                };
            content = content.child(group_content);
        }

        let content = if let Some(state) = self.collapse {
            Collapsible::new((self.id.clone(), "disclosure"), &state)
                .w_full()
                .min_w_0()
                .flex()
                .flex_col()
                .gap_1()
                .child(disclosure_trigger(&state, self.label.clone(), palette, cx))
                .child(
                    CollapsibleContent::new(&state)
                        .accessibility_label(self.label.clone())
                        .w_full()
                        .min_w_0()
                        .child(content),
                )
                .into_any_element()
        } else {
            content.into_any_element()
        };

        let rail_trigger = self.rail.as_ref().map(|state| {
            CollapsibleTrigger::new(state)
                .accessibility_label(format!("{} labels", self.label))
                .w_full()
                .min_w_0()
                .justify_center()
                .px_2()
                .py_2()
                .rounded(palette.radius)
                .text_color(palette.muted)
                .hover(|style| style.bg(palette.highlight))
                .focus_visible(|style| style.border_1().border_color(palette.accent))
                .child(if collapsed { "»" } else { "« Collapse" })
        });

        self.base
            .role(Role::Navigation)
            .aria_label(self.label)
            .when(collapsed, |sidebar| sidebar.w(rems(3.5)))
            .child(content)
            .children(rail_trigger)
    }
}

/// The rail's hover label, drawn by GPUI's tooltip layer.
struct RailTooltip {
    label: SharedString,
    palette: Palette,
}

impl Render for RailTooltip {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_2()
            .py_1()
            .rounded(self.palette.radius)
            .border_1()
            .border_color(self.palette.border)
            .bg(self.palette.surface)
            .text_color(self.palette.foreground)
            .text_size(self.palette.font_size)
            .child(self.label.clone())
    }
}

fn disclosure_trigger(
    state: &Entity<CollapsibleState>,
    label: SharedString,
    palette: Palette,
    cx: &App,
) -> CollapsibleTrigger {
    let indicator = if state.read(cx).is_open() { "−" } else { "+" };
    CollapsibleTrigger::new(state)
        .accessibility_label(label.clone())
        .w_full()
        .min_w_0()
        .justify_start()
        .items_start()
        .gap_2()
        .px_2()
        .py_2()
        .rounded(palette.radius)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(palette.foreground)
        .hover(|style| style.bg(palette.highlight))
        .focus_visible(|style| style.border_1().border_color(palette.accent))
        .when(state.read(cx).is_disabled(), |trigger| {
            trigger.opacity(0.45)
        })
        .child(div().flex_1().min_w_0().whitespace_normal().child(label))
        .child(indicator)
}

#[cfg(all(test, feature = "test-support"))]
mod tests {
    use gpui::{Render, Subscription, TestAppContext, px};
    use gpui_design::{Root, TabItem, TabsState, test_support};

    use super::super::super::Appearance;
    use super::*;

    struct Sample {
        tabs: Entity<TabsState>,
        rail: Entity<CollapsibleState>,
        _subscriptions: Vec<Subscription>,
    }

    impl Render for Sample {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Sidebar::new(
                "rail-sample",
                &self.tabs,
                "Sections",
                Appearance::Settings.palette(),
            )
            .group("Workspace", ["general", "members"])
            .rail(&self.rail)
        }
    }

    #[gpui::test]
    fn the_rail_keeps_named_current_entries_and_focus_while_narrow(cx: &mut TestAppContext) {
        cx.update(gpui_design::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let sample = cx.new(|cx| {
                let tabs = cx.new(|cx| {
                    TabsState::new(
                        [
                            TabItem::new("general", "General"),
                            TabItem::new("members", "Members"),
                        ],
                        cx,
                    )
                });
                let rail = cx.new(|cx| CollapsibleState::new(cx).default_open(true));
                let subscriptions = vec![
                    cx.observe(&tabs, |_, _, cx| cx.notify()),
                    cx.observe(&rail, |_, _, cx| cx.notify()),
                ];
                Sample {
                    tabs,
                    rail,
                    _subscriptions: subscriptions,
                }
            });
            Root::new(sample, window, cx)
        });
        let sample = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Sample>()
            .unwrap();
        let entry =
            |index: usize| ElementId::from((ElementId::from("rail-sample"), index.to_string()));
        cx.update(|window, cx| {
            window.activate_window();
            window.draw(cx).clear(cx);
            let expanded = test_support::find(window, &[], &"rail-sample".into())
                .unwrap()
                .bounds()
                .size
                .width;
            assert!(expanded > px(100.));
            assert!(
                test_support::snapshots(window)
                    .iter()
                    .any(|node| node.role() == Some(Role::Heading))
            );
            let focus = sample.read(cx).tabs.read(cx).tab_focus_handle(1).clone();
            focus.focus(window, cx);
            let rail = sample.read(cx).rail.clone();
            rail.update(cx, |rail, cx| rail.set_open(false, window, cx));
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|window, cx| {
            let frame = test_support::find(window, &[], &"rail-sample".into()).unwrap();
            assert_eq!(frame.role(), Some(Role::Navigation));
            assert!(frame.bounds().size.width < px(80.), "the rail is narrow");
            for (index, label) in [(0, "General"), (1, "Members")] {
                let button = test_support::find(window, &[], &entry(index)).unwrap();
                assert_eq!(button.role(), Some(Role::Button));
                assert_eq!(button.label(), Some(label), "icon-only entries keep names");
            }
            assert!(
                sample
                    .read(cx)
                    .tabs
                    .read(cx)
                    .tab_focus_handle(1)
                    .is_focused(window),
                "collapsing keeps focus on the mounted entry"
            );
            let group = test_support::find(window, &[], &"rail-sample".into())
                .map(|_| {
                    test_support::snapshots(window)
                        .into_iter()
                        .filter(|node| node.role() == Some(Role::Heading))
                        .count()
                })
                .unwrap();
            assert_eq!(group, 0, "group headings are hidden in the rail");
            let trigger = test_support::snapshots(window)
                .into_iter()
                .find(|node| node.label() == Some("Sections labels"))
                .unwrap();
            assert_eq!(trigger.expanded(), Some(false));
        });
    }
}
