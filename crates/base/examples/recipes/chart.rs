//! Copyable native line/bar/area, stacked, pie/donut and radar presentation over
//! the public headless Chart.

use gpui::{
    App, Bounds, ElementId, Entity, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    PathBuilder, Pixels, Point, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    Styled, Window, canvas, div, fill, point, px, rgb, size,
};
use gpui_deck::{
    Chart, ChartLayout, ChartState, Direction, Table, TableCell, TableColumn, TableRow, Toggle,
};
use std::f32::consts::TAU;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChartKind {
    Line,
    Bar,
    Area,
    StackedBar,
    StackedArea,
    Pie,
    Donut,
    Radar,
}

impl ChartKind {
    pub(crate) const ALL: [Self; 8] = [
        Self::Line,
        Self::Bar,
        Self::Area,
        Self::StackedBar,
        Self::StackedArea,
        Self::Pie,
        Self::Donut,
        Self::Radar,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Line => "Line",
            Self::Bar => "Bar",
            Self::Area => "Area",
            Self::StackedBar => "Stacked bar",
            Self::StackedArea => "Stacked area",
            Self::Pie => "Pie",
            Self::Donut => "Donut",
            Self::Radar => "Radar",
        }
    }

    /// The ChartState layout this presentation draws. Keep them in sync with
    /// `ChartState::set_layout` when switching kinds.
    pub(crate) fn layout(self) -> ChartLayout {
        match self {
            Self::Line | Self::Bar | Self::Area => ChartLayout::Grouped,
            Self::StackedBar | Self::StackedArea => ChartLayout::Stacked,
            Self::Pie | Self::Donut => ChartLayout::Pie,
            Self::Radar => ChartLayout::Radar,
        }
    }

    fn radial(self) -> bool {
        matches!(self, Self::Pie | Self::Donut | Self::Radar)
    }
}

#[derive(IntoElement)]
pub(crate) struct ChartRecipe {
    id: ElementId,
    state: Entity<ChartState>,
    kind: ChartKind,
    direction: Direction,
    plot_label: SharedString,
    table_label: SharedString,
    category_header: SharedString,
    format_value: Box<dyn Fn(f64) -> String>,
    colors: Vec<Hsla>,
}

impl ChartRecipe {
    pub(crate) fn new(
        id: impl Into<ElementId>,
        state: &Entity<ChartState>,
        kind: ChartKind,
        direction: Direction,
        plot_label: impl Into<SharedString>,
        table_label: impl Into<SharedString>,
        category_header: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            kind,
            direction,
            plot_label: plot_label.into(),
            table_label: table_label.into(),
            category_header: category_header.into(),
            format_value: Box::new(format_number),
            colors: Vec::new(),
        }
    }

    /// Format axis ticks, the inspected category and table cells in the same units.
    pub(crate) fn format_value(mut self, formatter: impl Fn(f64) -> String + 'static) -> Self {
        self.format_value = Box::new(formatter);
        self
    }

    /// Colors for series and their legend, or categories in a pie/donut chart.
    #[allow(dead_code)] // Consumers may use the built-in categorical colors.
    pub(crate) fn series_colors(mut self, colors: impl IntoIterator<Item = Hsla>) -> Self {
        self.colors = colors.into_iter().collect();
        self
    }
}

fn format_number(value: f64) -> String {
    // The shortest round-trip decimal, with scientific notation for extreme scales.
    let mut label = format!("{value:?}");
    if label.ends_with(".0") {
        label.truncate(label.len() - 2);
    }
    label
}

fn axis_ticks((min, max): (f64, f64)) -> impl Iterator<Item = f64> {
    // Trim floating-point arithmetic noise from the calculated middle tick only.
    // Keep the measured endpoints and all inspected/table values exact.
    let middle = format!("{:.14e}", min.midpoint(max))
        .parse::<f64>()
        .expect("a formatted float is a valid float");
    [
        Some(max),
        (min < middle && middle < max).then_some(middle),
        Some(min),
    ]
    .into_iter()
    .flatten()
}

fn default_colors(foreground: Hsla) -> [Hsla; 6] {
    let colors = if foreground.l > 0.5 {
        [0x38bdf8, 0xfb923c, 0x4ade80, 0xc084fc, 0xf472b6, 0xfacc15]
    } else {
        [0x0369a1, 0x9a3412, 0x166534, 0x7e22ce, 0xbe185d, 0x854d0e]
    };
    colors.map(|color| rgb(color).into())
}

