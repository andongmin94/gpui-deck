//! Timed notifications that preserve time while a user is interacting with them.

use std::time::Duration;

use gpui::{
    AnyElement, App, AppContext as _, Bounds, Context, Element, ElementId, Entity, EventEmitter,
    FocusHandle, GlobalElementId, HoverListenerMode, InspectorElementId, InteractiveElement as _,
    IntoElement, KeyBinding, LayoutId, ParentElement, Pixels, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Subscription, Task, WeakFocusHandle,
    Window, accesskit, div, prelude::FluentBuilder as _,
};
use web_time::Instant;

use crate::{Button, StyledExt as _, TestSupportExt as _, actions::Cancel};

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", Cancel, Some("Toast"))]);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastDismissReason {
    Timeout,
    CloseButton,
    Escape,
    Imperative,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastEvent {
    Opened,
    Dismissed(ToastDismissReason),
}

/// One retained notification. Applications own its message, actions and stacking.
///
/// Showing a toast never takes focus. Its countdown starts when mounted and
/// pauses while hovered, while it or a descendant owns focus, and while the
/// window is inactive. Resuming spends only the remaining duration. Explicitly
/// dismissing a focused toast restores the focus captured when it was shown;
/// closing an unfocused toast never moves another control's focus.
pub struct ToastState {
    focus: FocusHandle,
    previous_focus: Option<WeakFocusHandle>,
    duration: Duration,
    remaining: Duration,
    running_since: Option<Instant>,
    timer: Option<Task<()>>,
    open: bool,
    mounted: bool,
    hovered: bool,
    focused: bool,
    _subscriptions: Vec<Subscription>,
}

impl ToastState {
    /// Uses a five-second lifetime; configure it before mounting with
    /// [`Self::with_duration`]. Keep the matching [`Toast`] mounted even when closed.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle().tab_stop(false);
        let subscriptions = vec![
            cx.on_focus_in(&focus, window, |state, _, cx| {
                state.focused = true;
                state.pause(cx);
            }),
            cx.on_focus_out(&focus, window, |state, _, window, cx| {
                state.focused = false;
                state.resume(window, cx);
            }),
            cx.observe_window_activation(window, |state, window, cx| {
                if window.is_window_active() {
                    state.resume(window, cx);
                } else {
                    state.pause(cx);
                }
            }),
        ];
        let duration = Duration::from_secs(5);
        Self {
            focus,
            previous_focus: None,
            duration,
            remaining: duration,
            running_since: None,
            timer: None,
            open: false,
            mounted: false,
            hovered: false,
            focused: false,
            _subscriptions: subscriptions,
        }
    }

    /// A nonzero amount of active, unattended time before dismissal.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        assert!(!duration.is_zero(), "Toast duration must be nonzero");
        self.duration = duration;
        self.remaining = duration;
        self
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// The non-Tab-stop notification scope. Its action buttons keep their own
    /// ordinary Tab stops; applications may use this handle to announce/focus
    /// the notification on an explicit user request.
    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }

    /// Shows or replaces the notification and restarts its full lifetime.
    pub fn show(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.timer = None;
        self.running_since = None;
        self.remaining = self.duration;
        if !self.open {
            self.previous_focus = window.focused(cx).map(|focus| focus.downgrade());
            self.open = true;
            cx.emit(ToastEvent::Opened);
        }
        self.resume(window, cx);
        cx.notify();
        window.refresh();
    }

    pub fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.dismiss_with_reason(ToastDismissReason::Imperative, window, cx);
    }

    fn owns_focus(&self, window: &Window, cx: &App) -> bool {
        self.focus.is_focused(window) || self.focus.contains_focused(window, cx)
    }

    fn pause(&mut self, cx: &mut Context<Self>) {
        self.timer = None;
        if let Some(started) = self.running_since.take() {
            self.remaining = self
                .remaining
                .saturating_sub(cx.background_executor().now().duration_since(started));
        }
    }

    fn resume(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open
            || !self.mounted
            || self.hovered
            || self.focused
            || self.owns_focus(window, cx)
            || !window.is_window_active()
            || self.timer.is_some()
        {
            return;
        }
        self.running_since = Some(cx.background_executor().now());
        let remaining = self.remaining;
        self.timer = Some(cx.spawn_in(window, async move |state, cx| {
            cx.background_executor().timer(remaining).await;
            let _ = state.update_in(cx, |state, window, cx| {
                state.timer = None;
                state.running_since = None;
                state.remaining = Duration::ZERO;
                if state.hovered || state.focused || state.owns_focus(window, cx) {
                    return;
                }
                state.dismiss_with_reason(ToastDismissReason::Timeout, window, cx);
            });
        }));
    }

    fn hover(&mut self, hovered: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.hovered = hovered;
        if hovered {
            self.pause(cx);
        } else {
            self.resume(window, cx);
        }
    }

    fn dismiss_with_reason(
        &mut self,
        reason: ToastDismissReason,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.timer = None;
        self.running_since = None;
        if !self.open {
            return;
        }
        self.open = false;
        let owns_focus = self.owns_focus(window, cx);
        let restore = self.previous_focus.take().and_then(|focus| focus.upgrade());
        self.hovered = false;
        self.focused = false;
        if owns_focus {
            if let Some(focus) = restore {
                focus.focus(window, cx);
            } else {
                window.blur(cx);
            }
        }
        cx.emit(ToastEvent::Dismissed(reason));
        cx.notify();
        window.refresh();
    }
}

