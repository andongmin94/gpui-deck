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
    let counts = if choice[0] == 0 {
        [4800., 1800., 840., 520.]
    } else {
        [1600., 960., 650., 470.]
    };
    let stages = ["Visited", "Signed up", "Activated", "Paid"];
    let largest = (1..counts.len())
        .max_by_key(|&i| (counts[i - 1] - counts[i]) as i32)
        .unwrap();
    SeriesData {
        categories: stages.to_vec(),
        series: vec![ChartSeries::new("People", counts)],
        kind: ChartKind::Bar,
        format: |v| format!("{v:.0} people"),
        summary: format!(
            "Visit to paid {:.1}% · Largest loss: {} to {}, {:.0} people",
            counts[3] / counts[0] * 100.,
            stages[largest - 1],
            stages[largest],
            counts[largest - 1] - counts[largest]
        ),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[Filter {
        label: "Acquisition cohort",
        options: &["Self-serve", "Sales-led"],
    }];
    Report::open(
        "Signup conversion",
        "Stage",
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
