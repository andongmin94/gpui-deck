mod palette;
mod style;

use super::Palette;
use super::recipes::{DataTable, DataTableCell, DataTableColumn, DataTableRow, DataTableState};
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, Entity, SharedString, px};
pub(crate) fn data_table(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<DataTableState>,
    p: Palette,
) -> DataTable {
    DataTable::new(id, state, p).accessibility_label(label)
}

pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    struct Editions(Entity<DataTableState>);
    impl Render for Editions {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let p = palette::yellow();
            style::stage(
                p,
                style::surface(p).child(data_table("editions", "Print inventory", &self.0, p)),
            )
        }
    }
    cx.new(|cx| {
        let columns = [
            DataTableColumn::new("Edition", px(132.)),
            DataTableColumn::new("Stock", px(80.)).align_end(),
            DataTableColumn::new("Price", px(80.)).align_end(),
        ];
        let rows = [
            ("Night tide", 12, 24),
            ("Four suns", 0, 32),
            ("The grid", 26, 18),
            ("Stone fruit", 8, 19),
            ("Fire escape", 16, 28),
            ("Type study", 3, 22),
        ]
        .into_iter()
        .map(|(name, stock, price)| {
            DataTableRow::new(
                name,
                [
                    DataTableCell::text(name),
                    DataTableCell::number(stock.to_string(), f64::from(stock)),
                    DataTableCell::number(format!("${price}"), f64::from(price)),
                ],
            )
        });
        let table = cx.new(|cx| DataTableState::new(columns, rows, 4, window, cx));
        cx.observe(&table, |_, _, cx| cx.notify()).detach();
        Editions(table)
    })
    .into()
}
