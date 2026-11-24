mod palette;
mod report;
mod style;
use super::Palette;
use gpui::{AnyView, App, Window};
use report::{Data, Plot, Report, Row};

#[derive(Clone, Debug)]
pub(crate) struct InstallStage {
    pub(crate) name: String,
    pub(crate) duration_ms: u32,
}
pub(crate) fn open(points: Vec<InstallStage>, p: Palette, cx: &mut App) -> AnyView {
    Report::open(
        "Install diagnostics",
        &["Milliseconds", "Seconds"],
        move |choice| {
            let format: fn(f64) -> String = if choice == 0 {
                |v| format!("{v:.0} ms")
            } else {
                |v| format!("{:.2} s", v / 1000.)
            };
            let total = points.iter().map(|r| u64::from(r.duration_ms)).sum::<u64>();
            let largest = points.iter().max_by_key(|r| r.duration_ms);
            Data {
                legend: vec!["Duration"],
                columns: &["Stage", "Duration", "Share"],
                rows: points
                    .iter()
                    .map(|r| Row {
                        label: r.name.clone(),
                        values: vec![Some(f64::from(r.duration_ms))],
                        cells: vec![
                            format(f64::from(r.duration_ms)),
                            if total == 0 {
                                "—".into()
                            } else {
                                format!("{:.1}%", f64::from(r.duration_ms) / total as f64 * 100.)
                            },
                        ],
                    })
                    .collect(),
                plot: Plot::Horizontal,
                domain: None,
                reference: None,
                format,
                summary: largest.map_or_else(
                    || "No stages recorded.".into(),
                    |r| {
                        format!(
                            "Total {} · Longest: {} ({})",
                            format(total as f64),
                            r.name,
                            format(f64::from(r.duration_ms))
                        )
                    },
                ),
            }
        },
        p,
        cx,
    )
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let data = [
        ("Resolve", 120),
        ("Download", 840),
        ("Transform", 210),
        ("Write", 90),
    ]
    .into_iter()
    .map(|(name, duration_ms)| InstallStage {
        name: name.into(),
        duration_ms,
    })
    .collect();
    open(data, palette::air(), cx)
}
