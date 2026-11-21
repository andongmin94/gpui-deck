//! Application-owned data-table operations over shared native controls.

use std::{cmp::Ordering, collections::HashSet, rc::Rc};

use gpui::{
    AnyElement, App, AppContext as _, Context, Div, ElementId, Entity, Focusable as _,
    InteractiveElement as _, IntoElement, ParentElement as _, Pixels, RenderOnce, Role,
    ScrollHandle, SharedString, Stateful, StyleRefinement, Styled, Subscription, Window,
    accesskit::SortDirection, div, point, prelude::FluentBuilder as _, px,
};
use gpui_design::{
    Button, Checkbox, CheckboxState, Date, DropdownMenu, Field, Input, InputBase, InputState,
    MenuEvent, MenuItem, MenuOption, MenuState, ScrollArea, ScrollbarAxis, Table, TableCaption,
    TableCell, TableColumn, TableRow,
};

use super::super::Palette;
use super::{EmptyState, InputGroup, Pagination};

/// One fixed-width column. Cells sort by their [`DataTableCell`] keys.
pub(crate) struct DataTableColumn {
    label: SharedString,
    width: Pixels,
    align_end: bool,
    renderer: Option<Box<dyn Fn(&DataTableRow, &SharedString) -> AnyElement>>,
}

impl DataTableColumn {
    pub(crate) fn new(label: impl Into<SharedString>, width: Pixels) -> Self {
        assert!(f32::from(width).is_finite() && width > px(0.));
        Self {
            label: label.into(),
            width,
            align_end: false,
            renderer: None,
        }
    }

    /// Aligns heading and values to the end edge, as for numeric amounts.
    pub(crate) fn align_end(mut self) -> Self {
        self.align_end = true;
        self
    }

    /// Supply cell content while keeping the row's search text, native cell name and sort key.
    pub(crate) fn render_cell<E: IntoElement>(
        mut self,
        render: impl Fn(&DataTableRow, &SharedString) -> E + 'static,
    ) -> Self {
        self.renderer = Some(Box::new(move |row, text| {
            render(row, text).into_any_element()
        }));
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
enum SortKey {
    Text(String),
    Number(f64),
    Date(Date),
}

impl SortKey {
    fn rank(&self) -> u8 {
        match self {
            Self::Number(_) => 0,
            Self::Date(_) => 1,
            Self::Text(_) => 2,
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.total_cmp(b),
            (Self::Date(a), Self::Date(b)) => a.cmp(b),
            (Self::Text(a), Self::Text(b)) => a.cmp(b),
            _ => self.rank().cmp(&other.rank()),
        }
    }
}

/// Displayed text and the value it sorts by. Text sorts case-insensitively;
/// numbers and dates sort by value whatever their displayed format.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DataTableCell {
    text: SharedString,
    key: SortKey,
}

impl DataTableCell {
    pub(crate) fn text(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        Self {
            key: SortKey::Text(text.to_lowercase()),
            text,
        }
    }

    /// A finite number shown as formatted text, such as `$1,250.00`.
    pub(crate) fn number(text: impl Into<SharedString>, value: f64) -> Self {
        assert!(value.is_finite(), "a sortable number must be finite");
        Self {
            text: text.into(),
            key: SortKey::Number(value),
        }
    }

    /// A calendar date shown as formatted text, such as `Dec 10, 2026`.
    pub(crate) fn date(text: impl Into<SharedString>, value: Date) -> Self {
        Self {
            text: text.into(),
            key: SortKey::Date(value),
        }
    }
}

impl From<&str> for DataTableCell {
    fn from(text: &str) -> Self {
        Self::text(text.to_string())
    }
}

impl From<String> for DataTableCell {
    fn from(text: String) -> Self {
        Self::text(text)
    }
}

impl From<SharedString> for DataTableCell {
    fn from(text: SharedString) -> Self {
        Self::text(text)
    }
}

fn align(cell: TableCell, end: bool) -> TableCell {
    if end { cell.flex().justify_end() } else { cell }
}

/// An immutable row with a stable application ID and one cell per column.
pub(crate) struct DataTableRow {
    id: SharedString,
    cells: Vec<SharedString>,
    keys: Vec<SortKey>,
    disabled: bool,
}

impl DataTableRow {
    pub(crate) fn new(
        id: impl Into<SharedString>,
        cells: impl IntoIterator<Item = impl Into<DataTableCell>>,
    ) -> Self {
        let (cells, keys) = cells
            .into_iter()
            .map(|cell| {
                let cell = cell.into();
                (cell.text, cell.key)
            })
            .unzip();
        Self {
            id: id.into(),
            cells,
            keys,
            disabled: false,
        }
    }

