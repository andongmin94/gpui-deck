//! A modal surface with an edge-aligned drag handle and optional snap heights.

use gpui::{
    AnyElement, App, AppContext as _, Context, ElementId, Entity, EventEmitter, FocusHandle,
    InteractiveElement as _, IntoElement, MouseButton, MouseMoveEvent, MouseUpEvent, ParentElement,
    Pixels, Point, RenderOnce, SharedString, StyleRefinement, Styled, Subscription, Window, canvas,
    div, prelude::FluentBuilder as _, px,
};

use crate::{
    Dialog, DialogEvent, DialogPopup, DialogState, ElementExt as _, Placement, StyledExt as _,
    TestSupportExt as _,
};

/// Retained pointer geometry around the existing modal lifecycle.
///
/// DialogState remains the sole owner of open state, focus trapping, initial
/// focus and restoration. Only presses on the dedicated handle start dragging;
/// scrolling and editing inside the body retain their normal native behavior.
pub struct DrawerState {
    dialog: Entity<DialogState>,
    drag_start: Option<Pixels>,
    offset: Pixels,
    extent: Pixels,
    side: Placement,
    snap_points: Vec<f32>,
    snap_index: usize,
    _subscriptions: Vec<Subscription>,
}

impl DrawerState {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::with_dialog(cx.new(DialogState::new), window, cx)
    }

    /// Adopts an existing modal state, including its configured initial focus.
    /// Mount that state in this Drawer only, not in a second Dialog as well.
    pub fn with_dialog(
        dialog: Entity<DialogState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&dialog, |_, _, cx| cx.notify()),
            cx.subscribe(&dialog, |state, _, event: &DialogEvent, cx| {
                if matches!(event, DialogEvent::Closed(_)) {
                    state.drag_start = None;
                    state.offset = px(0.);
                }
                cx.emit(*event);
                cx.notify();
            }),
            cx.observe_window_activation(window, |state, window, cx| {
                if !window.is_window_active() {
                    state.cancel_drag(window, cx);
                }
            }),
        ];
        Self {
            dialog,
            drag_start: None,
            offset: px(0.),
            extent: px(0.),
            side: Placement::Bottom,
            snap_points: Vec::new(),
            snap_index: 0,
            _subscriptions: subscriptions,
        }
    }

    pub fn dialog(&self) -> &Entity<DialogState> {
        &self.dialog
    }

    /// Sets the physical window edge before mounting. The default is bottom.
    pub fn side(mut self, side: Placement) -> Self {
        assert!(
            side.is_vertical() || self.snap_points.is_empty(),
            "snap points require a vertical drawer"
        );
        self.side = side;
        self
    }

    /// Sets increasing fractions of the window height for a top or bottom
    /// drawer. The first fraction is its initial height.
    pub fn snap_points(mut self, points: impl IntoIterator<Item = f32>) -> Self {
        let points: Vec<_> = points.into_iter().collect();
        assert!(
            self.side.is_vertical(),
            "snap points require a vertical drawer"
        );
        assert!(
            !points.is_empty()
                && points.iter().all(|&p| p > 0. && p <= 1.)
                && points.windows(2).all(|pair| pair[0] < pair[1]),
            "snap points must increase strictly from above zero through at most one"
        );
        self.snap_points = points;
        self.snap_index = 0;
        self
    }

    /// Index of the current resting height, retained when the drawer closes.
    pub fn snap_index(&self) -> Option<usize> {
        (!self.snap_points.is_empty()).then_some(self.snap_index)
    }

    /// Changes the resting height, for example from an accessible size button.
    /// Returns false for an index outside the configured points.
    pub fn set_snap_index(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if index >= self.snap_points.len() {
            return false;
        }
        self.cancel_drag(window, cx);
        if self.snap_index != index {
            self.snap_index = index;
            cx.notify();
            window.refresh();
        }
        true
    }

    pub fn is_open(&self, cx: &App) -> bool {
        self.dialog.read(cx).is_open()
    }

    pub fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.dialog.read(cx).focus_handle().clone()
    }

    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_drag(window, cx);
        self.dialog.update(cx, |dialog, cx| dialog.open(window, cx));
    }

    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_drag(window, cx);
        self.dialog
            .update(cx, |dialog, cx| dialog.close(window, cx));
    }

    pub fn is_dragging(&self) -> bool {
        self.drag_start.is_some()
    }

    pub fn drag_offset(&self) -> Pixels {
        self.offset
    }

    fn outward_position(&self, position: Point<Pixels>) -> Pixels {
        match self.side {
            Placement::Top => -position.y,
            Placement::Bottom => position.y,
            Placement::Left => -position.x,
            Placement::Right => position.x,
        }
    }

    fn snap_height(&self, window: &Window) -> Option<Pixels> {
        self.snap_points
            .get(self.snap_index)
            .map(|fraction| window.viewport_size().height * *fraction)
    }

    fn start_drag(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if self.is_open(cx) {
            self.drag_start = Some(self.outward_position(position));
            self.offset = px(0.);
            cx.notify();
        }
    }

    fn drag_to(&mut self, position: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(start) = self.drag_start {
            let distance = self.outward_position(position) - start;
            self.offset = if let Some(rest) = self.snap_height(window) {
                let tallest =
                    window.viewport_size().height * self.snap_points[self.snap_points.len() - 1];
                distance.clamp(rest - tallest, rest)
            } else {
                distance.clamp(px(0.), self.extent)
            };
            cx.notify();
            window.refresh();
        }
    }

    fn finish_drag(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.drag_start.is_none() {
            return;
        }
        self.drag_to(position, window, cx);
        let dismiss = if let Some(rest) = self.snap_height(window) {
            let height = rest - self.offset;
            let viewport = window.viewport_size().height;
            // The closed position and each snap height are neighboring stops.
            if viewport > px(0.) && height < viewport * self.snap_points[0] / 2. {
                true
            } else {
                self.snap_index = (0..self.snap_points.len())
                    .min_by_key(|&index| (height - viewport * self.snap_points[index]).abs())
                    .unwrap();
                false
            }
        } else {
            self.extent > px(0.) && self.offset >= self.extent / 3.
        };
        self.cancel_drag(window, cx);
        if dismiss {
            self.dialog
                .update(cx, |dialog, cx| dialog.close(window, cx));
        }
    }

    fn cancel_drag(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.drag_start.take().is_some() || self.offset != px(0.) {
            self.offset = px(0.);
            cx.notify();
            window.refresh();
        }
    }
}