const RADIAL_WIDTH: f32 = 240.;
const RADIAL_HEIGHT: f32 = 200.;
/// The drawn radius as a share of the largest centered circle used for hits.
const RADIAL_SCALE: f32 = 0.76;
/// Radar draws smaller so spoke labels fit between its rim and the frame.
const RADAR_SCALE: f32 = 0.64;

struct PlotSeries {
    heights: Vec<f32>,
    /// Normalized `(start, end)` segments for stacked kinds.
    spans: Vec<(f32, f32)>,
    color: Hsla,
}

struct Plot {
    series: Vec<PlotSeries>,
    slices: Vec<((f32, f32), Hsla)>,
    categories: usize,
    baseline: f32,
    inspected: Option<usize>,
    kind: ChartKind,
    direction: Direction,
    foreground: Hsla,
}

fn stroke(points: &[Point<Pixels>], closed: bool, color: Hsla, window: &mut Window) {
    if points.len() < 2 {
        return;
    }
    let mut path = PathBuilder::stroke(px(if closed { 1.5 } else { 2. }));
    path.add_polygon(points, closed);
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
}

fn polygon(points: &[Point<Pixels>], color: Hsla, window: &mut Window) {
    if points.len() < 3 {
        return;
    }
    let mut path = PathBuilder::fill();
    path.add_polygon(points, true);
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
}

fn dot(center: Point<Pixels>, radius: Pixels, color: Hsla, window: &mut Window) {
    window.paint_quad(
        fill(
            Bounds::new(
                center - point(radius, radius),
                size(radius * 2., radius * 2.),
            ),
            color,
        )
        .corner_radii(radius),
    );
}

/// The point at `turn` (clockwise from the top, mirrored for RTL) on a circle.
fn radial_point(
    center: Point<Pixels>,
    radius: f32,
    turn: f32,
    direction: Direction,
) -> Point<Pixels> {
    let angle = turn * TAU;
    let side = if direction.is_rtl() { -1. } else { 1. };
    point(
        center.x + px(radius * angle.sin() * side),
        center.y - px(radius * angle.cos()),
    )
}

fn arc(
    center: Point<Pixels>,
    radius: f32,
    (start, end): (f32, f32),
    direction: Direction,
) -> impl DoubleEndedIterator<Item = Point<Pixels>> {
    let steps = ((end - start) * 96.).ceil().max(2.) as usize;
    (0..=steps).map(move |step| {
        let turn = start + (end - start) * step as f32 / steps as f32;
        radial_point(center, radius, turn, direction)
    })
}

impl Plot {
    fn paint(&self, bounds: Bounds<Pixels>, window: &mut Window) {
        if self.categories == 0 {
            return;
        }
        if self.kind.radial() {
            self.paint_radial(bounds, window);
        } else {
            self.paint_cartesian(bounds, window);
        }
    }