impl EventEmitter<ToastEvent> for ToastState {}

struct ToastMount {
    _release: Subscription,
}

// GPUI has no live-region builder. Project native metadata onto the same
// surface, without a second layout, focus or accessibility node.
struct ToastLive<E> {
    inner: E,
    live: Option<accesskit::Live>,
}

impl<E: Element> IntoElement for ToastLive<E> {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl<E: Element> Element for ToastLive<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;

    fn id(&self) -> Option<ElementId> {
        self.inner.id()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.inner.source_location()
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.inner.request_layout(id, inspector, window, cx)
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.inner
            .prepaint(id, inspector, bounds, layout, window, cx)
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner
            .paint(id, inspector, bounds, layout, prepaint, window, cx);
    }

    fn a11y_role(&self) -> Option<Role> {
        self.inner.a11y_role()
    }

    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node);
        if let Some(live) = self.live {
            node.set_live(live);
            node.set_live_atomic();
        }
    }

    fn a11y_synthetic_children(
        &mut self,
        prepaint: &mut Self::PrepaintState,
        builder: &mut gpui::A11ySubtreeBuilder,
    ) {
        Element::a11y_synthetic_children(&mut self.inner, prepaint, builder);
    }
}

/// A nonmodal status surface. The consumer chooses its location and content.
///
/// The surface has a native Status role (Alert with [`Self::urgent`]), a focus
/// scope for its actions, and no automatic focus or modal trap. Escape dismisses
/// only while focus is inside this toast. Hover uses either input modality.
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    state: Entity<ToastState>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    label: Option<SharedString>,
    urgent: bool,
}

impl Toast {
    pub fn new(id: impl Into<ElementId>, state: &Entity<ToastState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            style: StyleRefinement::default(),
            children: Vec::new(),
            label: None,
            urgent: false,
        }
    }

    /// The complete notification text announced as an atomic live update.
    /// Supplies both native name and value for platform announcement adapters.
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn urgent(mut self, urgent: bool) -> Self {
        self.urgent = urgent;
        self
    }
}

impl Styled for Toast {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Toast {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children);
    }
}

