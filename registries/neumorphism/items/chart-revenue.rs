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
        "Revenue against target",
        &["Show target", "Revenue only"],
        move |choice| {
            let target = choice == 0;
            let money = |cents: f64| format!("${:.2}", cents / 100.);
            Data {
                legend: if target {
                    vec!["Revenue", "Target"]
                } else {
                    vec!["Revenue"]
                },
                columns: &["Date", "Revenue", "Target"],
                rows: data
                    .points
                    .iter()
                    .map(|(date, value)| {
                        let revenue = value.map(|t| t.revenue_cents as f64);
                        let goal = value.map(|t| t.target_cents as f64);
                        Row {
                            label: date.to_string(),
                            values: if target {
                                vec![revenue, goal]
                            } else {
                                vec![revenue]
                            },
                            cells: vec![
                                revenue.map(money).unwrap_or_else(|| "—".into()),
                                goal.map(money).unwrap_or_else(|| "—".into()),
                            ],
                        }
                    })
                    .collect(),
                plot: Plot::Revenue,
                domain: None,
                reference: None,
                format: money,
                summary: format!(
                    "Revenue {} · Target {}",
                    money(data.totals.revenue_cents as f64),
                    money(data.totals.target_cents as f64)
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
