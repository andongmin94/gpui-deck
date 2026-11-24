mod palette;
mod report;
mod style;
use super::Palette;
use gpui::{AnyView, App, Window};
use report::{Data, Plot, Report, Row};

#[derive(Clone, Debug)]
pub(crate) struct LatencyPoint {
    pub(crate) label: String,
    pub(crate) p50_ms: u32,
    pub(crate) p95_ms: u32,
}
/// Render the caller's data with the same scale, inspection values and table as the preview.
pub(crate) fn open(points: Vec<LatencyPoint>, budget_ms: u32, p: Palette, cx: &mut App) -> AnyView {
    Report::open(
        "Service latency",
        &[],
        move |_| {
            let breaches = points.iter().filter(|r| r.p95_ms > budget_ms).count();
            let worst = points.iter().map(|r| r.p95_ms).max();
            Data {
                legend: vec!["p50", "p95"],
                columns: &["Day", "p50 (ms)", "p95 (ms)", "Budget"],
                rows: points
                    .iter()
                    .map(|r| Row {
                        label: r.label.clone(),
                        values: vec![Some(f64::from(r.p50_ms)), Some(f64::from(r.p95_ms))],
                        cells: vec![
                            r.p50_ms.to_string(),
                            r.p95_ms.to_string(),
                            if r.p95_ms > budget_ms {
                                "Over".into()
                            } else {
                                "Within".into()
                            },
                        ],
                    })
                    .collect(),
                plot: Plot::Line,
                domain: None,
                reference: Some((f64::from(budget_ms), "Budget")),
                format: |v| format!("{v:.0} ms"),
                summary: worst.map_or_else(
                    || "No observations.".into(),
                    |v| format!("Peak p95 {v} ms · {breaches}/{} over budget", points.len()),
                ),
            }
        },
        p,
        cx,
    )
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let data = [
        ("Mon", 80, 210),
        ("Tue", 76, 195),
        ("Wed", 95, 340),
        ("Thu", 130, 470),
        ("Fri", 100, 330),
        ("Sat", 82, 240),
        ("Sun", 79, 215),
    ]
    .into_iter()
    .map(|(label, p50_ms, p95_ms)| LatencyPoint {
        label: label.into(),
        p50_ms,
        p95_ms,
    })
    .collect();
    open(data, 300, palette::air(), cx)
}