impl RenderOnce for Toast {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let weak = self.state.downgrade();
        let owner_window = window.window_handle();
        window.with_element_namespace(self.id.clone(), |window| {
            window.use_keyed_state(("toast-mount", self.state.entity_id()), cx, |_, cx| {
                ToastMount {
                    _release: cx.on_release(move |_, cx| {
                        let _ = owner_window.update(cx, |_, window, cx| {
                            let _ = weak.update(cx, |state, cx| {
                                state.mounted = false;
                                // Removing the host can also remove its return target.
                                state.previous_focus = None;
                                state.dismiss(window, cx);
                            });
                        });
                    }),
                }
            })
        });
        self.state.update(cx, |state, cx| {
            state.mounted = true;
            state.resume(window, cx);
        });
        let model = self.state.read(cx);
        let base = div().id(self.id).test_support();
        if !model.open {
            return ToastLive {
                inner: base,
                live: None,
            };
        }
        let focus = model.focus.clone();
        let hover = self.state.clone();
        let dismiss = self.state;
        let surface = base
            .role(if self.urgent {
                Role::Alert
            } else {
                Role::Status
            })
            .when_some(self.label, |this, label| {
                this.aria_label(label.clone()).aria_value(label)
            })
            .key_context("Toast")
            .track_focus(&focus)
            .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
            .on_hover(move |hovered, window, cx| {
                hover.update(cx, |state, cx| state.hover(*hovered, window, cx));
            })
            .on_action(move |_: &Cancel, window, cx| {
                dismiss.update(cx, |state, cx| {
                    state.dismiss_with_reason(ToastDismissReason::Escape, window, cx);
                });
            })
            .children(self.children)
            .refine_style(&self.style);
        ToastLive {
            inner: surface,
            live: Some(if self.urgent {
                accesskit::Live::Assertive
            } else {
                accesskit::Live::Polite
            }),
        }
    }
}

/// A normal Button whose activation safely dismisses this notification.
#[derive(IntoElement)]
pub struct ToastClose {
    button: Button,
}

impl ToastClose {
    pub fn new(id: impl Into<ElementId>, state: &Entity<ToastState>) -> Self {
        let state = state.clone();
        Self {
            button: Button::new(id)
                .accessibility_label("Dismiss notification")
                .on_click(move |_, window, cx| {
                    state.update(cx, |state, cx| {
                        state.dismiss_with_reason(ToastDismissReason::CloseButton, window, cx);
                    });
                }),
        }
    }

    pub fn trigger(mut self, build: impl FnOnce(Button) -> Button) -> Self {
        self.button = build(self.button);
        self
    }
}

impl RenderOnce for ToastClose {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.button
    }
}

/// Identifies one notification pushed to a [`ToasterState`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToastKey(u64);

impl From<ToastKey> for ElementId {
    fn from(key: ToastKey) -> Self {
        ElementId::NamedInteger("toast".into(), key.0)
    }
}

struct ToasterEntry<T> {
    key: ToastKey,
    value: T,
    state: Entity<ToastState>,
    _subscription: Subscription,
}

/// A retained stack of notifications, each with its own [`ToastState`].
///
/// Applications push their own message values; the stack owns ordering,
/// lifetimes and removal. Only the newest [`Self::visible_limit`] entries are
/// mounted, so older ones wait without spending time and appear as newer ones
/// close. Each visible toast keeps the single-toast pause, focus and dismissal
/// contract.
pub struct ToasterState<T> {
    entries: Vec<ToasterEntry<T>>,
    visible: usize,
    duration: Duration,
    next: u64,
}

