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
    let all = [
        ("W1", 12400, 14000),
        ("W2", 15100, 14000),
        ("W3", 13800, 15000),
        ("W4", 17200, 15000),
        ("W5", 16400, 16000),
        ("W6", 18900, 16000),
        ("W7", 17600, 17000),
        ("W8", 21400, 17000),
    ];
    let rows = &all[if choice[0] == 0 { 4 } else { 0 }..];
    let actual = rows.iter().map(|r| r.1).sum::<i32>();
    let target = rows.iter().map(|r| r.2).sum::<i32>();
    let below = rows.iter().filter(|r| r.1 < r.2).count();
    SeriesData {
        categories: rows.iter().map(|r| r.0).collect(),
        series: vec![
            ChartSeries::new("Revenue", rows.iter().map(|r| f64::from(r.1))),
            ChartSeries::new("Target", rows.iter().map(|r| f64::from(r.2))),
        ],
        kind: ChartKind::Bar,
        format: |v| format!("${v:.0}"),
        summary: format!(
            "Revenue ${actual} · Target ${target} · Variance {:+} USD · {below} weeks below target",
            actual - target
        ),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[Filter {
        label: "Period",
        options: &["Last 4 weeks", "All 8 weeks"],
    }];
    Report::open("Revenue vs target", "Week", FILTERS, dataset, p, window, cx)
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    open(palette::yellow(), window, cx)
}
