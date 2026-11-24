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
        .shadow(palette::raised(p))
        .border_1()
        .rounded(p.radius)
        .border_color(p.border)
        .p_2()
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Pages(usize);
    impl Render for Pages {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            let owner = cx.entity();
            let files = ["Components 1–12", "Components 13–24", "Components 25–36"];
            style::stage(
                p,
                div().flex().flex_col().gap_3().child(files[self.0]).child(
                    pagination("pages", "Component directory pages", self.0, files.len(), p)
                        .on_change(move |page, _, cx| {
                            owner.update(cx, |this, cx| {
                                this.0 = page;
                                cx.notify();
                            })
                        }),
                ),
            )
        }
    }
    cx.new(|_| Pages(1)).into()
}
