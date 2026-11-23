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
        .bg(p.highlight)
        .border_2()
        .border_color(p.border);
    Skeleton { id, base }
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        style::surface(p)
            .flex()
            .items_center()
            .gap_3()
            .child(skeleton("photo", p).size(px(48.)).rounded_full())
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(skeleton("title", p).w(relative(0.55)))
                    .child(skeleton("text", p)),
            )
            .into_any_element()
    })
}
