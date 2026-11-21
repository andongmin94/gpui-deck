//! Installable billing workspace. Replace the sample ledger with application data.

mod palette;
#[path = "components/panel.rs"]
mod panel;

use gpui::{
    AppContext as _, Context, Entity, InteractiveElement as _, IntoElement, ParentElement as _,
    Render, StatefulInteractiveElement as _, Styled as _, Subscription, Window, div, px,
};
use gpui_deck::{ChartSeries, ChartState, Direction};

use super::recipes::{
    ChartKind, ChartRecipe, DataTable, DataTableCell, DataTableColumn, DataTableRow, DataTableState,
};
use palette::billing_palette;
use panel::{heading, panel};

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
// Invoice number, customer, billing month (zero-based), amount in USD, payment status.
const LEDGER: &[(&str, &str, usize, u32, &str)] = &[
    ("INV-1068", "Forma Studio", 5, 3_200, "Paid"),
    ("INV-1062", "Studio North", 5, 2_850, "Open"),
    ("INV-1054", "Orbit Labs", 4, 2_450, "Paid"),
    ("INV-1047", "Fieldwork", 3, 1_800, "Open"),
    ("INV-1041", "Signal Works", 3, 2_100, "Paid"),
    ("INV-1032", "Northline", 2, 1_700, "Paid"),
    ("INV-1023", "Plain Goods", 1, 1_350, "Paid"),
    ("INV-1010", "Tidal Studio", 0, 950, "Paid"),
];

fn dollars(value: f64) -> String {
    format!("${value:.2}")
}

pub(crate) struct BillingDashboard {
    chart: Entity<ChartState>,
    invoices: Entity<DataTableState>,
    _observations: [Subscription; 2],
}

impl BillingDashboard {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut billed = [0.; MONTHS.len()];
        let rows = LEDGER
            .iter()
            .map(|&(id, customer, month, amount, status)| {
                billed[month] += f64::from(amount);
                DataTableRow::new(
                    id,
                    [
                        DataTableCell::text(id),
                        DataTableCell::text(customer),
                        DataTableCell::text(status),
                        DataTableCell::number(dollars(amount.into()), amount.into()),
                    ],
                )
            })
            .collect::<Vec<_>>();
        let chart = cx.new(|cx| ChartState::new(MONTHS, [ChartSeries::new("Billed", billed)], cx));
        let invoices = cx.new(|cx| {
            DataTableState::new(
                [
                    DataTableColumn::new("Invoice", px(100.)),
                    DataTableColumn::new("Customer", px(140.)),
                    DataTableColumn::new("Status", px(80.)),
                    DataTableColumn::new("Amount", px(100.)).align_end(),
                ],
                rows,
                5,
                window,
                cx,
            )
        });
        let observations = [
            cx.observe(&chart, |_, _, cx| cx.notify()),
            cx.observe(&invoices, |_, _, cx| cx.notify()),
        ];
        Self {
            chart,
            invoices,
            _observations: observations,
        }
    }
}

impl Render for BillingDashboard {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let palette = billing_palette();
        let chart = self.chart.clone();
        let register = DataTable::new("invoices", &self.invoices, palette)
            .accessibility_label("Invoices")
            .action_label("Show month")
            .on_action(move |id, window, cx| {
                if let Some(&(_, _, month, _, _)) = LEDGER.iter().find(|row| row.0 == id.as_ref()) {
                    chart.update(cx, |chart, cx| chart.inspect(month, window, cx));
                }
            });
        let chart = ChartRecipe::new(
            "monthly-billings",
            &self.chart,
            ChartKind::Bar,
            Direction::Ltr,
            "Monthly billings in US dollars",
            "Monthly billing totals",
            "Month",
        )
        .format_value(dollars)
        .series_colors([palette.accent]);
        let panels = div().flex().gap(palette.gap).w_full();
        let panels = if window.viewport_size().width >= px(1100.) {
            panels.flex_row().items_start()
        } else {
            panels.flex_col()
        };
        div()
            .id("billing-dashboard")
            .size_full()
            .overflow_y_scroll()
            .bg(palette.canvas)
            .text_color(palette.foreground)
            .text_size(palette.font_size)
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .p(palette.padding)
                    .gap(palette.gap)
                    .child(heading("billing-title", "Billing dashboard", 1))
                    .child(
                        panels
                            .child(
                                panel("billings-title", "Monthly billings", palette).child(chart),
                            )
                            .child(
                                panel("invoices-title", "Invoice register", palette)
                                    .child(register),
                            ),
                    ),
            )
    }
}
