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
    let rows = if choice[0] == 0 {
        [(32, 28), (36, 39), (34, 31), (38, 42)]
    } else {
        [(24, 26), (28, 23), (26, 29), (30, 30)]
    };
    let variance = choice[1] == 1;
    let planned = rows.iter().map(|r| r.0).sum::<i32>();
    let delivered = rows.iter().map(|r| r.1).sum::<i32>();
    SeriesData {
        categories: vec!["S1", "S2", "S3", "S4"],
        series: if variance {
            vec![ChartSeries::new(
                "Delivered minus planned",
                rows.map(|r| f64::from(r.1 - r.0)),
            )]
        } else {
            vec![
                ChartSeries::new("Planned", rows.map(|r| f64::from(r.0))),
                ChartSeries::new("Delivered", rows.map(|r| f64::from(r.1))),
            ]
        },
        kind: ChartKind::Bar,
        format: if variance {
            |v| format!("{v:+} items")
        } else {
            |v| format!("{v} items")
        },
        summary: format!(
            "Planned {planned} items · Delivered {delivered} · Variance {:+} items",
            delivered - planned
        ),
    }
}

/// Open the report with an application-owned palette.
pub(crate) fn open(p: Palette, window: &mut Window, cx: &mut App) -> AnyView {
    const FILTERS: &[Filter] = &[
        Filter {
            label: "Team",
            options: &["Product", "Platform"],
        },
        Filter {
            label: "Measure",
            options: &["Planned and delivered", "Signed variance"],
        },
    ];
    Report::open(
        "Delivery capacity",
        "Sprint",
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
