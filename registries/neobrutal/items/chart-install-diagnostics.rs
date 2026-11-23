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
    let stages = ["Resolve", "Download", "Transform", "Write"];
    let ms = if choice[0] == 0 {
        [120., 840., 210., 90.]
    } else {
        [180., 1480., 240., 110.]
    };
    let total = ms.iter().sum::<f64>();
    let seconds = choice[1] == 1;
    let fmt: fn(f64) -> String = if seconds {
        |v| format!("{v:.2} s")
    } else {
        |v| format!("{v:.0} ms")
    };
    let scale = if seconds { 1000. } else { 1. };
    SeriesData {
        categories: stages.to_vec(),
        series: vec![ChartSeries::new("Duration", ms.map(|v| v / scale))],
        kind: ChartKind::Bar,
        format: fmt,
        summary: format!(
            "Total {} · Stage shares: resolve {:.1}%, download {:.1}%, transform {:.1}%, write {:.1}%",
            fmt(total / scale),
            ms[0] / total * 100.,
            ms[1] / total * 100.,
            ms[2] / total * 100.,
            ms[3] / total * 100.
        ),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[
        Filter {
            label: "Environment",
            options: &["Local", "CI"],
        },
        Filter {
            label: "Units",
            options: &["Milliseconds", "Seconds"],
        },
    ];
    Report::open(
        "Installation trace",
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
