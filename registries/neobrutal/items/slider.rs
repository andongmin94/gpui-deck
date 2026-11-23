//! slider in the application's Neobrutal palette.

#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Slider, SliderState, SliderThumb};
pub(crate) fn slider(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<SliderState>,
    p: Palette,
    cx: &App,
) -> Slider {
    let label = label.into();
    let model = state.read(cx);
    let count = model.values().len();
    let (start, end) = if count > 1 {
        (model.percentage(0), model.percentage(count - 1))
    } else {
        (0., model.percentage(0))
    };
    Slider::new(id, state)
        .accessibility_label(label.clone())
        .relative()
        .h(px(28.))
        .w_full()
        .child(
            div()
                .absolute()
                .left_0()
                .right_0()
                .top(px(9.))
                .h(px(10.))
                .rounded_full()
                .border_2()
                .border_color(p.border)
                .bg(p.field),
        )
        .child(
            div()
                .absolute()
                .left(relative(start))
                .w(relative(end - start))
                .top(px(11.))
                .h(px(6.))
                .bg(p.accent),
        )
        .children((0..count).map(|i| {
            SliderThumb::new(state, i)
                .accessibility_label(label.clone())
                .absolute()
                .left(relative(model.percentage(i)))
                .ml(px(-10.))
                .top(px(4.))
                .size(px(20.))
                .rounded_full()
                .border_2()
                .border_color(p.border)
                .bg(p.accent)
                .focus_visible(|s| s.bg(p.field))
        }))
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| SliderState::new(0.0..=100., 1., [65.], cx));
    control_style::sample(s, cx, |p, s, _, cx| {
        div()
            .w_full()
            .max_w(px(250.))
            .flex()
            .flex_col()
            .gap_2()
            .child(format!("Volume {:.0}%", s.read(cx).values()[0]))
            .child(slider("volume", "Volume", s, p, cx))
    })
}
