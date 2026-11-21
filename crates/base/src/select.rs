//! Selection behavior and composable, unstyled trigger/listbox parts.
//! Modified for GPUI Design: retained option navigation and focus lifecycle.

use std::{collections::HashSet, time::Duration};

use gpui::{
    AccessibleAction, AnyElement, App, ClickEvent, Context, ElementId, Entity, EventEmitter,
    FocusHandle, InteractiveElement, Interactivity, IntoElement, KeyBinding, KeyDownEvent,
    MouseButton, ParentElement, Pixels, RenderOnce, Role, ScrollHandle, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Subscription, Window, div,
    prelude::FluentBuilder as _, px,
};
use web_time::Instant;

use crate::{
    ElementExt as _, Popup, ScrollbarAxis, StyledExt as _, TestSupportExt as _,
    a11y_disabled::A11yDisabledExt as _,
    actions::{Cancel, Confirm, SelectDown, SelectFirst, SelectLast, SelectUp},
    global_state::{DeferredPopover, GlobalState},
    scroll_area::RevealViewport,
};

const CONTEXT: &str = "Select";

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("space", Confirm { secondary: false }, Some(CONTEXT)),
        KeyBinding::new("up", SelectUp, Some(CONTEXT)),
        KeyBinding::new("down", SelectDown, Some(CONTEXT)),
        KeyBinding::new("home", SelectFirst, Some(CONTEXT)),
        KeyBinding::new("end", SelectLast, Some(CONTEXT)),
        KeyBinding::new("enter", Confirm { secondary: false }, Some(CONTEXT)),
        KeyBinding::new("escape", Cancel, Some(CONTEXT)),
    ]);
}

/// One option in a select. Its unique value identifies it across list updates.
#[derive(Clone, Debug)]
pub struct SelectItem {
    value: SharedString,
    label: SharedString,
    disabled: bool,
}

impl SelectItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn value(&self) -> &SharedString {
        &self.value
    }

    pub fn label(&self) -> &SharedString {
        &self.label
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
}

fn collect_items(items: impl IntoIterator<Item = SelectItem>) -> Vec<SelectItem> {
    let items: Vec<_> = items.into_iter().collect();
    let mut values = HashSet::with_capacity(items.len());
    assert!(
        items.iter().all(|item| values.insert(&item.value)),
        "SelectItem values must be unique"
    );
    items
}

fn enabled_index(items: &[SelectItem], value: &str) -> Option<usize> {
    items
        .iter()
        .position(|item| !item.disabled && item.value.as_ref() == value)
}

/// Emitted once when a user commits a different value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectEvent {
    Change(SharedString),
}

/// Retained selection, keyboard cursor, disabled state and popup focus.
///
/// Applications can replace the ordered options with [`Self::set_items`]. Arrow keys wrap and
/// skip disabled items; opening highlights the committed item or first enabled
/// item. Enter/Space commits, Escape discards the pending cursor and restores
/// trigger focus. Home/End highlight an enabled edge while open. Typing searches
/// labels by a case-insensitive prefix, committing while closed and only moving
/// the cursor while open. Repeated letters cycle; a one-second pause resets the
/// prefix. The application owns option presentation, not this behavior.
pub struct SelectState {
    items: Vec<SelectItem>,
    selected: Option<usize>,
    highlighted: Option<usize>,
    open: bool,
    disabled: bool,
    open_on_pointer_down: bool,
    focus: FocusHandle,
    content_focus: FocusHandle,
    scroll: ScrollHandle,
    reveal_highlight: bool,
    deferred_context: Option<DeferredPopover>,
    dismiss_subscription: Option<Subscription>,
    search: String,
    searched_at: Option<Instant>,
}

impl SelectState {
    pub fn new(items: impl IntoIterator<Item = SelectItem>, cx: &mut Context<Self>) -> Self {
        Self {
            items: collect_items(items),
            selected: None,
            highlighted: None,
            open: false,
            disabled: false,
            open_on_pointer_down: false,
            focus: cx.focus_handle().tab_stop(true),
            content_focus: cx.focus_handle(),
            scroll: ScrollHandle::new(),
            reveal_highlight: false,
            deferred_context: None,
            dismiss_subscription: None,
            search: String::new(),
            searched_at: None,
        }
    }

    pub fn default_value(mut self, value: impl AsRef<str>) -> Self {
        self.selected = enabled_index(&self.items, value.as_ref());
        self
    }

    pub fn items(&self) -> &[SelectItem] {
        &self.items
    }

