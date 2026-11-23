//! Project overview with task search, completion filters and live progress summaries.
mod button;
mod data;
mod input;
mod layout;
mod palette;
use super::Palette;
use data::{Task, tasks};
use gpui::{
    AnyView, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Role,
    Styled as _, Window, div, relative,
};
use gpui_deck::{InputState, Progress, input::InputEvent};
use input::{editor_with_palette, field_with_palette};
use layout::{banner, choice, column, heading, panel, row, screen};

pub(crate) struct Dashboard {
    palette: Palette,
    tasks: Vec<Task>,
    query: Entity<InputState>,
    filter: Option<bool>,
}
impl Dashboard {
    pub(crate) fn new(p: Palette, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let query = cx.new(|cx| {
            editor_with_palette(window, cx, "Task, project or owner", Role::SearchInput, p)
        });
        cx.subscribe(&query, |_, _, event, cx| {
            if matches!(
                event,
                InputEvent::Change | InputEvent::Focus | InputEvent::Blur
            ) {
                cx.notify();
            }
        })
        .detach();
        Self {
            palette: p,
            tasks: tasks(),
            query,
            filter: None,
        }
    }
}
impl Render for Dashboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let complete = self.tasks.iter().filter(|t| t.done).count();
        let mut summary = panel(p).child(heading("Project progress"));
        for (i, name) in ["Website refresh", "Design system", "Customer research"]
            .into_iter()
            .enumerate()
        {
            let total = self
                .tasks
                .iter()
                .filter(|task| task.project == name)
                .count();
            let done = self
                .tasks
                .iter()
                .filter(|task| task.project == name && task.done)
                .count();
            let ratio = done as f32 / total as f32;
            summary = summary
                .child(format!("{name}   ·   {done}/{total} complete"))
                .child(
                    Progress::new(("project", i))
                        .accessibility_label(name)
                        .value(ratio * 100.)
                        .w_full()
                        .h_3()
                        .border_2()
                        .border_color(p.border)
                        .bg(p.field)
                        .child(div().h_full().w(relative(ratio)).bg(p.accent)),
                );
        }
        let mut filters = row();
        for (title, value) in [("All", None), ("Open", Some(false)), ("Done", Some(true))] {
            filters = filters.child(choice(p, title, title, self.filter == value).on_click(
                cx.listener(move |this, _, _, cx| {
                    this.filter = value;
                    cx.notify();
                }),
            ));
        }
        let mut list = panel(p)
            .child(heading("Work queue"))
            .child(field_with_palette(
                "task-search",
                "Search tasks",
                &self.query,
                None,
                window,
                cx,
                p,
            ))
            .child(filters);
        let query = self.query.read(cx).value().trim().to_lowercase();
        let mut visible = 0;
        for (i, t) in self.tasks.iter().enumerate() {
            if self.filter.is_some_and(|v| v != t.done)
                || !format!("{} {} {}", t.title, t.project, t.owner)
                    .to_lowercase()
                    .contains(&query)
            {
                continue;
            }
            visible += 1;
            list = list.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .border_t_1()
                    .border_color(p.border)
                    .pt_3()
                    .child(heading(t.title))
                    .child(format!("{}   /   {}", t.project, t.owner))
                    .child(
                        choice(
                            p,
                            ("task", i),
                            if t.done { "✓ Done" } else { "Mark complete" },
                            t.done,
                        )
                        .accessibility_label(format!(
                            "{}: {}",
                            t.title,
                            if t.done { "mark open" } else { "mark complete" }
                        ))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.tasks[i].done = !this.tasks[i].done;
                            cx.notify();
                        })),
                    ),
            );
        }
        if visible == 0 {
            list = list.child("No tasks match the selected filters.");
        }
        screen(
            "project-dashboard",
            p,
            column()
                .child(
                    banner(p, "Workspace  /  Operations", "Project overview")
                        .child(format!("{complete} of {} tasks complete", self.tasks.len())),
                )
                .child(summary)
                .child(list),
        )
    }
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| Dashboard::new(palette::yellow(), window, cx))
        .into()
}
