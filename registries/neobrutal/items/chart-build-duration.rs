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
        ("B1", 210, 108),
        ("B2", 195, 102),
        ("B3", 238, 114),
        ("B4", 226, 109),
        ("B5", 204, 96),
        ("B6", 188, 93),
    ];
    let rows = &all[if choice[0] == 0 { 0 } else { 3 }..];
    let mut series = Vec::new();
    if choice[1] != 2 {
        series.push(ChartSeries::new(
            "Cold build",
            rows.iter().map(|r| f64::from(r.1)),
        ));
    }
    if choice[1] != 1 {
        series.push(ChartSeries::new(
            "Cached build",
            rows.iter().map(|r| f64::from(r.2)),
        ));
    }
    let saved = rows.iter().map(|r| f64::from(r.1 - r.2)).sum::<f64>() / rows.len() as f64;
    let over = rows.iter().filter(|r| r.1 > 220).count();
    SeriesData {
        categories: rows.iter().map(|r| r.0).collect(),
        series,
        kind: ChartKind::Bar,
        format: |v| format!("{v} s"),
        summary: format!(
            "Mean time saved {saved:.1} s · Cold-build budget 220 s · {over} over budget"
        ),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[
        Filter {
            label: "Period",
            options: &["All 6 builds", "Latest 3"],
        },
        Filter {
            label: "Series",
            options: &["Cold and cached", "Cold only", "Cached only"],
        },
    ];
    Report::open("Build duration", "Build", FILTERS, dataset, p, window, cx)
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    open(palette::yellow(), window, cx)
}