    /// Replaces the options without emitting [`SelectEvent::Change`]. A committed
    /// value survives while enabled. An open list keeps its highlighted value,
    /// then prefers the committed value or first enabled option if it disappeared.
    /// With no enabled options the popup closes using its normal focus policy.
    pub fn set_items(
        &mut self,
        items: impl IntoIterator<Item = SelectItem>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let items = collect_items(items);
        let remap = |index: usize| enabled_index(&items, &self.items[index].value);
        self.selected = self.selected.and_then(remap);
        self.highlighted = self.highlighted.and_then(remap);
        self.items = items;
        self.search.clear();
        self.searched_at = None;
        if self.open {
            self.highlighted = self
                .highlighted
                .or(self.selected)
                .or_else(|| self.items.iter().position(|item| !item.disabled));
            if self.highlighted.is_none() {
                self.close(window, cx);
                return;
            }
            self.reveal_highlight = true;
        }
        cx.notify();
        window.refresh();
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    pub fn highlighted_index(&self) -> Option<usize> {
        self.highlighted
    }

    pub fn selected_item(&self) -> Option<&SelectItem> {
        self.selected.map(|ix| &self.items[ix])
    }

    /// Sets or clears the committed value without emitting Change.
    ///
    /// Unknown or disabled options are rejected without changing the value,
    /// pending cursor, popup or focus. An accepted update closes the popup using
    /// its normal focus lifecycle and clears the typeahead prefix. Returns
    /// whether the value was accepted, including an unchanged value. Application
    /// updates remain available when the control itself is disabled.
    pub fn set_selected_value(
        &mut self,
        value: Option<&str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let selected = match value {
            Some(value) => {
                let Some(index) = enabled_index(&self.items, value) else {
                    return false;
                };
                Some(index)
            }
            None => None,
        };
        self.close(window, cx);
        self.selected = selected;
        self.search.clear();
        self.searched_at = None;
        cx.notify();
        window.refresh();
        true
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }

    pub fn content_focus_handle(&self) -> &FocusHandle {
        &self.content_focus
    }

    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled == disabled {
            return;
        }
        if disabled {
            self.close(window, cx);
        }
        self.disabled = disabled;
        cx.notify();
        window.refresh();
    }

    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.open {
            return;
        }
        self.highlighted = self
            .selected
            .or_else(|| self.items.iter().position(|item| !item.disabled));
        if self.highlighted.is_none() {
            return;
        }
        self.open = true;
        self.reveal_highlight = true;
        self.search.clear();
        self.searched_at = None;
        self.deferred_context = Some(GlobalState::register_deferred_popover(cx));
        self.dismiss_subscription =
            Some(
                cx.on_focus_out(&self.content_focus, window, |_, event, window, cx| {
                    // Focus callbacks run while drawing. Close after that phase so
                    // notify/refresh can remove the popup. A detached focus handle
                    // can remain focused; never restore its now-hidden trigger.
                    cx.defer_in(window, move |state, window, cx| {
                        let detached = event
                            .blurred
                            .upgrade()
                            .is_some_and(|focus| focus.is_focused(window));
                        if !window.is_window_active()
                            || detached
                            || !state.content_focus.contains_focused(window, cx)
                        {
                            state.dismiss(window, cx);
                        }
                    });
                }),
            );
        self.content_focus.focus(window, cx);
        cx.notify();
        window.refresh();
    }

    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            return;
        }
        // A pointer press or an ancestor closing may already have moved focus.
        // Do not move it back into a control the user has just left.
        if self.content_focus.is_focused(window) || self.content_focus.contains_focused(window, cx)
        {
            self.focus.focus(window, cx);
        }
        self.dismiss(window, cx);
    }

    fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            return;
        }
        self.open = false;
        self.highlighted = None;
        self.reveal_highlight = false;
        self.deferred_context = None;
        self.dismiss_subscription = None;
        self.search.clear();
        self.searched_at = None;
        cx.notify();
        window.refresh();
    }

    fn move_cursor(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        if !self.open {
            self.open(window, cx);
            return;
        }
        let Some(current) = self.highlighted else {
            return;
        };
        let len = self.items.len();
        let next = (1..=len)
            .map(|step| {
                if forward {
                    (current + step) % len
                } else {
                    (current + len - step) % len
                }
            })
            .find(|&ix| !self.items[ix].disabled);
        if let Some(index) = next {
            self.highlight(index, true, window, cx);
        }
    }

    fn highlight(&mut self, ix: usize, reveal: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open || self.disabled || self.items[ix].disabled {
            return;
        }
        if self.highlighted == Some(ix) && !reveal {
            return;
        }
        self.highlighted = Some(ix);
        self.reveal_highlight = reveal;
        cx.notify();
        window.refresh();
    }

    fn edge(&mut self, last: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            cx.propagate();
            return;
        }
        let index = if last {
            self.items.iter().rposition(|item| !item.disabled)
        } else {
            self.items.iter().position(|item| !item.disabled)
        };
        if let Some(index) = index {
            self.highlight(index, true, window, cx);
        }
    }

    fn typeahead(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled
            || event.prefer_character_input
            || (!self.focus.is_focused(window) && !self.content_focus.is_focused(window))
        {
            return;
        }
        let modifiers = event.keystroke.modifiers;
        if modifiers.control || modifiers.alt || modifiers.platform || modifiers.function {
            return;
        }
        let text = event
            .keystroke
            .key_char
            .as_deref()
            .unwrap_or(&event.keystroke.key);
        let mut characters = text.chars();
        let Some(character) = characters.next() else {
            return;
        };
        if characters.next().is_some() || character.is_control() || character.is_whitespace() {
            return;
        }
        let now = cx.background_executor().now();
        if self
            .searched_at
            .is_none_or(|at| now.duration_since(at) >= Duration::from_secs(1))
        {
            self.search.clear();
        }
        self.search.extend(character.to_lowercase());
        self.searched_at = Some(now);
        let first = self.search.chars().next().unwrap();
        let repeated = self.search.chars().all(|character| character == first);
        let prefix = if repeated {
            first.to_string()
        } else {
            self.search.clone()
        };
        let current = if self.open {
            self.highlighted
        } else {
            self.selected
        };
        let count = self.items.len();
        let start = current.unwrap_or(0);
        let index = (0..count)
            .map(|distance| (start + distance) % count)
            .filter(|index| !repeated || current != Some(*index))
            .find(|&index| {
                !self.items[index].disabled
                    && self.items[index].label.to_lowercase().starts_with(&prefix)
            });
        if let Some(index) = index {
            if self.open {
                self.highlight(index, true, window, cx);
            } else if self.selected != Some(index) {
                self.selected = Some(index);
                cx.emit(SelectEvent::Change(self.items[index].value.clone()));
                cx.notify();
                window.refresh();
            }
        }
        cx.stop_propagation();
    }

    fn commit(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open || self.disabled || self.items[ix].disabled {
            return;
        }
        let changed = self.selected != Some(ix);
        self.selected = Some(ix);
        self.close(window, cx);
        if changed {
            cx.emit(SelectEvent::Change(self.items[ix].value.clone()));
        }
    }

    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        if !self.open {
            self.open(window, cx);
        } else if let Some(ix) = self.highlighted {
            self.commit(ix, window, cx);
        }
    }
}

