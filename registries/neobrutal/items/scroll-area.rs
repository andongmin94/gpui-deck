mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, ScrollHandle, SharedString, div, px};
use gpui_deck::ScrollArea;
pub(crate) fn scroll_area(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    handle: &ScrollHandle,
    p: Palette,
) -> ScrollArea {
    ScrollArea::new(id, handle)
        .accessibility_label(label)
        .w_full()
        .border_2()
        .rounded(p.radius)
        .border_color(p.border)
        .bg(p.surface)
        .content_styles(|s| s.flex().flex_col().gap_2().p_3())
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Archive(ScrollHandle);
    impl Render for Archive {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let p = palette::yellow();
            style::stage(
                p,
                scroll_area("prints", "Archived prints", &self.0, p)
                    .h(px(178.))
                    .children((1..=30).map(|i| {
                        div()
                            .flex_none()
                            .border_b_1()
                            .border_color(p.border)
                            .py_2()
                            .child(format!("Print edition {i:02}"))
                    })),
            )
        }
    }
    cx.new(|_| Archive(ScrollHandle::new())).into()
}
