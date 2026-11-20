//! Headless categorical chart data, visibility, and point inspection.
//!
//! Plot paths, axes, legend, tooltip, and table presentation belong to the
//! application. See the copyable native chart recipe in the catalogue example.

use crate::{
    Direction, ElementExt as _, TestSupportExt as _,
    actions::{SelectFirst, SelectLast, SelectLeft, SelectRight},
};
use gpui::{
    AnyElement, App, Bounds, Context, ElementId, Entity, EventEmitter, FocusHandle,
    InteractiveElement, IntoElement, KeyBinding, MouseButton, ParentElement, Pixels, Point,
    RenderOnce, Role, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    div,
};

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("left", SelectLeft, Some("Chart")),
        KeyBinding::new("right", SelectRight, Some("Chart")),
        KeyBinding::new("home", SelectFirst, Some("Chart")),
        KeyBinding::new("end", SelectLast, Some("Chart")),
    ]);
}

/// A named numeric series. Every series must have one finite value per category.
#[derive(Clone, Debug)]
pub struct ChartSeries {
    label: SharedString,
    values: Vec<f64>,
}
impl ChartSeries {
    pub fn new(label: impl Into<SharedString>, values: impl IntoIterator<Item = f64>) -> Self {
        let values: Vec<_> = values.into_iter().collect();
        assert!(
            values.iter().all(|value| value.is_finite()),
            "Chart values must be finite"
        );
        Self {
            label: label.into(),
            values,
        }
    }
    pub fn label(&self) -> &SharedString {
        &self.label
    }
    pub fn values(&self) -> &[f64] {
        &self.values
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChartEvent {
    Inspect(usize),
    Visibility(usize, bool),
}

/// How categories and visible series share the plot.
///
/// The layout determines the numeric range, pointer hit testing and the
/// geometry helpers. Radial angles are measured in turns clockwise from the
/// top; RTL mirrors them counterclockwise.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ChartLayout {
    /// Side-by-side series over evenly spaced category bands.
    #[default]
    Grouped,
    /// Category bands where positive and negative values stack separately.
    Stacked,
    /// One slice per category, sized by its nonnegative visible-series total.
    Pie,
    /// One evenly spaced spoke per category, radius normalized like Grouped.
    Radar,
}

/// A fixed categorical dataset and its interactive inspection/legend state.
/// The numeric range includes zero and only visible series. The empty or all-zero
/// dataset uses `0..1`; negative values are supported without a false baseline.
/// A Stacked layout's range covers each category's positive and negative sums.
pub struct ChartState {
    categories: Vec<SharedString>,
    series: Vec<ChartSeries>,
    visible: Vec<bool>,
    layout: ChartLayout,
    range: (f64, f64),
    inspected: Option<usize>,
    focus: FocusHandle,
    bounds: Bounds<Pixels>,
}
impl ChartState {
    pub fn new<L: Into<SharedString>>(
        categories: impl IntoIterator<Item = L>,
        series: impl IntoIterator<Item = ChartSeries>,
        cx: &mut Context<Self>,
    ) -> Self {
        let categories: Vec<_> = categories.into_iter().map(Into::into).collect();
        let series: Vec<_> = series.into_iter().collect();
        assert!(
            series
                .iter()
                .all(|series| series.values.len() == categories.len()),
            "Each chart series needs one value per category"
        );
        let visible = vec![true; series.len()];
        let range = Self::data_range(&series, &visible, ChartLayout::Grouped);
        Self {
            inspected: (!categories.is_empty()).then_some(0),
            visible,
            layout: ChartLayout::Grouped,
            range,
            categories,
            series,
            focus: cx.focus_handle(),
            bounds: Bounds::default(),
        }
    }
    pub fn categories(&self) -> &[SharedString] {
        &self.categories
    }
    pub fn series(&self) -> &[ChartSeries] {
        &self.series
    }
    pub fn is_visible(&self, index: usize) -> bool {
        self.visible[index]
    }
    pub fn inspected_index(&self) -> Option<usize> {
        self.inspected
    }
    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }

    pub fn set_visible(
        &mut self,
        index: usize,
        visible: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.visible[index] == visible {
            return false;
        }
        self.visible[index] = visible;
        self.range = Self::data_range(&self.series, &self.visible, self.layout);
        cx.emit(ChartEvent::Visibility(index, visible));
        cx.notify();
        window.refresh();
        true
    }
    pub fn layout(&self) -> ChartLayout {
        self.layout
    }
    /// Arranges the same data differently; visibility and inspection are kept.
    pub fn set_layout(
        &mut self,
        layout: ChartLayout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.layout == layout {
            return false;
        }
        self.layout = layout;
        self.range = Self::data_range(&self.series, &self.visible, layout);
        cx.notify();
        window.refresh();
        true
    }
    pub fn inspect(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if index >= self.categories.len() || self.inspected == Some(index) {
            return false;
        }
        self.inspected = Some(index);
        cx.emit(ChartEvent::Inspect(index));
        cx.notify();
        window.refresh();
        true
    }
    /// The visible domain is recomputed only when series visibility changes,
    /// so normalizing every plotted point remains linear in the dataset size.
    pub fn range(&self) -> (f64, f64) {
        self.range
    }

    fn data_range(series: &[ChartSeries], visible: &[bool], layout: ChartLayout) -> (f64, f64) {
        let visible = || {
            series
                .iter()
                .zip(visible)
                .filter(|(_, visible)| **visible)
                .map(|(series, _)| series)
        };
        let (min, max) = if layout == ChartLayout::Stacked {
            let categories = series.first().map_or(0, |series| series.values.len());
            (0..categories).fold((0_f64, 0_f64), |(min, max), category| {
                let (negative, positive) =
                    visible().fold((0_f64, 0_f64), |(negative, positive), series| {
                        let value = series.values[category];
                        (negative + value.min(0.), positive + value.max(0.))
                    });
                (min.min(negative), max.max(positive))
            })
        } else {
            visible()
                .flat_map(|series| series.values.iter())
                .fold((0_f64, 0_f64), |(min, max), &value| {
                    (min.min(value), max.max(value))
                })
        };
        if min == max { (0., 1.) } else { (min, max) }
    }
    /// The `(start, end)` values of a visible series segment in a Stacked
    /// layout. Earlier visible series of the same sign lie closer to zero.
    /// Returns `None` for a hidden series.
    pub fn stack_span(&self, series: usize, category: usize) -> Option<(f64, f64)> {
        if !self.visible[series] {
            return None;
        }
        let value = self.series[series].values[category];
        let start = self.series[..series]
            .iter()
            .zip(&self.visible)
            .filter(|(_, visible)| **visible)
            .map(|(series, _)| series.values[category])
            .filter(|earlier| (*earlier < 0.) == (value < 0.))
            .sum::<f64>();
        Some((start, start + value))
    }
    /// Each category's `(start, end)` pie slice in turns. A category whose
    /// visible-series total is zero or negative receives an empty slice; with
    /// no positive total every slice is empty.
    pub fn pie_slices(&self) -> Vec<(f32, f32)> {
        let totals: Vec<f64> = (0..self.categories.len())
            .map(|category| {
                self.series
                    .iter()
                    .zip(&self.visible)
                    .filter(|(_, visible)| **visible)
                    .map(|(series, _)| series.values[category])
                    .sum::<f64>()
                    .max(0.)
            })
            .collect();
        let sum: f64 = totals.iter().sum();
        let mut start = 0_f64;
        totals
            .iter()
            .map(|total| {
                let end = if sum > 0. { start + total / sum } else { 0. };
                let slice = (start as f32, end as f32);
                start = end;
                slice
            })
            .collect()
    }
    /// The spoke angle of a Radar category in turns.
    pub fn radar_turn(&self, category: usize) -> f32 {
        category as f32 / self.categories.len().max(1) as f32
    }
    /// A unit height above the visible data range's lower bound.
    pub fn normalized_value(&self, value: f64) -> f32 {
        let (min, max) = self.range();
        let scale = min.abs().max(max.abs());
        ((value / scale - min / scale) / (max / scale - min / scale)).clamp(0., 1.) as f32
    }
    pub fn inspected_description(&self) -> SharedString {
        let Some(index) = self.inspected else {
            return "No chart data".into();
        };
        let values = self
            .series
            .iter()
            .zip(&self.visible)
            .filter(|(_, visible)| **visible)
            .map(|(series, _)| format!("{}: {}", series.label, series.values[index]))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{}: {}",
            self.categories[index],
            if values.is_empty() {
                "No visible series"
            } else {
                &values
            }
        )
        .into()
    }
    fn step(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus.is_focused(window) {
            cx.propagate();
            return;
        }
        if let Some(index) = self.inspected {
            let next = if forward {
                (index + 1).min(self.categories.len() - 1)
            } else {
                index.saturating_sub(1)
            };
            self.inspect(next, window, cx);
        }
    }

    fn inspect_at_pointer(
        &mut self,
        position: Point<Pixels>,
        direction: Direction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.categories.len();
        if count == 0 || self.bounds.size.width <= gpui::px(0.) {
            return;
        }
        let index = match self.layout {
            ChartLayout::Grouped | ChartLayout::Stacked => {
                let fraction = direction.physical_fraction(
                    ((position.x - self.bounds.left()) / self.bounds.size.width).clamp(0., 1.),
                );
                ((fraction * count as f32) as usize).min(count - 1)
            }
            ChartLayout::Pie | ChartLayout::Radar => {
                let center = self.bounds.center();
                let radius = f32::from(self.bounds.size.width.min(self.bounds.size.height)) / 2.;
                let dx = f32::from(position.x - center.x);
                let dy = f32::from(position.y - center.y);
                if dx.hypot(dy) > radius {
                    return;
                }
                let tau = std::f32::consts::TAU;
                let clockwise = dx.atan2(-dy).rem_euclid(tau) / tau;
                let turn = if direction.is_rtl() {
                    (1. - clockwise).rem_euclid(1.)
                } else {
                    clockwise
                };
                if self.layout == ChartLayout::Radar {
                    (turn * count as f32).round() as usize % count
                } else {
                    let slices = self.pie_slices();
                    let Some(index) = slices
                        .iter()
                        .position(|(start, end)| start < end && (*start..*end).contains(&turn))
                    else {
                        return;
                    };
                    index
                }
            }
        };
        self.inspect(index, window, cx);
    }
}
impl EventEmitter<ChartEvent> for ChartState {}