impl<T: 'static> ToasterState<T> {
    /// Shows up to three notifications at a time, each for five seconds.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            visible: 3,
            duration: Duration::from_secs(5),
            next: 0,
        }
    }

    /// The number of newest notifications mounted at once.
    pub fn visible_limit(mut self, visible: usize) -> Self {
        assert!(visible > 0, "Toaster must show at least one notification");
        self.visible = visible;
        self
    }

    /// The nonzero lifetime given to each notification pushed afterward.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        assert!(!duration.is_zero(), "Toast duration must be nonzero");
        self.duration = duration;
        self
    }

    /// Adds a notification without moving focus and returns its key.
    pub fn push(&mut self, value: T, window: &mut Window, cx: &mut Context<Self>) -> ToastKey {
        let key = ToastKey(self.next);
        self.next += 1;
        let duration = self.duration;
        let state = cx.new(|cx| ToastState::new(window, cx).with_duration(duration));
        let subscription = cx.subscribe(&state, move |this, _, event: &ToastEvent, cx| {
            if matches!(event, ToastEvent::Dismissed(_)) {
                this.entries.retain(|entry| entry.key != key);
                cx.notify();
            }
        });
        state.update(cx, |state, cx| state.show(window, cx));
        self.entries.push(ToasterEntry {
            key,
            value,
            state,
            _subscription: subscription,
        });
        cx.notify();
        key
    }

    /// Replaces a notification's value in place and restarts its lifetime,
    /// for example when a pending operation completes. Returns false when the
    /// notification has already closed.
    pub fn replace(
        &mut self,
        key: ToastKey,
        value: T,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(entry) = self.entries.iter_mut().find(|entry| entry.key == key) else {
            return false;
        };
        entry.value = value;
        entry.state.update(cx, |state, cx| state.show(window, cx));
        cx.notify();
        true
    }

    pub fn dismiss(&mut self, key: ToastKey, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.entries.iter().find(|entry| entry.key == key) {
            entry
                .state
                .update(cx, |state, cx| state.dismiss(window, cx));
        }
    }

    pub fn dismiss_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for entry in &self.entries {
            entry
                .state
                .update(cx, |state, cx| state.dismiss(window, cx));
        }
    }

    /// The number of open notifications, including those waiting to appear.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Mounted notifications, newest first.
    pub fn visible(&self) -> impl Iterator<Item = (ToastKey, &T, &Entity<ToastState>)> {
        self.entries
            .iter()
            .rev()
            .take(self.visible)
            .map(|entry| (entry.key, &entry.value, &entry.state))
    }

    /// Moves focus into the newest notification on an explicit user request,
    /// such as an application shortcut. Dismissing it restores prior focus.
    pub fn focus_latest(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.entries.last() {
            entry.state.read(cx).focus.clone().focus(window, cx);
        }
    }
}

impl<T: 'static> Default for ToasterState<T> {
    fn default() -> Self {
        Self::new()
    }
}

type RenderToast<T> = Box<dyn Fn(ToastKey, &T, &Entity<ToastState>) -> Toast>;

/// The named region holding a [`ToasterState`]'s visible notifications, newest
/// first. The application builds each [`Toast`] and places the region.
#[derive(IntoElement)]
pub struct Toaster<T: 'static> {
    id: ElementId,
    state: Entity<ToasterState<T>>,
    label: SharedString,
    render: RenderToast<T>,
    style: StyleRefinement,
}

