//! Native chart plate, interactive category inspection and original-data disclosure.
use super::{Palette, palette, style};
use gpui::{prelude::*, *};
use gpui_deck::{Chart, ChartSeries, ChartState, Table, TableCell, TableColumn, TableRow};
use std::rc::Rc;

#[allow(dead_code)] // Each separately installed report selects one of these chart types.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Plot {
    Line,
    Revenue,
    Bars,
    StackedArea,
    Horizontal,
}

pub(super) struct Row {
    pub label: String,
    pub values: Vec<Option<f64>>,
    pub cells: Vec<String>,
}
pub(super) struct Data {
    pub legend: Vec<&'static str>,
    pub columns: &'static [&'static str],
    pub rows: Vec<Row>,
    pub plot: Plot,
    pub domain: Option<(f64, f64)>,
    pub reference: Option<(f64, &'static str)>,
    pub format: fn(f64) -> String,
    pub summary: String,
}

impl Data {
    pub(super) fn range(&self) -> (f64, f64, u32) {
        if let Some(domain) = self.domain {
            return (domain.0, domain.1, 4);
        }
        let mut min = 0.0_f64;
        let mut max = self.reference.map_or(0., |r| r.0);
        for row in &self.rows {
            if self.plot == Plot::StackedArea {
                max = max.max(row.values.iter().flatten().sum());
            }
            for &v in row.values.iter().flatten() {
                min = min.min(v);
                max = max.max(v);
            }
        }
        if self.plot == Plot::Horizontal {
            return (min, max.max(1.), 1);
        }
        // These series measure whole counts, cents, milliseconds or seconds. Use integer
        // intervals and the same endpoints for vertices, grid lines and visible labels.
        let rough = ((max - min) / 4.).max(1.);
        let magnitude = 10_f64.powf(rough.log10().floor());
        let normalized = rough / magnitude;
        let unit = if normalized < 1.5 {
            1.
        } else if normalized < 3.5 {
            2.
        } else if normalized < 7.5 {
            5.
        } else {
            10.
        };
        let step = unit * magnitude;
        let low = (min / step).floor() * step;
        let high = ((max / step).ceil() * step).max(low + step);
        (low, high, ((high - low) / step).round() as u32)
    }
}