    fn paint_cartesian(&self, bounds: Bounds<Pixels>, window: &mut Window) {
        let categories = self.categories;
        let direction = self.direction;
        let kind = self.kind;
        let x = |index: usize| {
            bounds.left()
                + bounds.size.width
                    * direction.physical_fraction((index as f32 + 0.5) / categories as f32)
        };
        let y = |height: f32| bounds.bottom() - bounds.size.height * height;
        for tick in 0..=4 {
            window.paint_quad(fill(
                Bounds::new(
                    point(bounds.left(), y(tick as f32 / 4.)),
                    size(bounds.size.width, px(1.)),
                ),
                self.foreground.opacity(0.2),
            ));
        }
        window.paint_quad(fill(
            Bounds::new(
                point(bounds.left(), y(self.baseline)),
                size(bounds.size.width, px(1.)),
            ),
            self.foreground.opacity(0.55),
        ));
        if let Some(index) = self.inspected {
            window.paint_quad(fill(
                Bounds::new(
                    point(x(index), bounds.top()),
                    size(px(1.), bounds.size.height),
                ),
                self.foreground.opacity(0.65),
            ));
        }
        let band = bounds.size.width / categories as f32 * 0.7;
        match kind {
            ChartKind::Line | ChartKind::Area => {
                for series in &self.series {
                    let points: Vec<_> = series
                        .heights
                        .iter()
                        .enumerate()
                        .map(|(index, &height)| point(x(index), y(height)))
                        .collect();
                    if kind == ChartKind::Area {
                        let mut area = points.clone();
                        area.push(point(points[points.len() - 1].x, y(self.baseline)));
                        area.push(point(points[0].x, y(self.baseline)));
                        polygon(&area, series.color.opacity(0.2), window);
                    }
                    stroke(&points, false, series.color, window);
                    self.paint_dots(&points, series.color, window);
                }
            }
            ChartKind::StackedArea => {
                for series in &self.series {
                    let tops: Vec<_> = series
                        .spans
                        .iter()
                        .enumerate()
                        .map(|(index, &(_, end))| point(x(index), y(end)))
                        .collect();
                    let mut area = tops.clone();
                    area.extend(
                        series
                            .spans
                            .iter()
                            .enumerate()
                            .rev()
                            .map(|(index, &(start, _))| point(x(index), y(start))),
                    );
                    polygon(&area, series.color.opacity(0.35), window);
                    stroke(&tops, false, series.color, window);
                    self.paint_dots(&tops, series.color, window);
                }
            }
            ChartKind::Bar => {
                let width = band / self.series.len().max(1) as f32;
                for (series_index, series) in self.series.iter().enumerate() {
                    let slot = if direction.is_rtl() {
                        self.series.len() - 1 - series_index
                    } else {
                        series_index
                    };
                    for (index, &height) in series.heights.iter().enumerate() {
                        let left = x(index) - band / 2. + width * slot as f32;
                        let top = y(height).min(y(self.baseline));
                        let height = (y(height) - y(self.baseline)).abs();
                        window.paint_quad(fill(
                            Bounds::new(
                                point(left, top),
                                size((width - px(2.)).max(px(1.)), height),
                            ),
                            series.color,
                        ));
                    }
                }
            }
            ChartKind::StackedBar => {
                for series in &self.series {
                    for (index, &(start, end)) in series.spans.iter().enumerate() {
                        let top = y(start.max(end));
                        let height = (y(start) - y(end)).abs();
                        window.paint_quad(fill(
                            Bounds::new(point(x(index) - band / 2., top), size(band, height)),
                            series.color,
                        ));
                    }
                }
            }
            ChartKind::Pie | ChartKind::Donut | ChartKind::Radar => unreachable!(),
        }
    }

    fn paint_dots(&self, points: &[Point<Pixels>], color: Hsla, window: &mut Window) {
        for (index, &center) in points.iter().enumerate() {
            let radius = if self.inspected == Some(index) {
                px(4.)
            } else {
                px(2.5)
            };
            dot(center, radius, color, window);
        }
    }

    fn paint_radial(&self, bounds: Bounds<Pixels>, window: &mut Window) {
        let center = bounds.center();
        let scale = if self.kind == ChartKind::Radar {
            RADAR_SCALE
        } else {
            RADIAL_SCALE
        };
        let radius = f32::from(bounds.size.width.min(bounds.size.height)) / 2. * scale;
        let direction = self.direction;
        if self.kind == ChartKind::Radar {
            let turn = |index: usize| index as f32 / self.categories as f32;
            let ring = |share: f32| -> Vec<_> {
                (0..self.categories)
                    .map(|index| radial_point(center, radius * share, turn(index), direction))
                    .collect()
            };
            for level in 1..=4 {
                stroke(
                    &ring(level as f32 / 4.),
                    true,
                    self.foreground.opacity(0.2),
                    window,
                );
            }
            if self.baseline > 0. {
                stroke(
                    &ring(self.baseline),
                    true,
                    self.foreground.opacity(0.55),
                    window,
                );
            }
            for index in 0..self.categories {
                let spoke = self.foreground.opacity(if self.inspected == Some(index) {
                    0.65
                } else {
                    0.2
                });
                stroke(
                    &[center, radial_point(center, radius, turn(index), direction)],
                    false,
                    spoke,
                    window,
                );
            }
            for series in &self.series {
                let points: Vec<_> = series
                    .heights
                    .iter()
                    .enumerate()
                    .map(|(index, &height)| {
                        radial_point(center, radius * height, turn(index), direction)
                    })
                    .collect();
                polygon(&points, series.color.opacity(0.2), window);
                stroke(&points, true, series.color, window);
                self.paint_dots(&points, series.color, window);
            }
            return;
        }
        let inner = if self.kind == ChartKind::Donut {
            radius * 0.55
        } else {
            0.
        };
        for (index, &(slice, color)) in self.slices.iter().enumerate() {
            if slice.0 >= slice.1 {
                continue;
            }
            let outer = if self.inspected == Some(index) {
                radius * 1.08
            } else {
                radius
            };
            let mut points: Vec<_> = arc(center, outer, slice, direction).collect();
            if inner > 0. {
                points.extend(arc(center, inner, slice, direction).rev());
            } else {
                points.push(center);
            }
            polygon(&points, color, window);
        }
    }
}

