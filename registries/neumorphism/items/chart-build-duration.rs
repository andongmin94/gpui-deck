mod palette;
mod report;
mod style;
use super::Palette;
use gpui::{AnyView, App, Window};
use report::{Data, Plot, Report, Row};

#[derive(Clone, Debug)]
pub(crate) struct BuildRun {
    pub(crate) label: String,
    pub(crate) cold_seconds: u32,
    pub(crate) cached_seconds: u32,
}
/// Render the caller's data with the same scale, inspection values and table as the preview.
pub(crate) fn open(
    points: Vec<BuildRun>,
    budget_seconds: u32,
    p: Palette,
    cx: &mut App,
) -> AnyView {
    Report::open(
        "Build duration",
        &["Both", "Cold only", "Cached only"],
        move |choice| {
            let count = points.len();
            let cold = points
                .iter()
                .map(|r| f64::from(r.cold_seconds))
                .sum::<f64>();
            let cached = points
                .iter()
                .map(|r| f64::from(r.cached_seconds))
                .sum::<f64>();
            let breaches = points
                .iter()
                .filter(|r| r.cold_seconds > budget_seconds)
                .count();
            Data {
                legend: match choice {
                    1 => vec!["Cold build"],
                    2 => vec!["Cached build"],
                    _ => vec!["Cold build", "Cached build"],
                },
                columns: &["Build", "Cold (s)", "Cached (s)", "Saved (s)"],
                rows: points
                    .iter()
                    .map(|r| {
                        let cold = Some(f64::from(r.cold_seconds));
                        let cached = Some(f64::from(r.cached_seconds));
                        Row {
                            label: r.label.clone(),
                            values: match choice {
                                1 => vec![cold],
                                2 => vec![cached],
                                _ => vec![cold, cached],
                            },
                            cells: vec![
                                r.cold_seconds.to_string(),
                                r.cached_seconds.to_string(),
                                (i64::from(r.cold_seconds) - i64::from(r.cached_seconds))
                                    .to_string(),
                            ],
                        }
                    })
                    .collect(),
                plot: Plot::Line,
                domain: None,
                reference: Some((f64::from(budget_seconds), "Budget")),
                format: |v| format!("{v:.0} s"),
                summary: if count == 0 {
                    "No build runs.".into()
                } else {
                    format!(
                        "Mean cold {:.1} s · Cached {:.1} s · {breaches}/{count} cold runs over budget",
                        cold / count as f64,
                        cached / count as f64
                    )
                },
            }
        },
        p,
        cx,
    )
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let data = [
        ("B1", 210, 108),
        ("B2", 195, 102),
        ("B3", 238, 114),
        ("B4", 226, 109),
        ("B5", 204, 96),
        ("B6", 188, 93),
    ]
    .into_iter()
    .map(|(label, cold_seconds, cached_seconds)| BuildRun {
        label: label.into(),
        cold_seconds,
        cached_seconds,
    })
    .collect();
    open(data, 220, palette::air(), cx)
}
