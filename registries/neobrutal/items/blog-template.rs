//! Editorial archive with a searchable topic index and full-length reading pages.
mod button;
mod data;
mod input;
mod layout;
mod palette;

use super::Palette;
use data::ARTICLES;
use gpui::{
    AnyView, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Role,
    Styled as _, Window, div,
};
use gpui_deck::{InputState, input::InputEvent};
use input::{editor_with_palette, field_with_palette};
use layout::{action, banner, choice, column, heading, panel, row, screen};

pub(crate) struct Blog {
    palette: Palette,
    query: Entity<InputState>,
    topic: Option<&'static str>,
    newest: bool,
    reading: Option<usize>,
}

impl Blog {
    pub(crate) fn new(p: Palette, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let query = cx.new(|cx| {
            editor_with_palette(window, cx, "Title, topic or keyword", Role::SearchInput, p)
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
            query,
            topic: None,
            newest: true,
            reading: None,
        }
    }
}

impl Render for Blog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        if let Some(index) = self.reading {
            let article = &ARTICLES[index];
            let mut story = panel(p).child(heading(article.intro));
            for (title, paragraphs) in article.sections {
                story = story.child(heading(*title));
                for text in *paragraphs {
                    story = story.child(*text);
                }
            }
            let body = column()
                .child(
                    action(p, "archive", "Back to archive").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.reading = None;
                            cx.notify();
                        },
                    )),
                )
                .child(banner(
                    p,
                    &format!(
                        "{}  /  {}  /  {}",
                        article.topic, article.date, article.duration
                    ),
                    article.title,
                ))
                .child(story);
            return screen("reading-page", p, body).into_any_element();
        }
        let mut topics = row();
        let mut categories = vec![None];
        for article in ARTICLES {
            if !categories.contains(&Some(article.topic)) {
                categories.push(Some(article.topic));
            }
        }
        for (i, topic) in categories.into_iter().enumerate() {
            topics = topics.child(
                choice(
                    p,
                    ("topic", i),
                    topic.unwrap_or("All topics"),
                    self.topic == topic,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.topic = topic;
                    cx.notify();
                })),
            );
        }
        let query = self.query.read(cx).value().trim().to_lowercase();
        let mut matches: Vec<_> = ARTICLES
            .iter()
            .enumerate()
            .filter(|(_, a)| {
                self.topic.is_none_or(|topic| topic == a.topic)
                    && format!("{} {} {}", a.title, a.topic, a.summary)
                        .to_lowercase()
                        .contains(&query)
            })
            .collect();
        matches.sort_by_key(|(_, a)| a.date);
        if self.newest {
            matches.reverse();
        }
        let mut body = column()
            .child(banner(p, "Studio journal", "Field notes"))
            .child(
                panel(p)
                    .child(field_with_palette(
                        "search-posts",
                        "Search articles",
                        &self.query,
                        None,
                        window,
                        cx,
                        p,
                    ))
                    .child(topics)
                    .child(
                        layout::framed(
                            p,
                            "sort",
                            if self.newest {
                                "Newest first"
                            } else {
                                "Oldest first"
                            },
                            false,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.newest = !this.newest;
                            cx.notify();
                        })),
                    ),
            )
            .child(div().text_sm().child(format!("{} articles", matches.len())));
        if matches.is_empty() {
            body = body.child(panel(p).child("No articles match this search."));
        }
        for (i, a) in matches {
            body = body.child(
                panel(p)
                    .child(
                        div()
                            .text_sm()
                            .child(format!("{}  /  {}  /  {}", a.topic, a.date, a.duration)),
                    )
                    .child(heading(a.title))
                    .child(a.summary)
                    .child(
                        action(p, ("read", i), "Read article")
                            .accessibility_label(format!("Read {}", a.title))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.reading = Some(i);
                                cx.notify();
                            })),
                    ),
            );
        }
        screen("blog-archive", p, body).into_any_element()
    }
}

pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| Blog::new(palette::yellow(), window, cx)).into()
}
