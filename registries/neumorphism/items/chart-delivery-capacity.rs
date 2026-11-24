mod palette;
mod report;
mod style;
use super::Palette;
use gpui::{AnyView, App, Window};
use report::{Data, Plot, Report, Row};

#[derive(Clone, Debug)]
pub(crate) struct Sprint {
    pub(crate) label: String,
    pub(crate) planned: u32,
    pub(crate) delivered: u32,
}
/// Render the caller's data with the same scale, inspection values and table as the preview.
pub(crate) fn open(points: Vec<Sprint>, p: Palette, cx: &mut App) -> AnyView {
    Report::open(
        "Delivery capacity",
        &["Compare", "Variance"],
        move |choice| {
            let variance = choice == 1;
            let planned = points.iter().map(|r| i64::from(r.planned)).sum::<i64>();
            let delivered = points.iter().map(|r| i64::from(r.delivered)).sum::<i64>();
            Data {
                legend: if variance {
                    vec!["Difference"]
                } else {
                    vec!["Planned", "Delivered"]
                },
                columns: &["Sprint", "Planned", "Delivered", "Difference"],
                rows: points
                    .iter()
                    .map(|r| {
                        let diff = i64::from(r.delivered) - i64::from(r.planned);
                        Row {
                            label: r.label.clone(),
                            values: if variance {
                                vec![Some(diff as f64)]
                            } else {
                                vec![Some(f64::from(r.planned)), Some(f64::from(r.delivered))]
                            },
                            cells: vec![
                                r.planned.to_string(),
                                r.delivered.to_string(),
                                format!("{diff:+}"),
                            ],
                        }
                    })
                    .collect(),
                plot: Plot::Bars,
                domain: None,
                reference: None,
                format: |v| format!("{v:.0}"),
                summary: format!(
                    "Planned {planned} · Delivered {delivered} · Net variance {:+}",
                    delivered - planned
                ),
            }
        },
        p,
        cx,
    )
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let data = [
        ("S1", 32, 28),
        ("S2", 36, 39),
        ("S3", 34, 31),
        ("S4", 38, 42),
    ]
    .into_iter()
    .map(|(label, planned, delivered)| Sprint {
        label: label.into(),
        planned,
        delivered,
    })
    .collect();
    open(data, palette::air(), cx)
}

#[cfg(test)]
mod tests {
    use super::report::{Data, Plot, Row};

    #[test]
    fn integral_counts_have_exact_ticks_and_zero_on_a_division() {
        let range = |values: &[f64]| {
            Data {
                legend: vec!["Difference"],
                columns: &[],
                plot: Plot::Bars,
                rows: values
                    .iter()
                    .map(|&v| Row {
                        label: String::new(),
                        values: vec![Some(v)],
                        cells: Vec::new(),
                    })
                    .collect(),
                domain: None,
                reference: None,
                format: |v| v.to_string(),
                summary: String::new(),
            }
            .range()
        };
        assert_eq!(range(&[28., 39., 31., 42.]), (0., 50., 5));
        assert_eq!(range(&[-4., 3., -3., 4.]), (-4., 4., 4));
        assert_eq!(range(&[0., 1.]), (0., 1., 1));
    }
}
