mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{Axis, ElementId, Entity, div, px};
use gpui_deck::{Resizable, ResizablePanel, ResizableState};

pub(crate) fn split(
    id: impl Into<ElementId>,
    state: &Entity<ResizableState>,
    first: impl IntoElement,
    second: impl IntoElement,
    p: Palette,
) -> Resizable {
    Resizable::new(id, state)
        .size_full()
        .border_1()
        .rounded(p.radius)
        .border_color(p.border)
        .bg(p.surface)
        .shadow(palette::inset(p))
        .overflow_hidden()
        .panel(first)
        .handle(move |_, handle| {
            handle
                .accessibility_label("Resize panels")
                .bg(p.border)
                .child(div().rounded_full().size(px(6.)).bg(p.accent))
        })
        .panel(second)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Demo(Entity<ResizableState>);
    impl Render for Demo {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            let sizes = self.0.read(cx).sizes();
            let pane = |name: &str, fraction: f32| {
                div()
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(format!("{name} · {:.0}%", fraction * 100.))
            };
            style::stage(
                p,
                div().h(px(148.)).w_full().child(split(
                    "panes",
                    &self.0,
                    pane("List", sizes[0]),
                    pane("Details", sizes[1]),
                    p,
                )),
            )
        }
    }
    cx.new(|cx| {
        let state = cx.new(|cx| {
            ResizableState::new(
                Axis::Horizontal,
                [
                    ResizablePanel::new(0.4).limits(0.25..=0.7),
                    ResizablePanel::new(0.6).limits(0.3..=0.75),
                ],
                cx,
            )
        });
        cx.observe(&state, |_, _, cx| cx.notify()).detach();
        Demo(state)
    })
    .into()
}
