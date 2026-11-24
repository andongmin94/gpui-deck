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
    struct Projects(Entity<DataTableState>);
    impl Render for Projects {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            style::stage(
                p,
                style::surface(p).child(data_table("projects", "Workspace projects", &self.0, p)),
            )
        }
    }
    cx.new(|cx| {
        let columns = [
            DataTableColumn::new("Project", px(110.)),
            DataTableColumn::new("Status", px(80.)),
            DataTableColumn::new("Seats", px(56.)).align_end(),
        ];
        let rows = [
            ("Atlas", "Active", 12),
            ("Beacon", "Paused", 4),
            ("Cedar", "Active", 8),
            ("Delta", "Active", 20),
            ("Echo", "Paused", 6),
            ("Foxtrot", "Active", 16),
            ("Grove", "Active", 3),
            ("Harbor", "Paused", 10),
        ]
        .into_iter()
        .map(|(name, status, seats)| {
            DataTableRow::new(
                name,
                [
                    DataTableCell::text(name),
                    DataTableCell::text(status),
                    DataTableCell::number(seats.to_string(), f64::from(seats)),
                ],
            )
        });
        let table = cx.new(|cx| DataTableState::new(columns, rows, 5, window, cx));
        cx.observe(&table, |_, _, cx| cx.notify()).detach();
        Projects(table)
    })
    .into()
}
