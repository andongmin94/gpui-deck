#[allow(dead_code)]
mod analytics;
mod palette;
mod style;
use super::Palette;
pub(crate) use analytics::{Record, Summary, Totals, demo, summarize};
use gpui::{prelude::*, *};

pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Model {
        records: Vec<Record>,
        days: usize,
        csv: bool,
    }
    impl Render for Model {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            let summary: Summary =
                summarize(&self.records, self.days, None).expect("validated sample");
            let t: Totals = summary.totals;
            let periods = div().flex().gap_2().children([7, 14, 30].map(|days| {
                style::action(("days", days), format!("{days} days"), p)
                    .selected(days == self.days)
                    .when(days == self.days, |s| {
                        s.shadow(palette::inset(p)).bg(p.highlight)
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.days = days;
                        cx.notify();
                    }))
            }));
            let mut card = style::surface(p)
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("Analytics model"),
                )
                .child(periods)
                .child(format!(
                    "{} visits · {} converted · ${:.2} revenue",
                    t.visits,
                    t.converted_visits,
                    t.revenue_cents as f64 / 100.
                ))
                .child(format!(
                    "{} to {}",
                    summary.points[0].0,
                    summary.points.last().unwrap().0
                ))
                .child(
                    style::action("csv", if self.csv { "Hide CSV" } else { "View CSV" }, p)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.csv = !this.csv;
                            cx.notify();
                        })),
                );
            if self.csv {
                card = card.child(palette::well(p).p_3().text_xs().child(summary.csv()));
            }
            style::stage(p, card)
        }
    }
    cx.new(|_| Model {
        records: demo(),
        days: 14,
        csv: false,
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::{Record, Totals, demo, summarize};

    fn row(date: &str, channel: &str, visits: u64, converted_visits: u64) -> Record {
        Record {
            date: date.into(),
            channel: channel.into(),
            totals: Totals {
                visits,
                converted_visits,
                revenue_cents: 12345,
                target_cents: 15000,
            },
        }
    }

    #[test]
    fn date_window_preserves_zeroes_gaps_and_weighted_conversion() {
        let records = [
            row("2024-02-28", "Direct", 10, 5),
            row("2024-02-28", "Organic", 990, 10),
            row("2024-03-01", "Organic", 0, 0),
        ];
        let report = summarize(&records, 3, None).unwrap();
        assert_eq!(report.points[1].0.to_string(), "2024-02-29");
        assert_eq!(report.points[1].1, None);
        assert_eq!(report.points[2].1.unwrap().conversion(), None);
        assert_eq!(report.totals.conversion(), Some(0.015));
        assert!(
            report
                .csv()
                .contains("2024-02-29,,,,,\r\n2024-03-01,0,0,12345,15000,\r\n")
        );
        let filtered = summarize(&records, 3, Some("Direct")).unwrap();
        assert_eq!(filtered.totals.visits, 10);
        assert_eq!(filtered.points[2].1, None);
    }

    #[test]
    fn invalid_records_and_aggregation_overflow_are_rejected() {
        let original = row("2026-09-20", "Direct", 2, 1);
        assert!(summarize(&[original.clone(), original], 1, None).is_err());
        for record in [
            row("2026-02-30", "Direct", 2, 1),
            row("2026-09-01", " ", 2, 1),
            row("2026-09-01", "Direct", 1, 2),
        ] {
            assert!(summarize(&[record], 1, None).is_err());
        }
        assert!(
            summarize(
                &[
                    row("2026-09-01", "Direct", u64::MAX, 1),
                    row("2026-09-01", "Organic", 1, 0)
                ],
                1,
                None
            )
            .is_err()
        );
    }

    #[test]
    fn published_demo_totals_match_the_registry_example() {
        let report = summarize(&demo(), 14, None).unwrap();
        assert_eq!(
            report.totals,
            Totals {
                visits: 23778,
                converted_visits: 1210,
                revenue_cents: 6064400,
                target_cents: 7581000
            }
        );
        assert_eq!(report.points.len(), 14);
    }
}
