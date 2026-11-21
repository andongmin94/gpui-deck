use std::sync::{Arc, LazyLock};

use gpui::{
    AnyView, App, AppContext as _, Context, Div, Image, ImageFormat, InteractiveElement as _,
    IntoElement, ObjectFit, ParentElement as _, Render, Role, StatefulInteractiveElement as _,
    Styled as _, StyledImage as _, Window, div, img, px, relative,
};
use gpui_deck::Button;

use super::frame;
#[path = "../../../base/examples/recipes/bubble.rs"]
#[allow(dead_code)] // This two-message preview only uses the basic incoming/outgoing surfaces.
mod bubble;
use crate::{
    recipes::{
        Alert, AspectRatio, Attachment, Avatar, AvatarGroup, Badge, ButtonGroup, EmptyState,
        Heading, InlineCode, Item, Kbd, KbdGroup, Marker, Message, Paragraph, Skeleton, Spinner,
    },
    theme::Theme,
};
use bubble::{Bubble, BubbleSide};

pub(super) const NAMES: &[&str] = &[
    "alert",
    "aspect-ratio",
    "attachment",
    "avatar",
    "badge",
    "bubble",
    "button-group",
    "empty",
    "item",
    "kbd",
    "marker",
    "message",
    "skeleton",
    "spinner",
    "typography",
];

pub(super) fn open(name: &str, theme: Theme, _: &mut Window, cx: &mut App) -> Option<AnyView> {
    let name = NAMES
        .iter()
        .copied()
        .find(|&registered| registered == name)?;
    Some(
        cx.new(|_| DisplayDemo {
            name,
            theme,
            copies: 1,
        })
        .into(),
    )
}

struct DisplayDemo {
    name: &'static str,
    theme: Theme,
    copies: u8,
}

fn column() -> Div {
    div()
        .w_full()
        .min_w_0()
        .max_w(px(352.))
        .flex()
        .flex_col()
        .gap_2()
}

static RECORDING_ART: LazyLock<Arc<Image>> = LazyLock::new(|| {
    Arc::new(Image::from_bytes(
        ImageFormat::Svg,
        include_bytes!("../../../base/examples/assets/preset-avatar.svg").to_vec(),
    ))
});

impl Render for DisplayDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let palette = theme.recipe_palette();
        let view = frame(theme);
        match self.name {
            "alert" => view.child(column().child(Alert::new(
                "sync-alert",
                "Offline mode",
                "Your changes will sync when you reconnect.",
                palette,
            ))),
            "aspect-ratio" => view.child(
                column().max_w(px(288.)).child(
                    AspectRatio::new("recording-frame", 16. / 9.)
                        .rounded(palette.radius)
                        .child(
                            img(Arc::clone(&RECORDING_ART))
                                .id("recording-art")
                                .role(Role::Image)
                                .aria_label("Recording artwork")
                                .size_full()
                                .object_fit(ObjectFit::Cover),
                        ),
                ),
            ),
            "attachment" => view.child(column().child(Attachment::new(
                "attachment",
                "session-notes.md",
                "text/markdown",
                "1.8 KB",
                palette,
            ))),
            "avatar" => view
                .child(
                    Avatar::new("studio", "Recording studio", "RS", palette)
                        .image(Arc::clone(&RECORDING_ART)),
                )
                .child(
                    AvatarGroup::new("collaborators", "Collaborators", palette)
                        .max_visible(2)
                        .avatar(Avatar::new("ada", "Ada Kim", "AK", palette))
                        .avatar(Avatar::new("maya", "Maya Chen", "MC", palette))
                        .avatar(Avatar::new("sam", "Sam Lee", "SL", palette)),
                ),
            "badge" => view
                .child(Badge::new("draft", "Draft", palette))
                .child(Badge::new("published", "Published", palette).accent(true)),
            "bubble" => view.child(
                column()
                    .child(
                        Bubble::new(
                            "question",
                            "Maya",
                            "10:24",
                            "Ready to record?",
                            BubbleSide::Incoming,
                        )
                        .colors(
                            theme.muted,
                            theme.foreground,
                            theme.muted_foreground,
                        ),
                    )
                    .child(
                        Bubble::new(
                            "reply",
                            "You",
                            "10:25",
                            "The microphones are ready.",
                            BubbleSide::Outgoing,
                        )
                        .colors(
                            theme.primary,
                            theme.primary_foreground,
                            theme.primary_foreground,
                        ),
                    ),
            ),
            "button-group" => view.child(
                ButtonGroup::new("copies", "Number of copies", palette)
                    .button(
                        Button::new("fewer")
                            .accessibility_label("Remove one copy")
                            .child("−")
                            .disabled(self.copies == 1)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.copies -= 1;
                                cx.notify();
                            })),
                    )
                    .text(if self.copies == 1 {
                        "1 copy".to_owned()
                    } else {
                        format!("{} copies", self.copies)
                    })
                    .button(
                        Button::new("more")
                            .accessibility_label("Add one copy")
                            .child("+")
                            .disabled(self.copies == 10)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.copies += 1;
                                cx.notify();
                            })),
                    ),
            ),
            "empty" => view.child(column().child(EmptyState::new(
                "no-recordings",
                "No recordings yet",
                "Completed recordings will appear here.",
                palette,
            ))),
            "item" => view.child(
                column().child(
                    Item::new("reviewer", "Maya Chen", palette)
                        .description("Reviews the weekly recording.")
                        .media(Avatar::new("reviewer-avatar", "Maya Chen", "MC", palette)),
                ),
            ),
            "kbd" => view.child(
                KbdGroup::new("shortcut", palette)
                    .key(Kbd::new("shift", "Shift", palette))
                    .key(Kbd::new("enter", "Enter", palette)),
            ),
            "marker" => view.child(column().child(Marker::new("unread", "New messages", palette))),
            "message" => view.child(column().child(Message::new(
                "note",
                "Maya Chen",
                "10:24",
                "I added the updated track list to the session notes.",
                palette,
            ))),
            "skeleton" => view
                .child(
                    Skeleton::new("portrait", palette)
                        .size(px(40.))
                        .rounded_full(),
                )
                .child(
                    column()
                        .max_w(px(160.))
                        .child(Skeleton::new("name", palette).w(relative(0.7)))
                        .child(Skeleton::new("description", palette)),
                ),
            "spinner" => view
                .child(Spinner::new("loading", "Loading recordings", palette).size(px(20.)))
                .child("Loading recordings"),
            "typography" => view.child(
                column()
                    .items_start()
                    .child(Heading::new("heading", "Session archive", 2, palette))
                    .child(
                        Paragraph::new("description", palette)
                            .text("Store each recording in ")
                            .code("sessions/")
                            .text(" with its notes."),
                    )
                    .child(InlineCode::new("path", "sessions/weekly/notes.md", palette)),
            ),
            _ => unreachable!("all registered recipes have a preview"),
        }
    }
}
