mod palette;
mod report;
mod style;
use super::Palette;
use gpui::{AnyView, App, Window};
use report::{Data, Plot, Report, Row};
#[allow(dead_code)]
pub(crate) mod analytics;
use analytics::Summary;
pub(crate) fn open(data: Summary, p: Palette, cx: &mut App) -> AnyView {
    Report::open(
        "Visit conversion",
        &[],
        move |_| {
            let percent = |v: f64| format!("{:.1}%", v * 100.);
            Data {
                legend: vec!["Conversion rate"],
                columns: &["Date", "Visits", "Converted", "Rate"],
                rows: data
                    .points
                    .iter()
                    .map(|(name, v)| {
                        let rate = v.and_then(analytics::Totals::conversion);
                        Row {
                            label: name.to_string(),
                            values: vec![rate],
                            cells: vec![
                                v.map(|t| t.visits.to_string())
                                    .unwrap_or_else(|| "—".into()),
                                v.map(|t| t.converted_visits.to_string())
                                    .unwrap_or_else(|| "—".into()),
                                rate.map(percent).unwrap_or_else(|| "—".into()),
                            ],
                        }
                    })
                    .collect(),
                plot: Plot::Line,
                domain: Some((0., 1.)),
                reference: None,
                format: percent,
                summary: format!(
                    "{} · {} converted visits / {} visits",
                    data.totals
                        .conversion()
                        .map(percent)
                        .unwrap_or_else(|| "—".into()),
                    data.totals.converted_visits,
                    data.totals.visits
                ),
            }
        },
        p,
        cx,
    )
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    open(
        analytics::summarize(&analytics::demo(), 14, None).expect("valid example records"),
        palette::air(),
        cx,
    )
}
