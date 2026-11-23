mod palette;
mod style;

use super::Palette;
use super::recipes::Pagination;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, SharedString, div};
pub(crate) fn pagination(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    page: usize,
    pages: usize,
    p: Palette,
) -> Pagination {
    Pagination::new(id, label, page, pages, p)
        .bg(p.surface)
        .border_2()
        .rounded(p.radius)
        .border_color(p.border)
        .p_2()
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Pages(usize);
    impl Render for Pages {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::yellow();
            let owner = cx.entity();
            let files = ["Prints 1–8", "Prints 9–16", "Prints 17–24", "Prints 25–32"];
            style::stage(
                p,
                div().flex().flex_col().gap_3().child(files[self.0]).child(
                    pagination("pages", "Print archive pages", self.0, files.len(), p).on_change(
                        move |page, _, cx| {
                            owner.update(cx, |this, cx| {
                                this.0 = page;
                                cx.notify();
                            })
                        },
                    ),
                ),
            )
        }
    }
    cx.new(|_| Pages(0)).into()
}
