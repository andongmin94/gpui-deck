//! Relay product page with feature navigation, monthly/yearly pricing and plan selection.
mod button;
mod data;
mod layout;
mod palette;
use super::Palette;
use data::{FAQS, FEATURES};
use gpui::{
    AnyView, App, AppContext as _, Context, IntoElement, ParentElement as _, Render, Styled as _,
    Window, div,
};
use layout::{action, banner, choice, column, heading, panel, row, screen};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Section {
    Overview,
    Pricing,
    Questions,
}
pub(crate) struct Landing {
    palette: Palette,
    section: Section,
    yearly: bool,
    selected: Option<&'static str>,
    question: Option<usize>,
}
impl Landing {
    pub(crate) fn new(palette: Palette) -> Self {
        Self {
            palette,
            section: Section::Overview,
            yearly: false,
            selected: None,
            question: None,
        }
    }
}
impl Render for Landing {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let mut nav = row();
        for (title, section) in [
            ("How it works", Section::Overview),
            ("Pricing", Section::Pricing),
            ("Questions", Section::Questions),
        ] {
            nav = nav.child(choice(p, title, title, self.section == section).on_click(
                cx.listener(move |this, _, _, cx| {
                    this.section = section;
                    cx.notify();
                }),
            ));
        }
        let mut body = column()
            .child(
                banner(
                    p,
                    "Relay  /  Async team updates",
                    "A clearer team check-in.",
                )
                .child("Share progress, blockers, and next steps in your own time."),
            )
            .child(nav);
        match self.section {
            Section::Overview => {
                let checkin = panel(p)
                    .child(heading("Today's team update"))
                    .child("What moved forward?")
                    .child("Finished the onboarding prototype and handed it to the design team.")
                    .child("What’s next?")
                    .child("Test the new flow with three teammates on Wednesday.");
                body = body.child(checkin);
                for &(title, detail) in FEATURES {
                    body = body.child(panel(p).child(heading(title)).child(detail));
                }
                body = body.child(
                    action(p, "view-plans", "Explore plans").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.section = Section::Pricing;
                            cx.notify();
                        },
                    )),
                );
            }
            Section::Pricing => {
                let mut cadence = row();
                for (title, yearly) in [("Monthly", false), ("Yearly", true)] {
                    cadence =
                        cadence.child(choice(p, title, title, self.yearly == yearly).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.yearly = yearly;
                                cx.notify();
                            }),
                        ));
                }
                body = body
                    .child(heading("Simple plans. Clear pricing."))
                    .child(cadence);
                let team = if self.yearly {
                    "$10 per person/month · $120 billed yearly"
                } else {
                    "$12 per person/month · billed monthly"
                };
                for (name, price, detail) in [
                    (
                        "Starter",
                        "Free",
                        "For up to five people getting into a rhythm.",
                    ),
                    (
                        "Team",
                        team,
                        "Unlimited members, shared spaces, and an extended update history.",
                    ),
                ] {
                    let selected = self.selected == Some(name);
                    body = body.child(
                        panel(p)
                            .child(heading(name))
                            .child(div().text_xl().child(price))
                            .child(detail)
                            .child(
                                action(
                                    p,
                                    name,
                                    if selected {
                                        format!("{name} selected")
                                    } else {
                                        format!("Choose {name}")
                                    },
                                )
                                .on_click(cx.listener(
                                    move |this, _, _, cx| {
                                        this.selected = Some(name);
                                        cx.notify();
                                    },
                                )),
                            ),
                    );
                }
                if let Some(name) = self.selected {
                    body = body.child(format!(
                        "Selected {name} · {}",
                        if name == "Starter" { "Free" } else { team }
                    ));
                }
            }
            Section::Questions => {
                for (i, &(question, answer)) in FAQS.iter().enumerate() {
                    let expanded = self.question == Some(i);
                    let mut faq = panel(p).child(
                        layout::disclosure(p, ("faq", i), question, expanded).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.question = if expanded { None } else { Some(i) };
                                cx.notify();
                            }),
                        ),
                    );
                    if expanded {
                        faq = faq.child(answer);
                    }
                    body = body.child(faq);
                }
            }
        }
        screen("relay-landing", p, body)
    }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| Landing::new(palette::yellow())).into()
}
