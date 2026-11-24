//! Editable account settings with a separate saved profile and live preview.
mod button;
mod input;
mod layout;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{
    Field, InputEditorStyle, InputState, Switch, Textarea, TextareaState, input::InputEvent,
};
use input::{editor_with_palette, field_with_palette};
use layout::{banner, column, heading, note, panel, row, screen};

#[derive(Clone, PartialEq)]
pub(crate) struct Profile {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) bio: String,
    pub(crate) notifications: bool,
    pub(crate) weekly_digest: bool,
}
pub(crate) struct Settings {
    palette: Palette,
    saved: Profile,
    name: Entity<InputState>,
    email: Entity<InputState>,
    bio: Entity<TextareaState>,
    notifications: bool,
    weekly_digest: bool,
    attempted: bool,
    feedback: String,
    on_save: Option<Box<dyn FnMut(&Profile) -> Result<(), String>>>,
}
impl Settings {
    pub(crate) fn new(
        p: Palette,
        profile: Profile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name = cx.new(|cx| {
            editor_with_palette(window, cx, "Your name", Role::TextInput, p)
                .default_value(profile.name.clone())
        });
        let email = cx.new(|cx| {
            editor_with_palette(window, cx, "name@example.com", Role::EmailInput, p)
                .default_value(profile.email.clone())
        });
        let bio = cx.new(|cx| {
            let mut state = TextareaState::new(window, cx)
                .rows(3)
                .default_value(profile.bio.clone());
            state.set_editor_style(InputEditorStyle {
                foreground: p.foreground,
                muted_foreground: p.muted,
                caret: p.foreground,
                selection: p.highlight,
                ..Default::default()
            });
            state
        });
        for field in [&name, &email] {
            cx.subscribe(field, |this: &mut Self, _, e, cx| this.edited(e, cx))
                .detach();
        }
        cx.subscribe(&bio, |this: &mut Self, _, e, cx| this.edited(e, cx))
            .detach();
        Self {
            palette: p,
            notifications: profile.notifications,
            weekly_digest: profile.weekly_digest,
            saved: profile,
            name,
            email,
            bio,
            attempted: false,
            feedback: String::new(),
            on_save: None,
        }
    }
    pub(crate) fn on_save(
        mut self,
        save: impl FnMut(&Profile) -> Result<(), String> + 'static,
    ) -> Self {
        self.on_save = Some(Box::new(save));
        self
    }
    fn edited(&mut self, event: &InputEvent, cx: &mut Context<Self>) {
        if matches!(event, InputEvent::Change) {
            self.feedback.clear();
        }
        cx.notify();
    }
    fn draft(&self, cx: &App) -> Profile {
        Profile {
            name: self.name.read(cx).value().to_string(),
            email: self.email.read(cx).value().to_string(),
            bio: self.bio.read(cx).value().to_string(),
            notifications: self.notifications,
            weekly_digest: self.weekly_digest,
        }
    }
    fn restore(&mut self, w: &mut Window, cx: &mut Context<Self>) {
        self.name
            .update(cx, |s, c| s.set_value(self.saved.name.clone(), w, c));
        self.email
            .update(cx, |s, c| s.set_value(self.saved.email.clone(), w, c));
        self.bio
            .update(cx, |s, c| s.set_value(self.saved.bio.clone(), w, c));
        self.notifications = self.saved.notifications;
        self.weekly_digest = self.saved.weekly_digest;
        self.attempted = false;
        self.feedback.clear();
        cx.notify();
    }
}
fn errors(p: &Profile) -> [Option<&'static str>; 3] {
    let mail = p.email.trim();
    let valid = mail.split_once('@').is_some_and(|(user, host)| {
        !user.is_empty()
            && !host.contains('@')
            && host
                .split_once('.')
                .is_some_and(|(a, b)| !a.is_empty() && !b.is_empty())
    }) && !mail.chars().any(char::is_whitespace);
    [
        (p.name.trim().is_empty()).then_some("Enter a display name."),
        (!valid).then_some("Enter a valid email address."),
        (p.bio.chars().count() > 240).then_some("Keep the introduction to 240 characters."),
    ]
}
fn toggle(id: &'static str, label: &'static str, value: bool, p: Palette) -> Switch {
    Switch::new(id)
        .accessibility_label(label)
        .checked(value)
        .flex()
        .items_center()
        .gap_3()
        .focus_visible(|s| s.underline())
        .child(
            palette::well(p)
                .flex()
                .items_center()
                .w(px(44.))
                .h(px(26.))
                .rounded_full()
                .p(px(3.))
                .when(value, |s| s.bg(p.accent).justify_end())
                .child(div().size(px(20.)).rounded_full().bg(p.surface)),
        )
        .child(label)
}
impl Render for Settings {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let draft = self.draft(cx);
        let dirty = draft != self.saved;
        let [name_error, email_error, bio_error] = if self.attempted {
            errors(&draft)
        } else {
            [None; 3]
        };
        let form = panel(p)
            .child(heading("Public profile"))
            .child(field_with_palette(
                "profile-name",
                "Display name",
                &self.name,
                name_error,
                w,
                cx,
                p,
            ))
            .child(field_with_palette(
                "profile-email",
                "Email address",
                &self.email,
                email_error,
                w,
                cx,
                p,
            ))
            .child(
                Field::new("profile-bio", &self.bio, "A little about you")
                    .error(bio_error.map(Into::into))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(palette::well(p).p_3().child(Textarea::new(&self.bio))),
            )
            .child(note(p, format!("{} / 240", draft.bio.chars().count())));
        let updates = cx.entity();
        let digest = cx.entity();
        let notifications = panel(p)
            .child(heading("Notifications"))
            .child(
                toggle("updates", "Project updates", self.notifications, p).on_change(
                    move |value, _, _, cx| {
                        updates.update(cx, |this, cx| {
                            this.notifications = value;
                            this.feedback.clear();
                            cx.notify();
                        })
                    },
                ),
            )
            .child(
                toggle("digest", "Weekly digest", self.weekly_digest, p).on_change(
                    move |value, _, _, cx| {
                        digest.update(cx, |this, cx| {
                            this.weekly_digest = value;
                            this.feedback.clear();
                            cx.notify();
                        })
                    },
                ),
            );
        let preview = panel(p)
            .child(note(p, "Live preview"))
            .child(heading(draft.name.clone()))
            .child(note(p, draft.email.clone()))
            .child(draft.bio.clone());
        let commit = layout::primary(p, "save-profile", "Save changes")
            .disabled(!dirty)
            .on_click(cx.listener(|this, _, _, cx| {
                this.attempted = true;
                let next = this.draft(cx);
                if errors(&next).iter().all(Option::is_none) {
                    match this.on_save.as_mut().map_or(Ok(()), |save| save(&next)) {
                        Ok(()) => {
                            this.saved = next;
                            this.feedback = "Profile updated.".into();
                            this.attempted = false;
                        }
                        Err(message) => this.feedback = message,
                    }
                }
                cx.notify();
            }));
        let discard = layout::framed(p, "discard-profile", "Discard changes", false)
            .disabled(!dirty)
            .on_click(cx.listener(|this, _, w, cx| this.restore(w, cx)));
        let wide = w.viewport_size().width >= px(700.);
        let columns = div()
            .flex()
            .w_full()
            .gap_5()
            .flex_col()
            .when(wide, |s| s.flex_row())
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_5()
                    .child(form)
                    .child(notifications),
            )
            .child(div().w_full().when(wide, |s| s.w(px(235.))).child(preview));
        let status = if self.feedback.is_empty() {
            if dirty {
                "Unsaved changes"
            } else {
                "All changes saved"
            }
            .to_owned()
        } else {
            self.feedback.clone()
        };
        screen(
            "settings",
            p,
            column()
                .child(banner(p, "Workspace / Settings", "Account settings"))
                .child(columns)
                .child(row().child(commit).child(discard).child(note(p, status))),
        )
    }
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    let profile = Profile {
        name: "Alex Kim".into(),
        email: "alex@example.com".into(),
        bio: "Front-end developer working on forms and data tables.".into(),
        notifications: true,
        weekly_digest: false,
    };
    cx.new(|cx| Settings::new(palette::air(), profile, w, cx))
        .into()
}
