mod palette;
mod style;

use super::Palette;
use super::recipes::{DataTable, DataTableCell, DataTableColumn, DataTableRow, DataTableState};
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, Entity, Role, SharedString, div, px};
pub(crate) fn data_table(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<DataTableState>,
    p: Palette,
) -> DataTable {
    DataTable::new(id, state, p).accessibility_label(label)
}

pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    struct Projects {
        table: Entity<DataTableState>,
        opened: Option<(SharedString, SharedString, SharedString)>,
    }
    impl Render for Projects {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            let owner = cx.entity();
            let table = data_table("projects", "Workspace projects", &self.table, p)
                .action_label("Open")
                .selected_value(self.opened.as_ref().map(|(name, _, _)| name.clone()))
                .on_action(move |id, _, cx| {
                    owner.update(cx, |this, cx| {
                        this.opened = this
                            .table
                            .read(cx)
                            .visible_rows()
                            .find(|row| row.id() == &id)
                            .map(|row| (id, row.cells()[1].clone(), row.cells()[2].clone()));
                        cx.notify();
                    })
                });
            let mut panel = style::surface(p)
                .px_2()
                .flex()
                .flex_col()
                .gap_3()
                .child(table);
            if let Some((name, status, seats)) = &self.opened {
                panel =
                    panel.child(
                        palette::well(p)
                            .p_3()
                            .flex()
                            .flex_col()
                            .items_start()
                            .gap_2()
                            .child(
                                div()
                                    .id("details")
                                    .role(Role::Status)
                                    .child(format!("{name} · {status} · {seats} seats")),
                            )
                            .child(style::action("close", "Close details", p).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.opened = None;
                                    cx.notify();
                                }),
                            )),
                    );
            }
            style::stage(p, panel)
        }
    }
    cx.new(|cx| {
        let columns = [
            DataTableColumn::new("Project", px(110.)),
            DataTableColumn::new("Status", px(106.)),
            DataTableColumn::new("Seats", px(96.)).align_end(),
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
        let table = cx.new(|cx| DataTableState::new(columns, rows, 3, window, cx));
        cx.observe(&table, |_, _, cx| cx.notify()).detach();
        Projects {
            table,
            opened: None,
        }
    })
    .into()
}