/// Unstyled plot interaction region. Grouped and Stacked category centers are
/// evenly spaced across its width; Pie and Radar categories are hit by angle
/// inside the largest centered circle. Hover/click inspect a category;
/// Left/Right and Home/End inspect while the plot holds focus. The current
/// exact values form its native value.
///
/// Supply a sized native plot as its child and keep axes/legend outside this
/// hit region. Include an ordinary Table of the same dataset for full nonvisual
/// access. The component does not choose geometry, colors, or numeric formatting.
#[derive(IntoElement)]
pub struct Chart {
    base: crate::ObservedElement<gpui::Stateful<gpui::Div>>,
    state: Entity<ChartState>,
    label: SharedString,
    direction: Option<Direction>,
}
impl Chart {
    pub fn new(
        id: impl Into<ElementId>,
        state: &Entity<ChartState>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            base: div().id(id).test_support(),
            state: state.clone(),
            label: label.into(),
            direction: None,
        }
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }
}
impl Styled for Chart {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}
impl ParentElement for Chart {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(children);
    }
}
impl RenderOnce for Chart {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let direction = self.direction.unwrap_or_else(|| Direction::current(cx));
        let model = self.state.read(cx);
        let focus = model.focus.clone();
        let description = model.inspected_description();
        let previous = self.state.clone();
        let next = self.state.clone();
        let first = self.state.clone();
        let last = self.state.clone();
        let geometry = self.state.clone();
        self.base.role(Role::Group).aria_label(self.label).aria_value(description)
            .aria_description("Use Left and Right to inspect categories. The accompanying data table contains all values.")
            .track_focus(&focus).key_context("Chart")
            .on_action(move |_:&SelectLeft,window,cx|{previous.update(cx,|state,cx|state.step(direction.horizontal_forward(false),window,cx));})
            .on_action(move |_:&SelectRight,window,cx|{next.update(cx,|state,cx|state.step(direction.horizontal_forward(true),window,cx));})
            .on_action(move |_:&SelectFirst,window,cx|{first.update(cx,|state,cx|{if state.focus.is_focused(window){state.inspect(0,window,cx);}else{cx.propagate();}});})
            .on_action(move |_:&SelectLast,window,cx|{last.update(cx,|state,cx|{if state.focus.is_focused(window){if !state.categories.is_empty(){state.inspect(state.categories.len()-1,window,cx);}}else{cx.propagate();}});})
            .on_mouse_down(MouseButton::Left,window.listener_for(&self.state,move |state,event: &gpui::MouseDownEvent,window,cx|{
                state.focus.focus(window,cx);
                state.inspect_at_pointer(event.position,direction,window,cx);
            }))
            .on_mouse_move(window.listener_for(&self.state,move |state,event: &gpui::MouseMoveEvent,window,cx|{
                state.inspect_at_pointer(event.position,direction,window,cx);
            }))
            .on_prepaint(move |bounds,_,cx|geometry.update(cx,|state,_|state.bounds=bounds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Root;
    use gpui::{AppContext as _, Render, Subscription, TestAppContext, point, px};
    struct Harness {
        state: Entity<ChartState>,
        _subscription: Subscription,
    }
    impl Render for Harness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Chart::new("chart", &self.state, "Session delta")
                .w(px(300.))
                .h(px(120.))
        }
    }
    #[gpui::test]
    fn inspection_and_legend_keep_exact_values_and_signed_domain(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let state = cx.new(|cx| {
                    ChartState::new(
                        ["Mon", "Tue", "Wed"],
                        [
                            ChartSeries::new("Desktop", [-10., 0., 20.]),
                            ChartSeries::new("Mobile", [5., 10., 15.]),
                        ],
                        cx,
                    )
                });
                let subscription = cx.observe(&state, |_, _, cx| cx.notify());
                Harness {
                    state,
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
            let state = view.read(cx).state.clone();
            state.read(cx).focus.clone().focus(window, cx);
            window.draw(cx).clear(cx);
            assert_eq!(state.read(cx).range(), (-10., 20.));
            assert_eq!(state.read(cx).normalized_value(-10.), 0.);
            assert_eq!(state.read(cx).normalized_value(20.), 1.);
        });
        cx.simulate_keystrokes("right");
        cx.update(|_, cx| {
            assert_eq!(
                view.read(cx)
                    .state
                    .read(cx)
                    .inspected_description()
                    .as_ref(),
                "Tue: Desktop: 0, Mobile: 10"
            )
        });
        cx.simulate_mouse_move(point(px(290.), px(50.)), None, gpui::Modifiers::default());
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            assert_eq!(state.read(cx).inspected_index(), Some(2));
            state.update(cx, |state, cx| {
                state.set_visible(0, false, window, cx);
                assert_eq!(state.range(), (0., 15.));
                assert_eq!(state.inspected_description().as_ref(), "Wed: Mobile: 15");
                state.set_visible(1, false, window, cx);
                assert_eq!(state.range(), (0., 1.));
                assert_eq!(state.normalized_value(0.), 0.);
                state.set_visible(0, true, window, cx);
                assert_eq!(state.range(), (-10., 20.));
                assert_eq!(state.normalized_value(20.), 1.);
            });
        });
        cx.simulate_mouse_down(
            point(px(10.), px(50.)),
            MouseButton::Left,
            gpui::Modifiers::default(),
        );
        cx.simulate_mouse_up(
            point(px(10.), px(50.)),
            MouseButton::Left,
            gpui::Modifiers::default(),
        );
        cx.update(|window, cx| {
            let state = view.read(cx).state.read(cx);
            assert_eq!(
                state.inspected_index(),
                Some(0),
                "a press inspects without an earlier move"
            );
            assert!(state.focus_handle().is_focused(window));
        });
    }

    #[gpui::test]
    fn layouts_share_stacked_ranges_and_radial_hit_testing(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| {
                let state = cx.new(|cx| {
                    ChartState::new(
                        ["Mon", "Tue", "Wed"],
                        [
                            ChartSeries::new("Desktop", [-10., 0., 20.]),
                            ChartSeries::new("Mobile", [5., 10., 15.]),
                        ],
                        cx,
                    )
                });
                let subscription = cx.observe(&state, |_, _, cx| cx.notify());
                Harness {
                    state,
                    _subscription: subscription,
                }
            });
            Root::new(view, window, cx)
        });
        let state = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap()
            .read_with(cx, |view, _| view.state.clone());
        cx.update(|window, cx| {
            state.update(cx, |state, cx| {
                assert!(state.set_layout(ChartLayout::Stacked, window, cx));
                assert_eq!(state.range(), (-10., 35.));
                assert_eq!(state.stack_span(0, 0), Some((0., -10.)));
                assert_eq!(state.stack_span(1, 0), Some((0., 5.)));
                assert_eq!(state.stack_span(1, 2), Some((20., 35.)));
                state.set_visible(0, false, window, cx);
                assert_eq!(state.stack_span(0, 2), None);
                assert_eq!(state.stack_span(1, 2), Some((0., 15.)));
                assert_eq!(state.range(), (0., 15.));
                state.set_visible(0, true, window, cx);
                assert!(state.set_layout(ChartLayout::Pie, window, cx));
                assert_eq!(
                    state.range(),
                    (-10., 20.),
                    "only stacking changes the range"
                );
                let slices = state.pie_slices();
                assert_eq!(slices[0], (0., 0.), "a negative total has no slice");
                assert!((slices[1].1 - 10. / 45.).abs() < 1e-6);
                assert_eq!(slices[2].1, 1.);
            });
            window.draw(cx).clear(cx);
        });
        // The plot is 300×120 at the window origin: center (150, 60), radius 60.
        let inspected =
            |cx: &mut gpui::VisualTestContext| cx.update(|_, cx| state.read(cx).inspected_index());
        cx.simulate_mouse_move(point(px(170.), px(40.)), None, gpui::Modifiers::default());
        assert_eq!(inspected(cx), Some(1), "an eighth turn lies in Tue's slice");
        cx.simulate_mouse_move(point(px(120.), px(60.)), None, gpui::Modifiers::default());
        assert_eq!(inspected(cx), Some(2), "three quarters lies in Wed's slice");
        cx.simulate_mouse_move(point(px(290.), px(60.)), None, gpui::Modifiers::default());
        assert_eq!(
            inspected(cx),
            Some(2),
            "outside the circle keeps inspection"
        );
        cx.update(|window, cx| {
            state.update(cx, |state, cx| {
                state.set_layout(ChartLayout::Radar, window, cx);
                assert_eq!(state.radar_turn(1), 1. / 3.);
            })
        });
        cx.simulate_mouse_move(point(px(150.), px(10.)), None, gpui::Modifiers::default());
        assert_eq!(
            inspected(cx),
            Some(0),
            "the top spoke is the first category"
        );
        cx.simulate_mouse_move(point(px(115.), px(80.)), None, gpui::Modifiers::default());
        assert_eq!(
            inspected(cx),
            Some(2),
            "the lower-left spoke is the third category"
        );
    }
}