    pub(crate) fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub(crate) fn id(&self) -> &SharedString {
        &self.id
    }

    pub(crate) fn cells(&self) -> &[SharedString] {
        &self.cells
    }

    pub(crate) fn is_disabled(&self) -> bool {
        self.disabled
    }
}

/// Retained client-side filtering, one-column sorting, and pagination.
///
/// The native InputState is the query's source of truth. Matching indices are
/// derived when rows, query text or sort order change. Each whitespace-separated
/// query term must occur somewhere in the row, ignoring case. Sorting preserves
/// source order for equal keys. The application supplies the rows and current
/// choice. Checkbox selection is independent of the current row
/// action and survives filtering, sorting, and paging by stable row ID. The
/// header checkbox changes only enabled rows on the displayed page. Observe
/// this entity when rendering it from a parent.
pub(crate) struct DataTableState {
    columns: Vec<DataTableColumn>,
    rows: Vec<DataTableRow>,
    input: Entity<InputState>,
    // The last text processed by the derived filter, not a second query value.
    filtered_query: SharedString,
    filtered: Vec<usize>,
    sort_column: Option<usize>,
    descending: bool,
    page: usize,
    page_size: usize,
    selected: HashSet<SharedString>,
    visible_columns: Vec<bool>,
    columns_menu: Entity<MenuState>,
    scroll: ScrollHandle,
    _input_observation: Subscription,
    _menu_subscriptions: Vec<Subscription>,
}

impl DataTableState {
    pub(crate) fn new(
        columns: impl IntoIterator<Item = DataTableColumn>,
        rows: impl IntoIterator<Item = DataTableRow>,
        page_size: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let columns: Vec<_> = columns.into_iter().collect();
        let rows: Vec<_> = rows.into_iter().collect();
        assert!(!columns.is_empty(), "a data table needs text columns");
        assert!(page_size > 0, "page size must be positive");
        Self::validate_rows(&rows, columns.len());
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Search all columns"));
        let input_observation = cx.observe(&input, |state, _, cx| {
            state.sync_query(cx);
            // Input focus and editing presentation also affect the frame.
            cx.notify();
        });
        let columns_menu = cx.new(|cx| {
            MenuState::new(
                columns
                    .iter()
                    .enumerate()
                    .map(|(index, column)| MenuItem::new(index.to_string(), column.label.clone())),
                cx,
            )
        });
        let menu_subscriptions = vec![
            cx.observe(&columns_menu, |_, _, cx| cx.notify()),
            cx.subscribe(&columns_menu, |state, _, event, cx| {
                let MenuEvent::Action(value) = event;
                let index: usize = value.parse().expect("column menu uses column indices");
                state.set_column_visible(index, !state.visible_columns[index], cx);
            }),
        ];
        Self {
            filtered: (0..rows.len()).collect(),
            visible_columns: vec![true; columns.len()],
            columns,
            rows,
            input,
            filtered_query: SharedString::default(),
            sort_column: None,
            descending: false,
            page: 0,
            page_size,
            selected: HashSet::new(),
            columns_menu,
            scroll: ScrollHandle::new(),
            _input_observation: input_observation,
            _menu_subscriptions: menu_subscriptions,
        }
    }

    pub(crate) fn query_input(&self) -> &Entity<InputState> {
        &self.input
    }

    pub(crate) fn page_index(&self) -> usize {
        self.page
    }

    pub(crate) fn page_size(&self) -> usize {
        self.page_size
    }

    /// Keep the former page's first row on screen after changing the positive page size.
    #[allow(dead_code)] // Optional for consumers with a fixed page size.
    pub(crate) fn set_page_size(&mut self, page_size: usize, cx: &mut Context<Self>) {
        assert!(page_size > 0, "page size must be positive");
        if self.page_size != page_size {
            self.page = self.page * self.page_size / page_size;
            self.page_size = page_size;
            self.reset_vertical_scroll();
            cx.notify();
        }
    }

