//! A carousel showing one or more slides per view, with native focus, discrete
//! swiping, and logical navigation.

use gpui::{
    AnyElement, App, AppContext as _, Axis, Context, DragMoveEvent, ElementId, Empty, Entity,
    EntityId, EventEmitter, FocusHandle, InteractiveElement, IntoElement, KeyBinding, MouseButton,
    ParentElement, Pixels, Point, Render, RenderOnce, Role, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::{
    Button, Direction, TestSupportExt as _,
    actions::{SelectDown, SelectFirst, SelectLast, SelectLeft, SelectRight, SelectUp},
};

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("left", SelectLeft, Some("Carousel && axis == horizontal")),
        KeyBinding::new("right", SelectRight, Some("Carousel && axis == horizontal")),
        KeyBinding::new("up", SelectUp, Some("Carousel && axis == vertical")),
        KeyBinding::new("down", SelectDown, Some("Carousel && axis == vertical")),
        KeyBinding::new("home", SelectFirst, Some("Carousel")),
        KeyBinding::new("end", SelectLast, Some("Carousel")),
    ]);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CarouselEvent {
    Change(usize),
}

/// Retained state for a fixed ordered set of slides. Labels name each slide.
///
/// A view shows `slides_per_view` consecutive slides starting at the selected
/// (leading) slide; one is the default. Only those slides are mounted, and the
/// application sizes them, for example with `relative(1. / n)` widths. Each
/// step moves by one slide, and the last view is always full: the leading index
/// ranges over [`snap_count`](Self::snap_count) positions. Looping wraps between
/// the last full view and the first. Application entities retain slide data;
/// there is no autoplay, timer, or focus movement merely because data changes.
pub struct CarouselState {
    labels: Vec<SharedString>,
    selected: Option<usize>,
    per_view: usize,
    looping: bool,
    axis: Axis,
    direction: Direction,
    focus: FocusHandle,
    slide_focus: Vec<FocusHandle>,
    drag_origin: Point<Pixels>,
    drag_committed: bool,
}

