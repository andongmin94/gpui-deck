//! A multi-panel split with shared native layout and focusable dividers.

use std::ops::RangeInclusive;

use gpui::{
    AccessibleAction, AnyElement, App, AppContext as _, Axis, Bounds, Context, DragMoveEvent,
    ElementId, Empty, Entity, EntityId, EventEmitter, FocusHandle, InteractiveElement,
    Interactivity, IntoElement, KeyBinding, MouseButton, MouseDownEvent, Orientation,
    ParentElement, Pixels, Point, Render, RenderOnce, Role, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, accesskit, div,
    prelude::FluentBuilder as _, px,
};

use crate::{
    ElementExt as _, TestSupportExt as _,
    a11y_disabled::A11yDisabledExt as _,
    actions::{Confirm, SelectDown, SelectFirst, SelectLast, SelectLeft, SelectRight, SelectUp},
};

const STEP: f32 = 0.02;
const EPSILON: f32 = 0.0001;

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new(
            "left",
            SelectLeft,
            Some("ResizableHandle && axis == horizontal"),
        ),
        KeyBinding::new(
            "right",
            SelectRight,
            Some("ResizableHandle && axis == horizontal"),
        ),
        KeyBinding::new("up", SelectUp, Some("ResizableHandle && axis == vertical")),
        KeyBinding::new(
            "down",
            SelectDown,
            Some("ResizableHandle && axis == vertical"),
        ),
        KeyBinding::new("home", SelectFirst, Some("ResizableHandle")),
        KeyBinding::new("end", SelectLast, Some("ResizableHandle")),
        KeyBinding::new(
            "enter",
            Confirm { secondary: false },
            Some("ResizableHandle"),
        ),
    ]);
}

/// Emitted once when the panel layout changes, including programmatic updates.
/// Sizes are fractions of the usable space, in panel order, and sum to one.
#[derive(Clone, Debug, PartialEq)]
pub enum ResizableEvent {
    Change(Vec<f32>),
}

/// One panel's initial size and constraints, as fractions of the usable space,
/// with optional pixel limits that apply once the group has been laid out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizablePanel {
    default_size: f32,
    minimum: f32,
    maximum: f32,
    pixel_minimum: Pixels,
    pixel_maximum: Option<Pixels>,
    collapsed: Option<f32>,
}

impl ResizablePanel {
    /// A panel starting at `default_size`, free to resize between zero and one.
    pub fn new(default_size: f32) -> Self {
        Self {
            default_size,
            minimum: 0.,
            maximum: 1.,
            pixel_minimum: px(0.),
            pixel_maximum: None,
            collapsed: None,
        }
    }

    /// Bounds the panel's expanded size in logical pixels of usable space, in
    /// addition to its fractional limits; the tighter bound wins. Pixel limits
    /// follow the measured group, so a narrower window raises a pixel minimum's
    /// share. When the measured space cannot satisfy every panel's pixel limits,
    /// the group falls back to the fractional limits alone.
    ///
    /// # Panics
    /// Bounds must be finite and satisfy `0 <= minimum <= maximum`.
    pub fn pixel_limits(mut self, limits: RangeInclusive<Pixels>) -> Self {
        let (minimum, maximum) = limits.into_inner();
        assert!(
            f32::from(minimum).is_finite()
                && f32::from(maximum).is_finite()
                && px(0.) <= minimum
                && minimum <= maximum,
            "Resizable pixel limits must satisfy 0 <= minimum <= maximum"
        );
        self.pixel_minimum = minimum;
        self.pixel_maximum = Some(maximum);
        self
    }

    /// The fractional limits for a measured usable length.
    fn measured(mut self, usable: Pixels) -> Self {
        if usable <= px(0.) {
            return self;
        }
        self.minimum = self.minimum.max(self.pixel_minimum / usable).min(1.);
        if let Some(maximum) = self.pixel_maximum {
            self.maximum = self.maximum.min(maximum / usable);
        }
        // Raising the minimum keeps it above any collapsed size.
        self.maximum = self.maximum.max(self.minimum);
        self
    }

    /// Bounds the panel's expanded size.
    ///
    /// # Panics
    /// Bounds must be finite and satisfy `0 <= minimum <= maximum <= 1`.
    pub fn limits(mut self, limits: RangeInclusive<f32>) -> Self {
        let (minimum, maximum) = limits.into_inner();
        assert!(
            minimum.is_finite()
                && maximum.is_finite()
                && 0. <= minimum
                && minimum <= maximum
                && maximum <= 1.,
            "Resizable panel limits must satisfy 0 <= minimum <= maximum <= 1"
        );
        self.minimum = minimum;
        self.maximum = maximum;
        self
    }

    /// Lets the panel collapse to `collapsed_size`, below its minimum. Pointer
    /// resizing snaps to the collapsed size once past halfway between the two.
    ///
    /// # Panics
    /// The collapsed size must be finite, non-negative, and below the minimum.
    pub fn collapsible(mut self, collapsed_size: f32) -> Self {
        assert!(
            collapsed_size.is_finite() && 0. <= collapsed_size,
            "Resizable collapsed size must be finite and non-negative"
        );
        self.collapsed = Some(collapsed_size);
        self
    }

    fn is_collapsed_size(&self, size: f32) -> bool {
        self.collapsed
            .is_some_and(|collapsed| (size - collapsed).abs() < EPSILON)
    }

    fn accepts(&self, size: f32) -> bool {
        self.is_collapsed_size(size)
            || (self.minimum - EPSILON <= size && size <= self.maximum + EPSILON)
    }

    /// The nearest size this panel accepts for a requested size.
    fn constrain(&self, requested: f32) -> f32 {
        match self.collapsed {
            Some(collapsed) if requested < self.minimum => {
                if requested < (collapsed + self.minimum) / 2. {
                    collapsed
                } else {
                    self.minimum
                }
            }
            _ => requested.clamp(self.minimum, self.maximum),
        }
    }