impl EventEmitter<SelectEvent> for SelectState {}

/// A select root with application-owned trigger and option-list presentation.
///
/// The root keeps the positioned listbox inside its keyboard/focus ancestry.
/// Its `trigger` is visual content; the root supplies activation and semantics.
/// Build `content` from [`SelectOption`] elements for the same retained state.
#[derive(IntoElement)]
pub struct Select {
    base: crate::ObservedElement<gpui::Stateful<gpui::Div>>,
    state: Entity<SelectState>,
    label: Option<SharedString>,
    field: crate::FieldMetadata,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    list_max_height: Option<Pixels>,
    style: StyleRefinement,
}

impl Select {
    pub fn new(id: impl Into<ElementId>, state: &Entity<SelectState>) -> Self {
        Self {
            base: div().id(id).test_support(),
            state: state.clone(),
            label: None,
            field: crate::FieldMetadata::default(),
            trigger: None,
            content: None,
            list_max_height: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Applies a [`crate::ControlField`]'s label, description, required and
    /// invalid metadata to this control's native node. The field label takes
    /// precedence over `accessibility_label`.
    pub fn field(mut self, metadata: crate::FieldMetadata) -> Self {
        self.field = metadata;
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Limits the height of the scrollable listbox. By default it can grow to
    /// the usable window height. Short lists keep their natural height.
    pub fn list_max_height(mut self, height: Pixels) -> Self {
        self.list_max_height = Some(height.max(px(0.)));
        self
    }
}

impl Styled for Select {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for Select {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Select {}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let model = self.state.read(cx);
        let open = model.open;
        let disabled = model.disabled;
        let focus = model.focus.clone();
        let content_focus = model.content_focus.clone();
        let scroll = model.scroll.clone();
        let value = model.selected_item().map(|item| item.label.clone());
        let trigger_state = self.state.clone();
        let press_state = self.state.clone();
        let accessible_state = self.state.clone();
        let up_state = self.state.clone();
        let down_state = self.state.clone();
        let first_state = self.state.clone();
        let last_state = self.state.clone();
        let typeahead_state = self.state.clone();
        let confirm_state = self.state.clone();
        let cancel_state = self.state.clone();
        let outside_state = self.state.clone();
        let tab_state = self.state.clone();
        let tab_prev_state = self.state.clone();
        let field = self.field;

        let trigger = div()
            .id("select-trigger")
            .when(!disabled, |this| {
                this.on_mouse_down(MouseButton::Left, move |_, _, cx| {
                    // Outside-down dismissal may already have changed the state.
                    press_state.update(cx, |state, _| state.open_on_pointer_down = open);
                })
                .on_click(move |event, window, cx| {
                    trigger_state.update(cx, |state, cx| {
                        let was_open = if matches!(event, ClickEvent::Mouse(_)) {
                            state.open_on_pointer_down
                        } else {
                            state.open
                        };
                        if was_open {
                            state.close(window, cx);
                        } else {
                            state.open(window, cx);
                        }
                    });
                })
            })
            .children(self.trigger);
        let popup =
            Popup::new(("select-popup", self.state.entity_id()), trigger).when(open, |this| {
                let available = Popup::available_height(window);
                let height = self.list_max_height.unwrap_or(available).min(available);
                this.content(
                    div()
                        .id("select-listbox")
                        .test_support()
                        .role(Role::ListBox)
                        .track_focus(&content_focus)
                        .on_mouse_down_out(move |_, window, cx| {
                            outside_state.update(cx, |state, cx| state.close(window, cx));
                        })
                        .child(RevealViewport::new(
                            div()
                                .id("select-viewport")
                                .max_h(height)
                                .overflow_y_scroll()
                                .track_scroll(&scroll)
                                .children(self.content),
                            scroll,
                            ScrollbarAxis::Vertical,
                        )),
                )
            });

        self.base
            .role(Role::ComboBox)
            .aria_expanded(open)
            .when_some(field.label.or(self.label), |this, label| {
                this.aria_label(label)
            })
            .when_some(field.description, |this, description| {
                this.aria_description(description)
            })
            .when_some(value, |this, value| this.aria_value(value))
            .key_context(CONTEXT)
            .when(!disabled || focus.is_focused(window), |this| {
                this.track_focus(&focus.tab_stop(!disabled))
            })
            .when(!disabled, |this| {
                this.on_a11y_action(AccessibleAction::Click, move |_, window, cx| {
                    accessible_state.update(cx, |state, cx| {
                        if state.is_open() {
                            state.close(window, cx);
                        } else {
                            state.open(window, cx);
                        }
                    });
                })
            })
            .on_action(move |_: &SelectUp, window, cx| {
                up_state.update(cx, |state, cx| state.move_cursor(false, window, cx));
            })
            .on_action(move |_: &SelectDown, window, cx| {
                down_state.update(cx, |state, cx| state.move_cursor(true, window, cx));
            })
            .on_action(move |_: &SelectFirst, window, cx| {
                first_state.update(cx, |state, cx| state.edge(false, window, cx));
            })
            .on_action(move |_: &SelectLast, window, cx| {
                last_state.update(cx, |state, cx| state.edge(true, window, cx));
            })
            .on_key_down(move |event, window, cx| {
                typeahead_state.update(cx, |state, cx| state.typeahead(event, window, cx));
            })
            .on_action(move |_: &Confirm, window, cx| {
                confirm_state.update(cx, |state, cx| state.confirm(window, cx));
            })
            .on_action(move |_: &Cancel, window, cx| {
                if !cancel_state.read(cx).is_open() {
                    cx.propagate();
                    return;
                }
                cx.stop_propagation();
                cancel_state.update(cx, |state, cx| state.close(window, cx));
            })
            .on_action(move |_: &crate::root::Tab, window, cx| {
                tab_state.update(cx, |state, cx| state.close(window, cx));
                cx.propagate();
            })
            .on_action(move |_: &crate::root::TabPrev, window, cx| {
                tab_prev_state.update(cx, |state, cx| state.close(window, cx));
                cx.propagate();
            })
            .child(popup)
            .refine_style(&self.style)
            .a11y_form(disabled, field.required, field.invalid)
    }
}

/// One unstyled option. Selection, hover, disabled and activation come from state.
#[derive(IntoElement)]
pub struct SelectOption {
    state: Entity<SelectState>,
    index: usize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SelectOption {
    /// Use the option's index in the current [`SelectState::items`] at render time.
    pub fn new(state: &Entity<SelectState>, index: usize) -> Self {
        Self {
            state: state.clone(),
            index,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for SelectOption {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SelectOption {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SelectOption {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let model = self.state.read(cx);
        let item = model
            .items
            .get(self.index)
            .expect("SelectOption index must identify an item");
        let disabled = model.disabled || item.disabled;
        let selected = model.selected == Some(self.index);
        let highlighted = model.highlighted == Some(self.index);
        let reveal = highlighted && model.reveal_highlight;
        let label = item.label.clone();
        let value = item.value.clone();
        let hover_value = value.clone();
        let hover_state = self.state.clone();
        let reveal_state = self.state.clone();
        let click_state = self.state;

        div()
            .id((ElementId::from("select-option"), value.clone()))
            .test_support()
            .role(Role::ListBoxOption)
            .aria_label(label)
            .aria_selected(selected)
            .when(highlighted, |this| this.aria_active_descendant())
            .when(!disabled, |this| {
                this.on_hover(move |hovered, window, cx| {
                    if *hovered {
                        hover_state.update(cx, |state, cx| {
                            if let Some(index) = enabled_index(&state.items, &hover_value) {
                                state.highlight(index, false, window, cx);
                            }
                        });
                    }
                })
                .on_click(move |_, window, cx| {
                    click_state.update(cx, |state, cx| {
                        if let Some(index) = enabled_index(&state.items, &value) {
                            state.commit(index, window, cx);
                        }
                    });
                })
            })
            .children(self.children)
            .refine_style(&self.style)
            .when(reveal, |this| {
                this.on_prepaint(move |bounds, window, cx| {
                    // Reveal only on open or keyboard navigation. A wheel
                    // gesture can then scroll away without being pulled back.
                    reveal_state.update(cx, |state, _| state.reveal_highlight = false);
                    window.request_autoscroll(bounds);
                })
            })
            .a11y_disabled(disabled)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use crate::Root;
    use gpui::{
        AppContext as _, KeyUpEvent, Keystroke, Modifiers, MouseButton, Render, ScrollDelta,
        ScrollEvent, TestAppContext, point, size,
    };

    struct Harness {
        select: Entity<SelectState>,
        shown: bool,
        _observation: Subscription,
    }

    impl Render for Harness {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().when(self.shown, |this| {
                this.child(
                    Select::new("select", &self.select)
                        .trigger(
                            div()
                                .debug_selector(|| "trigger".into())
                                .w(px(180.))
                                .h(px(32.))
                                .child("Choose"),
                        )
                        .content(
                            div().w(px(180.)).children(
                                self.select.read(cx).items().iter().enumerate().map(
                                    |(index, item)| {
                                        SelectOption::new(&self.select, index).h(px(32.)).child(
                                            div()
                                                .debug_selector(move || format!("option-{index}"))
                                                .size_full()
                                                .child(item.label().clone()),
                                        )
                                    },
                                ),
                            ),
                        ),
                )
            })
        }
    }

    #[gpui::test]
    fn long_list_reveals_pending_keyboard_choices_and_allows_manual_scrolling(
        cx: &mut TestAppContext,
    ) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let select = cx.new(|cx| {
                    let items = ('A'..='Z').map(|c| SelectItem::new(c.to_string(), c.to_string()));
                    SelectState::new(items, cx).default_value("U")
                });
                let observation = cx.observe(&select, |_, _, cx| cx.notify());
                Harness {
                    select,
                    shown: true,
                    _observation: observation,
                }
            });
            Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap();
        let select = view.read_with(cx, |view, _| view.select.clone());
        cx.simulate_resize(size(px(260.), px(132.)));
        cx.update(|window, cx| {
            window.activate_window();
            select.read(cx).focus_handle().clone().focus(window, cx);
            window.draw(cx).clear(cx);
        });

        let scroll = select.read_with(cx, |state, _| state.scroll.clone());
        for (key, index, selector) in [
            ("enter", 20, "option-20"),
            ("end", 25, "option-25"),
            ("down", 0, "option-0"),
            ("p", 15, "option-15"),
            ("home", 0, "option-0"),
        ] {
            cx.simulate_keystrokes(key);
            cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                window.simulate_next_frame(cx);
                window.draw(cx).clear(cx);
                let state = select.read(cx);
                assert_eq!(state.highlighted_index(), Some(index));
                assert_eq!(state.selected_index(), Some(20));
                assert!(state.content_focus_handle().is_focused(window));
                assert!(scroll.max_offset().y > px(0.));
                assert!(scroll.bounds().top() >= px(0.));
                assert!(scroll.bounds().bottom() <= window.viewport_size().height);
            });
            let bounds = cx.debug_bounds(selector).unwrap();
            assert!(bounds.top() >= scroll.bounds().top(), "{key}: {bounds:?}");
            assert!(
                bounds.bottom() <= scroll.bounds().bottom(),
                "{key}: {bounds:?}"
            );
        }

        cx.simulate_event(ScrollEvent {
            position: scroll.bounds().center(),
            delta: ScrollDelta::Pixels(point(px(0.), px(-120.))),
            ..Default::default()
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.simulate_next_frame(cx);
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("option-0").unwrap().bottom() <= scroll.bounds().top());

        cx.simulate_keystrokes("end");
        cx.update(|window, cx| {
            select.update(cx, |state, cx| {
                let items = state.items().iter().rev().cloned().collect::<Vec<_>>();
                state.set_items(items, window, cx);
            });
            window.draw(cx).clear(cx);
            window.simulate_next_frame(cx);
            window.draw(cx).clear(cx);
        });
        let moved = cx.debug_bounds("option-0").unwrap();
        assert!(moved.top() >= scroll.bounds().top() && moved.bottom() <= scroll.bounds().bottom());
        cx.simulate_keystrokes("enter");
        cx.update(|window, cx| {
            assert_eq!(
                select.read(cx).selected_item().unwrap().value().as_ref(),
                "Z"
            );
            assert!(select.read(cx).focus_handle().is_focused(window));
        });
    }

    #[gpui::test]
    fn edge_keys_and_typeahead_keep_pending_and_committed_selection_distinct(
        cx: &mut TestAppContext,
    ) {
        cx.update(crate::init);
        let changes = Rc::new(RefCell::new(Vec::new()));
        let recorded = changes.clone();
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let select = cx.new(|cx| {
                    SelectState::new(
                        [
                            SelectItem::new("unavailable-first", "Apricot").disabled(true),
                            SelectItem::new("apple", "Apple"),
                            SelectItem::new("avocado", "Avocado"),
                            SelectItem::new("banana", "Banana"),
                            SelectItem::new("blueberry", "Blueberry"),
                            SelectItem::new("unavailable-match", "Blackberry").disabled(true),
                            SelectItem::new("eclair", "Éclair"),
                            SelectItem::new("unavailable-last", "Zucchini").disabled(true),
                        ],
                        cx,
                    )
                    .default_value("banana")
                });
                cx.subscribe(&select, move |_, _, event: &SelectEvent, _| {
                    let SelectEvent::Change(value) = event;
                    recorded.borrow_mut().push(value.clone());
                })
                .detach();
                let observation = cx.observe(&select, |_, _, cx| cx.notify());
                Harness {
                    select,
                    shown: true,
                    _observation: observation,
                }
            });
            Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap();
        cx.update(|window, cx| {
            window.activate_window();
            view.read(cx)
                .select
                .read(cx)
                .focus_handle()
                .clone()
                .focus(window, cx);
            window.draw(cx).clear(cx);
        });

        // Closed Home/End do not open or change a select. Typing selects in
        // place; repeated letters cycle and a longer prefix narrows the match.
        cx.simulate_keystrokes("home end a a");
        cx.update(|window, cx| {
            let state = view.read(cx).select.read(cx);
            assert!(!state.is_open());
            assert_eq!(state.selected_index(), Some(2));
            assert!(state.focus_handle().is_focused(window));
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.simulate_keystrokes("b l");
        cx.update(|_, cx| {
            assert_eq!(view.read(cx).select.read(cx).selected_index(), Some(4));
        });
        assert_eq!(
            *changes.borrow(),
            ["apple", "avocado", "banana", "blueberry"]
                .map(SharedString::from)
                .to_vec()
        );

        cx.simulate_keystrokes("down home");
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            let state = view.read(cx).select.read(cx);
            assert!(state.is_open());
            assert_eq!(state.highlighted_index(), Some(1));
            assert_eq!(state.selected_index(), Some(4));
            assert!(state.content_focus_handle().is_focused(window));
        });
        cx.simulate_keystrokes("end");
        cx.update(|_, cx| {
            let state = view.read(cx).select.read(cx);
            assert_eq!(state.highlighted_index(), Some(6));
            assert_eq!(state.selected_index(), Some(4));
        });
        cx.simulate_keystrokes("a a");
        cx.update(|_, cx| {
            assert_eq!(view.read(cx).select.read(cx).highlighted_index(), Some(2));
        });
        assert_eq!(changes.borrow().len(), 4);
        cx.simulate_keystrokes("escape down b l enter");
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            let state = view.read(cx).select.read(cx);
            assert!(!state.is_open());
            assert_eq!(state.selected_index(), Some(4));
            assert!(state.focus_handle().is_focused(window));
        });
        assert_eq!(changes.borrow().len(), 4, "unchanged commits emit no event");

        // Native character preference, shortcuts, disabled controls, and
        // events outside the select's own focus scopes never change its value.
        cx.simulate_keystrokes("ctrl-a alt-a");
        let keystroke = Keystroke {
            key: "a".into(),
            key_char: Some("a".into()),
            modifiers: Modifiers::default(),
        };
        cx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: true,
        });
        cx.simulate_event(KeyUpEvent { keystroke });
        cx.update(|window, cx| {
            let select = view.read(cx).select.clone();
            assert_eq!(select.read(cx).selected_index(), Some(4));
            select.update(cx, |state, cx| state.set_disabled(true, window, cx));
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("a home end");
        cx.update(|window, cx| {
            let select = view.read(cx).select.clone();
            assert_eq!(select.read(cx).selected_index(), Some(4));
            select.update(cx, |state, cx| state.set_disabled(false, window, cx));
            window.blur(cx);
            let event = KeyDownEvent {
                keystroke: Keystroke {
                    key: "a".into(),
                    key_char: Some("a".into()),
                    modifiers: Modifiers::default(),
                },
                is_held: false,
                prefer_character_input: false,
            };
            select.update(cx, |state, cx| state.typeahead(&event, window, cx));
            assert_eq!(select.read(cx).selected_index(), Some(4));
            select.read(cx).focus_handle().clone().focus(window, cx);
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        let keystroke = Keystroke {
            key: "É".into(),
            key_char: Some("É".into()),
            modifiers: Modifiers::default(),
        };
        cx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: false,
        });
        cx.simulate_event(KeyUpEvent { keystroke });
        cx.update(|_, cx| {
            assert_eq!(view.read(cx).select.read(cx).selected_index(), Some(6));
        });
        assert_eq!(changes.borrow().last().unwrap().as_ref(), "eclair");

        // Application resets validate before disturbing a pending cursor, then
        // close with the same focus policy and never report a user change.
        cx.simulate_keystrokes("down home");
        cx.update(|window, cx| {
            let select = view.read(cx).select.clone();
            select.update(cx, |state, cx| {
                for value in ["missing", "unavailable-first"] {
                    assert!(!state.set_selected_value(Some(value), window, cx));
                    assert_eq!(state.selected_index(), Some(6));
                    assert_eq!(state.highlighted_index(), Some(1));
                    assert!(state.is_open());
                    assert!(state.content_focus_handle().is_focused(window));
                }
                assert!(state.set_selected_value(Some("banana"), window, cx));
                assert_eq!(state.selected_index(), Some(3));
                assert!(!state.is_open());
                assert!(state.focus_handle().is_focused(window));
                state.open(window, cx);
                assert!(state.set_selected_value(None, window, cx));
                assert_eq!(state.selected_index(), None);
                assert!(!state.is_open());
                assert!(state.focus_handle().is_focused(window));
                state.set_disabled(true, window, cx);
                assert!(state.set_selected_value(Some("apple"), window, cx));
                assert_eq!(state.selected_index(), Some(1));
                window.blur(cx);
                assert!(state.set_selected_value(None, window, cx));
                assert!(window.focused(cx).is_none());
            });
        });
        assert_eq!(
            changes.borrow().len(),
            5,
            "application updates emit no Change"
        );
    }

    #[gpui::test]
    fn refreshing_items_preserves_value_and_pending_focus(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let changes = Rc::new(RefCell::new(Vec::new()));
        let recorded = changes.clone();
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let select = cx.new(|cx| {
                    SelectState::new(
                        [
                            SelectItem::new("en", "English"),
                            SelectItem::new("ko", "Korean"),
                        ],
                        cx,
                    )
                    .default_value("en")
                });
                cx.subscribe(&select, move |_, _, event: &SelectEvent, _| {
                    recorded.borrow_mut().push(event.clone());
                })
                .detach();
                let observation = cx.observe(&select, |_, _, cx| cx.notify());
                Harness {
                    select,
                    shown: true,
                    _observation: observation,
                }
            });
            Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap();
        let select = view.read_with(cx, |view, _| view.select.clone());
        cx.update(|window, cx| {
            window.activate_window();
            select.read(cx).focus_handle().clone().focus(window, cx);
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("enter end");
        cx.update(|window, cx| {
            select.update(cx, |state, cx| {
                state.set_items(
                    [
                        SelectItem::new("ko", "한국어"),
                        SelectItem::new("en", "English"),
                        SelectItem::new("ja", "Japanese"),
                    ],
                    window,
                    cx,
                );
                assert_eq!(state.selected_index(), Some(1));
                assert_eq!(state.highlighted_index(), Some(0));
                assert!(state.content_focus_handle().is_focused(window));
            });
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("enter");
        cx.update(|window, cx| {
            select.update(cx, |state, cx| {
                assert_eq!(state.selected_item().unwrap().label().as_ref(), "한국어");
                state.open(window, cx);
            });
        });
        cx.simulate_keystrokes("end");
        cx.update(|window, cx| {
            select.update(cx, |state, cx| {
                state.set_items(
                    [
                        SelectItem::new("en", "English"),
                        SelectItem::new("ko", "Korean"),
                        SelectItem::new("ja", "Japanese").disabled(true),
                    ],
                    window,
                    cx,
                );
                assert_eq!(state.highlighted_index(), state.selected_index());
                assert_eq!(state.highlighted_index(), Some(1));
                state.set_items([SelectItem::new("en", "English")], window, cx);
                assert!(state.selected_item().is_none());
                assert_eq!(state.highlighted_index(), Some(0));
                state.set_items(
                    [SelectItem::new("en", "English").disabled(true)],
                    window,
                    cx,
                );
                assert!(!state.is_open());
                assert!(state.focus_handle().is_focused(window));
                state.set_items([], window, cx);
                state.open(window, cx);
                assert!(!state.is_open());
                state.set_items(
                    [
                        SelectItem::new("en", "English"),
                        SelectItem::new("ko", "Korean"),
                    ],
                    window,
                    cx,
                );
                state.open(window, cx);
            });
            window.draw(cx).clear(cx);
        });
        // A press that starts on Korean cannot commit a different row that
        // replaces it before the pointer is released at the same coordinates.
        let option = cx.debug_bounds("option-1").unwrap().center();
        cx.simulate_mouse_down(option, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| {
            select.update(cx, |state, cx| {
                state.set_items(
                    [
                        SelectItem::new("en", "English"),
                        SelectItem::new("ja", "Japanese"),
                    ],
                    window,
                    cx,
                );
            });
            window.draw(cx).clear(cx);
        });
        cx.simulate_mouse_up(option, MouseButton::Left, Modifiers::default());
        cx.update(|_, cx| {
            assert!(select.read(cx).selected_item().is_none());
            assert!(select.read(cx).is_open());
        });
        assert_eq!(*changes.borrow(), [SelectEvent::Change("ko".into())]);
    }

    #[gpui::test]
    fn trigger_toggle_and_host_unmount_clear_retained_popup(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let select = cx.new(|cx| SelectState::new([SelectItem::new("first", "First")], cx));
                let observation = cx.observe(&select, |_, _, cx| cx.notify());
                Harness {
                    select,
                    shown: true,
                    _observation: observation,
                }
            });
            Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap();
        cx.update(|window, cx| {
            window.activate_window();
            window.draw(cx).clear(cx);
        });
        let trigger = cx.debug_bounds("trigger").unwrap().center();
        cx.simulate_click(trigger, Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|_, cx| assert!(view.read(cx).select.read(cx).is_open()));
        cx.simulate_mouse_down(trigger, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.simulate_mouse_up(trigger, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|_, cx| assert!(!view.read(cx).select.read(cx).is_open()));

        cx.simulate_click(trigger, Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|_, cx| assert!(view.read(cx).select.read(cx).is_open()));
        view.update(cx, |view, cx| {
            view.shown = false;
            cx.notify();
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|window, cx| {
            let state = view.read(cx).select.read(cx);
            assert!(!state.is_open());
            assert!(!state.focus_handle().is_focused(window));
            assert!(!GlobalState::is_in_deferred_context(cx));
        });
    }
}
