mod palette;
mod style;

use super::Palette;
use gpui::{Animation, AnimationExt as _, ElementId, SharedString, div, px};
use gpui::{AnyView, App, Window, prelude::*};
use std::time::Duration;

/// Uniform tile widths keep the second strip aligned at the loop boundary.
pub(crate) fn marquee(
    id: impl Into<ElementId>,
    labels: &[SharedString],
    tile_px: f32,
    moving: bool,
    p: Palette,
) -> impl IntoElement {
    let id = id.into();
    let period = tile_px * labels.len() as f32;
    let strip = div().relative().flex().flex_none().items_center().children(
        labels.iter().cycle().take(labels.len() * 2).map(|label| {
            div()
                .w(px(tile_px))
                .flex_none()
                .px_2()
                .truncate()
                .child(label.clone())
        }),
    );
    div()
        .id(id.clone())
        .role(gpui::Role::Group)
        .aria_label(
            labels
                .iter()
                .map(|label| label.as_ref())
                .collect::<Vec<_>>()
                .join(", "),
        )
        .bg(p.surface)
        .shadow(palette::inset(p))
        .text_color(p.foreground)
        .border_1()
        .border_color(p.border)
        .rounded(p.radius)
        .w_full()
        .max_w(px(period))
        .overflow_hidden()
        .py_2()
        .child(if moving && !labels.is_empty() {
            strip
                .with_animation(
                    (id, "slide"),
                    Animation::new(Duration::from_secs_f32((period / 40.).max(1.))).repeat(),
                    move |s, t| s.left(px(-period * t)),
                )
                .into_any_element()
        } else {
            div()
                .flex()
                .flex_wrap()
                .gap_3()
                .px_2()
                .children(labels.iter().cloned())
                .into_any_element()
        })
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Demo {
        paused: bool,
    }
    impl Render for Demo {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            let labels = ["Accessibility", "GPUI Deck", "Rust", "Pretendard"].map(Into::into);
            let moving = !self.paused && !cx.reduce_motion();
            style::stage(
                p,
                div()
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap_3()
                    .child(marquee("technologies", &labels, 130., moving, p))
                    .when(!cx.reduce_motion(), |s| {
                        s.child(
                            style::action(
                                "motion",
                                if moving {
                                    "Pause motion"
                                } else {
                                    "Resume motion"
                                },
                                p,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.paused = !this.paused;
                                cx.notify();
                            })),
                        )
                    }),
            )
        }
    }
    cx.new(|_| Demo { paused: false }).into()
}