type Dataset = Box<dyn Fn(usize) -> Data>;
pub(super) struct Report {
    title: &'static str,
    filters: &'static [&'static str],
    choice: usize,
    expanded: bool,
    source: Option<Dataset>,
    visible: Vec<bool>,
    data: Rc<Data>,
    inspect: Entity<ChartState>,
    observer: Subscription,
    palette: Palette,
}
impl Report {
    pub(super) fn open(
        title: &'static str,
        filters: &'static [&'static str],
        source: impl Fn(usize) -> Data + 'static,
        palette: Palette,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| {
            Self::new(
                title,
                filters,
                source(0),
                Some(Box::new(source)),
                palette,
                cx,
            )
        })
        .into()
    }
    pub(super) fn inline(
        title: &'static str,
        data: Data,
        palette: Palette,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| Self::new(title, &[], data, None, palette, cx))
            .into()
    }
    fn new(
        title: &'static str,
        filters: &'static [&'static str],
        data: Data,
        source: Option<Dataset>,
        palette: Palette,
        cx: &mut Context<Self>,
    ) -> Self {
        let (inspect, observer) = Self::inspect(&data, cx);
        Self {
            title,
            filters,
            choice: 0,
            expanded: source.is_none(),
            source,
            visible: vec![true; data.legend.len()],
            data: Rc::new(data),
            inspect,
            observer,
            palette,
        }
    }
    fn inspect(data: &Data, cx: &mut Context<Self>) -> (Entity<ChartState>, Subscription) {
        let state = cx.new(|cx| {
            ChartState::new(
                data.rows.iter().map(|r| r.label.clone()),
                std::iter::empty::<ChartSeries>(),
                cx,
            )
        });
        let observation = cx.observe(&state, |_, _, cx| cx.notify());
        (state, observation)
    }
    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.choice != index
            && let Some(source) = &self.source
        {
            self.choice = index;
            self.data = Rc::new(source(index));
            self.visible = vec![true; self.data.legend.len()];
            (self.inspect, self.observer) = Self::inspect(&self.data, cx);
            cx.notify();
        }
    }
    fn table(&self) -> Table {
        let p = self.palette;
        let count = self.data.columns.len();
        let mut table = Table::new("data", (0..count).map(|_| TableColumn::flex(1.)))
            .accessibility_label(format!("{} exact data", self.title))
            .w_full()
            .text_size(px(12.));
        let cell = |r: usize, c: usize, value: String| {
            TableCell::new(("cell", r * count + c))
                .px_1()
                .py_2()
                .row_header(c == 0)
                .accessibility_label(value.clone())
                .child(value)
        };
        let header = TableRow::new("head")
            .bg(p.field)
            .font_weight(FontWeight::SEMIBOLD)
            .cells(
                self.data
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(i, s)| cell(0, i, (*s).into())),
            );
        table = table.header(header);
        for (i, row) in self.data.rows.iter().enumerate() {
            table = table.row(
                TableRow::new(("record", i))
                    .cell(cell(i + 1, 0, row.label.clone()))
                    .cells(
                        row.cells
                            .iter()
                            .enumerate()
                            .map(|(j, value)| cell(i + 1, j + 1, value.clone())),
                    ),
            );
        }
        table
    }
    fn chart(
        &self,
        selected: Option<usize>,
        description: &str,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let p = self.palette;
        let d = &self.data;
        let (low, high, intervals) = d.range();
        if d.plot == Plot::Horizontal {
            let mut bars = div().w_full().flex().flex_col().gap_3();
            for (i, row) in d.rows.iter().enumerate() {
                let value = row.values.first().copied().flatten().unwrap_or(0.);
                let state = self.inspect.clone();
                let bar = style::action(("stage", i), row.label.clone(), p)
                    .selected(selected == Some(i))
                    .when(selected == Some(i), |s| {
                        s.shadow(palette::inset(p)).bg(p.highlight)
                    })
                    .accessibility_label(format!("{} {}", row.label, (d.format)(value)))
                    .w(px(100.))
                    .on_click(move |_, window, cx| {
                        state.update(cx, |s, cx| {
                            s.inspect(i, window, cx);
                        })
                    });
                bars = bars.child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(bar)
                        .child(
                            palette::well(p).flex_1().h(px(18.)).child(
                                div()
                                    .h_full()
                                    .rounded_full()
                                    .bg(p.accent)
                                    .w(relative((value / high).clamp(0., 1.) as f32)),
                            ),
                        )
                        .child(div().w(px(56.)).text_size(px(12.)).child((d.format)(value))),
                );
            }
            return bars.into_any_element();
        }
        let data = self.data.clone();
        let visible = self.visible.clone();
        let image = Chart::new("plot", &self.inspect, self.title)
            .value_description(description.to_owned())
            .w_full()
            .h(px(180.))
            .rounded(px(6.))
            .border_1()
            .border_color(if self.inspect.read(cx).focus_handle().is_focused(window) {
                p.accent
            } else {
                p.border
            })
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| paint(&data, &visible, selected, p, bounds, window),
                )
                .size_full(),
            );
        let ticks = div()
            .h(px(180.))
            .flex()
            .flex_col()
            .justify_between()
            .text_size(px(11.))
            .text_color(p.muted)
            .children(
                (0..=intervals)
                    .map(|i| (d.format)(high - (high - low) * f64::from(i) / f64::from(intervals))),
            );
        let categories = d
            .rows
            .iter()
            .map(|row| {
                row.label.parse::<gpui_deck::Date>().map_or_else(
                    |_| row.label.clone(),
                    |date| format!("{:02}-{:02}", date.month(), date.day()),
                )
            })
            .collect::<Vec<_>>();
        let count = categories.len();
        let longest = categories
            .iter()
            .map(|label| label.chars().count())
            .max()
            .unwrap_or(1);
        let width = (f32::from(window.viewport_size().width).min(720.) - 120.).max(1.);
        let max_labels = (width / (longest as f32 * 7. + 10.)).floor().max(1.) as usize;
        let stride = count.div_ceil(max_labels).max(1);
        let axis = div()
            .w_full()
            .flex()
            .text_size(px(11.))
            .text_color(p.muted)
            .children(categories.into_iter().enumerate().map(|(i, label)| {
                // Each slot occupies exactly one chart category; skipping its label must
                // never pull another label away from the center of its line point or bar.
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .justify_center()
                    .when(i.is_multiple_of(stride), |slot| {
                        slot.child(div().flex_none().whitespace_nowrap().child(label))
                    })
            }));
        div()
            .w_full()
            .flex()
            .gap_2()
            .child(ticks)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(image)
                    .child(axis),
            )
            .into_any_element()
    }
}

