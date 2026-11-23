//! Shared Neobrutal surfaces and a host for the live examples.
use gpui::{prelude::*, *};
use gpui_deck::box_shadow;

use super::{Palette, palette::yellow};

pub(super) fn panel(p: Palette) -> Div {
    div()
        .flex()
        .flex_col()
        .rounded(p.radius)
        .border_2()
        .border_color(p.border)
        .bg(p.surface)
        .text_color(p.foreground)
        .shadow(vec![box_shadow(4., 4., 0., 0., p.border)])
}

type Drawing = Box<dyn Fn(&mut Window, &mut App) -> AnyElement>;
struct Example(Drawing);
impl Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        (self.0)(window, cx)
    }
}

// The application normally owns and observes the state. Each standalone example
// uses this small view to retain its state and redraw when the control changes.
pub(super) fn sample<S: 'static, E: IntoElement>(
    state: Entity<S>,
    cx: &mut App,
    draw: impl Fn(Palette, &Entity<S>, &mut Window, &mut App) -> E + 'static,
) -> AnyView {
    cx.new(|cx| {
        cx.observe(&state, |_, _, cx| cx.notify()).detach();
        Example(Box::new(move |window, cx| {
            let p = yellow();
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .p_4()
                .bg(p.canvas)
                .text_color(p.foreground)
                .text_size(p.font_size)
                .child(draw(p, &state, window, cx))
                .into_any_element()
        }))
    })
    .into()
}