impl CarouselState {
    pub fn new<L: Into<SharedString>>(
        labels: impl IntoIterator<Item = L>,
        cx: &mut Context<Self>,
    ) -> Self {
        let labels: Vec<_> = labels.into_iter().map(Into::into).collect();
        Self {
            selected: (!labels.is_empty()).then_some(0),
            slide_focus: labels.iter().map(|_| cx.focus_handle()).collect(),
            labels,
            per_view: 1,
            looping: false,
            axis: Axis::Horizontal,
            direction: Direction::Ltr,
            focus: cx.focus_handle(),
            drag_origin: Point::default(),
            drag_committed: false,
        }
    }

    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }
    /// Shows `count` slides per view; panics when `count` is zero.
    pub fn slides_per_view(mut self, count: usize) -> Self {
        assert!(count > 0, "A carousel view shows at least one slide");
        self.per_view = count;
        self.selected = self.selected.map(|index| index.min(self.last_start()));
        self
    }
    /// Changes the view size, keeping the leading slide where possible and
    /// moving focus out of slides that leave the view.
    pub fn set_slides_per_view(
        &mut self,
        count: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        assert!(count > 0, "A carousel view shows at least one slide");
        if self.per_view == count {
            return false;
        }
        let previous = self.visible_range();
        self.per_view = count;
        let Some(selected) = self.selected else {
            return true;
        };
        let start = selected.min(self.last_start());
        self.release_hidden_focus(previous, start, window, cx);
        self.selected = Some(start);
        if start != selected {
            cx.emit(CarouselEvent::Change(start));
        }
        cx.notify();
        window.refresh();
        true
    }
    /// The leading visible slide.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }
    /// The slides currently mounted, in order.
    pub fn visible_range(&self) -> std::ops::Range<usize> {
        self.selected.map_or(0..0, |start| {
            start..(start + self.per_view).min(self.labels.len())
        })
    }
    /// The number of distinct leading positions, one per full view.
    pub fn snap_count(&self) -> usize {
        if self.labels.is_empty() {
            0
        } else {
            self.last_start() + 1
        }
    }
    pub fn labels(&self) -> &[SharedString] {
        &self.labels
    }
    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }
    pub fn can_previous(&self) -> bool {
        self.snap_count() > 1 && (self.looping || self.selected.is_some_and(|index| index > 0))
    }
    pub fn can_next(&self) -> bool {
        self.snap_count() > 1
            && (self.looping || self.selected.is_some_and(|index| index < self.last_start()))
    }
    fn last_start(&self) -> usize {
        self.labels.len().saturating_sub(self.per_view)
    }
    fn release_hidden_focus(
        &self,
        previous: std::ops::Range<usize>,
        start: usize,
        window: &mut Window,
        cx: &mut App,
    ) {
        let next = start..start + self.per_view;
        if previous
            .filter(|index| !next.contains(index))
            .any(|index| self.slide_focus[index].contains_focused(window, cx))
        {
            self.focus.focus(window, cx);
        }
    }

    /// Makes `index` the leading slide when it starts a full view. Focus inside
    /// slides leaving the view returns to the carousel scope before that content
    /// is unmounted; focus in slides that remain visible or outside stays put.
    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if index >= self.snap_count() || self.selected == Some(index) {
            return false;
        }
        self.release_hidden_focus(self.visible_range(), index, window, cx);
        self.selected = Some(index);
        cx.emit(CarouselEvent::Change(index));
        cx.notify();
        window.refresh();
        true
    }

    pub fn previous(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.advance(false, window, cx)
    }
    pub fn next(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.advance(true, window, cx)
    }
    fn advance(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(index) = self.selected else {
            return false;
        };
        if if forward {
            !self.can_next()
        } else {
            !self.can_previous()
        } {
            return false;
        }
        let snaps = self.snap_count();
        let next = if forward {
            (index + 1) % snaps
        } else {
            (index + snaps - 1) % snaps
        };
        self.select(next, window, cx)
    }
    fn swipe_at_pointer(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.drag_committed {
            return;
        }
        let delta = match self.axis {
            Axis::Horizontal => position.x - self.drag_origin.x,
            Axis::Vertical => position.y - self.drag_origin.y,
        };
        if delta.abs() < px(40.) {
            return;
        }
        self.drag_committed = true;
        let forward = match self.axis {
            Axis::Horizontal => self.direction.horizontal_forward(delta < px(0.)),
            Axis::Vertical => delta < px(0.),
        };
        self.advance(forward, window, cx);
    }

    fn key_advance(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.focus.is_focused(window) {
            self.advance(forward, window, cx);
        } else {
            cx.propagate();
        }
    }
    fn key_edge(&mut self, last: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus.is_focused(window) {
            cx.propagate();
            return;
        }
        if !self.labels.is_empty() {
            self.select(if last { self.last_start() } else { 0 }, window, cx);
        }
    }
}
impl EventEmitter<CarouselEvent> for CarouselState {}

/// Named carousel region. Arrow keys act only while the region itself has focus,
/// leaving interactive slide descendants their own key bindings. Use native
/// Buttons or [`CarouselControl`] for pointer and keyboard previous/next actions.
#[derive(IntoElement)]
pub struct Carousel {
    base: crate::ObservedElement<gpui::Stateful<gpui::Div>>,
    state: Entity<CarouselState>,
    label: SharedString,
    axis: Axis,
    direction: Option<Direction>,
}
impl Carousel {
    pub fn new(
        id: impl Into<ElementId>,
        state: &Entity<CarouselState>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            base: div().id(id).test_support(),
            state: state.clone(),
            label: label.into(),
            axis: Axis::Horizontal,
            direction: None,
        }
    }
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }
}
impl Styled for Carousel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}
impl ParentElement for Carousel {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(children);
    }
}
impl RenderOnce for Carousel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let direction = self.direction.unwrap_or_else(|| Direction::current(cx));
        self.state.update(cx, |state, _| {
            state.axis = self.axis;
            state.direction = direction;
        });
        let focus = self.state.read(cx).focus.clone();
        let first = self.state.clone();
        let last = self.state.clone();
        let previous = self.state.clone();
        let next = self.state;
        let base = self
            .base
            .role(Role::Region)
            .aria_label(self.label)
            .track_focus(&focus)
            .on_action(move |_: &SelectFirst, window, cx| {
                first.update(cx, |state, cx| state.key_edge(false, window, cx));
            })
            .on_action(move |_: &SelectLast, window, cx| {
                last.update(cx, |state, cx| state.key_edge(true, window, cx));
            });
        match self.axis {
            Axis::Horizontal => base
                .key_context("Carousel axis=horizontal")
                .on_action(move |_: &SelectLeft, window, cx| {
                    previous.update(cx, |state, cx| {
                        state.key_advance(direction.horizontal_forward(false), window, cx)
                    });
                })
                .on_action(move |_: &SelectRight, window, cx| {
                    next.update(cx, |state, cx| {
                        state.key_advance(direction.horizontal_forward(true), window, cx)
                    });
                }),
            Axis::Vertical => base
                .key_context("Carousel axis=vertical")
                .on_action(move |_: &SelectUp, window, cx| {
                    previous.update(cx, |state, cx| state.key_advance(false, window, cx));
                })
                .on_action(move |_: &SelectDown, window, cx| {
                    next.update(cx, |state, cx| state.key_advance(true, window, cx));
                }),
        }
    }
}

