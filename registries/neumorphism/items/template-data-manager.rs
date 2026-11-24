//! Task record workspace with editable drafts, selection and confirmed deletion.
mod button;
mod input;
mod layout;
mod palette;
mod record_data;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Date, InputState, input::InputEvent};
use input::{editor_with_palette, field_with_palette};
use layout::{action, banner, choice, column, heading, note, panel, row, screen};
pub(crate) use record_data::{Record, Status};
use std::collections::BTreeSet;

pub(crate) struct DataManager {
    palette: Palette,
    records: Vec<Record>,
    selected: BTreeSet<String>,
    query: Entity<InputState>,
    filter: Option<Status>,
    descending: bool,
    page: usize,
    page_size: usize,
    show_owner: bool,
    editing: Option<Record>,
    name: Entity<InputState>,
    owner: Entity<InputState>,
    date: Entity<InputState>,
    status: Status,
    next_id: usize,
    confirm_delete: bool,
    confirm_discard: bool,
    message: String,
    on_save: Option<Box<dyn FnMut(&Record) -> Result<(), String>>>,
    on_delete: Option<Box<dyn FnMut(&BTreeSet<String>) -> Result<(), String>>>,
}
impl DataManager {
    pub(crate) fn new(
        p: Palette,
        records: Vec<Record>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let query = cx
            .new(|cx| editor_with_palette(window, cx, "Title, owner or ID", Role::SearchInput, p));
        let name = cx.new(|cx| editor_with_palette(window, cx, "Record title", Role::TextInput, p));
        let owner = cx.new(|cx| editor_with_palette(window, cx, "Owner", Role::TextInput, p));
        let date = cx.new(|cx| editor_with_palette(window, cx, "YYYY-MM-DD", Role::TextInput, p));
        cx.subscribe(&query, |this: &mut Self, _, event, cx| {
            if matches!(event, InputEvent::Change) {
                this.page = 0;
            }
            cx.notify();
        })
        .detach();
        for field in [&name, &owner, &date] {
            cx.subscribe(field, |_: &mut Self, _, _: &InputEvent, cx| cx.notify())
                .detach();
        }
        Self {
            palette: p,
            records,
            selected: BTreeSet::new(),
            query,
            filter: None,
            descending: false,
            page: 0,
            page_size: 5,
            show_owner: true,
            editing: None,
            name,
            owner,
            date,
            status: Status::Draft,
            next_id: 1,
            confirm_delete: false,
            confirm_discard: false,
            message: String::new(),
            on_save: None,
            on_delete: None,
        }
    }
    pub(crate) fn on_save(
        mut self,
        callback: impl FnMut(&Record) -> Result<(), String> + 'static,
    ) -> Self {
        self.on_save = Some(Box::new(callback));
        self
    }
    pub(crate) fn on_delete(
        mut self,
        callback: impl FnMut(&BTreeSet<String>) -> Result<(), String> + 'static,
    ) -> Self {
        self.on_delete = Some(Box::new(callback));
        self
    }
    fn edit(&mut self, record: Record, w: &mut Window, cx: &mut Context<Self>) {
        self.name
            .update(cx, |s, c| s.set_value(record.title.clone(), w, c));
        self.owner
            .update(cx, |s, c| s.set_value(record.owner.clone(), w, c));
        self.date
            .update(cx, |s, c| s.set_value(record.date.clone(), w, c));
        self.status = record.status;
        self.editing = Some(record);
        self.confirm_discard = false;
        self.message.clear();
        cx.notify();
    }
    fn draft(&self, cx: &App) -> Option<Record> {
        let base = self.editing.as_ref()?;
        Some(Record {
            id: base.id.clone(),
            title: self.name.read(cx).value().trim().to_string(),
            owner: self.owner.read(cx).value().trim().to_string(),
            date: self.date.read(cx).value().trim().to_string(),
            status: self.status,
        })
    }
    fn save(&mut self, cx: &mut Context<Self>) {
        let Some(record) = self.draft(cx) else {
            return;
        };
        if record.title.is_empty() {
            self.message = "Enter a title.".into();
        } else if !record.date.is_empty() && record.date.parse::<Date>().is_err() {
            self.message = "Enter a valid date in YYYY-MM-DD format.".into();
        } else if let Some(callback) = &mut self.on_save
            && let Err(message) = callback(&record)
        {
            self.message = message;
        } else {
            self.message = format!("Saved {}.", record.id);
            if let Some(old) = self.records.iter_mut().find(|r| r.id == record.id) {
                *old = record;
            } else {
                self.records.push(record);
            }
            self.editing = None;
            self.page = 0;
        }
        cx.notify();
    }
    fn editor(&self, base: &Record, w: &Window, cx: &mut Context<Self>) -> Div {
        let p = self.palette;
        let mut statuses = row();
        for status in Status::ALL {
            statuses = statuses.child(
                choice(p, status.label(), status.label(), self.status == status).on_click(
                    cx.listener(move |this, _, _, cx| {
                        this.status = status;
                        cx.notify();
                    }),
                ),
            );
        }
        let mut body = panel(p)
            .child(heading(if self.records.iter().any(|r| r.id == base.id) {
                "Edit record"
            } else {
                "Create record"
            }))
            .child(field_with_palette(
                "record-title",
                "Title",
                &self.name,
                None,
                w,
                cx,
                p,
            ))
            .child(field_with_palette(
                "record-owner",
                "Owner",
                &self.owner,
                None,
                w,
                cx,
                p,
            ))
            .child(field_with_palette(
                "record-date",
                "Due date",
                &self.date,
                None,
                w,
                cx,
                p,
            ))
            .child(note(p, "Status"))
            .child(statuses);
        if self.confirm_discard {
            body = body.child(heading("Discard edits?")).child(
                row()
                    .child(
                        action(p, "keep-draft", "Keep editing").on_click(cx.listener(
                            |this, _, _, cx| {
                                this.confirm_discard = false;
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        action(p, "discard-draft", "Discard draft").on_click(cx.listener(
                            |this, _, _, cx| {
                                this.editing = None;
                                this.confirm_discard = false;
                                this.message.clear();
                                cx.notify();
                            },
                        )),
                    ),
            );
        } else {
            body = body.child(
                row()
                    .child(
                        layout::primary(p, "save-record", "Save record")
                            .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
                    )
                    .child(action(p, "cancel-record", "Cancel").on_click(cx.listener(
                        |this, _, _, cx| {
                            if this.draft(cx) == this.editing {
                                this.editing = None;
                            } else {
                                this.confirm_discard = true;
                            }
                            cx.notify();
                        },
                    ))),
            );
        }
        body
    }
}
impl Render for DataManager {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let mut body = column().child(banner(p, "Workspace / Records", "Task records"));
        if let Some(ref record) = self.editing {
            return screen(
                "records-editor",
                p,
                body.child(self.editor(record, w, cx))
                    .child(note(p, self.message.clone())),
            )
            .into_any_element();
        }
        let query = self.query.read(cx).value().trim().to_lowercase();
        let mut visible: Vec<_> = self
            .records
            .iter()
            .filter(|r| {
                self.filter.is_none_or(|s| r.status == s)
                    && format!("{} {} {} {}", r.id, r.title, r.owner, r.status.label())
                        .to_lowercase()
                        .contains(&query)
            })
            .collect();
        visible.sort_by(|a, b| {
            a.title
                .to_lowercase()
                .cmp(&b.title.to_lowercase())
                .then(a.id.cmp(&b.id))
        });
        if self.descending {
            visible.reverse();
        }
        let pages = visible.len().div_ceil(self.page_size).max(1);
        self.page = self.page.min(pages - 1);
        let shown: Vec<_> = visible
            .iter()
            .skip(self.page * self.page_size)
            .take(self.page_size)
            .copied()
            .collect();
        let mut filters = row();
        for (i, status) in std::iter::once(None)
            .chain(Status::ALL.into_iter().map(Some))
            .enumerate()
        {
            filters = filters.child(
                choice(
                    p,
                    ("filter", i),
                    status.map_or("All", Status::label),
                    status == self.filter,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.filter = status;
                    this.page = 0;
                    cx.notify();
                })),
            );
        }
        let create = layout::primary(p, "create-record", "New record").on_click(cx.listener(
            |this, _, w, cx| {
                let id = loop {
                    let id = format!("REC-NEW-{}", this.next_id);
                    this.next_id += 1;
                    if !this.records.iter().any(|r| r.id == id) {
                        break id;
                    }
                };
                this.edit(
                    Record {
                        id,
                        title: String::new(),
                        owner: String::new(),
                        date: String::new(),
                        status: Status::Draft,
                    },
                    w,
                    cx,
                );
            },
        ));
        body = body.child(
            panel(p)
                .child(field_with_palette(
                    "record-search",
                    "Search records",
                    &self.query,
                    None,
                    w,
                    cx,
                    p,
                ))
                .child(filters)
                .child(
                    row()
                        .child(create)
                        .child(
                            choice(p, "sort", "Title descending", self.descending).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.descending = !this.descending;
                                    this.page = 0;
                                    cx.notify();
                                }),
                            ),
                        )
                        .child(
                            choice(p, "owner-column", "Show owners", self.show_owner).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.show_owner = !this.show_owner;
                                    cx.notify();
                                }),
                            ),
                        ),
                ),
        );
        let ids: BTreeSet<_> = shown.iter().map(|r| r.id.clone()).collect();
        let all = !ids.is_empty() && ids.is_subset(&self.selected);
        let select = choice(p, "select-page", "Select current page", all)
            .disabled(ids.is_empty())
            .on_click(cx.listener(move |this, _, _, cx| {
                if all {
                    this.selected.retain(|id| !ids.contains(id));
                } else {
                    this.selected.extend(ids.iter().cloned());
                }
                this.confirm_delete = false;
                cx.notify();
            }));
        let remove = action(p, "remove", "Delete selected")
            .disabled(self.selected.is_empty())
            .on_click(cx.listener(|this, _, _, cx| {
                this.confirm_delete = true;
                cx.notify();
            }));
        let mut list = panel(p)
            .child(heading("Records"))
            .child(note(
                p,
                format!(
                    "{} matching records · {} selected",
                    visible.len(),
                    self.selected.len()
                ),
            ))
            .child(row().child(select).child(remove));
        if self.confirm_delete {
            list =
                list.child(heading("Delete these records?")).child(
                    row()
                        .child(
                            action(p, "cancel-delete", "Cancel deletion").on_click(cx.listener(
                                |this, _, _, cx| {
                                    this.confirm_delete = false;
                                    cx.notify();
                                },
                            )),
                        )
                        .child(action(p, "confirm-delete", "Confirm deletion").on_click(
                            cx.listener(|this, _, _, cx| {
                                if let Some(callback) = &mut this.on_delete
                                    && let Err(message) = callback(&this.selected)
                                {
                                    this.message = message;
                                } else {
                                    this.message =
                                        format!("Deleted {} records.", this.selected.len());
                                    this.records.retain(|r| !this.selected.contains(&r.id));
                                    this.selected.clear();
                                }
                                this.confirm_delete = false;
                                cx.notify();
                            }),
                        )),
                );
        }
        if shown.is_empty() {
            list = list.child(note(p, "No matching records."));
        }
        for (i, record) in shown.into_iter().enumerate() {
            let id = record.id.clone();
            let edit = record.clone();
            let selected = self.selected.contains(&id);
            let subtitle = if self.show_owner {
                format!(
                    "{} · {} · {}",
                    record.id,
                    record.owner,
                    record.status.label()
                )
            } else {
                format!("{} · {}", record.id, record.status.label())
            };
            list = list.child(
                div()
                    .border_t_1()
                    .border_color(p.border)
                    .pt_3()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(heading(record.title.clone()))
                    .child(note(
                        p,
                        format!(
                            "{subtitle} · {}",
                            if record.date.is_empty() {
                                "No due date"
                            } else {
                                &record.date
                            }
                        ),
                    ))
                    .child(
                        row()
                            .child(
                                choice(
                                    p,
                                    ("select", i),
                                    if selected { "Selected" } else { "Select" },
                                    selected,
                                )
                                .accessibility_label(format!("Select {}", record.title))
                                .on_click(cx.listener(
                                    move |this, _, _, cx| {
                                        if !this.selected.remove(&id) {
                                            this.selected.insert(id.clone());
                                        }
                                        this.confirm_delete = false;
                                        cx.notify();
                                    },
                                )),
                            )
                            .child(
                                action(p, ("edit", i), "Edit")
                                    .accessibility_label(format!("Edit {}", record.title))
                                    .on_click(cx.listener(move |this, _, w, cx| {
                                        this.edit(edit.clone(), w, cx)
                                    })),
                            ),
                    ),
            );
        }
        let pager = row()
            .child(
                action(p, "previous", "Previous")
                    .disabled(self.page == 0)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.page = this.page.saturating_sub(1);
                        cx.notify();
                    })),
            )
            .child(note(p, format!("Page {} of {pages}", self.page + 1)))
            .child(
                action(p, "next", "Next")
                    .disabled(self.page + 1 >= pages)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.page += 1;
                        cx.notify();
                    })),
            )
            .child(
                action(p, "rows", format!("{} rows per page", self.page_size)).on_click(
                    cx.listener(|this, _, _, cx| {
                        this.page_size = if this.page_size == 5 { 10 } else { 5 };
                        this.page = 0;
                        cx.notify();
                    }),
                ),
            );
        screen(
            "records",
            p,
            body.child(list.child(pager))
                .child(note(p, self.message.clone())),
        )
        .into_any_element()
    }
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| DataManager::new(palette::air(), record_data::records(), w, cx))
        .into()
}
