//! Editorial workspace. Drafts and saved revisions are kept for this running session.
mod button;
mod cms_data;
mod input;
mod layout;
mod palette;
use super::Palette;
pub(crate) use cms_data::Post;
use cms_data::posts;
use gpui::{
    AnyView, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Role,
    Styled as _, Window, px,
};
use gpui_deck::{Field, InputEditorStyle, InputState, Textarea, TextareaState, input::InputEvent};
use input::{editor_with_palette, field_with_palette};
use layout::{action, banner, choice, column, heading, panel, row, screen};

pub(crate) struct Cms {
    palette: Palette,
    drafts: Vec<Post>,
    saved: Vec<Option<Post>>,
    on_save: Option<Box<dyn FnMut(&Post) -> Result<(), String>>>,
    current: usize,
    search: Entity<InputState>,
    title: Entity<InputState>,
    summary: Entity<InputState>,
    body: Entity<TextareaState>,
    filter: Option<bool>,
    previewing: bool,
    feedback: String,
}
impl Cms {
    pub(crate) fn new(
        p: Palette,
        mut drafts: Vec<Post>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut saved: Vec<_> = drafts.iter().cloned().map(Some).collect();
        if drafts.is_empty() {
            drafts.push(Post::untitled());
            saved.push(None);
        }
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
            saved,
            on_save: None,
            drafts,
            current: 0,
            search,
            title,
            summary,
            body,
            filter: None,
            previewing: false,
            feedback: String::new(),
        }
    }
    pub(crate) fn on_save(
        mut self,
        save: impl FnMut(&Post) -> Result<(), String> + 'static,
    ) -> Self {
        self.on_save = Some(Box::new(save));
        self
    }
    fn changed(&mut self, event: &InputEvent, cx: &mut Context<Self>) {
        if matches!(event, InputEvent::Change) {
            let draft = &mut self.drafts[self.current];
            draft.title = self.title.read(cx).value().to_string();
            draft.summary = self.summary.read(cx).value().to_string();
            draft.body = self.body.read(cx).value().to_string();
            self.feedback.clear();
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
        self.feedback.clear();
        cx.notify();
    }
}
impl Render for Cms {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let current = &self.drafts[self.current];
        let dirty = Some(current) != self.saved[self.current].as_ref();
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
                let blank = Post::untitled();
                this.drafts.push(blank);
                this.saved.push(None);
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
                            palette::well(p)
                                .w_full()
                                .p_2()
                                .min_h(px(108.))
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
                        this.feedback.clear();
                        cx.notify();
                    })),
                );
        }
        let save = layout::primary(p, "save-post", "Save changes")
            .disabled(!dirty || current.title.trim().is_empty())
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(save) = &mut this.on_save
                    && let Err(message) = save(&this.drafts[this.current])
                {
                    this.feedback = message;
                    cx.notify();
                    return;
                }
                this.saved[this.current] = Some(this.drafts[this.current].clone());
                this.feedback = "Saved.".into();
                cx.notify();
            }));
        let discard = layout::framed(p, "discard-post", "Restore saved", false)
            .disabled(!dirty)
            .on_click(cx.listener(|this, _, w, cx| {
                let i = this.current;
                if let Some(saved) = this.saved[i].clone() {
                    this.drafts[i] = saved;
                    this.select(i, w, cx);
                    this.feedback = "Restored the saved revision.".into();
                } else {
                    this.drafts.remove(i);
                    this.saved.remove(i);
                    if this.drafts.is_empty() {
                        this.drafts.push(Post::untitled());
                        this.saved.push(None);
                    }
                    this.select(i.saturating_sub(1).min(this.drafts.len() - 1), w, cx);
                    this.feedback = "Discarded the unsaved post.".into();
                }
            }));
        editor = editor.child(row().child(save).child(discard)).child(
            (if self.feedback.is_empty() {
                if dirty {
                    "Unsaved changes"
                } else {
                    "Saved revision"
                }
            } else {
                &self.feedback
            })
            .to_owned(),
        );
        screen(
            "editorial-workspace",
            p,
            column()
                .child(banner(
                    p,
                    "Edit drafts, preview posts and save your changes.",
                    "Posts",
                ))
                .child(archive)
                .child(editor),
        )
    }
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| Cms::new(palette::air(), posts(), window, cx))
        .into()
}