#[derive(Clone)]
struct DragCarousel(EntityId);
impl Render for DragCarousel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// One slide, mounted only while in the current view, with position/count
/// metadata and a stable focus scope.
/// Dragging its content by 40 logical pixels snaps once per gesture. GPUI owns
/// drag cancellation and suppresses click activation after a drag begins.
/// Axis and direction come from the enclosing [`Carousel`]. There is no inertia or
/// autoplay; use application animation only when the product calls for it.
#[derive(IntoElement)]
pub struct CarouselSlide {
    base: crate::ObservedElement<gpui::Stateful<gpui::Div>>,
    state: Entity<CarouselState>,
    index: usize,
}
impl CarouselSlide {
    pub fn new(state: &Entity<CarouselState>, index: usize) -> Self {
        Self {
            base: div()
                .id((
                    ElementId::from(("carousel-slide", state.entity_id())),
                    index.to_string(),
                ))
                .test_support(),
            state: state.clone(),
            index,
        }
    }
}
impl Styled for CarouselSlide {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}
impl ParentElement for CarouselSlide {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(children);
    }
}
impl RenderOnce for CarouselSlide {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let model = self.state.read(cx);
        assert!(
            self.index < model.labels.len(),
            "CarouselSlide index must identify a slide"
        );
        if !model.visible_range().contains(&self.index) {
            return Empty.into_any_element();
        }
        let id = self.state.entity_id();
        let drag = self.state.clone();
        self.base
            .role(Role::Group)
            .aria_label(model.labels[self.index].clone())
            .aria_position_in_set(self.index + 1)
            .aria_size_of_set(model.labels.len())
            .track_focus(&model.slide_focus[self.index].clone().tab_stop(false))
            .overflow_hidden()
            .on_mouse_down(
                MouseButton::Left,
                window.listener_for(&self.state, |state, event: &gpui::MouseDownEvent, _, _| {
                    state.drag_origin = event.position;
                    state.drag_committed = false;
                }),
            )
            .on_drag(DragCarousel(id), move |value, _, window, cx| {
                // GPUI starts dragging after this move's capture phase, so the
                // first displacement does not reach on_drag_move. Handle it now.
                drag.update(cx, |state, cx| {
                    state.drag_committed = false;
                    state.swipe_at_pointer(window.mouse_position(), window, cx);
                });
                cx.stop_propagation();
                cx.new(|_| value.clone())
            })
            .on_drag_move(window.listener_for(
                &self.state,
                move |state, event: &DragMoveEvent<DragCarousel>, window, cx| {
                    if event.drag(cx).0 == id {
                        state.swipe_at_pointer(event.event.position, window, cx);
                    }
                },
            ))
            .into_any_element()
    }
}

