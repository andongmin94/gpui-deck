//! A small billing app. The invoice register drives both the totals and the
//! chart; selecting open invoices and marking them paid updates all three.

use gpui::{
    AnyView, App, AppContext as _, Context, Div, Entity, FontWeight, Hsla, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, Role, StatefulInteractiveElement as _, Styled as _,
    Subscription, Window, div, prelude::FluentBuilder as _, px, rgb,
};
use gpui_design::{
    Button, Chart, ChartSeries, ChartState, Checkbox, CheckboxState, Table, TableCell, TableColumn,
    TableRow,
};

use crate::theme::Theme;

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];

struct Invoice {
    id: &'static str,
    customer: &'static str,
    amount: u32,
    month: usize,
    paid: bool,
    selected: bool,
}

fn invoices() -> Vec<Invoice> {
    [
        ("INV-1028", "Studio North", 2850, 6, false),
        ("INV-1027", "Forma Studio", 3200, 6, true),
        ("INV-1026", "Orbit Labs", 2450, 5, true),
        ("INV-1025", "Fieldwork", 1800, 4, false),
        ("INV-1024", "Signal Works", 2100, 4, true),
        ("INV-1023", "Northline", 1700, 3, true),
        ("INV-1022", "Plain Goods", 1350, 2, true),
        ("INV-1021", "Tidal Studio", 950, 1, true),
    ]
    .into_iter()
    .map(|(id, customer, amount, month, paid)| Invoice {
        id,
        customer,
        amount,
        month,
        paid,
        selected: false,
    })
    .collect()
}

fn collections(invoices: &[Invoice]) -> [u32; MONTHS.len()] {
    let mut months = [0; MONTHS.len()];
    for invoice in invoices.iter().filter(|invoice| invoice.paid) {
        months[invoice.month - 1] += invoice.amount;
    }
    months
}

fn dollars(amount: u32) -> String {
    if amount < 1000 {
        format!("${amount}")
    } else {
        format!("${},{:03}", amount / 1000, amount % 1000)
    }
}

fn chart_state(
    invoices: &[Invoice],
    month: usize,
    window: &mut Window,
    cx: &mut Context<ChartState>,
) -> ChartState {
    let values = collections(invoices).map(f64::from);
    let mut chart = ChartState::new(MONTHS, [ChartSeries::new("Collected", values)], cx);
    chart.inspect(month, window, cx);
    chart
}

struct InvoiceWorkspace {
    theme: Theme,
    invoices: Vec<Invoice>,
    chart: Entity<ChartState>,
    _observe: Subscription,
}

pub(super) fn open(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let invoices = invoices();
        let chart = cx.new(|cx| chart_state(&invoices, MONTHS.len() - 1, window, cx));
        let observe = cx.observe(&chart, |_, _, cx| cx.notify());
        InvoiceWorkspace {
            theme,
            invoices,
            chart,
            _observe: observe,
        }
    })
    .into()
}

fn heading(id: &'static str, label: &'static str, level: usize, size: f32) -> impl IntoElement {
    div()
        .id(id)
        .role(Role::Heading)
        .aria_level(level)
        .aria_label(label)
        .child(label)
        .text_size(px(size))
        .font_weight(FontWeight::SEMIBOLD)
}

fn card(theme: Theme) -> Div {
    div()
        .flex()
        .flex_col()
        .min_w_0()
        .rounded(px(12.))
        .bg(theme.background)
        .border_1()
        .border_color(theme.border)
}

impl InvoiceWorkspace {
    fn accent(&self) -> Hsla {
        if self.theme.foreground.l > 0.5 {
            rgb(0x75d7b7).into()
        } else {
            rgb(0x15765d).into()
        }
    }