impl EventEmitter<DialogEvent> for DrawerState {}

/// An edge-aligned modal whose panel follows outward pointer movement.
///
/// [`Styled`] controls the panel, and normal children form its body. Supply a
/// visible, noninteractive grip with [`Self::handle`]; its entire bounds become
/// the drag target. Native mouse capture keeps tracking outside that target.
/// Without snap points, releasing past one third of the panel extent closes
/// the modal. Vertical snap points settle at the nearest height, including
/// closed. Keyboard users keep Dialog's Escape and focus
/// trap, and may use a regular `DialogClose` with [`DrawerState::dialog`].
///
/// Placement and clipping are structural. The application owns colors, panel
/// dimensions, a backdrop, content scrolling and the handle's appearance.
#[derive(IntoElement)]
pub struct Drawer {
    id: ElementId,
    state: Entity<DrawerState>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    handle: Option<AnyElement>,
    backdrop: Option<AnyElement>,
    label: Option<SharedString>,
    description: Option<SharedString>,
    escape: bool,
    backdrop_closable: bool,
}

impl Drawer {
    pub fn new(id: impl Into<ElementId>, state: &Entity<DrawerState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            style: StyleRefinement::default(),
            children: Vec::new(),
            handle: None,
            backdrop: None,
            label: None,
            description: None,
            escape: true,
            backdrop_closable: true,
        }
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn accessibility_description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn handle(mut self, handle: impl IntoElement) -> Self {
        self.handle = Some(handle.into_any_element());
        self
    }

    pub fn backdrop(mut self, backdrop: impl IntoElement) -> Self {
        self.backdrop = Some(backdrop.into_any_element());
        self
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.escape = close;
        self
    }

    pub fn close_on_backdrop_press(mut self, close: bool) -> Self {
        self.backdrop_closable = close;
        self
    }
}

impl Styled for Drawer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Drawer {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children);
    }
}