/// Previous/next Button composition, including boundary disabled behavior.
#[derive(IntoElement)]
pub struct CarouselControl {
    button: Button,
    state: Entity<CarouselState>,
    forward: bool,
}
impl CarouselControl {
    pub fn previous(id: impl Into<ElementId>, state: &Entity<CarouselState>) -> Self {
        Self {
            button: Button::new(id).accessibility_label("Previous slide"),
            state: state.clone(),
            forward: false,
        }
    }
    pub fn next(id: impl Into<ElementId>, state: &Entity<CarouselState>) -> Self {
        Self {
            button: Button::new(id).accessibility_label("Next slide"),
            state: state.clone(),
            forward: true,
        }
    }
}
impl Styled for CarouselControl {
    fn style(&mut self) -> &mut StyleRefinement {
        self.button.style()
    }
}
impl ParentElement for CarouselControl {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.button.extend(children);
    }
}
impl RenderOnce for CarouselControl {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let enabled = if self.forward {
            self.state.read(cx).can_next()
        } else {
            self.state.read(cx).can_previous()
        };
        self.button
            .disabled(!enabled)
            .on_click(move |_, window, cx| {
                self.state.update(cx, |state, cx| {
                    state.advance(self.forward, window, cx);
                });
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Root;
    use gpui::{Modifiers, Subscription, TestAppContext, point};

    struct Harness {
        state: Entity<CarouselState>,
        button: FocusHandle,
        axis: Axis,
        direction: Direction,
        _subscription: Subscription,
    }
    impl Render for Harness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Carousel::new("carousel", &self.state, "Screenshots")
                .axis(self.axis)
                .direction(self.direction)
                .w(px(300.))
                .child(
                    CarouselSlide::new(&self.state, 0)
                        .w(px(300.))
                        .h(px(120.))
                        .child("First image"),
                )
                .child(
                    CarouselSlide::new(&self.state, 1)
                        .w(px(300.))
                        .h(px(120.))
                        .child(
                            Button::new("slide-action")
                                .track_focus(&self.button)
                                .child("Inspect image"),
                        ),
                )
                .child(CarouselControl::previous("previous", &self.state).child("Previous"))
                .child(CarouselControl::next("next", &self.state).child("Next"))
        }
    }
    #[gpui::test]
    fn keys_swipe_and_outgoing_slide_keep_focus_reachable(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let state = cx.new(|cx| CarouselState::new(["First", "Second"], cx));
                let subscription = cx.observe(&state, |_, _, cx| cx.notify());
                Harness {
                    state,
                    button: cx.focus_handle(),
                    axis: Axis::Horizontal,
                    direction: Direction::Ltr,
                    _subscription: subscription,
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
            let focus = view.read(cx).state.read(cx).focus.clone();
            focus.focus(window, cx);
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("right");
        cx.run_until_parked();
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            assert_eq!(state.read(cx).selected_index(), Some(1));
            assert!(!state.read(cx).can_next());
            let button = view.read(cx).button.clone();
            button.focus(window, cx);
            window.draw(cx).clear(cx);
            state.update(cx, |state, cx| {
                assert!(state.previous(window, cx));
                assert!(state.focus.is_focused(window));
            });
            window.draw(cx).clear(cx);
        });
        // A fast swipe can arrive as one coalesced move followed by release.
        cx.simulate_mouse_down(
            point(px(180.), px(50.)),
            MouseButton::Left,
            Modifiers::default(),
        );
        cx.simulate_mouse_move(
            point(px(130.), px(50.)),
            Some(MouseButton::Left),
            Modifiers::default(),
        );
        cx.simulate_mouse_up(
            point(px(130.), px(50.)),
            MouseButton::Left,
            Modifiers::default(),
        );
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert_eq!(view.read(cx).state.read(cx).selected_index(), Some(1));
        });
        for (axis, direction, key, end) in [
            (
                Axis::Horizontal,
                Direction::Rtl,
                "left",
                point(px(240.), px(90.)),
            ),
            (
                Axis::Vertical,
                Direction::Rtl,
                "down",
                point(px(180.), px(40.)),
            ),
        ] {
            view.update(cx, |view, cx| {
                view.axis = axis;
                view.direction = direction;
                cx.notify();
            });
            cx.update(|window, cx| {
                view.read(cx).state.clone().update(cx, |state, cx| {
                    state.select(0, window, cx);
                    state.focus.focus(window, cx);
                });
                window.draw(cx).clear(cx);
            });
            cx.simulate_keystrokes(key);
            cx.update(|window, cx| {
                view.read(cx).state.clone().update(cx, |state, cx| {
                    assert_eq!(state.selected_index(), Some(1));
                    state.select(0, window, cx);
                });
                window.draw(cx).clear(cx);
            });
            // Slides have no separate axis/direction configuration to drift.
            cx.simulate_mouse_down(
                point(px(180.), px(90.)),
                MouseButton::Left,
                Modifiers::default(),
            );
            cx.simulate_mouse_move(end, Some(MouseButton::Left), Modifiers::default());
            // A second move must not advance again, even with looping enabled.
            cx.update(|_, cx| {
                view.read(cx)
                    .state
                    .clone()
                    .update(cx, |state, _| state.looping = true)
            });
            cx.simulate_mouse_move(end, Some(MouseButton::Left), Modifiers::default());
            cx.simulate_mouse_up(end, MouseButton::Left, Modifiers::default());
            cx.update(|_, cx| {
                view.read(cx).state.clone().update(cx, |state, _| {
                    assert_eq!(state.selected_index(), Some(1));
                    state.looping = false;
                })
            });
        }
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            assert_eq!(state.read(cx).selected_index(), Some(1));
            state.update(cx, |state, cx| {
                assert!(!state.next(window, cx));
                state.looping = true;
                assert!(state.next(window, cx));
                assert_eq!(state.selected, Some(0));
            });
        });
    }

    struct MultiHarness {
        state: Entity<CarouselState>,
        actions: Vec<FocusHandle>,
        _subscription: Subscription,
    }
    impl Render for MultiHarness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Carousel::new("carousel", &self.state, "Albums")
                .flex()
                .w(px(300.))
                .children(self.actions.iter().enumerate().map(|(index, action)| {
                    CarouselSlide::new(&self.state, index)
                        .w(px(100.))
                        .h(px(80.))
                        .child(
                            Button::new(("open", index))
                                .track_focus(action)
                                .child("Open"),
                        )
                }))
        }
    }

    #[gpui::test]
    fn several_slides_per_view_snap_by_one_and_release_hidden_focus(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let state = cx
                    .new(|cx| CarouselState::new(["A", "B", "C", "D", "E"], cx).slides_per_view(3));
                let subscription = cx.observe(&state, |_, _, cx| cx.notify());
                MultiHarness {
                    state,
                    actions: (0..5).map(|_| cx.focus_handle()).collect(),
                    _subscription: subscription,
                }
            });
            Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<MultiHarness>()
            .unwrap();
        let state = view.read_with(cx, |view, _| view.state.clone());
        let mounted = |cx: &mut gpui::VisualTestContext| {
            cx.update(|window, _| {
                let mut slides: Vec<_> = crate::test_support::snapshots(window)
                    .into_iter()
                    .filter(|node| node.role() == Some(Role::Group))
                    .map(|node| (node.bounds().left(), node.label().unwrap().to_owned()))
                    .collect();
                slides.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                slides
                    .into_iter()
                    .map(|(_, label)| label)
                    .collect::<Vec<_>>()
            })
        };
        cx.update(|window, cx| {
            window.activate_window();
            state.read(cx).focus.clone().focus(window, cx);
            window.draw(cx).clear(cx);
        });
        assert_eq!(mounted(cx), ["A", "B", "C"]);
        cx.update(|_, cx| {
            let state = state.read(cx);
            assert_eq!(state.snap_count(), 3);
            assert!(!state.can_previous());
        });
        cx.simulate_keystrokes("end");
        cx.run_until_parked();
        assert_eq!(mounted(cx), ["C", "D", "E"], "the last view stays full");
        cx.update(|window, cx| {
            assert!(!state.read(cx).can_next());
            // Focus in a slide that stays visible is kept.
            let action = view.read(cx).actions[3].clone();
            action.focus(window, cx);
            state.update(cx, |state, cx| assert!(state.previous(window, cx)));
            assert!(action.is_focused(window));
            assert_eq!(state.read(cx).visible_range(), 1..4);
            // Focus in a slide that leaves the view returns to the carousel.
            state.update(cx, |state, cx| assert!(state.previous(window, cx)));
            assert!(state.read(cx).focus.is_focused(window));
            state.update(cx, |state, cx| {
                state.looping = true;
                assert!(
                    state.previous(window, cx),
                    "looping wraps to the last full view"
                );
                assert_eq!(state.selected_index(), Some(2));
                assert!(state.set_slides_per_view(4, window, cx));
                assert_eq!(
                    state.visible_range(),
                    1..5,
                    "a larger view keeps the last one full"
                );
            });
            window.draw(cx).clear(cx);
        });
        assert_eq!(mounted(cx), ["B", "C", "D", "E"]);
    }
}
