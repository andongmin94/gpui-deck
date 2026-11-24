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
        "Acquisition mix",
        &["Visits", "Converted visits"],
        move |choice| {
            let converted = choice == 1;
            let label = if converted {
                "Converted visits"
            } else {
                "Visits"
            };
            Data {
                legend: vec![label],
                columns: &["Channel", "Visits", "Converted", "Rate"],
                rows: data
                    .channels
                    .iter()
                    .map(|(name, t)| Row {
                        label: name.clone(),
                        values: vec![Some(
                            (if converted {
                                t.converted_visits
                            } else {
                                t.visits
                            }) as f64,
                        )],
                        cells: vec![
                            t.visits.to_string(),
                            t.converted_visits.to_string(),
                            t.conversion()
                                .map(|r| format!("{:.1}%", r * 100.))
                                .unwrap_or_else(|| "—".into()),
                        ],
                    })
                    .collect(),
                plot: Plot::Bars,
                domain: None,
                reference: None,
                format: |v| format!("{v:.0}"),
                summary: format!(
                    "{label}: {}",
                    if converted {
                        data.totals.converted_visits
                    } else {
                        data.totals.visits
                    }
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
