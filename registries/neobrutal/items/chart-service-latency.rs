mod colors;
mod palette;
mod style;

use super::Palette;
use super::recipes::ChartKind;
use gpui::{AnyView, App, Window};
use gpui_deck::ChartSeries;
mod report;
use report::{Filter, Report, SeriesData};

fn dataset(choice: &[usize]) -> SeriesData {
    let values = if choice[0] == 0 {
        [
            (80, 210),
            (76, 195),
            (95, 340),
            (130, 470),
            (100, 330),
            (82, 240),
            (79, 215),
        ]
    } else {
        [
            (105, 270),
            (112, 290),
            (118, 310),
            (110, 280),
            (108, 260),
            (102, 250),
            (99, 245),
        ]
    };
    let breaches = values.iter().filter(|r| r.1 > 300).count();
    let worst = values.iter().map(|r| r.1).max().unwrap();
    SeriesData {
        categories: vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
        series: vec![
            ChartSeries::new("p50", values.map(|r| f64::from(r.0))),
            ChartSeries::new("p95", values.map(|r| f64::from(r.1))),
            ChartSeries::new("Budget", [300.; 7]),
        ],
        kind: ChartKind::Line,
        format: |v| format!("{v:.0} ms"),
        summary: format!("p95 budget 300 ms · {breaches} days over budget · Peak p95 {worst} ms"),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[Filter {
        label: "Service",
        options: &["API", "Search"],
    }];
    Report::open(
        "Response-time budget",
        "Day",
        FILTERS,
        dataset,
        p,
        window,
        cx,
    )
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    open(palette::yellow(), window, cx)
}
