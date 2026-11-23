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
        ("R1", 148., 62.),
        ("R2", 232., 104.),
        ("R3", 196., 88.),
        ("R4", 284., 132.),
        ("R5", 341., 176.),
        ("R6", 378., 209.),
    ];
    let rows = &all[if choice[1] == 0 { 0 } else { 3 }..];
    let share = choice[0] == 1;
    let series = |part: f64, total: f64| if share { part / total * 100. } else { part };
    let installs = rows.iter().map(|r| r.1).sum::<f64>();
    let updates = rows.iter().map(|r| r.2).sum::<f64>();
    SeriesData {
        categories: rows.iter().map(|r| r.0).collect(),
        series: vec![
            ChartSeries::new("New installs", rows.iter().map(|r| series(r.1, r.1 + r.2))),
            ChartSeries::new("Updates", rows.iter().map(|r| series(r.2, r.1 + r.2))),
        ],
        kind: ChartKind::StackedBar,
        format: if share {
            |v| format!("{v:.1}%")
        } else {
            |v| format!("{v:.0} events")
        },
        summary: format!(
            "{installs:.0} new installs · {updates:.0} updates · Updates are {:.1}% of events",
            updates / (installs + updates) * 100.
        ),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[
        Filter {
            label: "Measure",
            options: &["Counts", "Share"],
        },
        Filter {
            label: "Period",
            options: &["All 6 releases", "Latest 3"],
        },
    ];
    Report::open(
        "Release adoption",
        "Release",
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