    fn totals(&self, compact: bool) -> Div {
        let invoiced = self.invoices.iter().map(|row| row.amount).sum::<u32>();
        let collected = collections(&self.invoices).iter().sum::<u32>();
        let accent = self.accent();
        let theme = self.theme;
        let amounts = [
            ("Total invoiced", invoiced),
            ("Collected", collected),
            ("Outstanding", invoiced - collected),
        ];
        card(theme)
            .flex_row()
            .children(
                amounts
                    .into_iter()
                    .enumerate()
                    .map(|(index, (label, amount))| {
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap(px(4.))
                            .px(px(if compact { 10. } else { 22. }))
                            .py(px(if compact { 12. } else { 16. }))
                            .when(index != 0, |cell| {
                                cell.border_l_1().border_color(theme.border)
                            })
                            .child(
                                div()
                                    .text_size(px(if compact { 11. } else { 13. }))
                                    .text_color(theme.muted_foreground)
                                    .child(label),
                            )
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_size(px(if compact { 20. } else { 28. }))
                                    .when(index == 1, |amount| amount.text_color(accent))
                                    .child(dollars(amount)),
                            )
                    }),
            )
    }

    fn chart_panel(&self, window: &Window, cx: &Context<Self>) -> Div {
        let theme = self.theme;
        let accent = self.accent();
        let chart = self.chart.read(cx);
        let inspected = chart.inspected_index().unwrap_or(0);
        let values = chart.series()[0].values();
        let ceiling = (chart.range().1 / 1000.).ceil() * 1000.;
        let plot_height = 144.;
        let description = chart.inspected_description_with(|amount| dollars(amount as u32));
        let axis_width = px(45.);
        let plot = Chart::new(
            "collections-plot",
            &self.chart,
            "Monthly collections in US dollars",
        )
        .value_description(description)
        .relative()
        .w_full()
        .h(px(plot_height))
        .when(chart.focus_handle().is_focused(window), |plot| {
            plot.border_1().border_color(accent)
        })
        .children([0., 0.5, 1.].map(|tick| {
            div()
                .absolute()
                .left_0()
                .right_0()
                .bottom(px(tick * plot_height))
                .h(px(1.))
                .bg(theme.border)
        }))
        .child(div().relative().size_full().flex().items_end().children(
            values.iter().enumerate().map(|(index, &value)| {
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .items_end()
                    .justify_center()
                    .child(
                        div()
                            .w(gpui::relative(0.65))
                            .max_w(px(46.))
                            .h(px(plot_height * (value / ceiling) as f32))
                            .rounded_t(px(4.))
                            .bg(if index == inspected {
                                accent
                            } else {
                                accent.opacity(0.48)
                            }),
                    )
            }),
        ));
        card(theme)
            .p(px(16.))
            .gap(px(12.))
            .child(heading("collections-title", "Collections", 2, 18.))
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child("Paid invoices by issue month"),
            )
            .child(
                div()
                    .mt(px(12.))
                    .flex()
                    .items_start()
                    .child(
                        div()
                            .w(axis_width)
                            .h(px(plot_height))
                            .flex_shrink_0()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .text_size(px(10.))
                            .text_color(theme.muted_foreground)
                            .children(
                                [ceiling, ceiling / 2., 0.]
                                    .map(|amount| div().child(dollars(amount as u32))),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap(px(6.))
                            .child(plot)
                            .child(div().flex().w_full().children(MONTHS.map(|month| {
                                div()
                                    .flex_1()
                                    .text_center()
                                    .text_size(px(11.))
                                    .text_color(theme.muted_foreground)
                                    .child(month)
                            }))),
                    ),
            )
            .child(
                div()
                    .mt(px(8.))
                    .pt(px(12.))
                    .border_t_1()
                    .border_color(theme.border)
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .text_color(theme.muted_foreground)
                            .child(format!("{} 2026", MONTHS[inspected])),
                    )
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(dollars(values[inspected] as u32)),
                    ),
            )
    }

    fn mark_paid(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for row in &mut self.invoices {
            if row.selected {
                row.paid = true;
                row.selected = false;
            }
        }
        self.chart.update(cx, |chart, cx| {
            let inspected = chart.inspected_index().unwrap_or(0);
            *chart = chart_state(&self.invoices, inspected, window, cx);
            cx.notify();
        });
        cx.notify();
    }

    fn invoice_panel(&self, compact: bool, cx: &mut Context<Self>) -> Div {
        let theme = self.theme;
        let accent = self.accent();
        let selected = self
            .invoices
            .iter()
            .filter(|invoice| invoice.selected)
            .count();
        let columns = [
            TableColumn::fixed(px(26.)),
            TableColumn::flex(1.),
            TableColumn::fixed(px(if compact { 48. } else { 68. })),
            TableColumn::fixed(px(if compact { 60. } else { 74. })),
        ];
        let header = TableRow::new("invoice-headings")
            .text_size(px(11.))
            .text_color(theme.muted_foreground)
            .cells(
                ["", "Customer", "Status", "Amount"]
                    .into_iter()
                    .enumerate()
                    .map(|(index, label)| {
                        let cell = TableCell::new(("invoice-heading", index))
                            .accessibility_label(if index == 0 { "Selection" } else { label })
                            .py(px(6.));
                        if index == 3 { cell.text_right() } else { cell }.child(label)
                    }),
            );
        let rows = self.invoices.iter().enumerate().map(|(index, row)| {
            let status = if row.paid { "Paid" } else { "Open" };
            let owner = cx.entity();
            let checkbox = Checkbox::new(("invoice-check", index))
                .accessibility_label(format!("Select {}", row.id))
                .checked(row.selected)
                .disabled(row.paid)
                .size(px(16.))
                .rounded(px(4.))
                .flex()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(theme.ring)
                .focus_visible(|style| style.border_2().border_color(accent))
                .when(row.selected, |check| {
                    check.bg(accent).text_color(theme.background).child("✓")
                })
                .when(row.paid, |check| check.opacity(0.3))
                .on_change(move |state, _, _, cx| {
                    owner.update(cx, |this, cx| {
                        this.invoices[index].selected = state == CheckboxState::Checked;
                        cx.notify();
                    });
                });
            let row_view = TableRow::new(("invoice", index))
                .selected(row.selected)
                .border_t_1()
                .border_color(theme.border);
            if row.selected {
                row_view.bg(accent.opacity(0.08))
            } else {
                row_view
            }
            .cell(
                TableCell::new(("check-cell", index))
                    .flex()
                    .flex_col()
                    .justify_center()
                    .child(checkbox),
            )
            .cell(
                TableCell::new(("customer", index))
                    .row_header(true)
                    .py(px(5.))
                    .accessibility_label(format!(
                        "{}, {}, issued {} 2026",
                        row.customer,
                        row.id,
                        MONTHS[row.month - 1]
                    ))
                    .child(div().font_weight(FontWeight::MEDIUM).child(row.customer))
                    .child(
                        div()
                            .text_size(px(10.))
                            .text_color(theme.muted_foreground)
                            .child(format!("{} · {}", row.id, MONTHS[row.month - 1])),
                    ),
            )
            .cell(
                TableCell::new(("status", index))
                    .flex()
                    .flex_col()
                    .items_start()
                    .justify_center()
                    .accessibility_label(status)
                    .child(
                        div()
                            .rounded(px(8.))
                            .px(px(7.))
                            .py(px(1.))
                            .text_size(px(11.))
                            .bg(if row.paid {
                                accent.opacity(0.12)
                            } else {
                                theme.muted
                            })
                            .text_color(if row.paid { accent } else { theme.foreground })
                            .child(status),
                    ),
            )
            .cell(
                TableCell::new(("amount", index))
                    .flex()
                    .flex_col()
                    .justify_center()
                    .text_right()
                    .child(dollars(row.amount)),
            )
        });
        let register = Table::new("invoice-register", columns)
            .accessibility_label("Invoice register")
            .w_full()
            .header(header)
            .rows(rows);
        let action = if selected == 0 {
            "Mark paid".to_owned()
        } else {
            format!("Mark paid ({selected})")
        };
        card(theme)
            .p(px(if compact { 12. } else { 16. }))
            .gap(px(8.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(heading("invoice-title", "Invoices", 2, 18.))
                    .child(
                        Button::new("mark-paid")
                            .accessibility_label("Mark selected invoices paid")
                            .disabled(selected == 0)
                            .h(px(28.))
                            .px(px(10.))
                            .rounded(px(6.))
                            .flex()
                            .items_center()
                            .bg(accent)
                            .text_color(theme.background)
                            .text_size(px(12.))
                            .focus_visible(|style| style.border_2().border_color(theme.foreground))
                            .styles(|styles| {
                                styles.disabled(|style| {
                                    style.bg(theme.muted).text_color(theme.muted_foreground)
                                })
                            })
                            .child(action)
                            .on_click(
                                cx.listener(|this, _, window, cx| this.mark_paid(window, cx)),
                            ),
                    ),
            )
            .child(register)
    }
}

