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
    let work = ["Design", "Build", "Test", "Operations"];
    let hours = if choice[0] == 0 {
        [48., 72., 36., 24.]
    } else {
        [36., 90., 54., 30.]
    };
    let total = hours.iter().sum::<f64>();
    let i = choice[1];
    SeriesData {
        categories: work.to_vec(),
        series: vec![ChartSeries::new("Hours", hours)],
        kind: ChartKind::Donut,
        format: |v| format!("{v} h"),
        summary: format!(
            "Total {total} h · {}: {} h ({:.1}%)",
            work[i],
            hours[i],
            hours[i] / total * 100.
        ),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[
        Filter {
            label: "Period",
            options: &["Current", "Next"],
        },
        Filter {
            label: "Inspect",
            options: &["Design", "Build", "Test", "Operations"],
        },
    ];
    Report::open(
        "Work allocation",
        "Workstream",
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