    fn validate_rows(rows: &[DataTableRow], column_count: usize) {
        let mut ids = HashSet::with_capacity(rows.len());
        for row in rows {
            assert_eq!(
                row.cells.len(),
                column_count,
                "each row needs one value per column"
            );
            assert!(ids.insert(row.id()), "row IDs must be unique");
        }
    }

    /// Reapply the current filter and sort, retaining selected IDs that are still selectable.
    #[allow(dead_code)] // Optional for consumers with static rows.
    pub(crate) fn set_rows(
        &mut self,
        rows: impl IntoIterator<Item = DataTableRow>,
        cx: &mut Context<Self>,
    ) {
        let rows: Vec<_> = rows.into_iter().collect();
        Self::validate_rows(&rows, self.columns.len());
        self.selected = rows
            .iter()
            .filter(|row| !row.is_disabled() && self.selected.contains(row.id()))
            .map(|row| row.id().clone())
            .collect();
        self.rows = rows;
        self.rebuild(self.page, cx);
        cx.notify();
    }

    pub(crate) fn page_count(&self) -> usize {
        self.filtered.len().div_ceil(self.page_size())
    }

    pub(crate) fn filtered_count(&self) -> usize {
        self.filtered.len()
    }

    pub(crate) fn visible_rows(&self) -> impl Iterator<Item = &DataTableRow> {
        self.filtered
            .iter()
            .skip(self.page * self.page_size)
            .take(self.page_size)
            .map(|&index| &self.rows[index])
    }

    pub(crate) fn is_row_selected(&self, id: &SharedString) -> bool {
        self.selected.contains(id)
    }

    pub(crate) fn filtered_selected_count(&self) -> usize {
        self.filtered
            .iter()
            .filter(|&&index| self.selected.contains(self.rows[index].id()))
            .count()
    }

    pub(crate) fn is_column_visible(&self, column: usize) -> bool {
        self.visible_columns[column]
    }

    fn page_selection(&self) -> CheckboxState {
        let mut enabled = 0;
        let mut selected = 0;
        for row in self.visible_rows().filter(|row| !row.disabled) {
            enabled += 1;
            selected += usize::from(self.is_row_selected(row.id()));
        }
        match (selected, enabled) {
            (0, _) => CheckboxState::Unchecked,
            (selected, enabled) if selected == enabled => CheckboxState::Checked,
            _ => CheckboxState::Indeterminate,
        }
    }

    fn set_row_selected(&mut self, id: SharedString, selected: bool, cx: &mut Context<Self>) {
        if !self.rows.iter().any(|row| row.id == id && !row.disabled) {
            return;
        }
        let changed = if selected {
            self.selected.insert(id)
        } else {
            self.selected.remove(&id)
        };
        if changed {
            cx.notify();
        }
    }

    fn set_page_selected(&mut self, selected: bool, cx: &mut Context<Self>) {
        let ids: Vec<_> = self
            .visible_rows()
            .filter(|row| !row.disabled)
            .map(|row| row.id().clone())
            .collect();
        for id in ids {
            self.set_row_selected(id, selected, cx);
        }
    }

    fn set_column_visible(&mut self, column: usize, visible: bool, cx: &mut Context<Self>) {
        if self.visible_columns[column] != visible {
            self.visible_columns[column] = visible;
            cx.notify();
        }
    }

    pub(crate) fn set_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let page = page.min(self.page_count().saturating_sub(1));
        if page != self.page {
            self.page = page;
            self.reset_vertical_scroll();
            cx.notify();
        }
    }

    fn sync_query(&mut self, cx: &Context<Self>) {
        if self.input.read(cx).value() != self.filtered_query {
            self.rebuild(0, cx);
        }
    }