    fn smallest(&self) -> f32 {
        self.collapsed.unwrap_or(self.minimum)
    }
}

struct Drag {
    handle: usize,
    origin: Vec<f32>,
    start: Pixels,
    usable: Pixels,
}

/// Retained panel sizes and divider focus for one mounted panel group.
///
/// Sizes divide the space remaining after the actual divider thicknesses, so
/// they stay proportional when the application's window changes size. Moving a
/// divider grows the adjacent panel and shrinks panels on the other side,
/// nearest first, within their constraints. To persist a layout, store
/// [`Self::sizes`] from [`ResizableEvent::Change`] in application storage and
/// restore it with [`Self::set_sizes`].
pub struct ResizableState {
    axis: Axis,
    panels: Vec<ResizablePanel>,
    /// The panels' limits for the measured usable length.
    effective: Vec<ResizablePanel>,
    usable: Pixels,
    sizes: Vec<f32>,
    restore: Vec<f32>,
    disabled: bool,
    root_focus: FocusHandle,
    panel_focus: Vec<FocusHandle>,
    handle_focus: Vec<FocusHandle>,
    panel_bounds: Vec<Bounds<Pixels>>,
    handle_bounds: Vec<Bounds<Pixels>>,
    grab: Pixels,
    drag: Option<Drag>,
}

impl ResizableState {
    /// Creates a group of at least two panels whose default sizes sum to one.
    ///
    /// # Panics
    /// Panics when fewer than two panels are supplied, a default size is outside
    /// its panel's constraints, or the defaults do not sum to one.
    pub fn new(
        axis: Axis,
        panels: impl IntoIterator<Item = ResizablePanel>,
        cx: &mut Context<Self>,
    ) -> Self {
        let panels: Vec<_> = panels.into_iter().collect();
        assert!(panels.len() >= 2, "Resizable requires at least two panels");
        for panel in &panels {
            if let Some(collapsed) = panel.collapsed {
                assert!(
                    collapsed < panel.minimum,
                    "Resizable collapsed size must be below the panel minimum"
                );
            }
        }
        let sizes: Vec<_> = panels.iter().map(|panel| panel.default_size).collect();
        assert!(
            valid_layout(&panels, &sizes),
            "Resizable default sizes must satisfy panel limits and sum to one"
        );
        let restore = panels
            .iter()
            .zip(&sizes)
            .map(|(panel, size)| {
                if panel.is_collapsed_size(*size) {
                    panel.minimum
                } else {
                    *size
                }
            })
            .collect();
        let count = panels.len();
        Self {
            axis,
            sizes,
            restore,
            disabled: false,
            root_focus: cx.focus_handle(),
            panel_focus: (0..count).map(|_| cx.focus_handle()).collect(),
            handle_focus: (1..count).map(|_| cx.focus_handle()).collect(),
            panel_bounds: vec![Bounds::default(); count],
            handle_bounds: vec![Bounds::default(); count - 1],
            effective: panels.clone(),
            usable: px(0.),
            panels,
            grab: px(0.),
            drag: None,
        }
    }

    pub fn axis(&self) -> Axis {
        self.axis
    }

    /// Current panel sizes as fractions of the usable space.
    pub fn sizes(&self) -> &[f32] {
        &self.sizes
    }

    pub fn panel_count(&self) -> usize {
        self.panels.len()
    }

    /// Focus handle for the divider following panel `index`.
    pub fn handle_focus(&self, index: usize) -> &FocusHandle {
        &self.handle_focus[index]
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Whether a collapsible panel is at its collapsed size.
    pub fn is_collapsed(&self, panel: usize) -> bool {
        self.panels[panel].is_collapsed_size(self.sizes[panel])
    }

    /// Replaces the whole layout, for example from application storage.
    /// Programmatic changes remain allowed while user resizing is disabled.
    ///
    /// # Panics
    /// The sizes must match the panel count, satisfy each panel's constraints,
    /// and sum to one.
    /// A layout that satisfies the fractional limits is then fitted to any pixel
    /// limits for the current size.
    pub fn set_sizes(&mut self, sizes: Vec<f32>, window: &mut Window, cx: &mut Context<Self>) {
        assert!(
            sizes.len() == self.panels.len() && valid_layout(&self.panels, &sizes),
            "Resizable sizes must match the panels, satisfy their limits and sum to one"
        );
        let sizes = fit(&self.effective, &sizes).unwrap_or(sizes);
        self.apply(sizes, window, cx);
    }

    /// Records the measured usable length, refits the layout to pixel limits,
    /// and reports whether the sizes changed.
    fn measure(&mut self, usable: Pixels, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if (usable - self.usable).abs() < px(0.5) {
            return false;
        }
        self.usable = usable;
        let measured: Vec<_> = self
            .panels
            .iter()
            .map(|panel| panel.measured(usable))
            .collect();
        let smallest: f32 = measured.iter().map(ResizablePanel::smallest).sum();
        let largest: f32 = measured.iter().map(|panel| panel.maximum).sum();
        self.effective = if smallest <= 1. + EPSILON && largest >= 1. - EPSILON {
            measured
        } else {
            self.panels.clone()
        };
        match fit(&self.effective, &self.sizes) {
            Some(sizes) if sizes != self.sizes => {
                self.apply(sizes, window, cx);
                true
            }
            _ => false,
        }
    }

    /// Collapses a collapsible panel, giving its space to its neighbors.
    /// Does nothing when the neighbors cannot absorb that space.
    pub fn collapse(&mut self, panel: usize, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(collapsed) = self.panels[panel].collapsed {
            self.resize_panel(panel, collapsed, window, cx);
        }
    }

    /// Restores a collapsed panel to its last expanded size.
    /// Does nothing when the neighbors cannot supply that space.
    pub fn expand(&mut self, panel: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_collapsed(panel) {
            self.resize_panel(panel, self.restore[panel], window, cx);
        }
    }

    /// Disables pointer, keyboard, and accessibility resizing, and normal Tab
    /// entry. An already focused divider retains its binding so Tab can exit.
    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled != disabled {
            self.disabled = disabled;
            cx.notify();
            window.refresh();
        }
    }

