use std::collections::HashSet;

use gpui::{
    AnyView, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render,
    RenderOnce, SharedString, Styled as _, Subscription, Window, px,
};
use gpui_deck::Button;

use super::{data::style_editor, frame};
use crate::recipes::{
    Badge, ButtonVariant, Card, DataTable, DataTableCell, DataTableColumn, DataTableRow,
    DataTableState,
};
use crate::theme::Theme;

pub(super) fn card(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|_| DigestCard {
        theme,
        subscribed: false,
    })
    .into()
}

struct DigestCard {
    theme: Theme,
    subscribed: bool,
}

impl Render for DigestCard {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let palette = theme.recipe_palette();
        let action = if self.subscribed {
            "Unsubscribe"
        } else {
            "Subscribe"
        };
        frame(theme)
            .bg(palette.canvas)
            .text_color(palette.foreground)
            .child(
                Card::new("digest", "Weekly digest", palette)
                    .max_w(px(420.))
                    .description("Project notifications")
                    .header_action(
                        Badge::new(
                            "status",
                            if self.subscribed { "On" } else { "Off" },
                            palette,
                        )
                        .accent(self.subscribed),
                    )
                    .content("A Friday summary of new releases and changes to your projects.")
                    .footer(
                        ButtonVariant::Primary
                            .apply(
                                Button::new("subscription").accessibility_label(action),
                                palette,
                            )
                            .child(action)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.subscribed = !this.subscribed;
                                cx.notify();
                            })),
                    ),
            )
    }
}

struct Invoices {
    theme: Theme,
    paid: HashSet<SharedString>,
    table: Entity<DataTableState>,
    _observe: Subscription,
}

#[derive(IntoElement)]
struct InvoiceStatus {
    id: SharedString,
    text: SharedString,
    theme: Theme,
}

impl RenderOnce for InvoiceStatus {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let paid = self.text.as_ref() == "Paid";
        Badge::new(self.id, self.text, self.theme.resolve(cx).recipe_palette()).accent(paid)
    }
}

fn invoice_rows(paid: &HashSet<SharedString>) -> Vec<DataTableRow> {
    [
        ("INV-101", 120),
        ("INV-102", 35),
        ("INV-103", 275),
        ("INV-104", 90),
    ]
    .into_iter()
    .map(|(id, amount)| {
        let paid = paid.contains(id);
        DataTableRow::new(
            id,
            [
                DataTableCell::text(id),
                DataTableCell::text(if paid { "Paid" } else { "Open" }),
                DataTableCell::number(format!("${amount}"), f64::from(amount)),
            ],
        )
        .disabled(paid)
    })
    .collect()
}

pub(super) fn data_table(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let paid = HashSet::new();
        let columns = [
            DataTableColumn::new("Invoice", px(100.)),
            DataTableColumn::new("Status", px(88.)).render_cell(move |row, text| InvoiceStatus {
                id: row.id().clone(),
                text: text.clone(),
                theme,
            }),
            DataTableColumn::new("Amount", px(98.)).align_end(),
        ];
        let table = cx.new(|cx| DataTableState::new(columns, invoice_rows(&paid), 2, window, cx));
        table
            .read(cx)
            .query_input()
            .clone()
            .update(cx, |input, cx| style_editor(input, theme, cx));
        let observe = cx.observe(&table, |_, _, cx| cx.notify());
        Invoices {
            theme,
            paid,
            table,
            _observe: observe,
        }
    })
    .into()
}

impl Render for Invoices {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let palette = theme.recipe_palette();
        let next_size = if self.table.read(cx).page_size() == 2 {
            4
        } else {
            2
        };
        let resize_label = format!("Show {next_size} rows per page");
        let owner = cx.entity();
        frame(theme)
            .bg(palette.canvas)
            .text_color(palette.foreground)
            .flex_col()
            .items_start()
            .child(
                ButtonVariant::Outline
                    .apply(
                        Button::new("page-size").accessibility_label(resize_label.clone()),
                        palette,
                    )
                    .child(resize_label)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.table
                            .update(cx, |table, cx| table.set_page_size(next_size, cx));
                    })),
            )
            .child(
                DataTable::new("invoices", &self.table, palette)
                    .accessibility_label("Invoices")
                    .action_label("Mark paid")
                    .on_action(move |id, _, cx| {
                        owner.update(cx, |this, cx| {
                            this.paid.insert(id);
                            this.table.update(cx, |table, cx| {
                                table.set_rows(invoice_rows(&this.paid), cx)
                            });
                        });
                    }),
            )
    }
}
