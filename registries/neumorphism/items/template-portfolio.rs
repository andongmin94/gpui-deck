//! A design practice portfolio with expandable cases and a contact route.
mod button;
mod layout;
mod palette;
mod portfolio_data;
use super::Palette;
use gpui::{
    AnyView, App, AppContext as _, Context, IntoElement, ParentElement as _, Render, Styled as _,
    Window, div,
};
use layout::{action, banner, choice, column, heading, panel, row, screen};
use portfolio_data::CASES;

pub(crate) struct Portfolio {
    palette: Palette,
    expanded: Option<usize>,
    contact: bool,
}
impl Portfolio {
    pub(crate) fn new(palette: Palette) -> Self {
        Self {
            palette,
            expanded: None,
            contact: false,
        }
    }
}
impl Render for Portfolio {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let mut nav = row();
        for (flag, title) in [(false, "Selected work"), (true, "Contact")] {
            nav = nav.child(
                choice(p, title, title, self.contact == flag).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.contact = flag;
                        cx.notify();
                    },
                )),
            );
        }
        let mut body = column()
            .child(
                banner(p, "Independent practice  /  Seoul", "Sora Han")
                    .child("Product designer and front-end developer."),
            )
            .child(nav);
        if self.contact {
            body=body.child(panel(p).child(heading("Have a complex workflow to simplify?"))
                .child("Send a short introduction with the project context, timeline, and the problem you are working on.")
                .child(action(p,"inquiry","Email hello@example.com").on_click(|_,_,cx|cx.open_url("mailto:hello@example.com"))));
        } else {
            body = body
                .child(panel(p).child(heading("About")).child(
                    "I design scheduling, reporting and inventory tools. My work covers the screen layout, interaction prototype and React implementation.",
                ))
                .child(heading("Selected work"));
            for (i, case) in CASES.iter().enumerate() {
                let expanded = self.expanded == Some(i);
                let mut card = panel(p)
                    .child(
                        div()
                            .text_sm()
                            .child(format!("{}   /   {}", case.year, case.discipline)),
                    )
                    .child(heading(case.title))
                    .child(case.summary)
                    .child(
                        layout::disclosure(
                            p,
                            ("case", i),
                            if expanded {
                                "Close case study"
                            } else {
                                "Read case study"
                            },
                            expanded,
                        )
                        .accessibility_label(format!("{} case study", case.title))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.expanded = if expanded { None } else { Some(i) };
                            cx.notify();
                        })),
                    );
                if expanded {
                    for (title, detail) in [
                        ("Challenge", case.challenge),
                        ("Approach", case.approach),
                        ("Deliverables", case.deliverables),
                        ("Outcome", case.outcome),
                    ] {
                        card = card.child(heading(title)).child(detail);
                    }
                }
                body = body.child(card);
            }
        }
        screen("portfolio", p, body)
    }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| Portfolio::new(palette::air())).into()
}