    fn rebuild(&mut self, requested_page: usize, cx: &Context<Self>) {
        let query = self.input.read(cx).value();
        let page = if query == self.filtered_query {
            requested_page
        } else {
            0
        };
        self.filtered_query = query;
        let terms: Vec<_> = self
            .filtered_query
            .split_whitespace()
            .map(str::to_lowercase)
            .collect();
        self.filtered = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                let text = row
                    .cells
                    .iter()
                    .map(|cell| cell.as_ref())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_lowercase();
                terms
                    .iter()
                    .all(|term| text.contains(term))
                    .then_some(index)
            })
            .collect();
        if let Some(column) = self.sort_column {
            let rows = &self.rows;
            let descending = self.descending;
            // Stable in both directions, so equal keys keep source order.
            self.filtered.sort_by(|&a, &b| {
                let order = rows[a].keys[column].compare(&rows[b].keys[column]);
                if descending { order.reverse() } else { order }
            });
        }
        self.page = page.min(self.page_count().saturating_sub(1));
        self.reset_vertical_scroll();
    }

    fn reset_vertical_scroll(&self) {
        self.scroll
            .set_offset(point(self.scroll.offset().x, px(0.)));
    }

    fn sort_by_column(&mut self, column: usize, cx: &mut Context<Self>) {
        self.descending = self.sort_column == Some(column) && !self.descending;
        self.sort_column = Some(column);
        self.rebuild(0, cx);
        cx.notify();
    }

    fn clear_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.set_value("", window, cx);
            input.focus(window, cx);
        });
        self.sync_query(cx);
        cx.notify();
    }
}

type RowAction = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

/// A copyable table with an application-controlled current row and action.
///
/// This composes Field/Input, Table, ScrollArea, EmptyState, and Pagination.
/// Header and row buttons keep ordinary Tab order; there is no grid focus or
/// arrow-key navigation. Checkbox selection and column visibility compose native
/// controls. Native table indices describe the displayed page. Column headers
/// expose the active sort direction; row actions retain a separate current row.
#[derive(IntoElement)]
pub(crate) struct DataTable {
    id: ElementId,
    base: Stateful<Div>,
    state: Entity<DataTableState>,
    palette: Palette,
    label: SharedString,
    action_label: SharedString,
    selected: Option<SharedString>,
    on_action: Option<RowAction>,
}

impl DataTable {
    pub(crate) fn new(
        id: impl Into<ElementId>,
        state: &Entity<DataTableState>,
        palette: Palette,
    ) -> Self {
        let id = id.into();
        Self {
            base: div()
                .id(id.clone())
                .w_full()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(palette.gap),
            id,
            state: state.clone(),
            palette,
            label: "Data table".into(),
            action_label: "Choose".into(),
            selected: None,
            on_action: None,
        }
    }

    pub(crate) fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub(crate) fn action_label(mut self, label: impl Into<SharedString>) -> Self {
        self.action_label = label.into();
        self
    }

    pub(crate) fn selected_value(mut self, selected: Option<SharedString>) -> Self {
        self.selected = selected;
        self
    }

    pub(crate) fn on_action(
        mut self,
        action: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Rc::new(action));
        self
    }

    fn button(palette: Palette, button: Button) -> Button {
        button
            .h(palette.control_height)
            .px_2()
            .border_1()
            .border_color(palette.border)
            .rounded(palette.radius)
            .bg(palette.field)
            .text_color(palette.foreground)
            .hover(|style| style.bg(palette.highlight))
            .focus_visible(|style| style.border_color(palette.accent))
            .styles(|styles| styles.disabled(|style| style.opacity(0.45)))
    }

    fn checkbox(palette: Palette, checkbox: Checkbox, state: CheckboxState) -> Checkbox {
        checkbox
            .state(state)
            .size(px(24.))
            .flex()
            .items_center()
            .justify_center()
            .border_1()
            .border_color(palette.border)
            .rounded(palette.radius)
            .bg(palette.field)
            .text_color(palette.foreground)
            .focus_visible(|style| style.border_color(palette.accent))
            .styles(|styles| styles.disabled(|style| style.opacity(0.45)))
            .child(match state {
                CheckboxState::Unchecked => "",
                CheckboxState::Checked => "✓",
                CheckboxState::Indeterminate => "−",
            })
    }
}