    fn apply(&mut self, sizes: Vec<f32>, window: &mut Window, cx: &mut Context<Self>) {
        if sizes == self.sizes {
            return;
        }
        for (index, size) in sizes.iter().enumerate() {
            let panel = &self.panels[index];
            if panel.is_collapsed_size(*size) {
                if *size == 0. && self.panel_focus[index].contains_focused(window, cx) {
                    let handle = index.min(self.handle_focus.len() - 1);
                    if self.disabled {
                        self.root_focus.focus(window, cx);
                    } else {
                        self.handle_focus[handle].focus(window, cx);
                    }
                }
            } else {
                self.restore[index] = *size;
            }
        }
        self.sizes = sizes;
        cx.emit(ResizableEvent::Change(self.sizes.clone()));
        cx.notify();
        window.refresh();
    }

    fn resize_panel(
        &mut self,
        panel: usize,
        target: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delta = target - self.sizes[panel];
        let layout = if panel + 1 < self.panels.len() {
            adjust(&self.effective, &self.sizes, panel, delta)
        } else {
            adjust(&self.effective, &self.sizes, panel - 1, -delta)
        };
        if let Some(layout) = layout {
            self.apply(layout, window, cx);
        }
    }

    /// Moves divider `handle` by one keyboard step. A collapsed panel expands to
    /// its minimum, and a collapsible panel at its minimum collapses.
    fn nudge(&mut self, handle: usize, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        let (grow, shrink) = if forward {
            (handle, handle + 1)
        } else {
            (handle + 1, handle)
        };
        let grow_panel = &self.effective[grow];
        let shrink_panel = &self.effective[shrink];
        let amount = if grow_panel.is_collapsed_size(self.sizes[grow]) {
            grow_panel.minimum - self.sizes[grow]
        } else if let Some(collapsed) = shrink_panel.collapsed
            && (self.sizes[shrink] - shrink_panel.minimum).abs() < EPSILON
        {
            self.sizes[shrink] - collapsed
        } else {
            STEP
        };
        let delta = if forward { amount } else { -amount };
        if let Some(layout) = adjust(&self.effective, &self.sizes, handle, delta) {
            self.apply(layout, window, cx);
        }
    }

    /// Home and End give the panel before the divider its smallest or largest
    /// allowed size, as far as the other panels permit.
    fn edge(&mut self, handle: usize, last: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        let panel = self.effective[handle];
        let target = if last {
            panel.maximum
        } else {
            panel.smallest()
        };
        let mut layout = None;
        // Approach the target from its side so a partially reachable edge
        // still moves as far as the remaining panels allow.
        let mut delta = target - self.sizes[handle];
        for _ in 0..32 {
            layout = adjust(&self.effective, &self.sizes, handle, delta);
            if layout.is_some() || delta.abs() < EPSILON {
                break;
            }
            delta /= 2.;
        }
        if let Some(layout) = layout {
            self.apply(layout, window, cx);
        }
    }

    /// Enter collapses the collapsible panel before the divider, or restores it.
    fn toggle(&mut self, handle: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.panels[handle].collapsed.is_none() {
            return;
        }
        if self.is_collapsed(handle) {
            self.expand(handle, window, cx);
        } else {
            self.collapse(handle, window, cx);
        }
    }

    fn pointer_coordinate(&self, point: Point<Pixels>) -> Pixels {
        match self.axis {
            Axis::Horizontal => point.x,
            Axis::Vertical => point.y,
        }
    }

    fn length(&self, bounds: Bounds<Pixels>) -> Pixels {
        match self.axis {
            Axis::Horizontal => bounds.size.width,
            Axis::Vertical => bounds.size.height,
        }
    }

    fn begin_drag(&mut self, handle: usize) {
        let usable = self
            .panel_bounds
            .iter()
            .fold(px(0.), |total, bounds| total + self.length(*bounds));
        self.drag = Some(Drag {
            handle,
            origin: self.sizes.clone(),
            start: self.pointer_coordinate(self.handle_bounds[handle].origin),
            usable,
        });
    }

    fn resize_at_pointer(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let Some(drag) = &self.drag else {
            return;
        };
        if drag.usable <= px(0.) {
            return;
        }
        let moved = self.pointer_coordinate(position) - self.grab - drag.start;
        if let Some(layout) = adjust(
            &self.effective,
            &drag.origin,
            drag.handle,
            moved / drag.usable,
        ) {
            self.apply(layout, window, cx);
        }
    }
}

impl EventEmitter<ResizableEvent> for ResizableState {}

fn valid_layout(panels: &[ResizablePanel], sizes: &[f32]) -> bool {
    let total: f32 = sizes.iter().sum();
    sizes.iter().all(|size| size.is_finite())
        && (total - 1.).abs() < EPSILON * sizes.len() as f32
        && panels
            .iter()
            .zip(sizes)
            .all(|(panel, size)| panel.accepts(*size))
}