impl RenderOnce for ChartRecipe {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let foreground = window.text_style().color;
        let focus_color = self.colors.first().copied().unwrap_or(foreground);
        let defaults = default_colors(foreground);
        let colors = if self.colors.is_empty() {
            &defaults[..]
        } else {
            &self.colors[..]
        };
        let color = |index: usize| colors[index % colors.len()];
        let muted = foreground.opacity(0.75);
        let border = foreground.opacity(0.25);
        let model = self.state.read(cx);
        let categories = model.categories();
        let data = model.series();
        let range = model.range();
        let baseline = model.normalized_value(0.);
        let inspected = model.inspected_index();
        let description = model.inspected_description_with(&self.format_value);
        let kind = self.kind;
        let visible: Vec<_> = (0..data.len())
            .map(|index| model.is_visible(index))
            .collect();
        let stacked = kind.layout() == ChartLayout::Stacked;
        let plotted: Vec<_> = data
            .iter()
            .enumerate()
            .filter(|(index, _)| visible[*index])
            .map(|(index, series)| PlotSeries {
                heights: series
                    .values()
                    .iter()
                    .map(|&value| model.normalized_value(value))
                    .collect(),
                spans: if stacked {
                    (0..categories.len())
                        .filter_map(|category| model.stack_span(index, category))
                        .map(|(start, end)| {
                            (model.normalized_value(start), model.normalized_value(end))
                        })
                        .collect()
                } else {
                    Vec::new()
                },
                color: color(index),
            })
            .collect();
        let direction = self.direction;
        let drawing = Plot {
            series: plotted,
            slices: model
                .pie_slices()
                .into_iter()
                .enumerate()
                .map(|(index, slice)| (slice, color(index)))
                .collect(),
            categories: categories.len(),
            baseline,
            inspected,
            kind,
            direction,
            foreground,
        };
        let mut plot = Chart::new((self.id.clone(), "plot"), &self.state, self.plot_label)
            .value_description(description.clone())
            .direction(direction)
            .relative()
            .flex_shrink_0()
            .border_1()
            .border_color(if model.focus_handle().is_focused(window) {
                focus_color
            } else {
                border
            })
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| drawing.paint(bounds, window),
                )
                .size_full(),
            );
        plot = if kind.radial() {
            plot.w(px(RADIAL_WIDTH)).h(px(RADIAL_HEIGHT))
        } else {
            plot.w_full().h(px(200.))
        };
        if kind == ChartKind::Radar {
            // Spoke labels sit just outside the drawn radius, inside the hit circle.
            let center = point(px(RADIAL_WIDTH / 2.), px(RADIAL_HEIGHT / 2.));
            let radius = RADIAL_WIDTH.min(RADIAL_HEIGHT) / 2. * (RADAR_SCALE + 0.18);
            plot = plot.children(categories.iter().enumerate().map(|(index, category)| {
                let at = radial_point(center, radius, model.radar_turn(index), direction);
                div()
                    .absolute()
                    .left(at.x - px(22.))
                    .top(at.y - px(8.))
                    .w(px(44.))
                    .text_center()
                    .text_size(px(11.))
                    .text_color(muted)
                    .child(category.clone())
            }));
        }
        let mut legend = direction.row(div()).flex_wrap().gap_2();
        for (index, series) in data.iter().enumerate() {
            let state = self.state.clone();
            legend = legend.child(
                Toggle::new((self.id.clone(), format!("legend-{index}")))
                    .accessibility_label(format!("Show {}", series.label()))
                    .pressed(visible[index])
                    .px_2()
                    .py_1()
                    .rounded(px(4.))
                    .border_1()
                    .border_color(color(index))
                    .text_color(if visible[index] { color(index) } else { muted })
                    .focus_visible(|style| {
                        style
                            .bg(focus_color.opacity(0.15))
                            .border_color(focus_color)
                    })
                    .child(format!(
                        "{} {}",
                        if visible[index] { "●" } else { "○" },
                        series.label()
                    ))
                    .on_change(move |visible, _, window, cx| {
                        state.update(cx, |state, cx| {
                            state.set_visible(index, visible, window, cx);
                        })
                    }),
            );
        }
        let mut table = Table::new(
            (self.id.clone(), "table"),
            (0..=data.len()).map(|_| TableColumn::flex(1.)),
        )
        .accessibility_label(self.table_label)
        .w_full()
        .border_1()
        .border_color(border);
        let mut header = TableRow::new((self.id.clone(), "header")).cell(
            TableCell::new((self.id.clone(), "category-header"))
                .accessibility_label(self.category_header.clone())
                .p_2()
                .child(self.category_header),
        );
        for (index, series) in data.iter().enumerate() {
            header = header.cell(
                TableCell::new((self.id.clone(), format!("series-{index}")))
                    .accessibility_label(series.label().clone())
                    .p_2()
                    .child(series.label().clone()),
            );
        }
        table = table.header(header);
        for (index, category) in categories.iter().enumerate() {
            let mut row = TableRow::new((self.id.clone(), format!("row-{index}"))).cell(
                TableCell::new((self.id.clone(), format!("category-{index}")))
                    .row_header(true)
                    .accessibility_label(category.clone())
                    .p_2()
                    .child(category.clone()),
            );
            for (series_index, series) in data.iter().enumerate() {
                let text: SharedString = (self.format_value)(series.values()[index]).into();
                row = row.cell(
                    TableCell::new((self.id.clone(), format!("value-{index}-{series_index}")))
                        .accessibility_label(text.clone())
                        .p_2()
                        .child(text),
                );
            }
            table = table.row(row);
        }
        let figure = if kind.radial() {
            let mut figure = direction
                .row(div())
                .w_full()
                .gap_4()
                .flex_wrap()
                .items_center();
            figure = figure.child(plot);
            if kind != ChartKind::Radar {
                // Slice colors follow categories; the key is decorative text.
                figure = figure.child(div().flex().flex_col().gap_1().text_size(px(12.)).children(
                    categories.iter().enumerate().map(|(index, category)| {
                        div()
                            .text_color(color(index))
                            .child(format!("● {category}"))
                    }),
                ));
            }
            figure.into_any_element()
        } else {
            let ticks = div()
                .min_w(px(56.))
                .h(px(200.))
                .flex_shrink_0()
                .flex()
                .flex_col()
                .justify_between()
                .text_color(muted)
                .text_size(px(12.))
                .children(axis_ticks(range).map(&self.format_value));
            let labels = direction
                .row(div())
                .w_full()
                .children(categories.iter().map(|category| {
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_center()
                        .text_size(px(12.))
                        .child(category.clone())
                }));
            div()
                .flex()
                .gap_2()
                .child(ticks)
                .child(
                    div()
                        .min_w_0()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(plot)
                        .child(labels),
                )
                .into_any_element()
        };
        let hint = match kind {
            ChartKind::Pie | ChartKind::Donut => {
                "Each slice represents a category's positive visible total. Hover a slice, or focus the plot and use Left/Right, Home/End."
            }
            ChartKind::Radar => {
                "Each spoke is a category. Hover near a spoke, or focus the plot and use Left/Right, Home/End."
            }
            ChartKind::StackedBar | ChartKind::StackedArea => {
                "Positive and negative values stack separately from zero. Hover a category, or focus the plot and use Left/Right, Home/End."
            }
            _ => "Hover a category, or focus the plot and use Left/Right, Home/End.",
        };
        div()
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_2()
            .child(legend)
            .child(figure)
            .child(
                div()
                    .id((self.id, "readout"))
                    .role(Role::Status)
                    .aria_label(description.clone())
                    .child(description),
            )
            .child(format!(
                "{hint} Legend buttons hide a series; the table keeps all values."
            ))
            .child(table)
    }
}

#[cfg(test)]
mod tests {
    use super::{axis_ticks, format_number};

    #[test]
    fn axis_labels_distinguish_fractional_values() {
        let labels = |range| axis_ticks(range).map(format_number).collect::<Vec<_>>();
        assert_eq!(labels((0., 0.4)), ["0.4", "0.2", "0"]);
        assert_eq!(labels((-0.1, 0.3)), ["0.3", "0.1", "-0.1"]);
        assert_eq!(labels((0., 4e-15)), ["4e-15", "2e-15", "0"]);
    }
}