impl Styled for DataTable {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for DataTable {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let palette = self.palette;
        let query = state.query_input();
        let input = InputBase::new((self.id.clone(), "query"))
            .role(None::<Role>)
            .semantic_owner::<Input>(query.entity_id())
            .h(palette.control_height)
            .w_full()
            .child(Input::new(query));
        let filter = Field::new(
            (self.id.clone(), "filter"),
            query,
            format!("Filter {}", self.label),
        )
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            InputGroup::new((self.id.clone(), "query-group"), input, palette)
                .focused(query.focus_handle(cx).is_focused(window)),
        );

        let columns_menu = DropdownMenu::new((self.id.clone(), "columns"), &state.columns_menu)
            .accessibility_label(format!("{} visible columns", self.label))
            .trigger(|button| {
                Self::button(palette, button)
                    .accessibility_label("Choose visible columns")
                    .child("Columns")
            })
            .content(
                div()
                    .w(px(200.))
                    .p_1()
                    .flex()
                    .flex_col()
                    .bg(palette.surface)
                    .border_1()
                    .border_color(palette.border)
                    .rounded(palette.radius)
                    .children(state.columns.iter().enumerate().map(|(index, column)| {
                        let visible = state.is_column_visible(index);
                        MenuOption::new(&state.columns_menu, index)
                            .checked(visible)
                            .px_2()
                            .py_1()
                            .text_color(palette.foreground)
                            .when(
                                state.columns_menu.read(cx).highlighted_index() == Some(index),
                                |option| option.bg(palette.highlight),
                            )
                            .child(format!(
                                "{} {}",
                                if visible { "✓" } else { " " },
                                column.label
                            ))
                    })),
            );
        let page_selection = state.page_selection();
        let page_owner = self.state.clone();
        let mut header = TableRow::new((self.id.clone(), "header"))
            .bg(palette.highlight)
            .cell(
                TableCell::new((self.id.clone(), "select-heading"))
                    .p_1()
                    .accessibility_label("Selection")
                    .child(Self::checkbox(
                        palette,
                        Checkbox::new((self.id.clone(), "select-page"))
                            .accessibility_label("Select all enabled rows on this page")
                            .disabled(!state.visible_rows().any(|row| !row.disabled))
                            .on_change(move |value, _, _, cx| {
                                page_owner.update(cx, |state, cx| {
                                    state.set_page_selected(value == CheckboxState::Checked, cx)
                                })
                            }),
                        page_selection,
                    )),
            );
        let visible_columns: Vec<_> = state
            .columns
            .iter()
            .enumerate()
            .filter(|&(index, _)| state.is_column_visible(index))
            .collect();
        for &(index, column) in &visible_columns {
            let (current, next, marker) = if state.sort_column != Some(index) {
                ("not sorted", "ascending", "↕")
            } else if state.descending {
                ("sorted descending", "ascending", "↓")
            } else {
                ("sorted ascending", "descending", "↑")
            };
            let owner = self.state.clone();
            header = header.cell(
                align(
                    TableCell::new((self.id.clone(), format!("heading-{index}")))
                        .accessibility_label(column.label.clone())
                        .sort_direction((state.sort_column == Some(index)).then_some(
                            if state.descending {
                                SortDirection::Descending
                            } else {
                                SortDirection::Ascending
                            },
                        ))
                        .p_1(),
                    column.align_end,
                )
                .child(Self::button(
                    palette,
                    Button::new((self.id.clone(), format!("sort-{index}")))
                        .accessibility_label(format!(
                            "{}, {}; sort {}",
                            column.label, current, next
                        ))
                        .on_click(move |_, _, cx| {
                            owner.update(cx, |state, cx| state.sort_by_column(index, cx))
                        })
                        .child(format!("{} {marker}", column.label)),
                )),
            );
        }
        header = header.cell(
            TableCell::new((self.id.clone(), "action-heading"))
                .p_2()
                .accessibility_label("Action")
                .child("Action"),
        );