impl Render for Report {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let d = &self.data;
        let p = self.palette;
        let selected = self.inspect.read(cx).inspected_index();
        let description = selected
            .and_then(|i| d.rows.get(i))
            .map(|r| {
                if d.plot == Plot::Horizontal {
                    return std::iter::once(r.label.clone())
                        .chain(
                            d.columns
                                .iter()
                                .skip(1)
                                .zip(&r.cells)
                                .map(|(name, value)| format!("{name} {value}")),
                        )
                        .collect::<Vec<_>>()
                        .join(" · ");
                }
                std::iter::once(r.label.clone())
                    .chain(
                        d.legend
                            .iter()
                            .zip(&r.values)
                            .enumerate()
                            .filter(|(i, _)| self.visible[*i])
                            .map(|(_, (k, v))| {
                                format!("{k} {}", v.map(d.format).unwrap_or_else(|| "—".into()))
                            }),
                    )
                    .collect::<Vec<_>>()
                    .join(" · ")
            })
            .unwrap_or_else(|| "No observations in this selection.".into());
        let colors = palette::chart_colors(p);
        let embedded = self.source.is_none();
        let legend = div()
            .flex()
            .flex_wrap()
            .gap_3()
            .children(d.legend.iter().enumerate().map(|(i, l)| {
                if embedded {
                    return style::action(("series", i), *l, p)
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(div().rounded_full().size(px(8.)).bg(colors[i]))
                        .accessibility_label(format!("Show {l}"))
                        .aria_toggled(if self.visible[i] {
                            Toggled::True
                        } else {
                            Toggled::False
                        })
                        .when(self.visible[i], |b| {
                            b.shadow(palette::inset(p)).bg(p.highlight)
                        })
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.visible[i] = !this.visible[i];
                            cx.notify();
                        }))
                        .into_any_element();
                }
                div()
                    .flex()
                    .gap_1()
                    .items_center()
                    .child(div().rounded_full().size(px(8.)).bg(colors[i]))
                    .child(*l)
                    .into_any_element()
            }))
            .when_some(d.reference, |s, (v, l)| {
                s.child(format!("{l} {}", (d.format)(v)))
            });
        let choices =
            div()
                .flex()
                .flex_wrap()
                .gap_2()
                .children(self.filters.iter().enumerate().map(|(i, label)| {
                    style::action(("option", i), *label, p)
                        .selected(i == self.choice)
                        .when(i == self.choice, |s| {
                            s.shadow(palette::inset(p)).bg(p.highlight)
                        })
                        .on_click(cx.listener(move |this, _, _, cx| this.select(i, cx)))
                }));
        let mut card = (if embedded {
            div().w_full().min_w_0()
        } else {
            style::surface(p)
        })
        .flex()
        .flex_col()
        .gap_4()
        .when(!embedded, |s| {
            s.child(
                div()
                    .text_size(px(24.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.title),
            )
            .child(div().text_color(p.muted).child(d.summary.clone()))
        })
        .when(!self.filters.is_empty(), |s| s.child(choices))
        .child(legend);
        if !d.rows.is_empty() {
            card = card.child(self.chart(selected, &description, window, cx));
        }
        card = card
            .child(
                div()
                    .id("inspection")
                    .role(Role::Status)
                    .text_size(px(13.))
                    .child(description),
            )
            .child(
                style::action(
                    "details",
                    if self.expanded {
                        "Hide exact data"
                    } else {
                        "View exact data"
                    },
                    p,
                )
                .selected(self.expanded)
                .on_click(cx.listener(|this, _, _, cx| {
                    this.expanded = !this.expanded;
                    cx.notify();
                })),
            );
        if self.expanded {
            card = card.child(self.table());
        }
        if embedded {
            card.into_any_element()
        } else {
            style::stage(p, card).into_any_element()
        }
    }
}

