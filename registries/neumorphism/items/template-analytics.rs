//! Daily analytics with exact values; monetary records are expressed in cents.
mod analytics;
mod button;
mod layout;
mod palette;
use super::{
    Palette,
    recipes::{ChartKind, ChartRecipe},
};
use analytics::Summary;
pub(crate) use analytics::{Record, Totals};
use gpui::{prelude::*, *};
use gpui_deck::{ChartSeries, ChartState, Direction};
use layout::{banner, choice, column, heading, note, panel, row, screen};
use std::collections::BTreeSet;
#[derive(Clone, Copy, Eq, PartialEq)]
enum Metric {
    Revenue,
    Channel,
    Conversion,
}
impl Metric {
    fn title(self) -> &'static str {
        match self {
            Self::Revenue => "Revenue and target",
            Self::Channel => "Acquisition channels",
            Self::Conversion => "Conversion rate",
        }
    }
    fn kind(self) -> ChartKind {
        match self {
            Self::Channel => ChartKind::Bar,
            _ => ChartKind::Line,
        }
    }
}
pub(crate) struct Analytics {
    palette: Palette,
    records: Vec<Record>,
    days: usize,
    channel: Option<String>,
    metric: Metric,
    summary: Result<Summary, &'static str>,
    chart: Entity<ChartState>,
    observer: Subscription,
    on_export: Option<Box<dyn FnMut(&str) -> Result<(), String>>>,
    export_message: String,
}
fn plot(
    data: Option<&Summary>,
    metric: Metric,
    cx: &mut Context<Analytics>,
) -> (Entity<ChartState>, Subscription) {
    let mut labels = Vec::new();
    let mut a = Vec::new();
    let mut b = Vec::new();
    if let Some(summary) = data {
        match metric {
            Metric::Channel => {
                for (name, v) in &summary.channels {
                    labels.push(name.clone());
                    a.push(v.visits as f64);
                }
            }
            Metric::Revenue => {
                for (day, point) in &summary.points {
                    if let Some(value) = point {
                        labels.push(day.to_string());
                        a.push(value.revenue_cents as f64 / 100.);
                        b.push(value.target_cents as f64 / 100.);
                    }
                }
            }
            Metric::Conversion => {
                for (day, point) in &summary.points {
                    if let Some(rate) = point.and_then(Totals::conversion) {
                        labels.push(day.to_string());
                        a.push(rate * 100.);
                    }
                }
            }
        }
    }
    let series = match metric {
        Metric::Revenue => vec![
            ChartSeries::new("Revenue", a),
            ChartSeries::new("Target", b),
        ],
        Metric::Channel => vec![ChartSeries::new("Visits", a)],
        Metric::Conversion => vec![ChartSeries::new("Conversion", a)],
    };
    let state = cx.new(|cx| ChartState::new(labels, series, cx));
    let subscription = cx.observe(&state, |_, _, cx| cx.notify());
    (state, subscription)
}
impl Analytics {
    pub(crate) fn new(p: Palette, records: Vec<Record>, cx: &mut Context<Self>) -> Self {
        let summary = analytics::summarize(&records, 14, None);
        let metric = Metric::Revenue;
        let (chart, observer) = plot(summary.as_ref().ok(), metric, cx);
        Self {
            palette: p,
            records,
            days: 14,
            metric,
            channel: None,
            summary,
            chart,
            observer,
            on_export: None,
            export_message: String::new(),
        }
    }
    /// Exact filtered rows, preserving dates with missing observations.
    pub(crate) fn csv(&self) -> Result<String, &'static str> {
        self.summary.as_ref().map(Summary::csv).map_err(|e| *e)
    }
    pub(crate) fn on_export(
        mut self,
        export: impl FnMut(&str) -> Result<(), String> + 'static,
    ) -> Self {
        self.on_export = Some(Box::new(export));
        self
    }
    fn filter(&mut self, cx: &mut Context<Self>) {
        self.summary = analytics::summarize(&self.records, self.days, self.channel.as_deref());
        self.export_message.clear();
        self.replot(cx);
    }
    fn replot(&mut self, cx: &mut Context<Self>) {
        (self.chart, self.observer) = plot(self.summary.as_ref().ok(), self.metric, cx);
        cx.notify();
    }
}
impl Render for Analytics {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let mut period = row().child(note(p, "Period"));
        for days in [7, 14, 30] {
            period = period.child(
                choice(p, ("days", days), format!("{days} days"), self.days == days).on_click(
                    cx.listener(move |this, _, _, cx| {
                        this.days = days;
                        this.filter(cx);
                    }),
                ),
            );
        }
        let mut channels = row().child(note(p, "Channel"));
        let names = self
            .records
            .iter()
            .map(|r| r.channel.clone())
            .collect::<BTreeSet<_>>();
        for (i, channel) in std::iter::once(None)
            .chain(names.into_iter().map(Some))
            .enumerate()
        {
            channels = channels.child(
                choice(
                    p,
                    ("channel", i),
                    channel.as_deref().unwrap_or("All channels").to_owned(),
                    channel == self.channel,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.channel = channel.clone();
                    this.filter(cx)
                })),
            );
        }
        let mut body = column()
            .child(banner(p, "Workspace / Analytics", "Analytics dashboard"))
            .child(panel(p).child(period).child(channels));
        let summary = match &self.summary {
            Ok(s) => s,
            Err(message) => {
                return screen("analytics", p, body.child(panel(p).child(*message)))
                    .into_any_element();
            }
        };
        let t = summary.totals;
        let observed = summary.points.iter().filter(|(_, v)| v.is_some()).count();
        let mut kpis = row().gap_3();
        for (label, value) in [
            ("Revenue", format!("${:.2}", t.revenue_cents as f64 / 100.)),
            ("Visits", t.visits.to_string()),
            (
                "Conversion",
                t.conversion()
                    .map_or_else(|| "—".into(), |v| format!("{:.1}%", v * 100.)),
            ),
        ] {
            kpis = kpis.child(
                div().min_w(px(105.)).flex_1().child(
                    panel(p)
                        .p_3()
                        .gap_1()
                        .child(note(p, label))
                        .child(heading(value)),
                ),
            );
        }
        let export = layout::primary(
            p,
            "export",
            if self.on_export.is_some() {
                "Export CSV"
            } else {
                "Copy CSV"
            },
        )
        .disabled(observed == 0)
        .on_click(cx.listener(|this, _, _, cx| {
            if let Ok(csv) = this.csv() {
                this.export_message = if let Some(export) = &mut this.on_export {
                    match export(&csv) {
                        Ok(()) => "CSV exported.".into(),
                        Err(error) => error,
                    }
                } else {
                    cx.write_to_clipboard(ClipboardItem::new_string(csv));
                    "Clipboard copy requested.".into()
                };
                cx.notify();
            }
        }));
        let mut tabs = row();
        for (i, metric) in [Metric::Revenue, Metric::Channel, Metric::Conversion]
            .into_iter()
            .enumerate()
        {
            tabs = tabs.child(
                choice(p, ("metric", i), metric.title(), metric == self.metric).on_click(
                    cx.listener(move |this, _, _, cx| {
                        this.metric = metric;
                        this.replot(cx);
                    }),
                ),
            );
        }
        let metric = self.metric;
        let chart = ChartRecipe::new(
            "analytics-plot",
            &self.chart,
            metric.kind(),
            Direction::Ltr,
            metric.title(),
            "Exact values",
            if metric == Metric::Channel {
                "Channel"
            } else {
                "Date"
            },
        )
        .series_colors(palette::chart_colors(p))
        .format_value(move |v| match metric {
            Metric::Revenue => format!("${v:.2}"),
            Metric::Channel => format!("{v:.0}"),
            Metric::Conversion => format!("{v:.2}%"),
        });
        body = body.child(kpis).child(
            panel(p)
                .child(tabs)
                .child(heading(metric.title()))
                .child(note(
                    p,
                    format!("{observed} observed days in the last {} days", self.days),
                ))
                .child(
                    div()
                        .id("chart-width")
                        .w_full()
                        .overflow_x_scroll()
                        .child(chart),
                )
                .child(export)
                .when(!self.export_message.is_empty(), |s| {
                    s.child(note(p, self.export_message.clone()))
                }),
        );
        screen("analytics", p, body).into_any_element()
    }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| Analytics::new(palette::air(), analytics::demo(), cx))
        .into()
}