/// The layout nearest `sizes` that satisfies `panels`: out-of-range panels are
/// clamped and the difference is shared by panels with room, leaving collapsed
/// panels collapsed. Returns `None` when no such layout exists.
fn fit(panels: &[ResizablePanel], sizes: &[f32]) -> Option<Vec<f32>> {
    let mut fitted: Vec<f32> = panels
        .iter()
        .zip(sizes)
        .map(|(panel, &size)| {
            if panel.is_collapsed_size(size) {
                size
            } else {
                size.clamp(panel.minimum, panel.maximum)
            }
        })
        .collect();
    for _ in 0..=panels.len() {
        let difference = 1. - fitted.iter().sum::<f32>();
        if difference.abs() < EPSILON {
            break;
        }
        let room: Vec<f32> = panels
            .iter()
            .zip(&fitted)
            .map(|(panel, &size)| {
                if panel.is_collapsed_size(size) {
                    0.
                } else if difference > 0. {
                    panel.maximum - size
                } else {
                    size - panel.minimum
                }
            })
            .collect();
        let open = room.iter().filter(|room| **room > EPSILON).count();
        if open == 0 {
            break;
        }
        let share = difference.abs() / open as f32;
        for (size, room) in fitted.iter_mut().zip(&room) {
            if *room > EPSILON {
                *size += share.min(*room).copysign(difference);
            }
        }
    }
    valid_layout(panels, &fitted).then_some(fitted)
}

/// Moves divider `handle` by `delta` from `base`. A positive delta grows the
/// panel before the divider and shrinks the following panels nearest first; a
/// negative delta does the opposite. Returns `None` when no valid layout moves
/// the divider in that direction.
fn adjust(panels: &[ResizablePanel], base: &[f32], handle: usize, delta: f32) -> Option<Vec<f32>> {
    if delta.abs() < EPSILON {
        return None;
    }
    let grow = if delta > 0. { handle } else { handle + 1 };
    let shrink: Vec<usize> = if delta > 0. {
        (handle + 1..panels.len()).collect()
    } else {
        (0..=handle).rev().collect()
    };
    let wanted = panels[grow].constrain(base[grow] + delta.abs()) - base[grow];
    if wanted < EPSILON {
        return None;
    }
    let mut sizes = base.to_vec();
    let mut remaining = wanted;
    for index in shrink {
        if remaining < EPSILON {
            break;
        }
        let next = panels[index].constrain(sizes[index] - remaining);
        let freed = (sizes[index] - next).max(0.);
        sizes[index] -= freed;
        remaining -= freed;
    }
    let freed = wanted - remaining;
    if freed < EPSILON {
        return None;
    }
    sizes[grow] = base[grow] + freed;
    valid_layout(panels, &sizes).then_some(sizes)
}

#[derive(Clone)]
struct DragResizable(EntityId);

impl Render for DragResizable {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// An unstyled panel group. Give its frame bounded dimensions, supply one
/// [`Self::panel`] per configured panel in order, and size each divider through
/// [`Self::handle`]. Horizontal groups use the handle's width; vertical groups
/// use its height.
///
/// Ordinary styles apply to the frame, including padding. The inner structure
/// owns panel sizes and has no gap; spacing belongs inside panel content.
/// Panels clip overflowing content, so compose ScrollArea when it should
/// scroll. A panel collapsed to zero size unmounts its content.
#[derive(IntoElement)]
pub struct Resizable {
    id: ElementId,
    base: crate::ObservedElement<gpui::Stateful<gpui::Div>>,
    state: Entity<ResizableState>,
    panels: Vec<AnyElement>,
    handle: Option<Box<dyn Fn(usize, ResizableHandle) -> ResizableHandle>>,
}

impl Resizable {
    pub fn new(id: impl Into<ElementId>, state: &Entity<ResizableState>) -> Self {
        let id = id.into();
        Self {
            base: div().id(id.clone()).test_support(),
            state: state.clone(),
            panels: Vec::new(),
            handle: None,
            id,
        }
    }

    /// Appends the next panel's content.
    pub fn panel(mut self, content: impl IntoElement) -> Self {
        self.panels.push(content.into_any_element());
        self
    }

