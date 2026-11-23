//! Editorial workspace. Drafts and saved revisions are kept for this running session.
mod button;
mod data;
mod input;
mod layout;
mod palette;
use super::Palette;
use data::{Post, posts};
use gpui::{
    AnyView, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Role,
    Styled as _, Window, div, px,
};
use gpui_deck::{Field, InputEditorStyle, InputState, Textarea, TextareaState, input::InputEvent};
use input::{editor_with_palette, field_with_palette};
use layout::{action, banner, choice, column, heading, panel, row, screen};

pub(crate) struct Cms {
    palette: Palette,
    drafts: Vec<Post>,
    saved: Vec<Post>,
    current: usize,
    search: Entity<InputState>,
    title: Entity<InputState>,
    summary: Entity<InputState>,
    body: Entity<TextareaState>,
    filter: Option<bool>,
    previewing: bool,
    feedback: &'static str,
}
impl Cms {
    pub(crate) fn new(p: Palette, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let drafts = posts();
        let search =
            cx.new(|cx| editor_with_palette(window, cx, "Title or summary", Role::SearchInput, p));
        let title = cx.new(|cx| {
            editor_with_palette(window, cx, "Untitled post", Role::TextInput, p)
                .default_value(drafts[0].title.clone())
        });
        let summary = cx.new(|cx| {
            editor_with_palette(window, cx, "Short description", Role::TextInput, p)
                .default_value(drafts[0].summary.clone())
        });
        let body = cx.new(|cx| {
            let mut editor = TextareaState::new(window, cx)
                .rows(5)
                .default_value(drafts[0].body.clone());
            editor.set_editor_style(InputEditorStyle {
                foreground: p.foreground,
                muted_foreground: p.muted,
                caret: p.foreground,
                selection: p.highlight,
                ..Default::default()
            });
            editor
        });
        for edit in [&title, &summary] {
            cx.subscribe(edit, |this: &mut Self, _, event, cx| {
                this.changed(event, cx);
            })
            .detach();
        }
        cx.subscribe(&body, |this: &mut Self, _, event, cx| {
            this.changed(event, cx);
        })
        .detach();
        cx.subscribe(&search, |_, _, event, cx| {
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
            saved: drafts.clone(),
            drafts,
            current: 0,
            search,
            title,
            summary,
            body,
            filter: None,
            previewing: false,
            feedback: "",
        }
    }
    fn changed(&mut self, event: &InputEvent, cx: &mut Context<Self>) {
        if matches!(event, InputEvent::Change) {
            let draft = &mut self.drafts[self.current];
            draft.title = self.title.read(cx).value().to_string();
            draft.summary = self.summary.read(cx).value().to_string();
            draft.body = self.body.read(cx).value().to_string();
            self.feedback = "";
        }
        if matches!(
            event,
            InputEvent::Change | InputEvent::Focus | InputEvent::Blur
        ) {
            cx.notify();
        }
    }
    fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.current = index;
        let post = &self.drafts[index];
        self.title.update(cx, |input, cx| {
            input.set_value(post.title.clone(), window, cx)
        });
        self.summary.update(cx, |input, cx| {
            input.set_value(post.summary.clone(), window, cx)
        });
        self.body.update(cx, |input, cx| {
            input.set_value(post.body.clone(), window, cx)
        });
        self.feedback = "";
        cx.notify();
    }
}
impl Render for Cms {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let current = &self.drafts[self.current];
        let dirty = current != &self.saved[self.current];
        let title = if current.title.trim().is_empty() {
            "Untitled post"
        } else {
            current.title.trim()
        };
        let query = self.search.read(cx).value().trim().to_lowercase();
        let mut filters = row();
        for (name, value) in [
            ("All posts", None),
            ("Drafts", Some(false)),
            ("Published", Some(true)),
        ] {
            filters = filters.child(choice(p, name, name, self.filter == value).on_click(
                cx.listener(move |this, _, _, cx| {
                    this.filter = value;
                    cx.notify();
                }),
            ));
        }
        let mut archive = panel(p)
            .child(heading("Content library"))
            .child(field_with_palette(
                "post-search",
                "Search posts",
                &self.search,
                None,
                window,
                cx,
                p,
            ))
            .child(filters);
        let mut choices = row();
        let mut count = 0;
        for (i, post) in self.drafts.iter().enumerate() {
            if self.filter.is_some_and(|status| status != post.published)
                || !format!("{} {}", post.title, post.summary)
                    .to_lowercase()
                    .contains(&query)
            {
                continue;
            }
            count += 1;
            let name = if post.title.trim().is_empty() {
                "Untitled post"
            } else {
                post.title.trim()
            };
            choices = choices.child(
                choice(p, ("post", i), name.to_owned(), i == self.current)
                    .on_click(cx.listener(move |this, _, w, cx| this.select(i, w, cx))),
            );
        }
        archive = if count > 0 {
            archive.child(choices)
        } else {
            archive.child("No posts match your search.")
        };
        archive = archive.child(action(p, "new-post", "New draft").on_click(cx.listener(
            |this, _, w, cx| {
                let blank = Post {
                    title: "Untitled post".into(),
                    summary: String::new(),
                    body: String::new(),
                    published: false,
                };
                this.drafts.push(blank.clone());
                this.saved.push(blank);
                this.filter = None;
                this.search
                    .update(cx, |input, cx| input.set_value("", w, cx));
                this.previewing = false;
                this.select(this.drafts.len() - 1, w, cx);
            },
        )));
        let mode = row()
            .child(
                choice(p, "edit", "Edit", !self.previewing).on_click(cx.listener(
                    |this, _, _, cx| {
                        this.previewing = false;
                        cx.notify();
                    },
                )),
            )
            .child(
                choice(p, "preview", "Preview", self.previewing).on_click(cx.listener(
                    |this, _, _, cx| {
                        this.previewing = true;
                        cx.notify();
                    },
                )),
            );
        let mut editor = panel(p).child(mode);
        if self.previewing {
            editor = editor
                .child(heading(title.to_owned()))
                .child(current.summary.clone())
                .child(current.body.clone());
        } else {
            editor = editor
                .child(field_with_palette(
                    "post-title",
                    "Title",
                    &self.title,
                    None,
                    window,
                    cx,
                    p,
                ))
                .child(field_with_palette(
                    "post-summary",
                    "Summary",
                    &self.summary,
                    None,
                    window,
                    cx,
                    p,
                ))
                .child(
                    Field::new("post-body", &self.body, "Body")
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .w_full()
                                .p_2()
                                .min_h(px(108.))
                                .border_2()
                                .border_color(p.border)
                                .rounded(p.radius)
                                .bg(p.field)
                                .child(Textarea::new(&self.body)),
                        ),
                )
                .child(
                    choice(
                        p,
                        "post-status",
                        if current.published {
                            "Published · switch to draft"
                        } else {
                            "Draft · mark published"
                        },
                        current.published,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        let post = &mut this.drafts[this.current];
                        post.published = !post.published;
                        this.feedback = "";
                        cx.notify();
                    })),
                );
        }
        let save = action(p, "save-post", "Save changes")
            .disabled(!dirty || current.title.trim().is_empty())
            .on_click(cx.listener(|this, _, _, cx| {
                this.saved[this.current] = this.drafts[this.current].clone();
                this.feedback = "Changes saved in this session.";
                cx.notify();
            }));
        let discard = layout::framed(p, "discard-post", "Restore saved", false)
            .disabled(!dirty)
            .on_click(cx.listener(|this, _, w, cx| {
                let i = this.current;
                this.drafts[i] = this.saved[i].clone();
                this.select(i, w, cx);
                this.feedback = "Restored the saved revision.";
            }));
        editor =
            editor
                .child(row().child(save).child(discard))
                .child(if self.feedback.is_empty() {
                    if dirty {
                        "Unsaved changes"
                    } else {
                        "Saved revision"
                    }
                } else {
                    self.feedback
                });
        screen(
            "editorial-workspace",
            p,
            column()
                .child(banner(p, "Publishing desk", "Editorial workspace"))
                .child(archive)
                .child(editor),
        )
    }
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| Cms::new(palette::yellow(), window, cx)).into()
}
