mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, Entity, SharedString, div, px};
use gpui_deck::{Carousel, CarouselControl, CarouselSlide, CarouselState};

pub(crate) fn carousel(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<CarouselState>,
    slides: impl IntoIterator<Item = impl IntoElement>,
    p: Palette,
) -> Carousel {
    let id = id.into();
    let mut root = Carousel::new(id.clone(), state, label)
        .w_full()
        .flex()
        .flex_col()
        .gap_3();
    for (i, slide) in slides.into_iter().enumerate() {
        root = root.child(CarouselSlide::new(state, i).w_full().child(slide));
    }
    let controls = div()
        .flex()
        .gap_2()
        .child(
            CarouselControl::previous((id.clone(), "back"), state)
                .border_1()
                .rounded(palette::control_radius(p))
                .shadow(palette::raised(p))
                .border_color(p.border)
                .bg(p.surface)
                .px_3()
                .py_1()
                .child("Previous"),
        )
        .child(
            CarouselControl::next((id, "next"), state)
                .border_1()
                .rounded(palette::control_radius(p))
                .shadow(palette::raised(p))
                .border_color(p.border)
                .bg(p.accent)
                .text_color(p.accent_foreground)
                .px_3()
                .py_1()
                .child("Next"),
        );
    root.child(controls)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    const TITLES: [&str; 3] = ["Dashboard", "Settings", "Analytics"];
    struct Gallery(Entity<CarouselState>);
    impl Render for Gallery {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            style::stage(
                p,
                carousel(
                    "gallery",
                    "Feature tour",
                    &self.0,
                    TITLES.map(|label| {
                        style::surface(p)
                            .h(px(130.))
                            .flex()
                            .justify_center()
                            .items_center()
                            .text_xl()
                            .child(label)
                    }),
                    p,
                ),
            )
        }
    }
    cx.new(|cx| {
        let state = cx.new(|cx| CarouselState::new(TITLES, cx).looping(true));
        cx.observe(&state, |_, _, cx| cx.notify()).detach();
        Gallery(state)
    })
    .into()
}