fn line(points: &[Point<Pixels>], color: Hsla, width: f32, window: &mut Window) {
    if points.len() < 2 {
        return;
    }
    let mut path = PathBuilder::stroke(px(width));
    path.add_polygon(points, false);
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
}
fn area(points: &[Point<Pixels>], color: Hsla, window: &mut Window) {
    if points.len() < 3 {
        return;
    }
    let mut path = PathBuilder::fill();
    path.add_polygon(points, true);
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
}

fn paint(
    d: &Data,
    visible: &[bool],
    selected: Option<usize>,
    p: Palette,
    b: Bounds<Pixels>,
    w: &mut Window,
) {
    let n = d.rows.len();
    if n == 0 {
        return;
    }
    let (low, high, intervals) = d.range();
    let y = |v: f64| b.bottom() - b.size.height * ((v - low) / (high - low)).clamp(0., 1.) as f32;
    let width = b.size.width / n as f32;
    let x = |i: usize| b.left() + width * (i as f32 + 0.5);
    let colors = palette::chart_colors(p);
    for i in 0..=intervals {
        let yy = y(low + (high - low) * f64::from(i) / f64::from(intervals));
        line(
            &[point(b.left(), yy), point(b.right(), yy)],
            p.border,
            1.,
            w,
        );
    }
    if let Some(i) = selected {
        w.paint_quad(fill(
            Bounds::new(
                point(x(i) - width * 0.5, b.top()),
                size(width, b.size.height),
            ),
            p.accent.opacity(0.05),
        ));
    }
    let mut lower = vec![0.; n];
    for (s, &color) in colors.iter().take(d.legend.len()).enumerate() {
        if !visible[s] {
            continue;
        }
        if d.plot == Plot::Bars {
            let series_width = width * 0.72 / d.legend.len() as f32;
            for (i, r) in d.rows.iter().enumerate() {
                if let Some(v) = r.values[s] {
                    let top = y(v).min(y(0.));
                    let height = (y(v) - y(0.)).abs();
                    w.paint_quad(
                        fill(
                            Bounds::new(
                                point(x(i) - width * 0.36 + series_width * s as f32, top),
                                size(series_width, height),
                            ),
                            color,
                        )
                        .corner_radii(px(2.)),
                    );
                }
            }
            continue;
        }
        let mut start = 0;
        while start < n {
            if d.rows[start].values[s].is_none() {
                start += 1;
                continue;
            }
            let mut end = start + 1;
            while end < n && d.rows[end].values[s].is_some() {
                end += 1;
            }
            let points: Vec<_> = (start..end)
                .map(|i| point(x(i), y(lower[i] + d.rows[i].values[s].unwrap())))
                .collect();
            if d.plot == Plot::StackedArea || (d.plot == Plot::Revenue && s == 0) {
                let mut polygon = points.clone();
                polygon.extend((start..end).rev().map(|i| point(x(i), y(lower[i]))));
                area(&polygon, color.opacity(0.23), w);
            }
            line(&points, color, 2., w);
            for &pos in &points {
                w.paint_quad(
                    fill(
                        Bounds::new(pos - point(px(2.), px(2.)), size(px(4.), px(4.))),
                        color,
                    )
                    .corner_radii(px(2.)),
                );
            }
            start = end;
        }
        if d.plot == Plot::StackedArea {
            for (i, r) in d.rows.iter().enumerate() {
                lower[i] += r.values[s].unwrap_or(0.);
            }
        }
    }
    if let Some((value, _)) = d.reference {
        let mut left = b.left();
        while left < b.right() {
            line(
                &[
                    point(left, y(value)),
                    point((left + px(5.)).min(b.right()), y(value)),
                ],
                p.error,
                1.,
                w,
            );
            left += px(9.);
        }
    }
}
