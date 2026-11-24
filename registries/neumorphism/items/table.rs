mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, SharedString, div};
use gpui_deck::{Table, TableCell, TableColumn, TableRow};

/// Set each column's width or flex weight, then add header, body and footer rows.
pub(crate) fn table(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    columns: impl IntoIterator<Item = TableColumn>,
    p: Palette,
) -> Table {
    Table::new(id, columns)
        .accessibility_label(label)
        .w_full()
        .border_1()
        .border_color(p.border)
        .rounded(p.radius)
        .bg(p.surface)
        .shadow(palette::raised(p))
}
pub(crate) fn row(
    id: impl Into<ElementId>,
    values: impl IntoIterator<Item = impl Into<SharedString>>,
    header: bool,
    p: Palette,
) -> TableRow {
    let id = id.into();
    let row = TableRow::new(id.clone());
    let row = if header {
        row.bg(p.field).text_color(p.foreground)
    } else {
        row
    };
    row.cells(values.into_iter().enumerate().map(|(i, value)| {
        let value = value.into();
        TableCell::new((id.clone(), format!("cell-{i}")))
            .px_2()
            .py_2()
            .accessibility_label(value.clone())
            .child(div().truncate().child(value))
    }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        table(
            "watchlist",
            "Market watchlist",
            [
                TableColumn::flex(1.),
                TableColumn::flex(1.),
                TableColumn::flex(1.),
                TableColumn::flex(1.),
            ],
            p,
        )
        .header(row(
            "columns",
            ["Symbol", "Price", "Change", "Status"],
            true,
            p,
        ))
        .row(row("tide", ["NVDA", "$181.92", "+2.41%", "Open"], false, p))
        .row(row(
            "sun",
            ["005930", "₩72,400", "+1.08%", "Selected"],
            false,
            p,
        ))
        .row(row(
            "grid",
            ["TSLA", "$318.27", "−0.64%", "Watch"],
            false,
            p,
        ))
        .into_any_element()
    })
}