impl RenderOnce for Drawer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let model = self.state.read(cx);
        let dialog = model.dialog.clone();
        let side = model.side;
        let snap_height = model.snap_height(window);
        let offset = model.offset;
        let drag = self.state.clone();
        let geometry = self.state.clone();
        let movement = self.state.clone();
        let release = self.state;
        let panel = DialogPopup::new()
            .relative()
            .flex()
            .when(side.is_vertical(), |panel| panel.flex_col().w_full())
            .when(side.is_horizontal(), |panel| panel.h_full())
            .max_w_full()
            .max_h_full()
            .refine_style(&self.style)
            .on_prepaint(move |bounds, _, cx| {
                geometry.update(cx, |state, _| {
                    state.extent = if side.is_vertical() {
                        bounds.size.height
                    } else {
                        bounds.size.width
                    }
                });
            });
        let panel = match (side, snap_height) {
            (_, Some(height)) => panel
                .min_h_0()
                .max_h_full()
                .h((height - offset).max(px(0.)))
                .overflow_hidden(),
            (Placement::Bottom, None) => panel.top(offset),
            (Placement::Top, None) => panel.top(-offset),
            (Placement::Left, None) => panel.left(-offset),
            (Placement::Right, None) => panel.left(offset),
        };
        let handle = self.handle.map(|handle| {
            div()
                .id((self.id, "handle"))
                .test_support()
                .flex_shrink_0()
                .when(side == Placement::Top, |handle| handle.mt_auto())
                .on_mouse_down(MouseButton::Left, move |event, _, cx| {
                    cx.stop_propagation();
                    drag.update(cx, |state, cx| state.start_drag(event.position, cx));
                })
                .child(handle)
        });
        let panel = match side {
            Placement::Bottom => panel.children(handle).children(self.children),
            // Keep the grip at the panel's lower edge when its body overflows.
            Placement::Top => panel
                .when(handle.is_some(), |panel| panel.justify_end())
                .children(self.children)
                .children(handle),
            Placement::Left | Placement::Right => {
                let body = div()
                    .min_w_0()
                    .min_h_0()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .children(self.children);
                if side == Placement::Left {
                    panel.child(body).children(handle)
                } else {
                    panel.children(handle).child(body)
                }
            }
        }
        .child(
            canvas(
                |_, _, _| {},
                move |_, _, window, _| {
                    window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
                        if phase.capture() && movement.read(cx).is_dragging() {
                            movement.update(cx, |state, cx| {
                                if event.dragging() {
                                    state.drag_to(event.position, window, cx);
                                } else {
                                    state.cancel_drag(window, cx);
                                }
                            });
                            cx.stop_propagation();
                        }
                    });
                    window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
                        if phase.capture()
                            && event.button == MouseButton::Left
                            && release.read(cx).is_dragging()
                        {
                            release.update(cx, |state, cx| {
                                state.finish_drag(event.position, window, cx);
                            });
                            cx.stop_propagation();
                        }
                    });
                },
            )
            .absolute()
            .inset_0()
            .size_full(),
        );
        let dialog = Dialog::new(&dialog);
        match side {
            Placement::Bottom => dialog.items_end(),
            Placement::Top => dialog.items_start(),
            Placement::Left => dialog.justify_start(),
            Placement::Right => dialog.justify_end(),
        }
        .overflow_hidden()
        .close_on_escape(self.escape)
        .close_on_backdrop_press(self.backdrop_closable)
        .when_some(self.label, |this, label| this.accessibility_label(label))
        .when_some(self.description, |this, description| {
            this.accessibility_description(description)
        })
        .when_some(self.backdrop, |this, backdrop| this.backdrop(backdrop))
        .popup(panel)
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Modifiers, Render, TestAppContext, VisualTestContext, point, size};

    use super::*;

    struct Harness {
        state: Entity<DrawerState>,
        trigger: FocusHandle,
        _subscription: Subscription,
    }

    impl Render for Harness {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let side = self.state.read(cx).side;
            div()
                .size_full()
                .child(
                    crate::Button::new("open")
                        .track_focus(&self.trigger)
                        .child("Open"),
                )
                .child(
                    Drawer::new("drawer", &self.state)
                        .accessibility_label("Session review")
                        .h(px(240.))
                        .w(px(320.))
                        .backdrop(div().size_full())
                        .handle(
                            div()
                                .debug_selector(|| "drawer-grip".into())
                                .when(side.is_vertical(), |grip| grip.h(px(30.)).w_full())
                                .when(side.is_horizontal(), |grip| grip.w(px(30.)).h_full()),
                        )
                        .child(
                            crate::Button::new("body-action")
                                .debug_selector(|| "body-action".into())
                                .flex_shrink_0()
                                .h(px(40.))
                                .child("Body action"),
                        ),
                )
        }
    }

    fn open_drawer(
        cx: &mut TestAppContext,
        configure: impl FnOnce(DrawerState) -> DrawerState + 'static,
    ) -> (&mut VisualTestContext, Entity<Harness>) {
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let view = cx.new(|cx| {
                let state = cx.new(|cx| configure(DrawerState::new(window, cx)));
                Harness {
                    _subscription: cx.observe(&state, |_, _, cx| cx.notify()),
                    state,
                    trigger: cx.focus_handle(),
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
            state.update(cx, |state, cx| state.open(window, cx));
            window.draw(cx).clear(cx);
        });
        (cx, view)
    }

    #[gpui::test]
    fn only_handle_drag_dismisses_after_threshold_and_returns_modal_focus(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (cx, view) = open_drawer(cx, |state| state);
        let handle = cx.debug_bounds("drawer-grip").unwrap().center();
        let short = point(handle.x, handle.y + px(30.));
        cx.simulate_mouse_down(handle, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_move(short, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.simulate_mouse_up(short, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| {
            let state = view.read(cx).state.read(cx);
            assert!(state.is_open(cx));
            assert_eq!(state.drag_offset(), px(0.));
            window.draw(cx).clear(cx);
        });
        let body = cx.debug_bounds("body-action").unwrap().center();
        let end = point(handle.x, handle.y + px(120.));
        cx.simulate_mouse_down(body, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_move(end, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_up(end, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| {
            assert!(view.read(cx).state.read(cx).is_open(cx));
            window.draw(cx).clear(cx);
        });
        cx.simulate_mouse_down(handle, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_move(end, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.simulate_mouse_up(end, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| {
            assert!(!view.read(cx).state.read(cx).is_open(cx));
            assert!(!view.read(cx).state.read(cx).is_dragging());
            assert!(view.read(cx).trigger.is_focused(window));
        });
        cx.simulate_resize(gpui::size(px(260.), px(180.)));
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            state.update(cx, |state, cx| state.open(window, cx));
            window.draw(cx).clear(cx);
            assert!(
                state.read(cx).extent <= window.viewport_size().height,
                "the drawer must fit the viewport so its grip stays reachable"
            );
        });
        assert!(cx.debug_bounds("drawer-grip").unwrap().top() >= px(0.));
    }

    #[gpui::test]
    fn outward_drags_on_each_edge_close_only_along_that_axis(cx: &mut TestAppContext) {
        cx.update(crate::init);
        for (side, delta) in [
            (Placement::Top, point(px(0.), px(-120.))),
            (Placement::Left, point(px(-120.), px(0.))),
            (Placement::Right, point(px(120.), px(0.))),
        ] {
            let (cx, view) = open_drawer(cx, move |state| state.side(side));
            cx.simulate_resize(size(px(400.), px(400.)));
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let grip = cx.debug_bounds("drawer-grip").unwrap();
            match side {
                Placement::Top => assert_eq!(grip.bottom(), px(240.)),
                Placement::Left => assert_eq!(grip.right(), px(320.)),
                Placement::Right => assert_eq!(grip.left(), px(80.)),
                Placement::Bottom => unreachable!(),
            }
            let start = grip.center();
            let across = start + point(delta.y, delta.x);
            cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
            cx.simulate_mouse_up(across, MouseButton::Left, Modifiers::default());
            cx.update(|_, cx| assert!(view.read(cx).state.read(cx).is_open(cx)));
            cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
            cx.simulate_mouse_move(start + delta, MouseButton::Left, Modifiers::default());
            cx.update(|window, cx| window.draw(cx).clear(cx));
            cx.simulate_mouse_up(start + delta, MouseButton::Left, Modifiers::default());
            cx.update(|window, cx| {
                assert!(!view.read(cx).state.read(cx).is_open(cx));
                assert!(view.read(cx).trigger.is_focused(window));
            });
        }
    }

    #[gpui::test]
    fn snap_heights_follow_the_grip_resize_and_close_at_the_lowest_stop(cx: &mut TestAppContext) {
        cx.update(crate::init);
        for side in [Placement::Bottom, Placement::Top] {
            let (cx, view) = open_drawer(cx, move |state| state.side(side).snap_points([0.4, 0.8]));
            let state = view.read_with(cx, |view, _| view.state.clone());
            cx.simulate_resize(size(px(360.), px(100.)));
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let grip = cx.debug_bounds("drawer-grip").unwrap();
            match side {
                Placement::Bottom => assert_eq!(grip.top(), px(60.)),
                Placement::Top => assert_eq!(grip.bottom(), px(40.)),
                _ => unreachable!(),
            }
            cx.simulate_resize(size(px(360.), px(400.)));
            cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert_eq!(state.read(cx).extent, px(160.));
            });
            let sign = if side == Placement::Bottom { 1. } else { -1. };
            let start = cx.debug_bounds("drawer-grip").unwrap().center();
            let end = start - point(px(0.), px(120.) * sign);
            cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
            cx.simulate_mouse_move(end, MouseButton::Left, Modifiers::default());
            cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert_eq!(state.read(cx).extent, px(280.));
            });
            cx.simulate_mouse_up(end, MouseButton::Left, Modifiers::default());
            cx.simulate_resize(size(px(360.), px(300.)));
            cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert_eq!(state.read(cx).snap_index(), Some(1));
                assert_eq!(state.read(cx).extent, px(240.));
                state.update(cx, |state, cx| {
                    assert!(!state.set_snap_index(2, window, cx));
                    assert!(state.set_snap_index(0, window, cx));
                });
                window.draw(cx).clear(cx);
                assert_eq!(state.read(cx).extent, px(120.));
            });
            let start = cx.debug_bounds("drawer-grip").unwrap().center();
            cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
            cx.simulate_mouse_up(
                start + point(px(0.), px(80.) * sign),
                MouseButton::Left,
                Modifiers::default(),
            );
            cx.update(|_, cx| assert!(!state.read(cx).is_open(cx)));
        }
    }
}
