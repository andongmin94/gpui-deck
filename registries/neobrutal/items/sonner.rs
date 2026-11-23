mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, Entity, SharedString, div};
use gpui_deck::{Toast, ToastClose, Toaster, ToasterState};
pub(crate) fn notifications(
    id: impl Into<ElementId>,
    queue: &Entity<ToasterState<SharedString>>,
    p: Palette,
) -> Toaster<SharedString> {
    Toaster::new(id, queue, move |key, text: &SharedString, state| {
        Toast::new(key, state)
            .accessibility_label(text.clone())
            .flex()
            .items_center()
            .gap_3()
            .border_2()
            .border_color(p.border)
            .rounded(p.radius)
            .bg(p.surface)
            .text_color(p.foreground)
            .p_3()
            .child(div().flex_1().child(text.clone()))
            .child(
                ToastClose::new((ElementId::from(key), "close"), state).trigger(|b| {
                    b.accessibility_label("Dismiss notification")
                        .child("Dismiss")
                }),
            )
    })
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Notices {
        queue: Entity<ToasterState<SharedString>>,
        count: usize,
    }
    impl Render for Notices {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::yellow();
            style::stage(
                p,
                div()
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap_3()
                    .child(
                        style::action("notify", "Add notification", p).on_click(cx.listener(
                            |this, _, window, cx| {
                                this.count += 1;
                                this.queue.update(cx, |q, cx| {
                                    q.push(
                                        format!("Edition {} saved", this.count).into(),
                                        window,
                                        cx,
                                    )
                                });
                            },
                        )),
                    )
                    .child(notifications("notifications", &self.queue, p)),
            )
        }
    }
    cx.new(|cx| {
        let queue = cx.new(|_| ToasterState::new());
        cx.observe(&queue, |_, _, cx| cx.notify()).detach();
        Notices { queue, count: 0 }
    })
    .into()
}