    /// Builds each divider from its index, which is the preceding panel's index.
    pub fn handle(
        mut self,
        build: impl Fn(usize, ResizableHandle) -> ResizableHandle + 'static,
    ) -> Self {
        self.handle = Some(Box::new(build));
        self
    }
}

impl Styled for Resizable {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Resizable {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let model = self.state.read(cx);
        let axis = model.axis;
        let sizes = model.sizes.clone();
        let panel_focus = model.panel_focus.clone();
        let root_focus = model.root_focus.clone();
        assert_eq!(
            self.panels.len(),
            sizes.len(),
            "Resizable needs one panel element per configured panel"
        );
        let mut layout = div()
            .id((self.id.clone(), "layout"))
            .flex()
            .flex_1()
            .w_full()
            .min_w_0()
            .min_h_0()
            .gap_0()
            .when(axis == Axis::Vertical, |this| this.flex_col());
        for (index, content) in self.panels.into_iter().enumerate() {
            if index > 0 {
                let handle = ResizableHandle::new(
                    (
                        self.id.clone(),
                        SharedString::from(format!("handle-{}", index - 1)),
                    ),
                    &self.state,
                    index - 1,
                );
                layout = layout.child(match &self.handle {
                    Some(build) => build(index - 1, handle),
                    None => handle,
                });
            }
            let size = sizes[index];
            let geometry = self.state.clone();
            layout = layout.child(
                div()
                    .id((
                        self.id.clone(),
                        SharedString::from(format!("panel-{index}")),
                    ))
                    .test_support()
                    .track_focus(&panel_focus[index].clone().tab_stop(false))
                    .min_w_0()
                    .min_h_0()
                    .flex_basis(px(0.))
                    .flex_grow(size)
                    .flex_shrink_0()
                    .overflow_hidden()
                    .when(size > 0., |this| this.child(content))
                    .on_prepaint(move |bounds, window, cx| {
                        geometry.update(cx, |state, cx| {
                            state.panel_bounds[index] = bounds;
                            if index + 1 == state.panels.len() {
                                // Panels share the usable space exactly.
                                let usable = state
                                    .panel_bounds
                                    .iter()
                                    .fold(px(0.), |total, bounds| total + state.length(*bounds));
                                state.measure(usable, window, cx);
                            }
                        });
                    }),
            );
        }
        self.base
            .track_focus(&root_focus.tab_stop(false))
            .flex()
            .flex_col()
            .child(layout)
    }
}

/// One focusable divider. Supply a non-interactive visual grip and an
/// accessible label describing the panel before it. Arrow keys along the
/// group's layout axis adjust that panel by two percentage points; Home and End
/// reach its smallest and largest allowed sizes; Enter collapses or restores a
/// collapsible panel. Other-axis keys propagate normally.
///
/// Native orientation describes the divider line, perpendicular to the layout
/// axis. Numeric accessibility values use percentages of the usable space for
/// the panel before the divider. Main-axis margins are reserved and reset.
#[derive(IntoElement)]
pub struct ResizableHandle {
    base: crate::ObservedElement<gpui::Stateful<gpui::Div>>,
    state: Entity<ResizableState>,
    index: usize,
    label: Option<SharedString>,
}

impl ResizableHandle {
    fn new(id: impl Into<ElementId>, state: &Entity<ResizableState>, index: usize) -> Self {
        Self {
            base: div().id(id).test_support(),
            state: state.clone(),
            index,
            label: None,
        }
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Styled for ResizableHandle {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for ResizableHandle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl InteractiveElement for ResizableHandle {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for ResizableHandle {}

impl RenderOnce for ResizableHandle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let index = self.index;
        let model = self.state.read(cx);
        let axis = model.axis;
        let disabled = model.disabled;
        let focus = model.handle_focus[index].clone();
        let panel = model.effective[index];
        let size = model.sizes[index];
        let entity_id = self.state.entity_id();
        let geometry = self.state.clone();
        let drag_start = self.state.clone();
        let increment = self.state.clone();
        let decrement = self.state.clone();
        let set_value = self.state.clone();

        self.base
            .role(Role::Splitter)
            .flex_shrink_0()
            .aria_numeric_value((size * 100.) as f64)
            .aria_min_numeric_value((panel.smallest() * 100.) as f64)
            .aria_max_numeric_value((panel.maximum * 100.) as f64)
            .aria_numeric_value_step((STEP * 100.) as f64)
            .when_some(self.label, |this, label| this.aria_label(label))
            .when(axis == Axis::Horizontal, |this| {
                this.mx_0()
                    .cursor_col_resize()
                    .aria_orientation(Orientation::Vertical)
                    .key_context("ResizableHandle axis=horizontal")
                    .on_action(window.listener_for(
                        &self.state,
                        move |state, _: &SelectLeft, window, cx| {
                            state.nudge(index, false, window, cx)
                        },
                    ))
                    .on_action(window.listener_for(
                        &self.state,
                        move |state, _: &SelectRight, window, cx| {
                            state.nudge(index, true, window, cx)
                        },
                    ))
            })
            .when(axis == Axis::Vertical, |this| {
                this.my_0()
                    .cursor_row_resize()
                    .aria_orientation(Orientation::Horizontal)
                    .key_context("ResizableHandle axis=vertical")
                    .on_action(window.listener_for(
                        &self.state,
                        move |state, _: &SelectUp, window, cx| {
                            state.nudge(index, false, window, cx)
                        },
                    ))
                    .on_action(window.listener_for(
                        &self.state,
                        move |state, _: &SelectDown, window, cx| {
                            state.nudge(index, true, window, cx)
                        },
                    ))
            })
            .when(!disabled || focus.is_focused(window), |this| {
                this.track_focus(&focus.tab_stop(!disabled))
            })
            .on_action(window.listener_for(
                &self.state,
                move |state, _: &SelectFirst, window, cx| {
                    state.edge(index, false, window, cx);
                },
            ))
            .on_action(window.listener_for(
                &self.state,
                move |state, _: &SelectLast, window, cx| {
                    state.edge(index, true, window, cx);
                },
            ))
            .on_action(
                window.listener_for(&self.state, move |state, _: &Confirm, window, cx| {
                    state.toggle(index, window, cx);
                }),
            )
            .when(disabled, |this| {
                this.cursor_default()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            })
            .when(!disabled, |this| {
                this.on_mouse_down(
                    MouseButton::Left,
                    window.listener_for(
                        &self.state,
                        move |state, event: &MouseDownEvent, window, cx| {
                            if state.disabled {
                                return;
                            }
                            state.grab = state.pointer_coordinate(event.position)
                                - state.pointer_coordinate(state.handle_bounds[index].origin);
                            state.handle_focus[index].focus(window, cx);
                        },
                    ),
                )
                .on_drag(DragResizable(entity_id), move |drag, _, window, cx| {
                    let position = window.mouse_position();
                    drag_start.update(cx, |state, cx| {
                        state.begin_drag(index);
                        state.resize_at_pointer(position, window, cx)
                    });
                    cx.stop_propagation();
                    cx.new(|_| drag.clone())
                })
                .on_drag_move(window.listener_for(
                    &self.state,
                    move |state, event: &DragMoveEvent<DragResizable>, window, cx| {
                        if event.drag(cx).0 == entity_id
                            && state.drag.as_ref().is_some_and(|drag| drag.handle == index)
                        {
                            state.resize_at_pointer(event.event.position, window, cx);
                        }
                    },
                ))
                .on_a11y_action(AccessibleAction::Increment, move |_, window, cx| {
                    increment.update(cx, |state, cx| state.nudge(index, true, window, cx));
                })
                .on_a11y_action(AccessibleAction::Decrement, move |_, window, cx| {
                    decrement.update(cx, |state, cx| state.nudge(index, false, window, cx));
                })
                .on_a11y_action(
                    AccessibleAction::SetValue,
                    move |data, window, cx| {
                        if let Some(accesskit::ActionData::NumericValue(value)) = data
                            && value.is_finite()
                        {
                            set_value.update(cx, |state, cx| {
                                if !state.disabled {
                                    let target =
                                        state.effective[index].constrain(*value as f32 / 100.);
                                    state.resize_panel(index, target, window, cx);
                                }
                            });
                        }
                    },
                )
            })
            .on_prepaint(move |bounds, _, cx| {
                geometry.update(cx, |state, _| state.handle_bounds[index] = bounds);
            })
            .a11y_disabled(disabled)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gpui::{Element as _, Modifiers, Subscription, TestAppContext, VisualTestContext, point};

    use super::*;
    use crate::{Button, Root};

    fn panels(sizes: &[f32]) -> Vec<ResizablePanel> {
        sizes
            .iter()
            .map(|size| ResizablePanel::new(*size).limits(0.2..=0.8))
            .collect()
    }

    fn close(actual: &[f32], expected: &[f32]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(a, b)| (a - b).abs() < 0.0001)
    }

    #[test]
    fn divider_moves_cascade_through_constraints_and_collapse_snaps() {
        let three = [
            ResizablePanel::new(0.4).limits(0.2..=0.6),
            ResizablePanel::new(0.3).limits(0.2..=0.6),
            ResizablePanel::new(0.3).limits(0.2..=0.6),
        ];
        let base = [0.4, 0.3, 0.3];
        // The nearest panel shrinks to its minimum, then the next one gives way.
        let moved = adjust(&three, &base, 0, 0.15).unwrap();
        assert!(close(&moved, &[0.55, 0.2, 0.25]), "{moved:?}");
        // The growing panel's maximum bounds the move.
        let moved = adjust(&three, &base, 0, 0.5).unwrap();
        assert!(close(&moved, &[0.6, 0.2, 0.2]), "{moved:?}");
        // No room: the divider does not move.
        assert_eq!(adjust(&three, &[0.6, 0.2, 0.2], 0, 0.1), None);

        let sidebar = [
            ResizablePanel::new(0.25).limits(0.15..=0.4).collapsible(0.),
            ResizablePanel::new(0.75).limits(0.3..=1.),
        ];
        // Less than halfway toward the collapsed size stops at the minimum.
        let moved = adjust(&sidebar, &[0.25, 0.75], 0, -0.15).unwrap();
        assert!(close(&moved, &[0.15, 0.85]), "{moved:?}");
        // Past halfway snaps closed.
        let moved = adjust(&sidebar, &[0.25, 0.75], 0, -0.2).unwrap();
        assert!(close(&moved, &[0., 1.]), "{moved:?}");
        // Reopening needs to pass halfway to the minimum.
        assert_eq!(adjust(&sidebar, &[0., 1.], 0, 0.05), None);
        let moved = adjust(&sidebar, &[0., 1.], 0, 0.1).unwrap();
        assert!(close(&moved, &[0.15, 0.85]), "{moved:?}");
    }

    struct Harness {
        state: Entity<ResizableState>,
        before: FocusHandle,
        actions: Vec<FocusHandle>,
        after: FocusHandle,
        extent: Pixels,
        _subscription: Subscription,
    }

    impl Render for Harness {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let axis = self.state.read(cx).axis();
            let mut group = Resizable::new("split", &self.state)
                .w(if axis == Axis::Horizontal {
                    self.extent
                } else {
                    px(120.)
                })
                .h(if axis == Axis::Vertical {
                    self.extent
                } else {
                    px(120.)
                })
                .p(px(10.))
                .flex_shrink_0()
                .handle(move |index, handle| {
                    handle
                        .accessibility_label(format!("Panel {} size", index + 1))
                        .when(axis == Axis::Horizontal, |handle| handle.w(px(10.)))
                        .when(axis == Axis::Vertical, |handle| handle.h(px(10.)))
                });
            for (index, focus) in self.actions.iter().enumerate() {
                group = group.panel(
                    Button::new(("action", index))
                        .track_focus(focus)
                        .h(px(30.))
                        .child(format!("Panel {index}")),
                );
            }
            div()
                .size(px(600.))
                .flex()
                .flex_col()
                .items_start()
                .child(
                    Button::new("before")
                        .track_focus(&self.before)
                        .h(px(30.))
                        .child("Before"),
                )
                .child(group)
                .child(
                    Button::new("after")
                        .track_focus(&self.after)
                        .h(px(30.))
                        .child("After"),
                )
        }
    }

    type Changes = Rc<RefCell<Vec<Vec<f32>>>>;

    fn harness(
        cx: &mut TestAppContext,
        axis: Axis,
        panels: Vec<ResizablePanel>,
    ) -> (&mut VisualTestContext, Entity<Harness>, Changes) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let view = cx.new(|cx| {
                let count = panels.len();
                let state = cx.new(|cx| ResizableState::new(axis, panels, cx));
                let subscription = cx.observe(&state, |_, _, cx| cx.notify());
                Harness {
                    state,
                    before: cx.focus_handle(),
                    actions: (0..count).map(|_| cx.focus_handle()).collect(),
                    after: cx.focus_handle(),
                    extent: px(320.),
                    _subscription: subscription,
                }
            });
            Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap();
        let changes = Rc::new(RefCell::new(Vec::new()));
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            let changes = changes.clone();
            cx.subscribe(&state, move |_, event: &ResizableEvent, _| {
                let ResizableEvent::Change(sizes) = event;
                changes.borrow_mut().push(sizes.clone());
            })
            .detach();
            window.activate_window();
            view.read(cx).before.clone().focus(window, cx);
            window.draw(cx).clear(cx);
        });
        cx.run_until_parked();
        (cx, view, changes)
    }

    fn draw(cx: &mut VisualTestContext) {
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn assert_sizes(cx: &mut VisualTestContext, view: &Entity<Harness>, expected: &[f32]) {
        cx.update(|_, cx| {
            let sizes = view.read(cx).state.read(cx).sizes().to_vec();
            assert!(close(&sizes, expected), "{sizes:?} != {expected:?}");
        });
    }

    #[gpui::test]
    fn divider_keyboard_and_native_semantics_preserve_child_focus(cx: &mut TestAppContext) {
        for axis in [Axis::Horizontal, Axis::Vertical] {
            let (cx, view, changes) = harness(cx, axis, panels(&[0.5, 0.5]));
            let increase = if axis == Axis::Horizontal {
                "right"
            } else {
                "down"
            };
            let decrease = if axis == Axis::Horizontal {
                "left"
            } else {
                "up"
            };
            let other_axis = if axis == Axis::Horizontal {
                "down"
            } else {
                "right"
            };
            cx.simulate_keystrokes("tab");
            cx.simulate_keystrokes(increase);
            draw(cx);
            cx.update(|window, cx| assert!(view.read(cx).actions[0].is_focused(window)));
            assert_sizes(cx, &view, &[0.5, 0.5]);

            cx.simulate_keystrokes("tab");
            cx.simulate_keystrokes(increase);
            draw(cx);
            assert_sizes(cx, &view, &[0.52, 0.48]);
            cx.simulate_keystrokes(other_axis);
            draw(cx);
            assert_sizes(cx, &view, &[0.52, 0.48]);
            // Application bindings can dispatch the same shared actions.
            if axis == Axis::Horizontal {
                cx.dispatch_action(SelectDown);
            } else {
                cx.dispatch_action(SelectRight);
            }
            draw(cx);
            assert_sizes(cx, &view, &[0.52, 0.48]);
            cx.simulate_keystrokes(decrease);
            cx.simulate_keystrokes("home end end enter");
            draw(cx);
            assert_sizes(cx, &view, &[0.8, 0.2]);
            assert_eq!(changes.borrow().len(), 4);

            cx.update(|window, cx| {
                let state = view.read(cx).state.clone();
                assert!(state.read(cx).handle_focus(0).is_focused(window));
                let element = ResizableHandle::new("probe", &state, 0)
                    .accessibility_label("First pane size")
                    .render(window, cx)
                    .into_element();
                let mut node = accesskit::Node::new(Role::Splitter);
                element.write_a11y_info(&mut node);
                assert_eq!(element.a11y_role(), Some(Role::Splitter));
                assert_eq!(node.label(), Some("First pane size"));
                assert_eq!(node.numeric_value(), Some(80.));
                assert_eq!(node.min_numeric_value(), Some(20.));
                assert_eq!(node.max_numeric_value(), Some(80.));
                assert_eq!(node.numeric_value_step(), Some(2.));
                assert_eq!(
                    node.orientation(),
                    Some(if axis == Axis::Horizontal {
                        Orientation::Vertical
                    } else {
                        Orientation::Horizontal
                    })
                );
                assert!(node.supports_action(AccessibleAction::SetValue));
                state.update(cx, |state, cx| state.set_disabled(true, window, cx));
                let element = ResizableHandle::new("disabled-probe", &state, 0)
                    .render(window, cx)
                    .into_element();
                let mut node = accesskit::Node::new(Role::Splitter);
                element.write_a11y_info(&mut node);
                assert!(node.is_disabled());
                assert!(!node.supports_action(AccessibleAction::SetValue));
            });
            draw(cx);
            cx.simulate_keystrokes("home");
            cx.simulate_keystrokes(decrease);
            draw(cx);
            assert_sizes(cx, &view, &[0.8, 0.2]);
            cx.simulate_keystrokes("tab");
            draw(cx);
            cx.update(|window, cx| assert!(view.read(cx).actions[1].is_focused(window)));
            cx.simulate_keystrokes("shift-tab");
            draw(cx);
            cx.update(|window, cx| assert!(view.read(cx).actions[0].is_focused(window)));
            assert_eq!(changes.borrow().len(), 4);
        }
    }

    #[gpui::test]
    fn collapsible_panel_toggles_unmounts_content_and_restores_size(cx: &mut TestAppContext) {
        let (cx, view, changes) = harness(
            cx,
            Axis::Horizontal,
            vec![
                ResizablePanel::new(0.3).limits(0.2..=0.5).collapsible(0.),
                ResizablePanel::new(0.4).limits(0.2..=0.8),
                ResizablePanel::new(0.3).limits(0.2..=0.5),
            ],
        );
        // Focus the first panel's content, then collapse it programmatically.
        cx.simulate_keystrokes("tab");
        draw(cx);
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            assert!(view.read(cx).actions[0].is_focused(window));
            state.update(cx, |state, cx| state.collapse(0, window, cx));
        });
        draw(cx);
        assert_sizes(cx, &view, &[0., 0.7, 0.3]);
        cx.update(|window, cx| {
            let view = view.read(cx);
            let state = view.state.read(cx);
            assert!(state.is_collapsed(0));
            assert!(state.handle_focus(0).is_focused(window));
            let content = (ElementId::from("split"), "panel-0").into();
            assert!(
                crate::test_support::find(window, &[], &("action", 0usize).into()).is_none(),
                "collapsed content is unmounted"
            );
            assert!(crate::test_support::find(window, &[], &content).is_some());
        });
        // A keyboard step from collapsed reopens to the minimum; another closes.
        cx.simulate_keystrokes("right");
        draw(cx);
        assert_sizes(cx, &view, &[0.2, 0.5, 0.3]);
        cx.simulate_keystrokes("left");
        draw(cx);
        assert_sizes(cx, &view, &[0., 0.7, 0.3]);
        // Enter restores the last expanded size.
        cx.simulate_keystrokes("enter");
        draw(cx);
        assert_sizes(cx, &view, &[0.2, 0.5, 0.3]);
        cx.simulate_keystrokes("right right enter enter");
        draw(cx);
        assert_sizes(cx, &view, &[0.24, 0.46, 0.3]);
        // The second divider moves the middle panel against the last one.
        cx.simulate_keystrokes("tab tab right");
        draw(cx);
        assert_sizes(cx, &view, &[0.24, 0.48, 0.28]);
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            assert!(state.read(cx).handle_focus(1).is_focused(window));
            state.update(cx, |state, cx| {
                state.set_sizes(vec![0.3, 0.4, 0.3], window, cx)
            });
        });
        assert_sizes(cx, &view, &[0.3, 0.4, 0.3]);
        assert_eq!(changes.borrow().len(), 10);
    }

    #[gpui::test]
    fn pointer_resize_uses_actual_pane_space_and_retains_sizes_on_parent_resize(
        cx: &mut TestAppContext,
    ) {
        for axis in [Axis::Horizontal, Axis::Vertical] {
            let (cx, view, _) = harness(cx, axis, panels(&[0.5, 0.5]));
            let (handle, origin, usable) = cx.update(|_, cx| {
                let state = view.read(cx).state.read(cx);
                let usable =
                    state.length(state.panel_bounds[0]) + state.length(state.panel_bounds[1]);
                assert_eq!(
                    usable,
                    px(290.),
                    "subtract frame padding and actual divider thickness"
                );
                assert_eq!(state.length(state.panel_bounds[0]), px(145.));
                (state.handle_bounds[0], state.panel_bounds[0].origin, usable)
            });
            let grab = px(2.);
            let start = match axis {
                Axis::Horizontal => point(handle.left() + grab, handle.center().y),
                Axis::Vertical => point(handle.center().x, handle.top() + grab),
            };
            let at = |ratio: f32| match axis {
                Axis::Horizontal => point(origin.x + usable * ratio + grab, start.y),
                Axis::Vertical => point(start.x, origin.y + usable * ratio + grab),
            };
            cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
            draw(cx);
            assert_sizes(cx, &view, &[0.5, 0.5]);
            cx.simulate_mouse_move(at(0.7), MouseButton::Left, Modifiers::default());
            draw(cx);
            assert_sizes(cx, &view, &[0.7, 0.3]);
            cx.simulate_mouse_move(at(1.2), MouseButton::Left, Modifiers::default());
            draw(cx);
            assert_sizes(cx, &view, &[0.8, 0.2]);
            cx.simulate_mouse_up(at(1.2), MouseButton::Left, Modifiers::default());
            draw(cx);
            cx.simulate_mouse_move(at(0.3), None, Modifiers::default());
            draw(cx);
            assert_sizes(cx, &view, &[0.8, 0.2]);
            cx.update(|_, cx| assert!(!cx.has_active_drag()));

            view.update(cx, |view, cx| {
                view.extent = px(420.);
                cx.notify();
            });
            draw(cx);
            cx.update(|_, cx| {
                let state = view.read(cx).state.read(cx);
                let first = state.length(state.panel_bounds[0]);
                let second = state.length(state.panel_bounds[1]);
                assert_eq!(first + second, px(390.));
                assert!((first / (first + second) - 0.8).abs() < 0.001);
            });
        }
    }

    #[gpui::test]
    fn pixel_limits_follow_the_measured_group_and_refit_on_resize(cx: &mut TestAppContext) {
        let (cx, view, changes) = harness(
            cx,
            Axis::Horizontal,
            vec![
                ResizablePanel::new(0.5).pixel_limits(px(116.)..=px(1000.)),
                ResizablePanel::new(0.5).limits(0.2..=1.),
            ],
        );
        // 320 px frame, 10 px padding each side and a 10 px divider: 290 usable.
        assert_sizes(cx, &view, &[0.5, 0.5]);
        assert!(
            changes.borrow().is_empty(),
            "a satisfied layout is not refitted"
        );
        view.update(cx, |view, cx| {
            view.extent = px(220.);
            cx.notify();
        });
        draw(cx);
        draw(cx);
        // 190 usable: the 116 px minimum is now a larger share.
        assert_sizes(cx, &view, &[116. / 190., 74. / 190.]);
        cx.update(|_, cx| {
            let state = view.read(cx).state.read(cx);
            assert!((state.length(state.panel_bounds[0]) - px(116.)).abs() < px(0.5));
        });
        assert_eq!(changes.borrow().len(), 1);
        cx.update(|window, cx| {
            view.read(cx)
                .state
                .read(cx)
                .handle_focus(0)
                .clone()
                .focus(window, cx);
        });
        draw(cx);
        cx.simulate_keystrokes("left");
        draw(cx);
        assert_sizes(cx, &view, &[116. / 190., 74. / 190.]);
        cx.simulate_keystrokes("right");
        draw(cx);
        assert_sizes(cx, &view, &[116. / 190. + STEP, 74. / 190. - STEP]);
        // A wider group keeps the layout instead of jumping back.
        view.update(cx, |view, cx| {
            view.extent = px(320.);
            cx.notify();
        });
        draw(cx);
        draw(cx);
        assert_sizes(cx, &view, &[116. / 190. + STEP, 74. / 190. - STEP]);
        // A stored layout below the pixel minimum is fitted on restore.
        cx.update(|window, cx| {
            view.read(cx).state.clone().update(cx, |state, cx| {
                state.set_sizes(vec![0.3, 0.7], window, cx);
            })
        });
        assert_sizes(cx, &view, &[116. / 290., 174. / 290.]);
    }
}