        let columns = std::iter::once(TableColumn::fixed(px(40.)))
            .chain(
                visible_columns
                    .iter()
                    .map(|(_, column)| TableColumn::fixed(column.width)),
            )
            .chain(std::iter::once(TableColumn::fixed(px(130.))));
        let width = visible_columns
            .iter()
            .fold(px(170.), |width, (_, column)| width + column.width);
        let mut table = Table::new((self.id.clone(), "table"), columns)
            .accessibility_label(self.label.clone())
            .w(width)
            .header(header);
        for row in state.visible_rows() {
            let row_id: ElementId = (self.id.clone(), format!("row-{}", row.id())).into();
            let current = self.selected.as_ref() == Some(row.id());
            let selected = state.is_row_selected(row.id());
            let row_selection = if selected {
                CheckboxState::Checked
            } else {
                CheckboxState::Unchecked
            };
            let selection_owner = self.state.clone();
            let selected_id = row.id().clone();
            let mut rendered = TableRow::new(row_id.clone())
                .selected(selected)
                .border_b_1()
                .border_color(palette.border)
                .cell(
                    TableCell::new((row_id.clone(), "selection-cell"))
                        .p_1()
                        .child(Self::checkbox(
                            palette,
                            Checkbox::new((row_id.clone(), "select"))
                                .accessibility_label(format!("Select {}", row.cells()[0]))
                                .disabled(row.disabled)
                                .on_change(move |value, _, _, cx| {
                                    selection_owner.update(cx, |state, cx| {
                                        state.set_row_selected(
                                            selected_id.clone(),
                                            value == CheckboxState::Checked,
                                            cx,
                                        )
                                    })
                                }),
                            row_selection,
                        )),
                );
            if current || selected {
                rendered = rendered.bg(palette.highlight);
            }
            for (visible_index, &(index, column)) in visible_columns.iter().enumerate() {
                let cell = &row.cells()[index];
                let content = match &column.renderer {
                    Some(render) => render(row, cell),
                    None => cell.clone().into_any_element(),
                };
                rendered = rendered.cell(
                    align(
                        TableCell::new((row_id.clone(), format!("column-{index}")))
                            .row_header(visible_index == 0)
                            .accessibility_label(cell.clone())
                            .p_2(),
                        column.align_end,
                    )
                    .child(content),
                );
            }
            let value = row.id().clone();
            let action = self.on_action.clone();
            let label = if current {
                format!("{}: current", row.cells()[0])
            } else {
                format!("{}: {}", self.action_label, row.cells()[0])
            };
            let button = Self::button(
                palette,
                Button::new((row_id.clone(), "action"))
                    .accessibility_label(label)
                    .disabled(row.disabled || current || action.is_none())
                    .on_click(move |_, window, cx| {
                        if let Some(action) = &action {
                            action(value.clone(), window, cx);
                        }
                    })
                    .child(if current {
                        SharedString::from("Current")
                    } else {
                        self.action_label.clone()
                    }),
            );
            table = table
                .row(rendered.cell(TableCell::new((row_id, "action-cell")).p_1().child(button)));
        }
        let count = state.filtered_count();
        let noun = if count == 1 { "row" } else { "rows" };
        let matching_rows = format!("{count} matching {noun}");
        table = table.caption(
            TableCaption::new((self.id.clone(), "caption"))
                .py_2()
                .text_sm()
                .text_color(palette.muted)
                .child(matching_rows.clone()),
        );

        let empty = state.filtered_count() == 0;
        let page = state.page_index();
        let page_count = state.page_count();
        let viewport = ScrollArea::new((self.id.clone(), "scroll"), &state.scroll)
            .accessibility_label(format!("{} rows", self.label))
            .axis(ScrollbarAxis::Both)
            .w_full()
            .h(px(200.))
            .content_styles(|style| style.pb(px(16.)).pr(px(16.)))
            .scrollbar_styles(|styles| styles.thumb(|thumb| thumb.bg(palette.muted).radius(px(3.))))
            .child(table);
        let clear = self.state.clone();
        let pagination = self.state.clone();
        self.base
            .child(filter)
            .child(columns_menu)
            .when(!empty, |base| base.child(viewport))
            .when(empty, |base| {
                base.child(
                    EmptyState::new(
                        (self.id.clone(), "empty"),
                        "No matching rows",
                        "Change the filter or clear it to show every row.",
                        palette,
                    )
                    .action(Self::button(
                        palette,
                        Button::new((self.id.clone(), "clear-filter"))
                            .accessibility_label("Clear filter")
                            .on_click(move |_, window, cx| {
                                clear.update(cx, |state, cx| state.clear_filter(window, cx))
                            })
                            .child("Clear filter"),
                    )),
                )
            })
            .child(div().text_sm().text_color(palette.muted).child(format!(
                "{} of {matching_rows} selected",
                state.filtered_selected_count(),
            )))
            .child(
                Pagination::new(
                    (self.id, "pagination"),
                    format!("{} pages", self.label),
                    page,
                    page_count,
                    palette,
                )
                .on_change(move |page, _, cx| {
                    pagination.update(cx, |state, cx| state.set_page(page, cx))
                }),
            )
    }
}

