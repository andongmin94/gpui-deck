//! Creator profile with category navigation, external destinations and clipboard contact.
mod button;
mod layout;
mod link_hub_data;
mod palette;
use super::Palette;
use gpui::{
    AnyView, App, AppContext as _, ClipboardItem, Context, IntoElement, ParentElement as _, Render,
    Styled as _, Window, div,
};
use layout::{action, banner, choice, column, panel, row, screen};
use link_hub_data::LINKS;

pub(crate) struct LinkHub {
    palette: Palette,
    category: &'static str,
    copied: bool,
}
impl LinkHub {
    pub(crate) fn new(palette: Palette) -> Self {
        Self {
            palette,
            category: "All",
            copied: false,
        }
    }
}
impl Render for LinkHub {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let mut groups = row();
        for category in ["All", "Guides", "Code", "Contact"] {
            groups = groups.child(
                choice(p, category, category, self.category == category).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.category = category;
                        cx.notify();
                    },
                )),
            );
        }
        let mut body = column()
            .max_w(gpui::px(540.))
            .child(
                banner(p, "Links", "Neumorphism UI")
                    .child("Documentation, component examples and source code."),
            )
            .child(groups);
        for (i, link) in LINKS
            .iter()
            .enumerate()
            .filter(|(_, link)| self.category == "All" || self.category == link.group)
        {
            body = body.child(
                panel(p)
                    .child(
                        div()
                            .text_sm()
                            .child(format!("{}   /   {}", link.group, link.detail)),
                    )
                    .child(
                        action(p, ("destination", i), link.title)
                            .on_click(move |_, _, cx| cx.open_url(link.href)),
                    ),
            );
        }
        body = body.child(
            panel(p).child("Get in touch").child(
                action(
                    p,
                    "copy-contact",
                    if self.copied {
                        "Copied hello@example.com"
                    } else {
                        "Copy email address"
                    },
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string("hello@example.com".into()));
                    this.copied = true;
                    cx.notify();
                })),
            ),
        );
        screen("link-hub", p, body)
    }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| LinkHub::new(palette::air())).into()
}
