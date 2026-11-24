mod palette;
mod report;
mod style;
use super::Palette;
use gpui::{AnyView, App, Window};
use report::{Data, Plot, Report, Row};

#[derive(Clone, Debug)]
pub(crate) struct Release {
    pub(crate) label: String,
    pub(crate) installs: u32,
    pub(crate) updates: u32,
}
/// Render the caller's data with the same scale, inspection values and table as the preview.
pub(crate) fn open(points: Vec<Release>, p: Palette, cx: &mut App) -> AnyView {
    Report::open(
        "Release activity",
        &["Count", "Share"],
        move |choice| {
            let share = choice == 1;
            let total = points
                .iter()
                .map(|r| u64::from(r.installs) + u64::from(r.updates))
                .sum::<u64>();
            let updates = points.iter().map(|r| u64::from(r.updates)).sum::<u64>();
            Data {
                legend: vec!["Installs", "Updates"],
                columns: &["Release", "Installs", "Updates", "Total"],
                rows: points
                    .iter()
                    .map(|r| {
                        let total = u64::from(r.installs) + u64::from(r.updates);
                        let value = |x: u32| {
                            if share {
                                (total != 0).then(|| f64::from(x) / total as f64 * 100.)
                            } else {
                                Some(f64::from(x))
                            }
                        };
                        Row {
                            label: r.label.clone(),
                            values: vec![value(r.installs), value(r.updates)],
                            cells: vec![
                                r.installs.to_string(),
                                r.updates.to_string(),
                                total.to_string(),
                            ],
                        }
                    })
                    .collect(),
                plot: Plot::StackedArea,
                domain: share.then_some((0., 100.)),
                reference: None,
                format: if share {
                    |v| format!("{v:.1}%")
                } else {
                    |v| format!("{v:.0}")
                },
                summary: if total == 0 {
                    "No release activity.".into()
                } else {
                    format!(
                        "{total} events · Updates {:.1}%",
                        updates as f64 / total as f64 * 100.
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
        ("R1", 148, 62),
        ("R2", 232, 104),
        ("R3", 196, 88),
        ("R4", 284, 132),
        ("R5", 341, 176),
        ("R6", 378, 209),
    ]
    .into_iter()
    .map(|(label, installs, updates)| Release {
        label: label.into(),
        installs,
        updates,
    })
    .collect();
    open(data, palette::air(), cx)
}