impl Render for InvoiceWorkspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let width = window.viewport_size().width;
        let stacked = width < px(820.);
        let compact = width < px(500.);
        let chart = self.chart_panel(window, cx);
        let register = self.invoice_panel(compact, cx);
        div()
            .id("billing-workspace")
            .size_full()
            .overflow_y_scroll()
            .bg(theme.muted)
            .text_color(theme.foreground)
            .text_size(px(13.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(if compact { 12. } else { 16. }))
                    .p(px(if compact { 10. } else { 22. }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .flex_wrap()
                            .gap_1()
                            .child(heading(
                                "billing-title",
                                "Billing overview",
                                1,
                                if compact { 22. } else { 26. },
                            ))
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .text_color(theme.muted_foreground)
                                    .child("Jan – Jun 2026"),
                            ),
                    )
                    .child(self.totals(compact))
                    .child(
                        div()
                            .flex()
                            .gap(px(if compact { 12. } else { 16. }))
                            .when(stacked, |layout| layout.flex_col())
                            .when(!stacked, |layout| layout.items_start())
                            .child(chart.when(!stacked, |chart| {
                                chart.w(gpui::relative(0.39)).flex_shrink_0()
                            }))
                            .child(register.when(!stacked, |table| table.flex_1())),
                    ),
            )
    }
}