#[cfg(all(test, feature = "test-support"))]
mod tests {
    use gpui::TestAppContext;

    use super::*;

    fn sample(window: &mut Window, cx: &mut App) -> Entity<DataTableState> {
        cx.new(|cx| {
            DataTableState::new(
                [
                    DataTableColumn::new("Name", px(100.)),
                    DataTableColumn::new("Status", px(100.)),
                ],
                [
                    DataTableRow::new("a", ["Alpha", "Ready"]),
                    DataTableRow::new("b", ["Beta", "Paused"]).disabled(true),
                    DataTableRow::new("c", ["Charlie", "Ready"]),
                    DataTableRow::new("d", ["Delta", "Ready"]),
                    DataTableRow::new("e", ["Echo", "Paused"]),
                ],
                3,
                window,
                cx,
            )
        })
    }

    #[gpui::test]
    fn checkbox_selection_is_page_scoped_and_retains_stable_ids(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        window.update(|window, cx| {
            let table = sample(window, cx);
            table.update(cx, |table, cx| {
                table.set_row_selected("a".into(), true, cx);
                table.set_row_selected("b".into(), true, cx);
                assert!(!table.is_row_selected(&"b".into()));
                assert_eq!(table.page_selection(), CheckboxState::Indeterminate);
                table.set_page_selected(true, cx);
                assert_eq!(table.page_selection(), CheckboxState::Checked);
                assert_eq!(table.filtered_selected_count(), 2);

                table.set_page(1, cx);
                assert_eq!(table.page_selection(), CheckboxState::Unchecked);
                table.set_page_selected(true, cx);
                assert_eq!(table.filtered_selected_count(), 4);
                table.set_page(0, cx);
                table.set_page_selected(false, cx);
                assert_eq!(table.filtered_selected_count(), 2);
                assert!(table.is_row_selected(&"d".into()));
                assert!(table.is_row_selected(&"e".into()));

                table.sort_by_column(0, cx);
                table.sort_by_column(0, cx);
                assert_eq!(table.visible_rows().next().unwrap().id().as_ref(), "e");
                assert_eq!(table.page_selection(), CheckboxState::Indeterminate);
                table
                    .input
                    .update(cx, |input, cx| input.set_value("Paused", window, cx));
                table.sync_query(cx);
                assert_eq!(table.filtered_count(), 2);
                assert_eq!(table.filtered_selected_count(), 1);
                table.set_page_selected(false, cx);
                assert!(!table.is_row_selected(&"e".into()));
                assert!(table.is_row_selected(&"d".into()));
                table.clear_filter(window, cx);
                assert_eq!(table.filtered_selected_count(), 1);
            });
        });
    }

    #[gpui::test]
    fn hiding_columns_preserves_filter_sort_and_selection(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        window.update(|window, cx| {
            let table = sample(window, cx);
            table.update(cx, |table, cx| {
                table.set_row_selected("c".into(), true, cx);
                table.sort_by_column(0, cx);
                table.set_page(1, cx);
                table.set_column_visible(0, false, cx);
                table.set_column_visible(1, false, cx);
                assert!(!table.is_column_visible(0));
                assert!(!table.is_column_visible(1));
                assert_eq!(table.page_index(), 1);
                table
                    .input
                    .update(cx, |input, cx| input.set_value("Charlie Ready", window, cx));
                table.sync_query(cx);
                assert_eq!(table.filtered_count(), 1);
                assert_eq!(table.filtered_selected_count(), 1);
                assert_eq!(table.sort_column, Some(0));
                table.set_column_visible(0, true, cx);
                assert!(table.is_column_visible(0));
                assert_eq!(table.visible_rows().next().unwrap().id().as_ref(), "c");
            });
        });
    }

