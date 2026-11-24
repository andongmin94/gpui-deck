mod palette;
mod style;

use super::Palette;
use super::recipes::Skeleton as BaseSkeleton;
use gpui::{Animation, AnimationExt as _, ElementId, StyleRefinement, div, px, relative};
use gpui::{AnyView, App, Window, prelude::*};
use std::time::Duration;

#[derive(IntoElement)]
pub(crate) struct Skeleton {
    id: ElementId,
    base: BaseSkeleton,
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if cx.reduce_motion() {
            self.base.into_any_element()
        } else {
            self.base
                .with_animation(
                    (self.id, "pulse"),
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |base, t| base.opacity(0.55 + 0.45 * (std::f32::consts::PI * t).cos().powi(2)),
                )
                .into_any_element()
        }
    }
}

pub(crate) fn skeleton(id: impl Into<ElementId>, p: Palette) -> Skeleton {
    let id = id.into();
    let base = BaseSkeleton::new(id.clone(), p)
        .opacity(1.)
        .bg(p.field)
        .shadow(palette::inset(p))
        .border_1()
        .border_color(p.border);
    Skeleton { id, base }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        style::surface(p)
            .id("loading")
            .role(gpui::Role::Status)
            .aria_label("Loading content")
            .w_full()
            .max_w(px(520.))
            .mx_auto()
            .p_0()
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(skeleton("cover", p).w_full().h(px(150.)))
            .child(
                div()
                    .p_5()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(skeleton("avatar", p).size(px(48.)).rounded_full())
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(skeleton("title", p).h(px(14.)).w(relative(0.58)))
                                    .child(skeleton("subtitle", p).h(px(10.)).w_full()),
                            ),
                    )
                    .child(skeleton("first-line", p).h(px(10.)).w_full())
                    .child(skeleton("second-line", p).h(px(10.)).w(relative(0.72))),
            )
            .into_any_element()
    })
}