impl<T: 'static> Toaster<T> {
    /// `render` builds each visible notification; give its Toast the key as ID.
    pub fn new(
        id: impl Into<ElementId>,
        state: &Entity<ToasterState<T>>,
        render: impl Fn(ToastKey, &T, &Entity<ToastState>) -> Toast + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            label: "Notifications".into(),
            render: Box::new(render),
            style: StyleRefinement::default(),
        }
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl<T: 'static> Styled for Toaster<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: 'static> RenderOnce for Toaster<T> {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let toasts: Vec<_> = self
            .state
            .read(cx)
            .visible()
            .map(|(key, value, state)| (self.render)(key, value, state))
            .collect();
        div()
            .id(self.id)
            .test_support()
            .role(Role::Group)
            .aria_label(self.label)
            .flex()
            .flex_col()
            .children(toasts)
            .refine_style(&self.style)
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Modifiers, Render, TestAppContext, VisualTestContext, point, px};

    use super::*;

    struct Harness {
        state: Entity<ToastState>,
        trigger: FocusHandle,
        close: FocusHandle,
        urgent: bool,
        show_host: bool,
        _subscription: Subscription,
    }

    impl Render for Harness {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            if !self.show_host {
                return div().into_any_element();
            }
            let close = self.close.clone();
            let surface = Toast::new("notice", &self.state)
                .accessibility_label("Saved locally")
                .urgent(self.urgent)
                .w(px(240.))
                .h(px(60.))
                .child("Saved locally")
                .child(
                    ToastClose::new("close-notice", &self.state)
                        .trigger(|button| button.track_focus(&close).child("Dismiss")),
                )
                .render(window, cx)
                .into_element();
            if self.state.read(cx).is_open() {
                let role = if self.urgent {
                    Role::Alert
                } else {
                    Role::Status
                };
                let mut node = accesskit::Node::new(role);
                surface.write_a11y_info(&mut node);
                assert_eq!(surface.a11y_role(), Some(role));
                assert_eq!(node.label(), Some("Saved locally"));
                assert_eq!(node.value(), Some("Saved locally"));
                assert_eq!(
                    node.live(),
                    Some(if self.urgent {
                        accesskit::Live::Assertive
                    } else {
                        accesskit::Live::Polite
                    })
                );
                assert!(node.is_live_atomic());
            }
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    Button::new("trigger")
                        .track_focus(&self.trigger)
                        .h(px(30.))
                        .child("Show"),
                )
                .child(
                    div()
                        .debug_selector(|| "notice".into())
                        .w(px(240.))
                        .h(px(60.))
                        .child(surface),
                )
                .into_any_element()
        }
    }

    #[gpui::test]
    fn lifetime_pauses_for_hover_and_focus_and_dismissal_restores_focus(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let state =
                    cx.new(|cx| ToastState::new(window, cx).with_duration(Duration::from_secs(4)));
                Harness {
                    _subscription: cx.observe(&state, |_, _, cx| cx.notify()),
                    state,
                    trigger: cx.focus_handle(),
                    close: cx.focus_handle(),
                    urgent: false,
                    show_host: true,
                }
            });
            crate::Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap();
        cx.update(|window, cx| {
            window.activate_window();
            view.read(cx).trigger.clone().focus(window, cx);
            let state = view.read(cx).state.clone();
            state.update(cx, |state, cx| state.show(window, cx));
            window.draw(cx).clear(cx);
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        let notice = cx.debug_bounds("notice").unwrap().center();
        cx.simulate_mouse_move(notice, None, Modifiers::default());
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(8));
        cx.run_until_parked();
        cx.update(|_, cx| assert!(view.read(cx).state.read(cx).is_open()));
        cx.simulate_mouse_move(point(px(600.), px(400.)), None, Modifiers::default());
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(2));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert!(view.read(cx).state.read(cx).is_open());
            view.read(cx).close.clone().focus(window, cx);
            view.update(cx, |view, cx| {
                view.urgent = true;
                cx.notify();
            });
            window.draw(cx).clear(cx);
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(8));
        cx.run_until_parked();
        cx.update(|_, cx| assert!(view.read(cx).state.read(cx).is_open()));
        cx.simulate_keystrokes("escape");
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert!(!view.read(cx).state.read(cx).is_open());
            assert!(view.read(cx).trigger.is_focused(window));
            let state = view.read(cx).state.clone();
            state.update(cx, |state, cx| state.show(window, cx));
            window.draw(cx).clear(cx);
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(4));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert!(!view.read(cx).state.read(cx).is_open());
            assert!(view.read(cx).trigger.is_focused(window));
        });
        let replaced = cx.update(|window, cx| {
            let previous = view.read(cx).state.clone();
            previous.update(cx, |state, cx| state.show(window, cx));
            window.draw(cx).clear(cx);
            let replacement =
                cx.new(|cx| ToastState::new(window, cx).with_duration(Duration::from_secs(4)));
            replacement.update(cx, |state, cx| state.show(window, cx));
            view.update(cx, |view, cx| {
                view._subscription = cx.observe(&replacement, |_, _, cx| cx.notify());
                view.state = replacement;
                cx.notify();
            });
            window.draw(cx).clear(cx);
            previous
        });
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert!(
                !replaced.read(cx).is_open(),
                "replacing the state at the same UI ID must release the old notification"
            );
            assert!(view.read(cx).state.read(cx).is_open());
        });
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            state.update(cx, |state, cx| state.show(window, cx));
            state.read(cx).focus.clone().focus(window, cx);
            window.draw(cx).clear(cx);
            view.update(cx, |view, cx| {
                view.show_host = false;
                cx.notify();
            });
            window.draw(cx).clear(cx);
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert!(!view.read(cx).state.read(cx).is_open());
            assert!(
                !view.read(cx).trigger.is_focused(window),
                "unmounting a notification must not focus its hidden trigger"
            );
        });
    }

    struct StackHarness {
        toaster: Entity<ToasterState<&'static str>>,
        trigger: FocusHandle,
        _subscription: Subscription,
    }

    impl Render for StackHarness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    Button::new("trigger")
                        .track_focus(&self.trigger)
                        .h(px(30.))
                        .child("Show"),
                )
                .child(
                    Toaster::new("stack", &self.toaster, |key, message, state| {
                        Toast::new(key, state)
                            .accessibility_label(*message)
                            .h(px(40.))
                            .child(*message)
                            .child(
                                ToastClose::new((ElementId::from(key), "close"), state)
                                    .trigger(|button| button.child("Dismiss")),
                            )
                    })
                    .w(px(240.)),
                )
        }
    }

    #[gpui::test]
    fn toaster_limits_mounted_toasts_and_queues_older_ones(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let toaster = cx.new(|_| {
                    ToasterState::new()
                        .visible_limit(3)
                        .with_duration(Duration::from_secs(4))
                });
                StackHarness {
                    _subscription: cx.observe(&toaster, |_, _, cx| cx.notify()),
                    toaster,
                    trigger: cx.focus_handle(),
                }
            });
            crate::Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<StackHarness>()
            .unwrap();
        let toaster = cx.update(|_, cx| view.read(cx).toaster.clone());
        let draw = |cx: &mut VisualTestContext| {
            cx.update(|window, cx| window.draw(cx).clear(cx));
            cx.run_until_parked();
        };
        let messages = |cx: &mut VisualTestContext| {
            cx.update(|_, cx| {
                toaster
                    .read(cx)
                    .visible()
                    .map(|(_, message, _)| *message)
                    .collect::<Vec<_>>()
            })
        };
        let first = cx.update(|window, cx| {
            window.activate_window();
            view.read(cx).trigger.clone().focus(window, cx);
            toaster.update(cx, |toaster, cx| {
                let first = toaster.push("One", window, cx);
                for message in ["Two", "Three", "Four"] {
                    toaster.push(message, window, cx);
                }
                first
            })
        });
        draw(cx);
        assert_eq!(messages(cx), ["Four", "Three", "Two"]);
        cx.update(|window, cx| {
            assert_eq!(toaster.read(cx).len(), 4);
            assert!(
                view.read(cx).trigger.is_focused(window),
                "showing never takes focus"
            );
            let region = crate::test_support::find(window, &[], &"stack".into()).unwrap();
            assert_eq!(region.role(), Some(Role::Group));
            assert_eq!(region.label(), Some("Notifications"));
            assert!(crate::test_support::find(window, &[], &first.into()).is_none());
        });

        // The queued toast spends no time until mounted.
        cx.executor().advance_clock(Duration::from_secs(4));
        cx.run_until_parked();
        draw(cx);
        assert_eq!(messages(cx), ["One"]);
        cx.executor().advance_clock(Duration::from_secs(3));
        cx.run_until_parked();
        cx.update(|window, cx| {
            toaster.update(cx, |toaster, cx| {
                assert!(toaster.replace(first, "One finished", window, cx));
            })
        });
        draw(cx);
        cx.executor().advance_clock(Duration::from_secs(3));
        cx.run_until_parked();
        assert_eq!(
            messages(cx),
            ["One finished"],
            "replacing restarts its lifetime"
        );
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert!(toaster.read(cx).is_empty());
            toaster.update(cx, |toaster, cx| {
                assert!(!toaster.replace(first, "Gone", window, cx));
                toaster.push("Five", window, cx);
            });
        });
        draw(cx);

        // An explicit shortcut can enter the newest toast; Escape returns.
        cx.update(|window, cx| toaster.update(cx, |toaster, cx| toaster.focus_latest(window, cx)));
        draw(cx);
        cx.simulate_keystrokes("escape");
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert!(toaster.read(cx).is_empty());
            assert!(view.read(cx).trigger.is_focused(window));
            toaster.update(cx, |toaster, cx| {
                toaster.push("Six", window, cx);
                toaster.push("Seven", window, cx);
                toaster.dismiss_all(window, cx);
            });
        });
        cx.run_until_parked();
        cx.update(|_, cx| assert!(toaster.read(cx).is_empty()));
    }
}