    #[gpui::test]
    fn numbers_and_dates_sort_by_value_not_display_text(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        window.update(|window, cx| {
            let date = |day| Date::new(2026, 12, day).unwrap();
            let table = cx.new(|cx| {
                DataTableState::new(
                    [
                        DataTableColumn::new("Name", px(100.)),
                        DataTableColumn::new("Amount", px(100.)).align_end(),
                        DataTableColumn::new("Due", px(100.)),
                    ],
                    [
                        DataTableRow::new(
                            "a",
                            [
                                DataTableCell::text("Alpha"),
                                DataTableCell::number("$900.00", 900.),
                                DataTableCell::date("Dec 9, 2026", date(9)),
                            ],
                        ),
                        DataTableRow::new(
                            "b",
                            [
                                DataTableCell::text("Beta"),
                                DataTableCell::number("$1,250.00", 1250.),
                                DataTableCell::date("Dec 10, 2026", date(10)),
                            ],
                        ),
                        DataTableRow::new(
                            "c",
                            [
                                DataTableCell::text("Charlie"),
                                DataTableCell::number("-$5.00", -5.),
                                DataTableCell::date("Dec 1, 2026", date(1)),
                            ],
                        ),
                    ],
                    5,
                    window,
                    cx,
                )
            });
            table.update(cx, |table, cx| {
                let order = |table: &DataTableState| {
                    table
                        .visible_rows()
                        .map(|row| row.id().to_string())
                        .collect::<Vec<_>>()
                };
                // Text order would put "$1,250.00" first and "-$5.00" last.
                table.sort_by_column(1, cx);
                assert_eq!(order(table), ["c", "a", "b"]);
                table.sort_by_column(1, cx);
                assert_eq!(order(table), ["b", "a", "c"]);
                // Text order would put "Dec 10" before "Dec 9".
                table.sort_by_column(2, cx);
                assert_eq!(order(table), ["c", "a", "b"]);
                // Filtering still searches the displayed text.
                table
                    .input
                    .update(cx, |input, cx| input.set_value("1,250", window, cx));
                table.sync_query(cx);
                assert_eq!(order(table), ["b"]);
            });
        });
    }

    #[gpui::test]
    fn refreshed_rows_and_new_page_sizes_preserve_the_current_view(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        window.update(|window, cx| {
            let table = sample(window, cx);
            table.update(cx, |table, cx| {
                for id in ["a", "c", "d"] {
                    table.set_row_selected(id.into(), true, cx);
                }
                table
                    .input
                    .update(cx, |input, cx| input.set_value("Ready", window, cx));
                table.sync_query(cx);
                table.set_column_visible(1, false, cx);
                table.sort_by_column(0, cx);
                table.sort_by_column(0, cx);
                table.set_page_size(1, cx);
                table.set_page(2, cx);

                table.set_rows(
                    [
                        DataTableRow::new("b", ["Beta", "Ready"]),
                        DataTableRow::new("c", ["Charlie", "Ready"]).disabled(true),
                        DataTableRow::new("e", ["Echo", "Paused"]),
                        DataTableRow::new("d", ["Delta updated", "Ready"]),
                    ],
                    cx,
                );
                assert_eq!(table.page_index(), 2);
                assert_eq!(table.filtered_count(), 3);
                assert_eq!(table.visible_rows().next().unwrap().id().as_ref(), "b");
                assert!(!table.is_column_visible(1));
                assert_eq!(table.filtered_selected_count(), 1);
                assert!(table.is_row_selected(&"d".into()));
                assert!(!table.is_row_selected(&"a".into()));
                assert!(!table.is_row_selected(&"c".into()));

                table.set_page_size(2, cx);
                assert_eq!((table.page_index(), table.page_size()), (1, 2));
                assert_eq!(table.visible_rows().next().unwrap().id().as_ref(), "b");
                table.set_rows([DataTableRow::new("d", ["Delta", "Ready"])], cx);
                assert_eq!(table.page_index(), 0);
                assert_eq!(table.filtered_selected_count(), 1);
                table.set_rows([], cx);
                assert_eq!((table.page_index(), table.page_count()), (0, 0));
                assert!(!table.is_row_selected(&"d".into()));
            });
        });
    }
}
